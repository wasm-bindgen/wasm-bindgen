/*
 * CSSOM View Module - Element scrollTop/scrollLeft type corrections
 *
 * The CSSOM View spec defines scrollTop and scrollLeft as `unrestricted double`,
 * but the Mozilla/Gecko IDL files use `long`. This override provides the correct
 * f64 types behind the unstable API gate.
 *
 * Spec: https://drafts.csswg.org/cssom-view/#extension-to-the-element-interface
 * Issue: https://github.com/nicksenger/wasm-bindgen/issues/4525
 */

partial interface Element {
  attribute unrestricted double scrollTop;
  attribute unrestricted double scrollLeft;
};

/* Mozilla-specific HTMLElement.scrollTop (not in spec, but exists in stable) */
partial interface HTMLElement {
  attribute unrestricted double scrollTop;
};
