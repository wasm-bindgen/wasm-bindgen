pub mod externref;
pub mod multi_value;
pub mod threads;

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
