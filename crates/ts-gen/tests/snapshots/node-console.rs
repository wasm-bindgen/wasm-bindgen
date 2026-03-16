#[allow(unused_imports)]
use wasm_bindgen::prelude::*;
#[allow(unused_imports)]
use js_sys::*;
/// Extension trait for awaiting `js_sys::Promise<T>`.
///
/// Since `IntoFuture` can't be implemented for `js_sys::Promise` from
/// generated code (orphan rule), use `.into_future().await` instead:
/// ```ignore
/// use bindings::PromiseExt;
/// let data: ArrayBuffer = promise.into_future().await?;
/// ```
pub trait PromiseExt {
    type Output;
    fn into_future(self) -> wasm_bindgen_futures::JsFuture<Self::Output>;
}
impl<T: 'static + wasm_bindgen::convert::FromWasmAbi> PromiseExt for js_sys::Promise<T> {
    type Output = T;
    fn into_future(self) -> wasm_bindgen_futures::JsFuture<T> {
        wasm_bindgen_futures::JsFuture::from(self)
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = Object)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub type ConsoleOptions;
    #[wasm_bindgen(method, getter)]
    pub fn stdout(this: &ConsoleOptions) -> Object;
    #[wasm_bindgen(method, setter)]
    pub fn set_stdout(this: &ConsoleOptions, val: &Object);
    #[wasm_bindgen(method, getter)]
    pub fn stderr(this: &ConsoleOptions) -> Option<Object>;
    #[wasm_bindgen(method, setter)]
    pub fn set_stderr(this: &ConsoleOptions, val: Option<&Object>);
    /// Ignore errors when writing to the underlying streams.
    /// @default true
    #[wasm_bindgen(method, getter, js_name = "ignoreErrors")]
    pub fn ignore_errors(this: &ConsoleOptions) -> Option<bool>;
    #[wasm_bindgen(method, setter, js_name = "ignoreErrors")]
    pub fn set_ignore_errors(this: &ConsoleOptions, val: Option<bool>);
    /// Set color support for this `Console` instance.
    /// @default 'auto'
    #[wasm_bindgen(method, getter, js_name = "colorMode")]
    pub fn color_mode(this: &ConsoleOptions) -> Option<JsValue>;
    #[wasm_bindgen(method, setter, js_name = "colorMode")]
    pub fn set_color_mode(this: &ConsoleOptions, val: Option<bool>);
    #[wasm_bindgen(method, setter, js_name = "colorMode")]
    pub fn set_color_mode_with_str(this: &ConsoleOptions, val: Option<&str>);
    /// Set group indentation.
    /// @default 2
    #[wasm_bindgen(method, getter, js_name = "groupIndentation")]
    pub fn group_indentation(this: &ConsoleOptions) -> Option<f64>;
    #[wasm_bindgen(method, setter, js_name = "groupIndentation")]
    pub fn set_group_indentation(this: &ConsoleOptions, val: Option<f64>);
}
impl ConsoleOptions {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        #[allow(unused_imports)]
        use wasm_bindgen::JsCast;
        JsCast::unchecked_into(js_sys::Object::new())
    }
    pub fn builder() -> ConsoleOptionsBuilder {
        ConsoleOptionsBuilder {
            inner: Self::new(),
            required: 1u64,
        }
    }
}
pub struct ConsoleOptionsBuilder {
    inner: ConsoleOptions,
    required: u64,
}
#[allow(unused_mut)]
impl ConsoleOptionsBuilder {
    pub fn stdout(mut self, val: &Object) -> Self {
        self.inner.set_stdout(val);
        self.required &= 18446744073709551614u64;
        self
    }
    pub fn stderr(mut self, val: Option<&Object>) -> Self {
        self.inner.set_stderr(val);
        self
    }
    pub fn ignore_errors(mut self, val: Option<bool>) -> Self {
        self.inner.set_ignore_errors(val);
        self
    }
    pub fn color_mode(mut self, val: Option<bool>) -> Self {
        self.inner.set_color_mode(val);
        self
    }
    pub fn color_mode_with_str(mut self, val: Option<&str>) -> Self {
        self.inner.set_color_mode_with_str(val);
        self
    }
    pub fn group_indentation(mut self, val: Option<f64>) -> Self {
        self.inner.set_group_indentation(val);
        self
    }
    pub fn build(self) -> Result<ConsoleOptions, JsValue> {
        if self.required != 0 {
            let mut missing = Vec::new();
            if self.required & 1u64 != 0 {
                missing.push("missing required property `stdout`");
            }
            return Err(
                JsValue::from_str(
                    &format!("{}: {}", stringify!(ConsoleOptions), missing.join(", ")),
                ),
            );
        }
        Ok(self.inner)
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = Object)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub type Console;
    /// Creates a new `Console` with one or two writable stream instances.
    /// `stdout` is a writable stream to print log or info output.
    /// `stderr` is used for warning or error output.
    ///
    /// ## Arguments
    ///
    /// * `stdout` - A writable stream for log output
    /// * `stderr` - A writable stream for error output
    /// * `ignoreErrors` - Whether to ignore errors when writing
    #[wasm_bindgen(constructor, catch)]
    pub fn new(stdout: &Object) -> Result<Console, JsValue>;
    #[wasm_bindgen(constructor, catch, js_name = "Console")]
    pub fn new_with_stderr(stdout: &Object, stderr: &Object) -> Result<Console, JsValue>;
    #[wasm_bindgen(constructor, catch, js_name = "Console")]
    pub fn new_with_stderr_and_ignore_errors(
        stdout: &Object,
        stderr: &Object,
        ignore_errors: bool,
    ) -> Result<Console, JsValue>;
    /// A simple assertion test. If `condition` is falsy, an `AssertionError` is
    /// written with an optional message.
    ///
    /// ## Arguments
    ///
    /// * `condition` - The condition to test
    /// * `data` - Optional message or data
    #[wasm_bindgen(method, variadic)]
    pub fn assert(this: &Console, data: &[JsValue]);
    #[wasm_bindgen(method, variadic, catch, js_name = "assert")]
    pub fn try_assert(this: &Console, data: &[JsValue]) -> Result<(), JsValue>;
    #[wasm_bindgen(method, variadic, js_name = "assert")]
    pub fn assert_with_condition(this: &Console, condition: bool, data: &[JsValue]);
    #[wasm_bindgen(method, variadic, catch, js_name = "assert")]
    pub fn try_assert_with_condition(
        this: &Console,
        condition: bool,
        data: &[JsValue],
    ) -> Result<(), JsValue>;
    /// Clears the console when possible.
    #[wasm_bindgen(method)]
    pub fn clear(this: &Console);
    #[wasm_bindgen(method, catch, js_name = "clear")]
    pub fn try_clear(this: &Console) -> Result<(), JsValue>;
    /// Maintains an internal counter and outputs to `stdout` the number of
    /// times `count()` has been called with the given `label`.
    ///
    /// ## Arguments
    ///
    /// * `label` - The counter label
    #[wasm_bindgen(method)]
    pub fn count(this: &Console);
    #[wasm_bindgen(method, catch, js_name = "count")]
    pub fn try_count(this: &Console) -> Result<(), JsValue>;
    #[wasm_bindgen(method, js_name = "count")]
    pub fn count_with_label(this: &Console, label: &str);
    #[wasm_bindgen(method, catch, js_name = "count")]
    pub fn try_count_with_label(this: &Console, label: &str) -> Result<(), JsValue>;
    /// Resets the internal counter for the given `label`.
    ///
    /// ## Arguments
    ///
    /// * `label` - The counter label to reset
    #[wasm_bindgen(method, js_name = "countReset")]
    pub fn count_reset(this: &Console);
    #[wasm_bindgen(method, catch, js_name = "countReset")]
    pub fn try_count_reset(this: &Console) -> Result<(), JsValue>;
    #[wasm_bindgen(method, js_name = "countReset")]
    pub fn count_reset_with_label(this: &Console, label: &str);
    #[wasm_bindgen(method, catch, js_name = "countReset")]
    pub fn try_count_reset_with_label(
        this: &Console,
        label: &str,
    ) -> Result<(), JsValue>;
    /// Prints to `stderr` with newline. Multiple arguments can be passed.
    ///
    /// ## Arguments
    ///
    /// * `data` - Values to output
    #[wasm_bindgen(method, variadic)]
    pub fn debug(this: &Console, data: &[JsValue]);
    #[wasm_bindgen(method, variadic, catch, js_name = "debug")]
    pub fn try_debug(this: &Console, data: &[JsValue]) -> Result<(), JsValue>;
    /// Prints to `stderr` with newline. Multiple arguments can be passed.
    ///
    /// ## Arguments
    ///
    /// * `data` - Values to output
    #[wasm_bindgen(method, variadic)]
    pub fn error(this: &Console, data: &[JsValue]);
    #[wasm_bindgen(method, variadic, catch, js_name = "error")]
    pub fn try_error(this: &Console, data: &[JsValue]) -> Result<(), JsValue>;
    /// Increases indentation of subsequent lines by spaces for `groupIndentation` length.
    ///
    /// ## Arguments
    ///
    /// * `data` - Optional label for the group
    #[wasm_bindgen(method, variadic)]
    pub fn group(this: &Console, data: &[JsValue]);
    #[wasm_bindgen(method, variadic, catch, js_name = "group")]
    pub fn try_group(this: &Console, data: &[JsValue]) -> Result<(), JsValue>;
    /// An alias for `group()`.
    ///
    /// ## Arguments
    ///
    /// * `data` - Optional label for the group
    #[wasm_bindgen(method, variadic, js_name = "groupCollapsed")]
    pub fn group_collapsed(this: &Console, data: &[JsValue]);
    #[wasm_bindgen(method, variadic, catch, js_name = "groupCollapsed")]
    pub fn try_group_collapsed(this: &Console, data: &[JsValue]) -> Result<(), JsValue>;
    /// Decreases indentation of subsequent lines.
    #[wasm_bindgen(method, js_name = "groupEnd")]
    pub fn group_end(this: &Console);
    #[wasm_bindgen(method, catch, js_name = "groupEnd")]
    pub fn try_group_end(this: &Console) -> Result<(), JsValue>;
    /// Prints to `stdout` with newline. Multiple arguments can be passed.
    ///
    /// ## Arguments
    ///
    /// * `data` - Values to output
    #[wasm_bindgen(method, variadic)]
    pub fn info(this: &Console, data: &[JsValue]);
    #[wasm_bindgen(method, variadic, catch, js_name = "info")]
    pub fn try_info(this: &Console, data: &[JsValue]) -> Result<(), JsValue>;
    /// Prints to `stdout` with newline. Multiple arguments can be passed.
    /// First argument is used as the primary message and additional arguments
    /// are used as substitution values similar to `printf(3)`.
    ///
    /// ## Arguments
    ///
    /// * `data` - Values to output
    #[wasm_bindgen(method, variadic)]
    pub fn log(this: &Console, data: &[JsValue]);
    #[wasm_bindgen(method, variadic, catch, js_name = "log")]
    pub fn try_log(this: &Console, data: &[JsValue]) -> Result<(), JsValue>;
    /// Try to construct a table with the columns of the properties of
    /// `tabularData` (or use `properties`) and rows of `tabularData`.
    ///
    /// ## Arguments
    ///
    /// * `tabularData` - Data to display as a table
    /// * `properties` - Column headers to display
    #[wasm_bindgen(method)]
    pub fn table(this: &Console);
    #[wasm_bindgen(method, catch, js_name = "table")]
    pub fn try_table(this: &Console) -> Result<(), JsValue>;
    #[wasm_bindgen(method, js_name = "table")]
    pub fn table_with_tabular_data(this: &Console, tabular_data: &JsValue);
    #[wasm_bindgen(method, catch, js_name = "table")]
    pub fn try_table_with_tabular_data(
        this: &Console,
        tabular_data: &JsValue,
    ) -> Result<(), JsValue>;
    #[wasm_bindgen(method, js_name = "table")]
    pub fn table_with_tabular_data_and_properties(
        this: &Console,
        tabular_data: &JsValue,
        properties: &Array<JsString>,
    );
    #[wasm_bindgen(method, catch, js_name = "table")]
    pub fn try_table_with_tabular_data_and_properties(
        this: &Console,
        tabular_data: &JsValue,
        properties: &Array<JsString>,
    ) -> Result<(), JsValue>;
    /// Starts a timer identified by the given `label`.
    ///
    /// ## Arguments
    ///
    /// * `label` - The timer label
    #[wasm_bindgen(method)]
    pub fn time(this: &Console);
    #[wasm_bindgen(method, catch, js_name = "time")]
    pub fn try_time(this: &Console) -> Result<(), JsValue>;
    #[wasm_bindgen(method, js_name = "time")]
    pub fn time_with_label(this: &Console, label: &str);
    #[wasm_bindgen(method, catch, js_name = "time")]
    pub fn try_time_with_label(this: &Console, label: &str) -> Result<(), JsValue>;
    /// Stops a timer previously started with `time()` and prints the result.
    ///
    /// ## Arguments
    ///
    /// * `label` - The timer label
    #[wasm_bindgen(method, js_name = "timeEnd")]
    pub fn time_end(this: &Console);
    #[wasm_bindgen(method, catch, js_name = "timeEnd")]
    pub fn try_time_end(this: &Console) -> Result<(), JsValue>;
    #[wasm_bindgen(method, js_name = "timeEnd")]
    pub fn time_end_with_label(this: &Console, label: &str);
    #[wasm_bindgen(method, catch, js_name = "timeEnd")]
    pub fn try_time_end_with_label(this: &Console, label: &str) -> Result<(), JsValue>;
    /// For a timer previously started with `time()`, prints the elapsed time
    /// and other `data` arguments.
    ///
    /// ## Arguments
    ///
    /// * `label` - The timer label
    /// * `data` - Additional values to output
    #[wasm_bindgen(method, variadic, js_name = "timeLog")]
    pub fn time_log(this: &Console, data: &[JsValue]);
    #[wasm_bindgen(method, variadic, catch, js_name = "timeLog")]
    pub fn try_time_log(this: &Console, data: &[JsValue]) -> Result<(), JsValue>;
    #[wasm_bindgen(method, variadic, js_name = "timeLog")]
    pub fn time_log_with_label(this: &Console, label: &str, data: &[JsValue]);
    #[wasm_bindgen(method, variadic, catch, js_name = "timeLog")]
    pub fn try_time_log_with_label(
        this: &Console,
        label: &str,
        data: &[JsValue],
    ) -> Result<(), JsValue>;
    /// Prints to `stderr` the string `'Trace: '`, followed by `util.format()`
    /// output of the message and current stack trace.
    ///
    /// ## Arguments
    ///
    /// * `data` - Values to output alongside the trace
    #[wasm_bindgen(method, variadic)]
    pub fn trace(this: &Console, data: &[JsValue]);
    #[wasm_bindgen(method, variadic, catch, js_name = "trace")]
    pub fn try_trace(this: &Console, data: &[JsValue]) -> Result<(), JsValue>;
    /// Prints to `stderr` with newline. Multiple arguments can be passed.
    ///
    /// ## Arguments
    ///
    /// * `data` - Values to output
    #[wasm_bindgen(method, variadic)]
    pub fn warn(this: &Console, data: &[JsValue]);
    #[wasm_bindgen(method, variadic, catch, js_name = "warn")]
    pub fn try_warn(this: &Console, data: &[JsValue]) -> Result<(), JsValue>;
    /// Starts a JavaScript CPU profile with an optional label.
    ///
    /// ## Arguments
    ///
    /// * `label` - The profile label
    #[wasm_bindgen(method)]
    pub fn profile(this: &Console);
    #[wasm_bindgen(method, catch, js_name = "profile")]
    pub fn try_profile(this: &Console) -> Result<(), JsValue>;
    #[wasm_bindgen(method, js_name = "profile")]
    pub fn profile_with_label(this: &Console, label: &str);
    #[wasm_bindgen(method, catch, js_name = "profile")]
    pub fn try_profile_with_label(this: &Console, label: &str) -> Result<(), JsValue>;
    /// Stops the current JavaScript CPU profiling session.
    ///
    /// ## Arguments
    ///
    /// * `label` - The profile label to stop
    #[wasm_bindgen(method, js_name = "profileEnd")]
    pub fn profile_end(this: &Console);
    #[wasm_bindgen(method, catch, js_name = "profileEnd")]
    pub fn try_profile_end(this: &Console) -> Result<(), JsValue>;
    #[wasm_bindgen(method, js_name = "profileEnd")]
    pub fn profile_end_with_label(this: &Console, label: &str);
    #[wasm_bindgen(method, catch, js_name = "profileEnd")]
    pub fn try_profile_end_with_label(
        this: &Console,
        label: &str,
    ) -> Result<(), JsValue>;
    /// Adds an event with the label to the Timeline panel of the inspector.
    ///
    /// ## Arguments
    ///
    /// * `label` - The timestamp label
    #[wasm_bindgen(method, js_name = "timeStamp")]
    pub fn time_stamp(this: &Console);
    #[wasm_bindgen(method, catch, js_name = "timeStamp")]
    pub fn try_time_stamp(this: &Console) -> Result<(), JsValue>;
    #[wasm_bindgen(method, js_name = "timeStamp")]
    pub fn time_stamp_with_label(this: &Console, label: &str);
    #[wasm_bindgen(method, catch, js_name = "timeStamp")]
    pub fn try_time_stamp_with_label(this: &Console, label: &str) -> Result<(), JsValue>;
}
