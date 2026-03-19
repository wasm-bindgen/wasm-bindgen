#[allow(unused_imports)]
use js_sys::*;
#[allow(unused_imports)]
use wasm_bindgen::prelude::*;
#[doc = r" Extension trait for awaiting `js_sys::Promise<T>`."]
#[doc = r""]
#[doc = r" Since `IntoFuture` can't be implemented for `js_sys::Promise` from"]
#[doc = r" generated code (orphan rule), use `.into_future().await` instead:"]
#[doc = r" ```ignore"]
#[doc = r" use bindings::PromiseExt;"]
#[doc = r" let data: ArrayBuffer = promise.into_future().await?;"]
#[doc = r" ```"]
#[allow(dead_code)]
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
    # [wasm_bindgen (extends = Object)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub type ConsoleOptions;
    #[wasm_bindgen(method, getter)]
    pub fn stdout(this: &ConsoleOptions) -> Object;
    #[wasm_bindgen(method, setter)]
    pub fn set_stdout(this: &ConsoleOptions, val: &Object);
    #[wasm_bindgen(method, getter)]
    pub fn stderr(this: &ConsoleOptions) -> Option<Object>;
    #[wasm_bindgen(method, setter)]
    pub fn set_stderr(this: &ConsoleOptions, val: &Object);
    #[wasm_bindgen(method, setter, js_name = "stderr")]
    pub fn set_stderr_with_null(this: &ConsoleOptions, val: &Null);
    #[doc = " Ignore errors when writing to the underlying streams."]
    #[doc = " @default true"]
    #[wasm_bindgen(method, getter, js_name = "ignoreErrors")]
    pub fn ignore_errors(this: &ConsoleOptions) -> Option<bool>;
    #[wasm_bindgen(method, setter, js_name = "ignoreErrors")]
    pub fn set_ignore_errors(this: &ConsoleOptions, val: bool);
    #[wasm_bindgen(method, setter, js_name = "ignoreErrors")]
    pub fn set_ignore_errors_with_null(this: &ConsoleOptions, val: &Null);
    #[doc = " Set color support for this `Console` instance."]
    #[doc = " @default 'auto'"]
    #[wasm_bindgen(method, getter, js_name = "colorMode")]
    pub fn color_mode(this: &ConsoleOptions) -> Option<JsValue>;
    #[wasm_bindgen(method, setter, js_name = "colorMode")]
    pub fn set_color_mode(this: &ConsoleOptions, val: bool);
    #[wasm_bindgen(method, setter, js_name = "colorMode")]
    pub fn set_color_mode_with_str(this: &ConsoleOptions, val: &str);
    #[wasm_bindgen(method, setter, js_name = "colorMode")]
    pub fn set_color_mode_with_null(this: &ConsoleOptions, val: &Null);
    #[doc = " Set group indentation."]
    #[doc = " @default 2"]
    #[wasm_bindgen(method, getter, js_name = "groupIndentation")]
    pub fn group_indentation(this: &ConsoleOptions) -> Option<f64>;
    #[wasm_bindgen(method, setter, js_name = "groupIndentation")]
    pub fn set_group_indentation(this: &ConsoleOptions, val: f64);
    #[wasm_bindgen(method, setter, js_name = "groupIndentation")]
    pub fn set_group_indentation_with_null(this: &ConsoleOptions, val: &Null);
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
    pub fn stderr(mut self, val: &Object) -> Self {
        self.inner.set_stderr(val);
        self
    }
    pub fn stderr_with_null(mut self, val: &Null) -> Self {
        self.inner.set_stderr_with_null(val);
        self
    }
    pub fn ignore_errors(mut self, val: bool) -> Self {
        self.inner.set_ignore_errors(val);
        self
    }
    pub fn ignore_errors_with_null(mut self, val: &Null) -> Self {
        self.inner.set_ignore_errors_with_null(val);
        self
    }
    pub fn color_mode(mut self, val: bool) -> Self {
        self.inner.set_color_mode(val);
        self
    }
    pub fn color_mode_with_str(mut self, val: &str) -> Self {
        self.inner.set_color_mode_with_str(val);
        self
    }
    pub fn color_mode_with_null(mut self, val: &Null) -> Self {
        self.inner.set_color_mode_with_null(val);
        self
    }
    pub fn group_indentation(mut self, val: f64) -> Self {
        self.inner.set_group_indentation(val);
        self
    }
    pub fn group_indentation_with_null(mut self, val: &Null) -> Self {
        self.inner.set_group_indentation_with_null(val);
        self
    }
    pub fn build(self) -> Result<ConsoleOptions, JsValue> {
        if self.required != 0 {
            let mut missing = Vec::new();
            if self.required & 1u64 != 0 {
                missing.push("missing required property `stdout`");
            }
            return Err(JsValue::from_str(&format!(
                "{}: {}",
                stringify!(ConsoleOptions),
                missing.join(", ")
            )));
        }
        Ok(self.inner)
    }
}
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = Object)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub type Console;
    #[doc = " Creates a new `Console` with one or two writable stream instances."]
    #[doc = " `stdout` is a writable stream to print log or info output."]
    #[doc = " `stderr` is used for warning or error output."]
    #[doc = " "]
    #[doc = " ## Arguments"]
    #[doc = " "]
    #[doc = " * `stdout` - A writable stream for log output"]
    #[doc = " * `stderr` - A writable stream for error output"]
    #[doc = " * `ignoreErrors` - Whether to ignore errors when writing"]
    #[wasm_bindgen(constructor, catch)]
    pub fn new(stdout: &Object) -> Result<Console, JsValue>;
    #[doc = " Creates a new `Console` with one or two writable stream instances."]
    #[doc = " `stdout` is a writable stream to print log or info output."]
    #[doc = " `stderr` is used for warning or error output."]
    #[doc = " "]
    #[doc = " ## Arguments"]
    #[doc = " "]
    #[doc = " * `stdout` - A writable stream for log output"]
    #[doc = " * `stderr` - A writable stream for error output"]
    #[doc = " * `ignoreErrors` - Whether to ignore errors when writing"]
    #[wasm_bindgen(constructor, catch, js_name = "Console")]
    pub fn new_with_stderr(stdout: &Object, stderr: &Object) -> Result<Console, JsValue>;
    #[doc = " Creates a new `Console` with one or two writable stream instances."]
    #[doc = " `stdout` is a writable stream to print log or info output."]
    #[doc = " `stderr` is used for warning or error output."]
    #[doc = " "]
    #[doc = " ## Arguments"]
    #[doc = " "]
    #[doc = " * `stdout` - A writable stream for log output"]
    #[doc = " * `stderr` - A writable stream for error output"]
    #[doc = " * `ignoreErrors` - Whether to ignore errors when writing"]
    #[wasm_bindgen(constructor, catch, js_name = "Console")]
    pub fn new_with_stderr_and_ignore_errors(
        stdout: &Object,
        stderr: &Object,
        ignore_errors: bool,
    ) -> Result<Console, JsValue>;
    #[doc = " A simple assertion test. If `condition` is falsy, an `AssertionError` is"]
    #[doc = " written with an optional message."]
    #[doc = " "]
    #[doc = " ## Arguments"]
    #[doc = " "]
    #[doc = " * `condition` - The condition to test"]
    #[doc = " * `data` - Optional message or data"]
    #[wasm_bindgen(method, variadic)]
    pub fn assert(this: &Console, data: &[JsValue]);
    #[doc = " A simple assertion test. If `condition` is falsy, an `AssertionError` is"]
    #[doc = " written with an optional message."]
    #[doc = " "]
    #[doc = " ## Arguments"]
    #[doc = " "]
    #[doc = " * `condition` - The condition to test"]
    #[doc = " * `data` - Optional message or data"]
    #[wasm_bindgen(method, variadic, catch, js_name = "assert")]
    pub fn try_assert(this: &Console, data: &[JsValue]) -> Result<(), JsValue>;
    #[doc = " A simple assertion test. If `condition` is falsy, an `AssertionError` is"]
    #[doc = " written with an optional message."]
    #[doc = " "]
    #[doc = " ## Arguments"]
    #[doc = " "]
    #[doc = " * `condition` - The condition to test"]
    #[doc = " * `data` - Optional message or data"]
    #[wasm_bindgen(method, variadic, js_name = "assert")]
    pub fn assert_with_condition(this: &Console, condition: bool, data: &[JsValue]);
    #[doc = " A simple assertion test. If `condition` is falsy, an `AssertionError` is"]
    #[doc = " written with an optional message."]
    #[doc = " "]
    #[doc = " ## Arguments"]
    #[doc = " "]
    #[doc = " * `condition` - The condition to test"]
    #[doc = " * `data` - Optional message or data"]
    #[wasm_bindgen(method, variadic, catch, js_name = "assert")]
    pub fn try_assert_with_condition(
        this: &Console,
        condition: bool,
        data: &[JsValue],
    ) -> Result<(), JsValue>;
    #[doc = " Clears the console when possible."]
    #[wasm_bindgen(method)]
    pub fn clear(this: &Console);
    #[doc = " Clears the console when possible."]
    #[wasm_bindgen(method, catch, js_name = "clear")]
    pub fn try_clear(this: &Console) -> Result<(), JsValue>;
    #[doc = " Maintains an internal counter and outputs to `stdout` the number of"]
    #[doc = " times `count()` has been called with the given `label`."]
    #[doc = " "]
    #[doc = " ## Arguments"]
    #[doc = " "]
    #[doc = " * `label` - The counter label"]
    #[wasm_bindgen(method)]
    pub fn count(this: &Console);
    #[doc = " Maintains an internal counter and outputs to `stdout` the number of"]
    #[doc = " times `count()` has been called with the given `label`."]
    #[doc = " "]
    #[doc = " ## Arguments"]
    #[doc = " "]
    #[doc = " * `label` - The counter label"]
    #[wasm_bindgen(method, catch, js_name = "count")]
    pub fn try_count(this: &Console) -> Result<(), JsValue>;
    #[doc = " Maintains an internal counter and outputs to `stdout` the number of"]
    #[doc = " times `count()` has been called with the given `label`."]
    #[doc = " "]
    #[doc = " ## Arguments"]
    #[doc = " "]
    #[doc = " * `label` - The counter label"]
    #[wasm_bindgen(method, js_name = "count")]
    pub fn count_with_label(this: &Console, label: &str);
    #[doc = " Maintains an internal counter and outputs to `stdout` the number of"]
    #[doc = " times `count()` has been called with the given `label`."]
    #[doc = " "]
    #[doc = " ## Arguments"]
    #[doc = " "]
    #[doc = " * `label` - The counter label"]
    #[wasm_bindgen(method, catch, js_name = "count")]
    pub fn try_count_with_label(this: &Console, label: &str) -> Result<(), JsValue>;
    #[doc = " Resets the internal counter for the given `label`."]
    #[doc = " "]
    #[doc = " ## Arguments"]
    #[doc = " "]
    #[doc = " * `label` - The counter label to reset"]
    #[wasm_bindgen(method, js_name = "countReset")]
    pub fn count_reset(this: &Console);
    #[doc = " Resets the internal counter for the given `label`."]
    #[doc = " "]
    #[doc = " ## Arguments"]
    #[doc = " "]
    #[doc = " * `label` - The counter label to reset"]
    #[wasm_bindgen(method, catch, js_name = "countReset")]
    pub fn try_count_reset(this: &Console) -> Result<(), JsValue>;
    #[doc = " Resets the internal counter for the given `label`."]
    #[doc = " "]
    #[doc = " ## Arguments"]
    #[doc = " "]
    #[doc = " * `label` - The counter label to reset"]
    #[wasm_bindgen(method, js_name = "countReset")]
    pub fn count_reset_with_label(this: &Console, label: &str);
    #[doc = " Resets the internal counter for the given `label`."]
    #[doc = " "]
    #[doc = " ## Arguments"]
    #[doc = " "]
    #[doc = " * `label` - The counter label to reset"]
    #[wasm_bindgen(method, catch, js_name = "countReset")]
    pub fn try_count_reset_with_label(this: &Console, label: &str) -> Result<(), JsValue>;
    #[doc = " Prints to `stderr` with newline. Multiple arguments can be passed."]
    #[doc = " "]
    #[doc = " ## Arguments"]
    #[doc = " "]
    #[doc = " * `data` - Values to output"]
    #[wasm_bindgen(method, variadic)]
    pub fn debug(this: &Console, data: &[JsValue]);
    #[doc = " Prints to `stderr` with newline. Multiple arguments can be passed."]
    #[doc = " "]
    #[doc = " ## Arguments"]
    #[doc = " "]
    #[doc = " * `data` - Values to output"]
    #[wasm_bindgen(method, variadic, catch, js_name = "debug")]
    pub fn try_debug(this: &Console, data: &[JsValue]) -> Result<(), JsValue>;
    #[doc = " Prints to `stderr` with newline. Multiple arguments can be passed."]
    #[doc = " "]
    #[doc = " ## Arguments"]
    #[doc = " "]
    #[doc = " * `data` - Values to output"]
    #[wasm_bindgen(method, variadic)]
    pub fn error(this: &Console, data: &[JsValue]);
    #[doc = " Prints to `stderr` with newline. Multiple arguments can be passed."]
    #[doc = " "]
    #[doc = " ## Arguments"]
    #[doc = " "]
    #[doc = " * `data` - Values to output"]
    #[wasm_bindgen(method, variadic, catch, js_name = "error")]
    pub fn try_error(this: &Console, data: &[JsValue]) -> Result<(), JsValue>;
    #[doc = " Increases indentation of subsequent lines by spaces for `groupIndentation` length."]
    #[doc = " "]
    #[doc = " ## Arguments"]
    #[doc = " "]
    #[doc = " * `data` - Optional label for the group"]
    #[wasm_bindgen(method, variadic)]
    pub fn group(this: &Console, data: &[JsValue]);
    #[doc = " Increases indentation of subsequent lines by spaces for `groupIndentation` length."]
    #[doc = " "]
    #[doc = " ## Arguments"]
    #[doc = " "]
    #[doc = " * `data` - Optional label for the group"]
    #[wasm_bindgen(method, variadic, catch, js_name = "group")]
    pub fn try_group(this: &Console, data: &[JsValue]) -> Result<(), JsValue>;
    #[doc = " An alias for `group()`."]
    #[doc = " "]
    #[doc = " ## Arguments"]
    #[doc = " "]
    #[doc = " * `data` - Optional label for the group"]
    #[wasm_bindgen(method, variadic, js_name = "groupCollapsed")]
    pub fn group_collapsed(this: &Console, data: &[JsValue]);
    #[doc = " An alias for `group()`."]
    #[doc = " "]
    #[doc = " ## Arguments"]
    #[doc = " "]
    #[doc = " * `data` - Optional label for the group"]
    #[wasm_bindgen(method, variadic, catch, js_name = "groupCollapsed")]
    pub fn try_group_collapsed(this: &Console, data: &[JsValue]) -> Result<(), JsValue>;
    #[doc = " Decreases indentation of subsequent lines."]
    #[wasm_bindgen(method, js_name = "groupEnd")]
    pub fn group_end(this: &Console);
    #[doc = " Decreases indentation of subsequent lines."]
    #[wasm_bindgen(method, catch, js_name = "groupEnd")]
    pub fn try_group_end(this: &Console) -> Result<(), JsValue>;
    #[doc = " Prints to `stdout` with newline. Multiple arguments can be passed."]
    #[doc = " "]
    #[doc = " ## Arguments"]
    #[doc = " "]
    #[doc = " * `data` - Values to output"]
    #[wasm_bindgen(method, variadic)]
    pub fn info(this: &Console, data: &[JsValue]);
    #[doc = " Prints to `stdout` with newline. Multiple arguments can be passed."]
    #[doc = " "]
    #[doc = " ## Arguments"]
    #[doc = " "]
    #[doc = " * `data` - Values to output"]
    #[wasm_bindgen(method, variadic, catch, js_name = "info")]
    pub fn try_info(this: &Console, data: &[JsValue]) -> Result<(), JsValue>;
    #[doc = " Prints to `stdout` with newline. Multiple arguments can be passed."]
    #[doc = " First argument is used as the primary message and additional arguments"]
    #[doc = " are used as substitution values similar to `printf(3)`."]
    #[doc = " "]
    #[doc = " ## Arguments"]
    #[doc = " "]
    #[doc = " * `data` - Values to output"]
    #[wasm_bindgen(method, variadic)]
    pub fn log(this: &Console, data: &[JsValue]);
    #[doc = " Prints to `stdout` with newline. Multiple arguments can be passed."]
    #[doc = " First argument is used as the primary message and additional arguments"]
    #[doc = " are used as substitution values similar to `printf(3)`."]
    #[doc = " "]
    #[doc = " ## Arguments"]
    #[doc = " "]
    #[doc = " * `data` - Values to output"]
    #[wasm_bindgen(method, variadic, catch, js_name = "log")]
    pub fn try_log(this: &Console, data: &[JsValue]) -> Result<(), JsValue>;
    #[doc = " Try to construct a table with the columns of the properties of"]
    #[doc = " `tabularData` (or use `properties`) and rows of `tabularData`."]
    #[doc = " "]
    #[doc = " ## Arguments"]
    #[doc = " "]
    #[doc = " * `tabularData` - Data to display as a table"]
    #[doc = " * `properties` - Column headers to display"]
    #[wasm_bindgen(method)]
    pub fn table(this: &Console);
    #[doc = " Try to construct a table with the columns of the properties of"]
    #[doc = " `tabularData` (or use `properties`) and rows of `tabularData`."]
    #[doc = " "]
    #[doc = " ## Arguments"]
    #[doc = " "]
    #[doc = " * `tabularData` - Data to display as a table"]
    #[doc = " * `properties` - Column headers to display"]
    #[wasm_bindgen(method, catch, js_name = "table")]
    pub fn try_table(this: &Console) -> Result<(), JsValue>;
    #[doc = " Try to construct a table with the columns of the properties of"]
    #[doc = " `tabularData` (or use `properties`) and rows of `tabularData`."]
    #[doc = " "]
    #[doc = " ## Arguments"]
    #[doc = " "]
    #[doc = " * `tabularData` - Data to display as a table"]
    #[doc = " * `properties` - Column headers to display"]
    #[wasm_bindgen(method, js_name = "table")]
    pub fn table_with_tabular_data(this: &Console, tabular_data: &JsValue);
    #[doc = " Try to construct a table with the columns of the properties of"]
    #[doc = " `tabularData` (or use `properties`) and rows of `tabularData`."]
    #[doc = " "]
    #[doc = " ## Arguments"]
    #[doc = " "]
    #[doc = " * `tabularData` - Data to display as a table"]
    #[doc = " * `properties` - Column headers to display"]
    #[wasm_bindgen(method, catch, js_name = "table")]
    pub fn try_table_with_tabular_data(
        this: &Console,
        tabular_data: &JsValue,
    ) -> Result<(), JsValue>;
    #[doc = " Try to construct a table with the columns of the properties of"]
    #[doc = " `tabularData` (or use `properties`) and rows of `tabularData`."]
    #[doc = " "]
    #[doc = " ## Arguments"]
    #[doc = " "]
    #[doc = " * `tabularData` - Data to display as a table"]
    #[doc = " * `properties` - Column headers to display"]
    #[wasm_bindgen(method, js_name = "table")]
    pub fn table_with_tabular_data_and_properties(
        this: &Console,
        tabular_data: &JsValue,
        properties: &Array<JsString>,
    );
    #[doc = " Try to construct a table with the columns of the properties of"]
    #[doc = " `tabularData` (or use `properties`) and rows of `tabularData`."]
    #[doc = " "]
    #[doc = " ## Arguments"]
    #[doc = " "]
    #[doc = " * `tabularData` - Data to display as a table"]
    #[doc = " * `properties` - Column headers to display"]
    #[wasm_bindgen(method, catch, js_name = "table")]
    pub fn try_table_with_tabular_data_and_properties(
        this: &Console,
        tabular_data: &JsValue,
        properties: &Array<JsString>,
    ) -> Result<(), JsValue>;
    #[doc = " Starts a timer identified by the given `label`."]
    #[doc = " "]
    #[doc = " ## Arguments"]
    #[doc = " "]
    #[doc = " * `label` - The timer label"]
    #[wasm_bindgen(method)]
    pub fn time(this: &Console);
    #[doc = " Starts a timer identified by the given `label`."]
    #[doc = " "]
    #[doc = " ## Arguments"]
    #[doc = " "]
    #[doc = " * `label` - The timer label"]
    #[wasm_bindgen(method, catch, js_name = "time")]
    pub fn try_time(this: &Console) -> Result<(), JsValue>;
    #[doc = " Starts a timer identified by the given `label`."]
    #[doc = " "]
    #[doc = " ## Arguments"]
    #[doc = " "]
    #[doc = " * `label` - The timer label"]
    #[wasm_bindgen(method, js_name = "time")]
    pub fn time_with_label(this: &Console, label: &str);
    #[doc = " Starts a timer identified by the given `label`."]
    #[doc = " "]
    #[doc = " ## Arguments"]
    #[doc = " "]
    #[doc = " * `label` - The timer label"]
    #[wasm_bindgen(method, catch, js_name = "time")]
    pub fn try_time_with_label(this: &Console, label: &str) -> Result<(), JsValue>;
    #[doc = " Stops a timer previously started with `time()` and prints the result."]
    #[doc = " "]
    #[doc = " ## Arguments"]
    #[doc = " "]
    #[doc = " * `label` - The timer label"]
    #[wasm_bindgen(method, js_name = "timeEnd")]
    pub fn time_end(this: &Console);
    #[doc = " Stops a timer previously started with `time()` and prints the result."]
    #[doc = " "]
    #[doc = " ## Arguments"]
    #[doc = " "]
    #[doc = " * `label` - The timer label"]
    #[wasm_bindgen(method, catch, js_name = "timeEnd")]
    pub fn try_time_end(this: &Console) -> Result<(), JsValue>;
    #[doc = " Stops a timer previously started with `time()` and prints the result."]
    #[doc = " "]
    #[doc = " ## Arguments"]
    #[doc = " "]
    #[doc = " * `label` - The timer label"]
    #[wasm_bindgen(method, js_name = "timeEnd")]
    pub fn time_end_with_label(this: &Console, label: &str);
    #[doc = " Stops a timer previously started with `time()` and prints the result."]
    #[doc = " "]
    #[doc = " ## Arguments"]
    #[doc = " "]
    #[doc = " * `label` - The timer label"]
    #[wasm_bindgen(method, catch, js_name = "timeEnd")]
    pub fn try_time_end_with_label(this: &Console, label: &str) -> Result<(), JsValue>;
    #[doc = " For a timer previously started with `time()`, prints the elapsed time"]
    #[doc = " and other `data` arguments."]
    #[doc = " "]
    #[doc = " ## Arguments"]
    #[doc = " "]
    #[doc = " * `label` - The timer label"]
    #[doc = " * `data` - Additional values to output"]
    #[wasm_bindgen(method, variadic, js_name = "timeLog")]
    pub fn time_log(this: &Console, data: &[JsValue]);
    #[doc = " For a timer previously started with `time()`, prints the elapsed time"]
    #[doc = " and other `data` arguments."]
    #[doc = " "]
    #[doc = " ## Arguments"]
    #[doc = " "]
    #[doc = " * `label` - The timer label"]
    #[doc = " * `data` - Additional values to output"]
    #[wasm_bindgen(method, variadic, catch, js_name = "timeLog")]
    pub fn try_time_log(this: &Console, data: &[JsValue]) -> Result<(), JsValue>;
    #[doc = " For a timer previously started with `time()`, prints the elapsed time"]
    #[doc = " and other `data` arguments."]
    #[doc = " "]
    #[doc = " ## Arguments"]
    #[doc = " "]
    #[doc = " * `label` - The timer label"]
    #[doc = " * `data` - Additional values to output"]
    #[wasm_bindgen(method, variadic, js_name = "timeLog")]
    pub fn time_log_with_label(this: &Console, label: &str, data: &[JsValue]);
    #[doc = " For a timer previously started with `time()`, prints the elapsed time"]
    #[doc = " and other `data` arguments."]
    #[doc = " "]
    #[doc = " ## Arguments"]
    #[doc = " "]
    #[doc = " * `label` - The timer label"]
    #[doc = " * `data` - Additional values to output"]
    #[wasm_bindgen(method, variadic, catch, js_name = "timeLog")]
    pub fn try_time_log_with_label(
        this: &Console,
        label: &str,
        data: &[JsValue],
    ) -> Result<(), JsValue>;
    #[doc = " Prints to `stderr` the string `'Trace: '`, followed by `util.format()`"]
    #[doc = " output of the message and current stack trace."]
    #[doc = " "]
    #[doc = " ## Arguments"]
    #[doc = " "]
    #[doc = " * `data` - Values to output alongside the trace"]
    #[wasm_bindgen(method, variadic)]
    pub fn trace(this: &Console, data: &[JsValue]);
    #[doc = " Prints to `stderr` the string `'Trace: '`, followed by `util.format()`"]
    #[doc = " output of the message and current stack trace."]
    #[doc = " "]
    #[doc = " ## Arguments"]
    #[doc = " "]
    #[doc = " * `data` - Values to output alongside the trace"]
    #[wasm_bindgen(method, variadic, catch, js_name = "trace")]
    pub fn try_trace(this: &Console, data: &[JsValue]) -> Result<(), JsValue>;
    #[doc = " Prints to `stderr` with newline. Multiple arguments can be passed."]
    #[doc = " "]
    #[doc = " ## Arguments"]
    #[doc = " "]
    #[doc = " * `data` - Values to output"]
    #[wasm_bindgen(method, variadic)]
    pub fn warn(this: &Console, data: &[JsValue]);
    #[doc = " Prints to `stderr` with newline. Multiple arguments can be passed."]
    #[doc = " "]
    #[doc = " ## Arguments"]
    #[doc = " "]
    #[doc = " * `data` - Values to output"]
    #[wasm_bindgen(method, variadic, catch, js_name = "warn")]
    pub fn try_warn(this: &Console, data: &[JsValue]) -> Result<(), JsValue>;
    #[doc = " Starts a JavaScript CPU profile with an optional label."]
    #[doc = " "]
    #[doc = " ## Arguments"]
    #[doc = " "]
    #[doc = " * `label` - The profile label"]
    #[wasm_bindgen(method)]
    pub fn profile(this: &Console);
    #[doc = " Starts a JavaScript CPU profile with an optional label."]
    #[doc = " "]
    #[doc = " ## Arguments"]
    #[doc = " "]
    #[doc = " * `label` - The profile label"]
    #[wasm_bindgen(method, catch, js_name = "profile")]
    pub fn try_profile(this: &Console) -> Result<(), JsValue>;
    #[doc = " Starts a JavaScript CPU profile with an optional label."]
    #[doc = " "]
    #[doc = " ## Arguments"]
    #[doc = " "]
    #[doc = " * `label` - The profile label"]
    #[wasm_bindgen(method, js_name = "profile")]
    pub fn profile_with_label(this: &Console, label: &str);
    #[doc = " Starts a JavaScript CPU profile with an optional label."]
    #[doc = " "]
    #[doc = " ## Arguments"]
    #[doc = " "]
    #[doc = " * `label` - The profile label"]
    #[wasm_bindgen(method, catch, js_name = "profile")]
    pub fn try_profile_with_label(this: &Console, label: &str) -> Result<(), JsValue>;
    #[doc = " Stops the current JavaScript CPU profiling session."]
    #[doc = " "]
    #[doc = " ## Arguments"]
    #[doc = " "]
    #[doc = " * `label` - The profile label to stop"]
    #[wasm_bindgen(method, js_name = "profileEnd")]
    pub fn profile_end(this: &Console);
    #[doc = " Stops the current JavaScript CPU profiling session."]
    #[doc = " "]
    #[doc = " ## Arguments"]
    #[doc = " "]
    #[doc = " * `label` - The profile label to stop"]
    #[wasm_bindgen(method, catch, js_name = "profileEnd")]
    pub fn try_profile_end(this: &Console) -> Result<(), JsValue>;
    #[doc = " Stops the current JavaScript CPU profiling session."]
    #[doc = " "]
    #[doc = " ## Arguments"]
    #[doc = " "]
    #[doc = " * `label` - The profile label to stop"]
    #[wasm_bindgen(method, js_name = "profileEnd")]
    pub fn profile_end_with_label(this: &Console, label: &str);
    #[doc = " Stops the current JavaScript CPU profiling session."]
    #[doc = " "]
    #[doc = " ## Arguments"]
    #[doc = " "]
    #[doc = " * `label` - The profile label to stop"]
    #[wasm_bindgen(method, catch, js_name = "profileEnd")]
    pub fn try_profile_end_with_label(this: &Console, label: &str) -> Result<(), JsValue>;
    #[doc = " Adds an event with the label to the Timeline panel of the inspector."]
    #[doc = " "]
    #[doc = " ## Arguments"]
    #[doc = " "]
    #[doc = " * `label` - The timestamp label"]
    #[wasm_bindgen(method, js_name = "timeStamp")]
    pub fn time_stamp(this: &Console);
    #[doc = " Adds an event with the label to the Timeline panel of the inspector."]
    #[doc = " "]
    #[doc = " ## Arguments"]
    #[doc = " "]
    #[doc = " * `label` - The timestamp label"]
    #[wasm_bindgen(method, catch, js_name = "timeStamp")]
    pub fn try_time_stamp(this: &Console) -> Result<(), JsValue>;
    #[doc = " Adds an event with the label to the Timeline panel of the inspector."]
    #[doc = " "]
    #[doc = " ## Arguments"]
    #[doc = " "]
    #[doc = " * `label` - The timestamp label"]
    #[wasm_bindgen(method, js_name = "timeStamp")]
    pub fn time_stamp_with_label(this: &Console, label: &str);
    #[doc = " Adds an event with the label to the Timeline panel of the inspector."]
    #[doc = " "]
    #[doc = " ## Arguments"]
    #[doc = " "]
    #[doc = " * `label` - The timestamp label"]
    #[wasm_bindgen(method, catch, js_name = "timeStamp")]
    pub fn try_time_stamp_with_label(this: &Console, label: &str) -> Result<(), JsValue>;
}
