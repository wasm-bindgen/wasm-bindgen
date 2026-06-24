//! A tiny and incomplete Wasm interpreter
//!
//! This module contains a tiny and incomplete Wasm interpreter built on top of
//! `walrus`'s module structure. Each `Interpreter` contains some state
//! about the execution of a Wasm instance. The "incomplete" part here is
//! related to the fact that this is *only* used to execute the various
//! descriptor functions for wasm-bindgen.
//!
//! As a recap, the wasm-bindgen macro generate "descriptor functions" which
//! basically as a mapping of rustc's trait resolution in executable code. This
//! allows us to detect, after the macro is invoke, what trait selection did and
//! what types of functions look like. By executing descriptor functions they'll
//! each invoke a known import (with only one argument) some number of times,
//! which gives us a list of `u32` values to then decode.
//!
//! The interpreter here is only geared towards this one exact use case, so it's
//! quite small and likely not extra-efficient.

#![deny(missing_docs)]

use crate::wasm_conventions;
use anyhow::{bail, ensure};
use std::collections::{BTreeMap, HashMap, HashSet};
use walrus::ir::InstrSeqId;
use walrus::{ExportId, FunctionId, GlobalId, GlobalKind, LocalFunction, LocalId, Module, ValType};

/// A ready-to-go interpreter of a Wasm module.
///
/// An interpreter currently represents effectively cached state. It is reused
/// between calls to `interpret` and is precomputed from a `Module`. It houses
/// state like the Wasm stack, Wasm memory, etc.
#[derive(Default)]
pub struct Interpreter {
    // Function index of the `__wbindgen_describe` and
    // `__wbindgen_describe_cast` imported functions. We special case this
    // to know when the environment's imported function is called.
    describe_id: Option<FunctionId>,
    describe_cast_id: Option<FunctionId>,

    // Linear memory mirroring the module's own, used for stack loads/stores
    // during descriptor execution.
    mem: Vec<i32>,
    scratch: Vec<i32>,

    // GlobalId of __stack_pointer, if found. Used to validate that the stack
    // pointer is restored after each descriptor execution.
    stack_pointer: Option<GlobalId>,

    // The stack pointer value at the start of each interpret_descriptor call,
    // used to validate restoration and to unwind early exits (describe_cast).
    stack_pointer_initial: i32,

    // Live state of all locally-defined integer globals, snapshotted from the
    // module at construction and mutated freely during interpretation.
    //
    // The interpreter itself only tracks 32-bit values, so wasm64 i64 globals
    // are truncated the same way we already truncate other descriptor-time
    // i64 values.
    globals: HashMap<GlobalId, i32>,

    // The descriptor which we're assembling, a list of `u32` entries. This is
    // very specific to wasm-bindgen and is the purpose for the existence of
    // this module.
    descriptor: Vec<u32>,

    /// The `__wbindgen_skip_interpret_calls`'s id.
    skip_interpret: Option<ExportId>,

    /// Some functions that need to skip interpret, such as `__wasm_call_ctors`
    /// and `__wasm_call_dtors`.
    skip_calls: HashSet<FunctionId>,
    stopped: bool,

    /// Emscripten `env::invoke_*` trampolines. Emscripten rewrites direct calls
    /// that may unwind/longjmp into indirect calls through the function table,
    /// wrapped in these imported helpers (`invoke_v(fnptr)`, `invoke_vi(fnptr,
    /// a)`, ...). The first argument is the table index of the real target; the
    /// rest are forwarded. Maps the import id to its number of result values.
    invoke_imports: HashMap<FunctionId, usize>,

    /// Function table contents (index -> function), reconstructed from active
    /// element segments. Used to resolve the target of an `invoke_*` call.
    funcref_table: BTreeMap<i32, FunctionId>,
}

fn skip_calls(module: &Module, id: FunctionId) -> HashSet<FunctionId> {
    use walrus::ir::*;

    let func = module.funcs.get(id);

    let local = match &func.kind {
        walrus::FunctionKind::Local(l) => l,
        _ => panic!("can only call locally defined functions"),
    };

    let entry = local.entry_block();
    let block = local.block(entry);

    block
        .instrs
        .iter()
        .filter_map(|(instr, _)| match instr {
            // There are only up to three calls for now:
            //   1. __wasm_call_ctors (`#[link_section = ".init_array"]`)
            //   2. __wbindgen_skip_interpret_calls (The original symbol, we don't care about it)
            //   3. __wasm_call_dtors (This symbol may not be present in Rust program, but may be present if C program is linked)
            Instr::Call(Call { func }) | Instr::ReturnCall(ReturnCall { func }) => Some(*func),
            // Typically, there are no other instructions, or only a return instruction.
            //
            // When coverage is turned on, there may be llvm coverage instrumentation
            // instructions.
            _ => None,
        })
        .collect()
}

impl Interpreter {
    /// Creates a new interpreter from a provided `Module`, precomputing all
    /// information necessary to interpret further.
    ///
    /// Note that the `module` passed in to this function must be the same as
    /// the `module` passed to `interpret` below.
    pub fn new(module: &Module) -> Result<Interpreter, anyhow::Error> {
        let mut ret = Interpreter {
            // Mirror the module's own linear memory so the stack pointer's
            // snapshotted value is directly valid as an index. If there is no
            // memory, descriptor functions can't do any loads/stores, so an
            // empty vec is fine (any attempted access will panic).
            mem: module
                .memories
                .iter()
                .next()
                .map_or(vec![], |m| vec![0; m.initial as usize * 65536 / 4]),
            ..Default::default()
        };

        // Snapshot all locally-defined integer globals so the interpreter can
        // read/write them during descriptor execution. Imported globals
        // (common in PIC/dynamic-linked modules such as those produced by
        // emscripten) have no compile-time value, so we initialize them to a
        // safe placeholder. Descriptor functions don't depend on the actual
        // value of PIC base globals (`__memory_base`, `__table_base`,
        // `GOT.mem.*`, `GOT.func.*`); they only need consistent reads/writes.
        for global in module.globals.iter() {
            match global.kind {
                GlobalKind::Local(walrus::ConstExpr::Value(walrus::ir::Value::I32(n))) => {
                    ret.globals.insert(global.id(), n);
                }
                GlobalKind::Local(walrus::ConstExpr::Value(walrus::ir::Value::I64(n))) => {
                    ret.globals.insert(global.id(), n as i32);
                }
                GlobalKind::Import(_) if global.ty == ValType::I32 || global.ty == ValType::I64 => {
                    ret.globals.insert(global.id(), 0);
                }
                _ => {}
            }
        }

        if let Some(sp) = wasm_conventions::get_stack_pointer(module) {
            ret.stack_pointer = Some(sp);
            // For an imported stack pointer (PIC builds), the runtime sets it
            // at instantiation. We can't know the real value, but descriptors
            // only need sp to land inside our `mem` buffer for any
            // load/store. Seed it with a sentinel near the top of mem so that
            // typical small `sp -= N` adjustments stay in-range and non-zero.
            if matches!(module.globals.get(sp).kind, GlobalKind::Import(_)) {
                let top = (ret.mem.len() as i32).saturating_mul(4);
                ret.globals.insert(sp, top);
            }
        }

        // Figure out where the `__wbindgen_describe` imported function is, if
        // it exists. We'll special case calls to this function as our
        // interpretation should only invoke this function as an imported
        // function.
        for import in module.imports.iter() {
            let id = match import.kind {
                walrus::ImportKind::Function(id) => id,
                _ => continue,
            };
            if import.module != "__wbindgen_placeholder__" {
                continue;
            }
            if import.name == "__wbindgen_describe" {
                ret.describe_id = Some(id);
            } else if import.name == "__wbindgen_describe_cast" {
                ret.describe_cast_id = Some(id);
            }
        }

        // Collect emscripten `invoke_*` trampolines so calls to them can be
        // redirected to their real (table-indexed) targets during descriptor
        // interpretation.
        for import in module.imports.iter() {
            let id = match import.kind {
                walrus::ImportKind::Function(id) => id,
                _ => continue,
            };
            if import.module == "env" && import.name.starts_with("invoke_") {
                let results = module.types.get(module.funcs.get(id).ty()).results().len();
                ret.invoke_imports.insert(id, results);
            }
        }

        // Reconstruct the function table from active element segments so an
        // `invoke_*(fnptr, ..)` can resolve `fnptr` to a concrete function.
        for element in module.elements.iter() {
            let offset = match &element.kind {
                walrus::ElementKind::Active { offset, .. } => offset,
                _ => continue,
            };
            let base = match offset {
                walrus::ConstExpr::Value(walrus::ir::Value::I32(n)) => *n,
                walrus::ConstExpr::Value(walrus::ir::Value::I64(n)) => *n as i32,
                walrus::ConstExpr::Global(g) => ret.globals.get(g).copied().unwrap_or(0),
                _ => continue,
            };
            if let walrus::ElementItems::Functions(funcs) = &element.items {
                for (i, func) in funcs.iter().enumerate() {
                    ret.funcref_table.insert(base + i as i32, *func);
                }
            }
        }

        // Setup skip_interpret id and skip_calls
        if let Some(export) = module
            .exports
            .iter()
            .find(|export| export.name == "__wbindgen_skip_interpret_calls")
        {
            let id = match export.item {
                walrus::ExportItem::Function(id) => id,
                _ => panic!("__wbindgen_skip_interpret_calls must be an export function"),
            };
            ret.skip_interpret = Some(export.id());
            ret.skip_calls = skip_calls(module, id);
        }

        Ok(ret)
    }

    /// Interprets the execution of the descriptor function `func`.
    ///
    /// This function will execute `func` in the `module` provided. Note that
    /// the `module` provided here must be the same as the one passed to `new`
    /// when this `Interpreter` was constructed.
    ///
    /// The `func` must be a wasm-bindgen descriptor function meaning that it
    /// doesn't do anything like use floats or i64. Instead all it should do is
    /// call other functions, sometimes some stack pointer manipulation, and
    /// then call the one imported `__wbindgen_describe` function. Anything else
    /// will cause this interpreter to panic.
    ///
    /// When the descriptor has finished running the assembled descriptor list
    /// is returned. The descriptor returned can then be re-parsed into an
    /// actual `Descriptor` in the cli-support crate.
    ///
    /// # Return value
    ///
    /// Returns `Some` if `func` was found in the `module` and `None` if it was
    /// not found in the `module`.
    pub fn interpret_descriptor(&mut self, id: FunctionId, module: &Module) -> &[u32] {
        self.descriptor.truncate(0);
        self.stopped = false;

        if let Some(sp) = self.stack_pointer {
            self.stack_pointer_initial = self.globals[&sp];
        }

        let func = module.funcs.get(id);
        let ty = module.types.get(func.ty());

        self.call(id, module, &vec![0; ty.params().len()]);

        // Validate the stack pointer was restored to its value at function entry.
        if let Some(sp) = self.stack_pointer {
            assert_eq!(self.globals[&sp], self.stack_pointer_initial);
        }
        &self.descriptor
    }

    /// Returns the function id of the `__wbindgen_describe_cast`
    /// imported function.
    pub fn describe_cast_id(&self) -> Option<FunctionId> {
        self.describe_cast_id
    }

    /// Returns the export id of the `__wbindgen_skip_interpret_calls`.
    pub fn skip_interpret(&self) -> Option<ExportId> {
        self.skip_interpret
    }

    fn call(&mut self, id: FunctionId, module: &Module, args: &[i32]) {
        let func = module.funcs.get(id);
        log::trace!("starting a call of {id:?} {:?}", func.name);
        log::trace!("arguments {args:?}");
        let local = match &func.kind {
            walrus::FunctionKind::Local(l) => l,
            walrus::FunctionKind::Import(imp) => {
                let i = module.imports.get(imp.import);
                panic!(
                    "can only call locally defined functions (got import {:?}::{:?}, fn name {:?})",
                    i.module, i.name, func.name
                );
            }
            _ => panic!("can only call locally defined functions"),
        };

        let mut frame = Frame {
            module,
            func: local,
            interp: self,
            locals: BTreeMap::new(),
        };

        assert_eq!(local.args.len(), args.len());
        for (arg, val) in local.args.iter().zip(args) {
            frame.locals.insert(*arg, *val);
        }

        let _ = frame.eval(local.entry_block()).unwrap_or_else(|err| {
            if let Some(name) = &module.funcs.get(id).name {
                panic!("{name}: {err}")
            } else {
                panic!("{err}")
            }
        });
    }
}

struct Frame<'a> {
    module: &'a Module,
    func: &'a LocalFunction,
    interp: &'a mut Interpreter,
    locals: BTreeMap<LocalId, i32>,
}

/// Result of evaluating an instruction sequence: either it ran to the end, a
/// branch escaped to an enclosing block, or the function returned. Needed to
/// interpret the control flow emscripten emits around `invoke_*` (the "did it
/// throw?" guard) in descriptor functions.
enum Flow {
    Normal,
    Branch(InstrSeqId),
    Return,
}

impl Frame<'_> {
    fn eval(&mut self, seq: InstrSeqId) -> anyhow::Result<Flow> {
        use walrus::ir::*;

        for (instr, _) in self.func.block(seq).iter() {
            if self.interp.stopped {
                return Ok(Flow::Normal);
            }
            let stack = &mut self.interp.scratch;

            match instr {
                Instr::Const(c) => match c.value {
                    Value::I32(n) => stack.push(n),
                    // For wasm64, truncate i64 constants to i32. The descriptor
                    // protocol only uses small values, and pointer arithmetic
                    // in our tiny interpreter fits in i32.
                    Value::I64(n) => stack.push(n as i32),
                    _ => bail!("non-integer constant"),
                },
                Instr::LocalGet(e) => stack.push(self.locals.get(&e.local).cloned().unwrap_or(0)),
                Instr::LocalSet(e) => {
                    let val = stack.pop().unwrap();
                    self.locals.insert(e.local, val);
                }
                Instr::LocalTee(e) => {
                    let val = *stack.last().unwrap();
                    self.locals.insert(e.local, val);
                }

                Instr::GlobalGet(e) => {
                    let val = *self.interp.globals.get(&e.global).unwrap_or_else(|| {
                        panic!(
                            "global {:?} not found, this is a bug in wasm-bindgen",
                            e.global
                        )
                    });
                    stack.push(val);
                }
                Instr::GlobalSet(e) => {
                    let val = stack.pop().unwrap();
                    self.interp.globals.insert(e.global, val);
                }

                // Support simple arithmetic, mainly for the stack pointer
                // manipulation
                Instr::Binop(e) => {
                    let rhs = stack.pop().unwrap();
                    let lhs = stack.pop().unwrap();
                    stack.push(match e.op {
                        BinaryOp::I32Sub | BinaryOp::I64Sub => lhs - rhs,
                        BinaryOp::I32Add | BinaryOp::I64Add => lhs + rhs,
                        BinaryOp::I32And | BinaryOp::I64And => lhs & rhs,
                        BinaryOp::I32Or | BinaryOp::I64Or => lhs | rhs,
                        BinaryOp::I32Xor | BinaryOp::I64Xor => lhs ^ rhs,
                        BinaryOp::I32Shl | BinaryOp::I64Shl => lhs << (rhs & 31),
                        BinaryOp::I32Eq | BinaryOp::I64Eq => (lhs == rhs) as i32,
                        BinaryOp::I32Ne | BinaryOp::I64Ne => (lhs != rhs) as i32,
                        BinaryOp::I32LtU | BinaryOp::I64LtU => ((lhs as u32) < rhs as u32) as i32,
                        BinaryOp::I32LtS | BinaryOp::I64LtS => (lhs < rhs) as i32,
                        BinaryOp::I32GtU | BinaryOp::I64GtU => ((lhs as u32) > rhs as u32) as i32,
                        BinaryOp::I32GtS | BinaryOp::I64GtS => (lhs > rhs) as i32,
                        BinaryOp::I32LeU | BinaryOp::I64LeU => ((lhs as u32) <= rhs as u32) as i32,
                        BinaryOp::I32LeS | BinaryOp::I64LeS => (lhs <= rhs) as i32,
                        BinaryOp::I32GeU | BinaryOp::I64GeU => ((lhs as u32) >= rhs as u32) as i32,
                        BinaryOp::I32GeS | BinaryOp::I64GeS => (lhs >= rhs) as i32,
                        op => bail!("invalid binary op {op:?}"),
                    });
                }

                // Support unary ops, mainly for wasm64's I32WrapI64 and
                // I64ExtendI32U (pointer conversions).
                Instr::Unop(e) => {
                    let val = stack.pop().unwrap();
                    stack.push(match e.op {
                        // i64→i32 wrap: already truncated in our i32 representation
                        UnaryOp::I32WrapI64 => val,
                        // i32→i64 extend: already fits in our i32 representation
                        UnaryOp::I64ExtendUI32 | UnaryOp::I64ExtendSI32 => val,
                        UnaryOp::I32Eqz | UnaryOp::I64Eqz => (val == 0) as i32,
                        op => bail!("invalid unary op {op:?}"),
                    });
                }

                // Support small loads/stores to the stack. These show up in debug
                // mode where there's some traffic on the linear stack even when in
                // theory there doesn't need to be.
                Instr::Load(e) => {
                    let address = stack.pop().unwrap();
                    let address = address as u32 + e.arg.offset as u32;
                    ensure!(
                        address > 0,
                        "Read a negative or zero address value from the stack. Did we run out of memory?"
                    );
                    let width = e.kind.width();
                    // Also support word-aligned 8-byte types.
                    ensure!(address % width.min(4) == 0);
                    let val = self.interp.mem[address as usize / 4];
                    if width == 4 {
                        stack.push(val)
                    } else if width == 1 {
                        let result = val.to_le_bytes()[(address % 4) as usize];
                        let LoadKind::I32_8 { kind } = e.kind else {
                            panic!("Unhandled load kind {:?}", e.kind)
                        };
                        match kind {
                            ExtendedLoad::SignExtend => {
                                stack.push(result as i8 as i32);
                            }
                            ExtendedLoad::ZeroExtend | ExtendedLoad::ZeroExtendAtomic => {
                                stack.push(result as i32);
                            }
                        };
                    } else if width == 8 {
                        // i64 load. Our stack/memory model is i32-based (mirroring
                        // the width-8 store below, which keeps only the low 4
                        // bytes). `-Cinstrument-coverage` emits i64 profiler
                        // counter loads inside the `__wbindgen_describe_*` helper
                        // functions; the loaded values never influence the
                        // descriptor protocol, so pushing the low word is
                        // sufficient for the interpreter to run to completion.
                        stack.push(val);
                    } else {
                        panic!("Unhandled load width {width}");
                    }
                }
                Instr::Store(e) => {
                    let value = stack.pop().unwrap();
                    let address = stack.pop().unwrap();
                    let address = address as u32 + e.arg.offset as u32;
                    ensure!(
                        address > 0,
                        "Read a negative or zero address value from the stack. Did we run out of memory?"
                    );
                    let width = e.kind.width();
                    // Also support word-aligned 8-byte types.
                    ensure!(address % width.min(4) == 0);
                    let index = address as usize / 4;
                    if width == 8 {
                        // Oops our stack is of i32s so we can't really handle a
                        // store of width 8. Just treat the more signifcant 4
                        // bytes as 0.
                        self.interp.mem[index] = value;
                        self.interp.mem[index + 1] = 0;
                    } else if width == 4 {
                        self.interp.mem[index] = value;
                    } else if width == 1 {
                        let mut bytes = self.interp.mem[index].to_le_bytes();
                        bytes[(address % 4) as usize] = value as u8;
                        self.interp.mem[index] = i32::from_le_bytes(bytes);
                    } else {
                        panic!("Unhandled store width {width}");
                    }
                }

                Instr::Return(_) => {
                    log::trace!("return");
                    return Ok(Flow::Return);
                }

                Instr::Drop(_) => {
                    log::trace!("drop");
                    stack.pop().unwrap();
                }

                Instr::Call(Call { func }) | Instr::ReturnCall(ReturnCall { func }) => {
                    let func = *func;
                    // If this function is calling the `__wbindgen_describe`
                    // function, which we've precomputed the id for, then
                    // it's telling us about the next `u32` element in the
                    // descriptor to return. We "call" the imported function
                    // here by directly inlining it.
                    if Some(func) == self.interp.describe_id {
                        let val = stack.pop().unwrap();
                        log::trace!("__wbindgen_describe({val})");
                        self.interp.descriptor.push(val as u32);

                    // If this function is calling the `__wbindgen_describe_cast`
                    // function then it's just a marker for the parent function
                    // to be treated as a cast.
                    } else if Some(func) == self.interp.describe_cast_id {
                        log::trace!("__wbindgen_describe_cast()");
                        // `__wbindgen_describe_cast` marks the end of the cast
                        // descriptor. Stop here, ignoring anything on the stack.
                        // Restore SP to its entry value since the normal function
                        // epilogue won't run.
                        if let Some(sp) = self.interp.stack_pointer {
                            self.interp
                                .globals
                                .insert(sp, self.interp.stack_pointer_initial);
                        }
                        self.interp.stopped = true;
                        return Ok(Flow::Normal);

                    // ... otherwise this is a normal call so we recurse.
                    } else {
                        // Skip the constructor function.
                        //
                        // Complex logic can be implemented in the ctor, our simple interpreter will fail
                        // to execute due to missing instructions.
                        //
                        // For example, executing `1 + 1` fails due to the lack of `I32.And` instruction.
                        //
                        // Because `wasm-ld` may insert a call to ctor from the beginning of every function that
                        // your module exports, the interpreter will enter the ctor logic when parsing the
                        // `wasm-bindgen` function, causing failure.
                        if self.interp.skip_calls.contains(&func) {
                            continue;
                        }

                        // Skip profiling related functions which we don't want to interpret.
                        if self
                            .module
                            .funcs
                            .get(func)
                            .name
                            .as_ref()
                            .is_some_and(|name| {
                                name.starts_with("__llvm_profile_init")
                                    || name.starts_with("__llvm_profile_register_function")
                                    || name.starts_with("__llvm_profile_register_function")
                            })
                        {
                            continue;
                        }

                        let ty = self.module.types.get(self.module.funcs.get(func).ty());
                        let mut args = (0..ty.params().len())
                            .map(|_| stack.pop().unwrap())
                            .collect::<Vec<_>>();
                        args.reverse();

                        // Redirect emscripten `invoke_*(fnptr, ..args)` to the
                        // table-indexed target, forwarding the trailing args.
                        if self.interp.invoke_imports.contains_key(&func) {
                            let (&fnptr, rest) = args
                                .split_first()
                                .expect("invoke_* always takes a function pointer");
                            let target =
                                *self.interp.funcref_table.get(&fnptr).unwrap_or_else(|| {
                                    panic!(
                                        "invoke_* target {fnptr} not found in the function table"
                                    )
                                });
                            self.interp.call(target, self.module, rest);
                        } else {
                            self.interp.call(func, self.module, &args);
                        }
                    }

                    if let Instr::ReturnCall(_) = instr {
                        log::trace!("return_call");
                        return Ok(Flow::Return);
                    }
                }

                Instr::Block(block) => match self.eval(block.seq)? {
                    Flow::Branch(t) if t == block.seq => {}
                    Flow::Normal => {}
                    other => return Ok(other),
                },

                Instr::Loop(block) => loop {
                    match self.eval(block.seq)? {
                        // A branch back to the loop header re-runs it.
                        Flow::Branch(t) if t == block.seq => {
                            if self.interp.stopped {
                                return Ok(Flow::Normal);
                            }
                            continue;
                        }
                        // Falling off the end of a loop body exits the loop.
                        Flow::Normal => break,
                        other => return Ok(other),
                    }
                },

                Instr::IfElse(e) => {
                    let cond = stack.pop().unwrap();
                    let seq = if cond != 0 {
                        e.consequent
                    } else {
                        e.alternative
                    };
                    match self.eval(seq)? {
                        Flow::Branch(t) if t == seq => {}
                        Flow::Normal => {}
                        other => return Ok(other),
                    }
                }

                Instr::Br(e) => return Ok(Flow::Branch(e.block)),

                Instr::BrIf(e) => {
                    let cond = stack.pop().unwrap();
                    if cond != 0 {
                        return Ok(Flow::Branch(e.block));
                    }
                }

                Instr::BrTable(e) => {
                    let idx = stack.pop().unwrap();
                    let target = e.blocks.get(idx as usize).copied().unwrap_or(e.default);
                    return Ok(Flow::Branch(target));
                }

                Instr::Select(_) => {
                    let cond = stack.pop().unwrap();
                    let rhs = stack.pop().unwrap();
                    let lhs = stack.pop().unwrap();
                    stack.push(if cond != 0 { lhs } else { rhs });
                }

                Instr::Try(block) => match self.eval(block.seq)? {
                    Flow::Branch(t) if t == block.seq => {}
                    Flow::Normal => {}
                    other => return Ok(other),
                },

                Instr::TryTable(block) => match self.eval(block.seq)? {
                    Flow::Branch(t) if t == block.seq => {}
                    Flow::Normal => {}
                    other => return Ok(other),
                },

                // All other instructions shouldn't be used by our various
                // descriptor functions. LLVM optimizations may mean that some
                // of the above instructions aren't actually needed either, but
                // the above instructions have empirically been required when
                // executing our own test suite in wasm-bindgen.
                //
                // Note that LLVM may change over time to generate new
                // instructions in debug mode, and we'll have to react to those
                // sorts of changes as they arise.
                s => bail!("unknown instruction {s:?}"),
            }
        }

        Ok(Flow::Normal)
    }
}

#[cfg(test)]
mod smoke_tests;
