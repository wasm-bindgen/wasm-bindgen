//! JSPI instrumentation for the emscripten target: lifecycle hooks instead of
//! shadow-stack management.
//!
//! Under emscripten the fiber runtime is emscripten's own (`-sJSPI_HOOKS`, or
//! `-sREENTRANT_JSPI` which gives every fiber its own shadow stack), reached
//! through the four hook exports its libjspi provides:
//!
//! ```text
//! __jspi_enter:   [] -> [i64 token]
//! __jspi_exit:    [i64 token, i32 error] -> []
//! __jspi_suspend: [] -> [i64 token]
//! __jspi_resume:  [i64 token, i32 error] -> []
//! ```
//!
//! This pass is the wasm-bindgen counterpart of binaryen's `--jspi-hooks`,
//! which emscripten runs on its own `JSPI_EXPORTS`/`JSPI_IMPORTS` after
//! wasm-bindgen has finished: each `#[wasm_bindgen(jspi)]` export is wrapped
//! with `enter`/`exit` and each `#[wasm_bindgen(suspending)]` import with
//! `suspend`/`resume`. The token returned by the "before" hook is kept in a
//! local (which JSPI preserves across the suspension) and handed to the
//! "after" hook, never interpreted. An exceptional completion delivers the
//! after hook with `error=1` and rethrows the exception unchanged.
//!
//! The `catch` rejection-to-data protocol of `transforms::jspi` is kept: a
//! `catch`-marked suspending import catches the `WebAssembly.JSTag` exception
//! JSPI throws at the resume point for a rejected promise, reports it through
//! `__wbindgen_jspi_set_rejected` and returns the reason as its value. The
//! resume hook sees `error=0` there, since the exception is consumed rather
//! than rethrown.
//!
//! Emscripten defaults to the legacy exception-handling instructions and
//! engines reject modules mixing the two dialects, so the wrappers are emitted
//! in whichever form the module already uses.

use super::jspi::Rejection;
use crate::wit::{NonstandardWitSection, WasmBindgenAux};
use anyhow::{anyhow, bail, Error};
use std::collections::HashMap;
use walrus::ir::{self, LegacyCatch};
use walrus::{
    ExportId, ExportItem, FunctionBuilder, FunctionId, InstrSeqBuilder, Module, RefType, ValType,
};

const HOOK_NAMES: [&str; 4] = [
    "__jspi_enter",
    "__jspi_exit",
    "__jspi_suspend",
    "__jspi_resume",
];

#[derive(Clone, Copy)]
struct Hooks {
    enter: FunctionId,
    exit: FunctionId,
    suspend: FunctionId,
    resume: FunctionId,
}

impl Hooks {
    fn locate(module: &Module) -> Result<Self, Error> {
        let before = (&[][..], &[ValType::I64][..]);
        let after = (&[ValType::I64, ValType::I32][..], &[][..]);
        let mut ids = [None; 4];
        for (i, (name, sig)) in HOOK_NAMES
            .iter()
            .zip([before, after, before, after])
            .enumerate()
        {
            let func = module
                .exports
                .iter()
                .find(|e| e.name == *name)
                .and_then(|e| match e.item {
                    ExportItem::Function(f) => Some(f),
                    _ => None,
                })
                .ok_or_else(|| {
                    anyhow!(
                        "JSPI on the emscripten target requires the `{name}` hook export \
                         provided by emscripten's JSPI runtime; link with `-sJSPI_HOOKS` \
                         (or `-sREENTRANT_JSPI`)"
                    )
                })?;
            let ty = module.types.get(module.funcs.get(func).ty());
            if ty.params() != sig.0 || ty.results() != sig.1 {
                bail!(
                    "emscripten JSPI hook export `{name}` has type {:?} -> {:?}, \
                     expected {:?} -> {:?}",
                    ty.params(),
                    ty.results(),
                    sig.0,
                    sig.1
                );
            }
            ids[i] = Some(func);
        }
        Ok(Hooks {
            enter: ids[0].unwrap(),
            exit: ids[1].unwrap(),
            suspend: ids[2].unwrap(),
            resume: ids[3].unwrap(),
        })
    }
}

pub(super) fn run(
    module: &mut Module,
    aux: &WasmBindgenAux,
    wit: &mut NonstandardWitSection,
    jspi_exports: Vec<ExportId>,
    suspending_imports: Vec<(FunctionId, bool)>,
    legacy_eh: bool,
) -> Result<(), Error> {
    let hooks = Hooks::locate(module)?;

    for export_id in jspi_exports {
        let inner = match module.exports.get(export_id).item {
            ExportItem::Function(f) => f,
            _ => bail!("jspi export is not a function"),
        };
        let wrapper = wrap(module, inner, hooks.enter, hooks.exit, None, legacy_eh);
        let name = module.funcs.get(inner).name.clone();
        module.funcs.get_mut(wrapper).name = name.map(|n| format!("{n} jspi wrapper"));
        module.exports.get_mut(export_id).item = wrapper.into();
    }

    if suspending_imports.is_empty() {
        return Ok(());
    }

    let rejection = if suspending_imports.iter().any(|(_, c)| *c) {
        let set_rejected = aux.jspi_set_rejected.ok_or_else(|| {
            anyhow!(
                "could not locate `__wbindgen_jspi_set_rejected`; it is defined by \
                 the wasm-bindgen runtime"
            )
        })?;
        let js_tag = super::catch_handler::get_or_import_js_tag(module);
        Some(Rejection {
            js_tag,
            set_rejected,
        })
    } else {
        None
    };

    let mut wrappers = HashMap::new();
    for (import, catch) in suspending_imports {
        if wrappers.contains_key(&import) {
            continue;
        }
        let rejection = if catch {
            let ty = module.types.get(module.funcs.get(import).ty());
            if ty.results() != [ValType::Ref(RefType::EXTERNREF)] {
                bail!(
                    "unexpected ABI for a `catch` suspending import: expected \
                     an externref return, found {:?}",
                    ty.results()
                );
            }
            rejection
        } else {
            None
        };
        let wrapper = wrap(
            module,
            import,
            hooks.suspend,
            hooks.resume,
            rejection,
            legacy_eh,
        );
        let name = module.funcs.get(import).name.clone();
        module.funcs.get_mut(wrapper).name = Some(match name {
            Some(n) => format!("{n} suspending wrapper"),
            None => "suspending wrapper".to_string(),
        });
        wrappers.insert(import, wrapper);
    }
    super::jspi::rewrite_calls(module, &wrappers);

    for (_, func, adapter) in wit.implements.iter_mut() {
        if aux.imports_with_suspending.contains(adapter) {
            if let Some(wrapper) = wrappers.get(func) {
                *func = *wrapper;
            }
        }
    }

    Ok(())
}

/// Wrap `target` with the `before`/`after` hook pair.
///
/// Standardized form (the optional `$rejected` arm only for `catch`
/// suspending imports):
///
/// ```wat
/// (func $wrapper (param ...) (result ...)
///     (local $tok i64) (local $exn exnref)
///     call $before
///     local.set $tok
///     block $done (result ...)
///         block $catch_all (result exnref)
///             block $rejected (result externref)
///                 try_table (result ...) (catch $__wbindgen_jstag $rejected)
///                                        (catch_all_ref $catch_all)
///                     local.get <params>...
///                     call $target
///                 end
///                 local.set <results>...
///                 (call $after (local.get $tok) (i32.const 0))
///                 (call $__wbindgen_jspi_set_rejected (i32.const 0))
///                 local.get <results>...
///                 br $done
///             end
///             local.set <result>
///             (call $after (local.get $tok) (i32.const 0))
///             (call $__wbindgen_jspi_set_rejected (i32.const 1))
///             local.get <result>
///             br $done
///         end
///         local.set $exn
///         (call $after (local.get $tok) (i32.const 1))
///         local.get $exn
///         throw_ref
///     end)
/// ```
///
/// Legacy form:
///
/// ```wat
/// (func $wrapper (param ...) (result ...)
///     (local $tok i64)
///     call $before
///     local.set $tok
///     block $done (result ...)
///         try (result ...)
///             local.get <params>...
///             call $target
///         catch $__wbindgen_jstag
///             local.set <result>
///             (call $after (local.get $tok) (i32.const 0))
///             (call $__wbindgen_jspi_set_rejected (i32.const 1))
///             local.get <result>
///             br $done
///         catch_all
///             (call $after (local.get $tok) (i32.const 1))
///             rethrow 0
///         end
///         local.set <results>...
///         (call $after (local.get $tok) (i32.const 0))
///         (call $__wbindgen_jspi_set_rejected (i32.const 0))
///         local.get <results>...
///     end)
/// ```
fn wrap(
    module: &mut Module,
    target: FunctionId,
    before: FunctionId,
    after: FunctionId,
    rejection: Option<Rejection>,
    legacy_eh: bool,
) -> FunctionId {
    let ty = module.types.get(module.funcs.get(target).ty());
    let params = ty.params().to_vec();
    let results = ty.results().to_vec();

    let results_ty: ir::InstrSeqType = match results.len() {
        0 => ir::InstrSeqType::Simple(None),
        1 => ir::InstrSeqType::Simple(Some(results[0])),
        _ => module.types.add(&[], &results).into(),
    };

    let mut builder = FunctionBuilder::new(&mut module.types, &params, &results);
    let param_locals: Vec<_> = params.iter().map(|ty| module.locals.add(*ty)).collect();
    let result_locals: Vec<_> = results.iter().map(|ty| module.locals.add(*ty)).collect();
    let tok = module.locals.add(ValType::I64);

    let call_after = |seq: &mut InstrSeqBuilder, error: i32| {
        seq.local_get(tok).i32_const(error).call(after);
    };
    let stash_results = |seq: &mut InstrSeqBuilder| {
        for local in result_locals.iter().rev() {
            seq.local_set(*local);
        }
    };
    let unstash_results = |seq: &mut InstrSeqBuilder| {
        for local in &result_locals {
            seq.local_get(*local);
        }
    };
    let store_rejected = |seq: &mut InstrSeqBuilder, value: i32| {
        if let Some(rejection) = rejection {
            seq.i32_const(value).call(rejection.set_rejected);
        }
    };

    let done_seq = builder.dangling_instr_seq(results_ty).id();
    let try_seq = builder.dangling_instr_seq(results_ty).id();
    {
        let mut seq = builder.instr_seq(try_seq);
        for local in &param_locals {
            seq.local_get(*local);
        }
        seq.call(target);
    }

    if legacy_eh {
        let mut catches = Vec::new();
        if let Some(rejection) = rejection {
            let handler = builder.dangling_instr_seq(results_ty).id();
            let mut seq = builder.instr_seq(handler);
            stash_results(&mut seq);
            call_after(&mut seq, 0);
            store_rejected(&mut seq, 1);
            unstash_results(&mut seq);
            seq.br(done_seq);
            catches.push(LegacyCatch::Catch {
                tag: rejection.js_tag,
                handler,
            });
        }
        let catch_all = builder.dangling_instr_seq(results_ty).id();
        {
            let mut seq = builder.instr_seq(catch_all);
            call_after(&mut seq, 1);
            seq.instr(ir::Rethrow { relative_depth: 0 });
        }
        catches.push(LegacyCatch::CatchAll { handler: catch_all });

        let mut seq = builder.instr_seq(done_seq);
        seq.instr(ir::Try {
            seq: try_seq,
            catches,
        });
        stash_results(&mut seq);
        call_after(&mut seq, 0);
        store_rejected(&mut seq, 0);
        unstash_results(&mut seq);
    } else {
        let exnref_ty: ir::InstrSeqType = ValType::Ref(RefType::EXNREF).into();
        let externref_ty: ir::InstrSeqType = ValType::Ref(RefType::EXTERNREF).into();
        let exn = module.locals.add(ValType::Ref(RefType::EXNREF));

        let catch_all_seq = builder.dangling_instr_seq(exnref_ty).id();
        let rejected_seq = rejection.map(|_| builder.dangling_instr_seq(externref_ty).id());

        let mut catches = Vec::new();
        if let (Some(rejection), Some(rejected_seq)) = (rejection, rejected_seq) {
            catches.push(ir::TryTableCatch::Catch {
                tag: rejection.js_tag,
                label: rejected_seq,
            });
        }
        catches.push(ir::TryTableCatch::CatchAllRef {
            label: catch_all_seq,
        });
        {
            let mut seq = builder.instr_seq(rejected_seq.unwrap_or(catch_all_seq));
            seq.instr(ir::TryTable {
                seq: try_seq,
                catches,
            });
            stash_results(&mut seq);
            call_after(&mut seq, 0);
            store_rejected(&mut seq, 0);
            unstash_results(&mut seq);
            seq.br(done_seq);
        }
        if let Some(rejected_seq) = rejected_seq {
            let mut seq = builder.instr_seq(catch_all_seq);
            seq.instr(ir::Block { seq: rejected_seq });
            stash_results(&mut seq);
            call_after(&mut seq, 0);
            store_rejected(&mut seq, 1);
            unstash_results(&mut seq);
            seq.br(done_seq);
        }
        {
            let mut seq = builder.instr_seq(done_seq);
            seq.instr(ir::Block { seq: catch_all_seq });
            seq.local_set(exn);
            call_after(&mut seq, 1);
            seq.local_get(exn);
            seq.instr(ir::ThrowRef {});
        }
    }

    let mut body = builder.func_body();
    body.call(before).local_set(tok);
    body.instr(ir::Block { seq: done_seq });

    builder.finish(param_locals, &mut module.funcs)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::transforms::parse_wat;
    use walrus::ir::{Instr, InstrSeq, Visitor};
    use walrus::FunctionKind;

    const HOOKS_WAT: &str = r#"
        (func $__jspi_enter (result i64) i64.const 0)
        (func $__jspi_exit (param i64 i32))
        (func $__jspi_suspend (result i64) i64.const 0)
        (func $__jspi_resume (param i64 i32))
        (export "__jspi_enter" (func $__jspi_enter))
        (export "__jspi_exit" (func $__jspi_exit))
        (export "__jspi_suspend" (func $__jspi_suspend))
        (export "__jspi_resume" (func $__jspi_resume))
        (func $__wbindgen_jspi_set_rejected (param i32))
    "#;

    fn module(catch: bool, legacy: bool) -> (Module, ExportId, FunctionId) {
        let import = if catch {
            r#"(import "env" "sleep" (func $sleep (param i32) (result externref)))"#
        } else {
            r#"(import "env" "sleep" (func $sleep (param i32) (result i32)))"#
        };
        let body = if legacy {
            "try i32.const 1 drop catch_all end"
        } else {
            ""
        };
        let wat = format!(
            r#"(module
                {import}
                (memory 1)
                {HOOKS_WAT}
                (func $work (param i32) (result i32)
                    {body}
                    local.get 0
                    call $sleep
                    drop
                    local.get 0)
                (export "work" (func $work)))"#
        );
        let module = parse_wat(&wat);
        let export = module
            .exports
            .iter()
            .find(|e| e.name == "work")
            .unwrap()
            .id();
        let import = module.funcs.by_name("sleep").unwrap();
        (module, export, import)
    }

    fn run(catch: bool, legacy: bool) -> Module {
        let (mut module, export, import) = module(catch, legacy);
        let aux = WasmBindgenAux {
            jspi_set_rejected: module.funcs.by_name("__wbindgen_jspi_set_rejected"),
            ..Default::default()
        };
        let mut wit = NonstandardWitSection::default();
        super::run(
            &mut module,
            &aux,
            &mut wit,
            vec![export],
            vec![(import, catch)],
            legacy,
        )
        .unwrap();
        let features =
            wasmparser::WasmFeatures::default() | wasmparser::WasmFeatures::LEGACY_EXCEPTIONS;
        wasmparser::Validator::new_with_features(features)
            .validate_all(&module.emit_wasm())
            .expect("transformed module should validate");
        module
    }

    /// The hook calls on every path of `func`, in order of appearance.
    fn hook_calls(module: &Module, func: FunctionId) -> Vec<String> {
        struct Scan<'a> {
            module: &'a Module,
            calls: Vec<String>,
            legacy_try: usize,
            try_table: usize,
        }
        impl<'instr> Visitor<'instr> for Scan<'_> {
            fn start_instr_seq(&mut self, seq: &'instr InstrSeq) {
                for (instr, _) in &seq.instrs {
                    match instr {
                        Instr::Call(c) => {
                            if let Some(name) = &self.module.funcs.get(c.func).name {
                                if name.starts_with("__jspi_") {
                                    self.calls.push(name.clone());
                                }
                            }
                        }
                        Instr::Try(_) => self.legacy_try += 1,
                        Instr::TryTable(_) => self.try_table += 1,
                        _ => {}
                    }
                }
            }
        }
        let FunctionKind::Local(local) = &module.funcs.get(func).kind else {
            panic!("wrapper should be local");
        };
        let mut scan = Scan {
            module,
            calls: Vec::new(),
            legacy_try: 0,
            try_table: 0,
        };
        ir::dfs_in_order(&mut scan, local, local.entry_block());
        scan.calls.push(format!(
            "try={} try_table={}",
            scan.legacy_try, scan.try_table
        ));
        scan.calls
    }

    fn wrapper_of_export(module: &Module, name: &str) -> FunctionId {
        match module.exports.iter().find(|e| e.name == name).unwrap().item {
            ExportItem::Function(f) => f,
            _ => unreachable!(),
        }
    }

    fn check(catch: bool, legacy: bool) {
        let module = run(catch, legacy);
        let eh = if legacy {
            "try=1 try_table=0"
        } else {
            "try=0 try_table=1"
        };

        let export_wrapper = wrapper_of_export(&module, "work");
        assert_ne!(export_wrapper, module.funcs.by_name("work").unwrap());
        assert_eq!(
            hook_calls(&module, export_wrapper),
            ["__jspi_enter", "__jspi_exit", "__jspi_exit", eh]
        );

        // `work` now calls the suspending wrapper, not the import.
        let import = module.funcs.by_name("sleep").unwrap();
        let work = module.funcs.by_name("work").unwrap();
        let FunctionKind::Local(local) = &module.funcs.get(work).kind else {
            unreachable!()
        };
        let mut callee = None;
        ir::dfs_in_order(&mut CalleeScan(&mut callee), local, local.entry_block());
        let import_wrapper = callee.expect("work should call something");
        assert_ne!(import_wrapper, import);
        let resumes = if catch { 3 } else { 2 };
        let mut expected = vec!["__jspi_suspend".to_string()];
        expected.extend(std::iter::repeat_n("__jspi_resume".to_string(), resumes));
        expected.push(eh.to_string());
        assert_eq!(hook_calls(&module, import_wrapper), expected);
    }

    struct CalleeScan<'a>(&'a mut Option<FunctionId>);
    impl<'instr> Visitor<'instr> for CalleeScan<'_> {
        fn visit_call(&mut self, call: &ir::Call) {
            *self.0 = Some(call.func);
        }
    }

    #[test]
    fn modern() {
        check(false, false);
    }

    #[test]
    fn modern_catch() {
        check(true, false);
    }

    #[test]
    fn legacy() {
        check(false, true);
    }

    #[test]
    fn legacy_catch() {
        check(true, true);
    }

    #[test]
    fn missing_hooks() {
        let mut module = parse_wat(r#"(module (func $work) (export "work" (func $work)))"#);
        let export = module.exports.iter().next().unwrap().id();
        let err = super::run(
            &mut module,
            &WasmBindgenAux::default(),
            &mut NonstandardWitSection::default(),
            vec![export],
            vec![],
            false,
        )
        .unwrap_err();
        assert!(err.to_string().contains("-sJSPI_HOOKS"), "{err}");
    }
}
