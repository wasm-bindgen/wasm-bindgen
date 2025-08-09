function fn_ret(val) {
  return () => val;
}

export function processWithCallback(value, callback) {
  const val = callback(value);
  return fn_ret(val);
}

export function transformWithCallback(value, callback) {
  const val = callback(value);
  return fn_ret(val);
}

export function processWithMutCallback(values, callback) {
  let result = null;
  for (const value of values) {
    result = callback(value);
  }
  return fn_ret(result);
}

export function processJsString(input, callback) {
  const val = callback(input);
  return fn_ret(val);
}

export function processOption(value, callback) {
  const val = callback(value);
  return fn_ret(val);
}
