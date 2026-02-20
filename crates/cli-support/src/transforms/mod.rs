pub mod catch_handler;
pub mod externref;
pub mod multi_value;
pub mod threads;

use walrus::ir::{Try, TryTable, Visitor};

/// Returns true if the module has local exception tags (indicating exception handling support).
pub fn has_local_exception_tags(module: &walrus::Module) -> bool {
    module
        .tags
        .iter()
        .any(|t| matches!(t.kind, walrus::TagKind::Local))
}

/// The version of exception handling used in the module.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExceptionHandlingVersion {
    /// No exception handling instructions found
    None,
    /// Legacy EH (phase 1): try/catch/catch_all instructions
    Legacy,
    /// Modern EH (phase 4): try_table instruction
    Modern,
}

/// Detect which exception handling version is used in the module.
///
/// Scans all functions for `Try` (legacy) vs `TryTable` (modern) instructions.
/// If both are present, returns `Modern` as the module likely supports both.
pub fn detect_exception_handling_version(module: &walrus::Module) -> ExceptionHandlingVersion {
    struct EhDetector {
        has_try: bool,
        has_try_table: bool,
    }

    impl<'instr> Visitor<'instr> for EhDetector {
        fn visit_try(&mut self, _: &Try) {
            self.has_try = true;
        }

        fn visit_try_table(&mut self, _: &TryTable) {
            self.has_try_table = true;
        }
    }

    let mut detector = EhDetector {
        has_try: false,
        has_try_table: false,
    };

    for func in module.funcs.iter() {
        if let walrus::FunctionKind::Local(local) = &func.kind {
            walrus::ir::dfs_in_order(&mut detector, local, local.entry_block());
        }
        // Early exit if we've found both
        if detector.has_try && detector.has_try_table {
            break;
        }
    }

    match (detector.has_try_table, detector.has_try) {
        (true, _) => ExceptionHandlingVersion::Modern,
        (false, true) => ExceptionHandlingVersion::Legacy,
        (false, false) => ExceptionHandlingVersion::None,
    }
}

/// If a start function is present, it removes it from the `start` section
/// of the Wasm module and then moves it to an exported function, named
/// `__wbindgen_start`.
pub(crate) fn unstart_start_function(module: &mut walrus::Module) -> bool {
    let start = match module.start.take() {
        Some(id) => id,
        None => return false,
    };
    module.exports.add("__wbindgen_start", start);
    true
}

#[cfg(test)]
mod tests {
    use super::*;
    use walrus::ModuleConfig;

    fn parse_wat(wat: &str) -> walrus::Module {
        let wasm = wat::parse_str(wat).unwrap();
        ModuleConfig::new()
            .generate_producers_section(false)
            .parse(&wasm)
            .unwrap()
    }

    #[test]
    fn detect_eh_version_none() {
        let wat = r#"
            (module
                (func $foo
                    i32.const 1
                    drop
                )
                (export "foo" (func $foo))
            )
        "#;
        let module = parse_wat(wat);
        assert_eq!(
            detect_exception_handling_version(&module),
            ExceptionHandlingVersion::None
        );
    }

    #[test]
    fn detect_eh_version_legacy() {
        // Legacy EH uses try/catch_all/end syntax
        let wat = r#"
            (module
                (func $foo
                    try
                        i32.const 1
                        drop
                    catch_all
                    end
                )
                (export "foo" (func $foo))
            )
        "#;
        let module = parse_wat(wat);
        assert_eq!(
            detect_exception_handling_version(&module),
            ExceptionHandlingVersion::Legacy
        );
    }

    #[test]
    fn detect_eh_version_modern() {
        // Modern EH uses try_table with catch_all targeting a block label
        let wat = r#"
            (module
                (func $foo
                    (block $catch
                        (try_table (catch_all $catch)
                            i32.const 1
                            drop
                        )
                    )
                )
                (export "foo" (func $foo))
            )
        "#;
        let module = parse_wat(wat);
        assert_eq!(
            detect_exception_handling_version(&module),
            ExceptionHandlingVersion::Modern
        );
    }

    #[test]
    fn detect_eh_version_both_prefers_modern() {
        // If a module somehow has both, we prefer modern
        let wat = r#"
            (module
                (func $foo
                    try
                        i32.const 1
                        drop
                    catch_all
                    end
                )
                (func $bar
                    (block $catch
                        (try_table (catch_all $catch)
                            i32.const 1
                            drop
                        )
                    )
                )
                (export "foo" (func $foo))
                (export "bar" (func $bar))
            )
        "#;
        let module = parse_wat(wat);
        assert_eq!(
            detect_exception_handling_version(&module),
            ExceptionHandlingVersion::Modern
        );
    }
}
