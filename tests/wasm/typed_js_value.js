// JavaScript functions for testing typed JsValue round-tripping

export function acceptsUntypedValue(val) {
    return typeof val === 'string';
}

export function acceptsUntypedRef(val) {
    return typeof val === 'object' && val !== null;
}

export function acceptsTypedValue(val) {
    return val;
}

export function acceptsTypedRef(val) {
    return val;
}

// Test RefFromWasmAbi behavior
let lastSeenRef = null;
export function testRefFromWasmAbi(val) {
    const currentRef = val;
    const isSameRef = lastSeenRef === currentRef;
    lastSeenRef = currentRef;
    return isSameRef;
}

export function resetRefTracking() {
    lastSeenRef = null;
}

export function modifyObjectProperty(obj, key, value) {
    obj[key] = value;
    return obj[key];
}

export function testRefFromWasmAbiViaRust(value, rustFn) {
    // Call the Rust function that was passed in
    // This will exercise RefFromWasmAbi on the Rust side when it receives the reference
    return rustFn(value);
}

// Note: rust_receives_typed_jsvalue_ref will be available from the wasm module
// and can be called directly from tests
