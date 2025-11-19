use anyhow::{bail, Error};
use base64::{prelude::BASE64_STANDARD, Engine as _};
use std::collections::HashSet;
use std::fmt::Write;
use walrus::Module;

pub struct Config {
    base64: bool,
    fetch_path: Option<String>,
}

pub struct Output {
    module: Module,
    base64: bool,
    fetch_path: Option<String>,
}

impl Config {
    pub fn new() -> Config {
        Config {
            base64: false,
            fetch_path: None,
        }
    }

    pub fn base64(&mut self, base64: bool) -> &mut Self {
        self.base64 = base64;
        self
    }

    pub fn fetch(&mut self, path: Option<String>) -> &mut Self {
        self.fetch_path = path;
        self
    }

    pub fn generate(&mut self, wasm: &[u8]) -> Result<Output, Error> {
        if !self.base64 && self.fetch_path.is_none() {
            bail!("one of --base64 or --fetch is required");
        }
        let module = Module::from_buffer(wasm)?;
        Ok(Output {
            module,
            base64: self.base64,
            fetch_path: self.fetch_path.clone(),
        })
    }
}

// Function to ensure we always append a valid typescript parameter name based
// on parameter index
fn push_index_identifier(i: usize, s: &mut String) {
    let letter = b'a' + ((i % 26) as u8);
    s.push(letter as char);
    if i >= 26 {
        write!(s, "{}", i / 26).unwrap();
    }
}

fn args_are_optional(name: &str) -> bool {
    name == "__wbindgen_thread_destroy"
}

pub fn interface(module: &Module) -> Result<String, Error> {
    let mut exports = String::new();
    module_export_types(module, |name, ty| {
        if name.contains(':') {
            // This can happen when `name` is namespaced, like `__wbgt__reference_test::foo`.
            // We should quote the name, as : is not valid in TypeScript identifiers.
            writeln!(exports, "  readonly {name:?}: {ty};").unwrap();
        } else {
            writeln!(exports, "  readonly {name}: {ty};").unwrap();
        }
    });
    Ok(exports)
}

pub fn typescript(module: &Module) -> Result<String, Error> {
    let mut exports = "/* tslint:disable */\n/* eslint-disable */\n".to_string();
    module_export_types(module, |name, ty| {
        writeln!(exports, "export const {name}: {ty};").unwrap();
    });
    Ok(exports)
}

/// Iterates over all the exports in a module and generates TypeScript types. All
/// name-type pairs are passed to the `export` function.
fn module_export_types(module: &Module, mut export: impl FnMut(&str, &str)) {
    for entry in module.exports.iter() {
        match entry.item {
            walrus::ExportItem::Function(id) => {
                let func = module.funcs.get(id);
                let ty = module.types.get(func.ty());
                let ts_type = function_type_to_ts(ty, args_are_optional(&entry.name));
                export(&entry.name, &ts_type);
            }
            walrus::ExportItem::Memory(_) => export(&entry.name, "WebAssembly.Memory"),
            walrus::ExportItem::Table(_) => export(&entry.name, "WebAssembly.Table"),
            walrus::ExportItem::Global(_) => continue,
            walrus::ExportItem::Tag(_) => export(&entry.name, "WebAssembly.Tag"),
        };
    }
}
fn val_type_to_ts(ty: walrus::ValType) -> &'static str {
    // see https://webassembly.github.io/spec/js-api/index.html#towebassemblyvalue
    // and https://webassembly.github.io/spec/js-api/index.html#tojsvalue
    match ty {
        walrus::ValType::I32 | walrus::ValType::F32 | walrus::ValType::F64 => "number",
        walrus::ValType::I64 => "bigint",
        // there could be anything behind a reference
        walrus::ValType::Ref(_) => "any",
        // V128 currently isn't supported in JS and therefore doesn't have a
        // specific type in the spec. When it does get support, this type will
        // still be technically correct, but should be updated to something more
        // specific.
        walrus::ValType::V128 => "any",
    }
}
fn function_type_to_ts(function: &walrus::Type, all_args_optional: bool) -> String {
    let mut out = String::new();

    // parameters
    out.push('(');
    for (i, arg_type) in function.params().iter().enumerate() {
        if i > 0 {
            out.push_str(", ");
        }

        push_index_identifier(i, &mut out);
        if all_args_optional {
            out.push('?');
        }
        out.push_str(": ");
        out.push_str(val_type_to_ts(*arg_type));
    }
    out.push(')');

    // arrow
    out.push_str(" => ");

    // results
    let results = function.results();
    // this match follows the spec:
    // https://webassembly.github.io/spec/js-api/index.html#exported-function-exotic-objects
    match results.len() {
        0 => out.push_str("void"),
        1 => out.push_str(val_type_to_ts(results[0])),
        _ => {
            out.push('[');
            for (i, result) in results.iter().enumerate() {
                if i > 0 {
                    out.push_str(", ");
                }
                out.push_str(val_type_to_ts(*result));
            }
            out.push(']');
        }
    }

    out
}

impl Output {
    pub fn typescript(&self) -> Result<String, Error> {
        let mut ts = typescript(&self.module)?;
        if self.base64 {
            ts.push_str("export const booted: Promise<boolean>;\n");
        }
        Ok(ts)
    }

    pub fn js_and_wasm(mut self) -> Result<(String, Option<Vec<u8>>), Error> {
        let mut js_imports = String::new();
        let mut exports = String::new();
        let mut set_exports = String::new();
        let mut imports = String::new();

        let mut set = HashSet::new();
        for entry in self.module.imports.iter() {
            if !set.insert(&entry.module) {
                continue;
            }

            let mut name = String::new();
            push_index_identifier(set.len(), &mut name);

            js_imports.push_str(&format!(
                "import * as import_{name} from '{}';\n",
                entry.module
            ));
            imports.push_str(&format!("'{}': import_{name}, ", entry.module));
        }

        for entry in self.module.exports.iter() {
            exports.push_str("export let ");
            exports.push_str(&entry.name);
            exports.push_str(";\n");
            set_exports.push_str(&entry.name);
            set_exports.push_str(" = wasm.exports.");
            set_exports.push_str(&entry.name);
            set_exports.push_str(";\n");
        }

        // This is sort of tricky, but the gist of it is that if there's a start
        // function we want to defer execution of the start function until after
        // all our module's exports are bound. That way we'll execute it as soon
        // as we're ready, but the module's imports and such will be able to
        // work as everything is wired up.
        //
        // This ends up helping out in situations such as:
        //
        // * The start function calls an imported function
        // * That imported function in turn tries to access the Wasm module
        //
        // If we don't do this then the second step won't work because the start
        // function is automatically executed before the promise of
        // instantiation resolves, meaning that we won't actually have anything
        // bound for it to access.
        //
        // If we remove the start function here (via `unstart`) then we'll
        // reexport it as `__wasm2es6js_start` so be manually executed here.
        if self.unstart() {
            set_exports.push_str("wasm.exports.__wasm2es6js_start();\n");
        }

        let inst = format!(
            "
            WebAssembly.instantiate(bytes,{{ {imports} }})
                .then(obj => {{
                    const wasm = obj.instance;
                    {set_exports}
                }})
            ",
        );
        let wasm = self.module.emit_wasm();
        let (bytes, booted) = if self.base64 {
            (
                format!(
                    "
                    let bytes;
                    const base64 = \"{base64}\";
                    if (typeof Buffer === 'undefined') {{
                        bytes = Uint8Array.from(atob(base64), c => c.charCodeAt(0));
                    }} else {{
                        bytes = Buffer.from(base64, 'base64');
                    }}
                    ",
                    base64 = BASE64_STANDARD.encode(&wasm)
                ),
                inst,
            )
        } else if let Some(ref path) = self.fetch_path {
            (
                String::new(),
                format!(
                    "
                    fetch('{path}')
                        .then(res => res.arrayBuffer())
                        .then(bytes => {inst})
                    "
                ),
            )
        } else {
            bail!("the option --base64 or --fetch is required");
        };
        let js = format!(
            "\
            {js_imports}
            {bytes}
            export const booted = {booted};
            {exports}
            ",
        );
        let wasm = if self.base64 { None } else { Some(wasm) };
        Ok((js, wasm))
    }

    /// See comments above for what this is doing, but in a nutshell this
    /// removes the start section, if any, and moves it to an exported function.
    /// Returns whether a start function was found and removed.
    fn unstart(&mut self) -> bool {
        let start = match self.module.start.take() {
            Some(id) => id,
            None => return false,
        };
        self.module.exports.add("__wasm2es6js_start", start);
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_push_index_identifier() {
        fn index_identifier(i: usize) -> String {
            let mut s = String::new();
            push_index_identifier(i, &mut s);
            s
        }

        assert_eq!(index_identifier(0), "a");
        assert_eq!(index_identifier(1), "b");
        assert_eq!(index_identifier(25), "z");
        assert_eq!(index_identifier(26), "a1");
        assert_eq!(index_identifier(27), "b1");
        assert_eq!(index_identifier(51), "z1");
        assert_eq!(index_identifier(52), "a2");
        assert_eq!(index_identifier(53), "b2");
        assert_eq!(index_identifier(260), "a10");
        assert_eq!(index_identifier(261), "b10");
    }
}
