// Fixtures for async_import.rs: promises resolving to non-handle values.

exports.asyncU32 = async function() {
  await null;
  return 4294967295; // u32::MAX also catches a signed reinterpretation
};

exports.asyncI32 = async function() {
  await null;
  return -2147483648; // i32::MIN
};

exports.asyncF64 = async function() {
  await null;
  return 1.5;
};

exports.asyncBool = async function() {
  await null;
  return true;
};

exports.asyncString = async function() {
  await null;
  return 'hello';
};

exports.asyncOptSome = async function() {
  await null;
  return 7;
};

exports.asyncOptNone = async function() {
  await null;
  return undefined;
};

exports.asyncU32Throws = async function() {
  await null;
  throw new Error('boom');
};

// Echoes the argument back through a promise, so the argument direction is
// covered as well as the return direction.
exports.asyncEchoU32 = async function(x) {
  await null;
  if (typeof x !== 'number') {
    throw new Error(`expected a number, got ${typeof x}`);
  }
  return x;
};
