exports.async_return_js_string = async function () {
  return "JsString from JavaScript!";
};

exports.async_return_jsvalue = async function () {
  return 123;
};

exports.async_return_custom_import = async function () {
  return {
    get value() {
      return 999;
    },
  };
};

exports.async_return_js_object = async function () {
  return { key: "value", nested: { prop: 42 } };
};

exports.async_return_js_array = async function () {
  return [1, 2, 3];
};

exports.async_return_js_number = async function () {
  return 456;
};

exports.async_return_custom_with_catch = async function () {
  return {
    get value() {
      return 999;
    },
  };
};

exports.async_return_unit = async function () {};

exports.async_return_unit_with_catch = async function () {};

exports.async_throw_jsstring_error = async function () {
  throw "JsString error message!";
};

exports.async_throw_unit_error = async function () {
  throw new Error("Unit error!");
};

exports.async_return_bigint = async function () {
  return 9007199254740991n;
};

exports.async_return_uint8array = async function () {
  return new Uint8Array([1, 2, 3, 4, 5]);
};

exports.async_return_f64 = async function () {
  return 3.14159;
};

exports.async_return_i32 = async function () {
  return -42;
};

exports.async_return_u32 = async function () {
  return 42;
};

exports.async_return_f32 = async function () {
  return 3.14;
};

exports.async_return_i8 = async function () {
  return -127;
};

exports.async_return_u8 = async function () {
  return 255;
};

exports.async_return_i16 = async function () {
  return -32767;
};

exports.async_return_u16 = async function () {
  return 65535;
};

exports.async_return_i64 = async function () {
  return 9007199254740991n;
};

exports.async_return_u64 = async function () {
  return 18446744073709551615n;
};

exports.async_return_bool = async function () {
  return true;
};

exports.async_return_char = async function () {
  return "A";
};

exports.async_return_string = async function () {
  return "Hello async!";
};

exports.async_return_option_i32_some = async function () {
  return 42;
};

exports.async_return_option_i32_none = async function () {
  return null;
};

exports.async_return_option_string_none = async function () {
  return undefined;
};

exports.async_return_option_string_some = async function () {
  return "optional string";
};

exports.async_return_option_jsstring_some = async function () {
  return "JsString option value";
};

exports.async_return_option_jsstring_none = async function () {
  return null;
};

exports.async_return_result_option_jsstring_some = async function () {
  return "Result Option JsString value";
};

exports.async_return_result_option_jsstring_none = async function () {
  return null;
};

exports.async_return_result_u32_ok = async function () {
  return 42;
};

exports.async_return_result_u32_err = async function () {
  throw new Error("u32 error!");
};

exports.async_return_result_option_u32_ok_some = async function () {
  return 123;
};

exports.async_return_result_option_u32_ok_none = async function () {
  return null;
};

exports.async_return_result_option_u32_err = async function () {
  throw new Error("Option<u32> error!");
};

