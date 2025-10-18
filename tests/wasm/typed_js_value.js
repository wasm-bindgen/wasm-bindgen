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
