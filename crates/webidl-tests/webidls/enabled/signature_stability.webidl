[Constructor()]
interface SignatureStability {
  DOMString process();
};

// Test case for mixin method override: stable uses double, unstable uses long
// This tests that the stable method name is NOT changed when unstable adds
// a method with the same JS name but different parameter types.
[Constructor()]
interface ImageDataLike {
};

[Constructor()]
interface CanvasLike {
};

CanvasLike includes CanvasImageDataMixin;

interface mixin CanvasImageDataMixin {
  // Stable method uses double (f64) parameters
  undefined putImageData(ImageDataLike imagedata, double dx, double dy);
};

// Test case for identical Rust signatures with different attributes.
// Stable has [Throws], unstable doesn't - both become &js_sys::Function
// but should generate different Rust return types (Result vs plain).
interface Position {
};

callback PositionCallback = undefined (Position position);

[Constructor()]
interface GeolocationLike {
  [Throws]
  undefined getCurrentPosition(PositionCallback successCallback);
};

// Test case for WebGL texImage2D pattern: multiple stable overloads of the same
// JS operation, where one overload references an unstable type (UnstableFrame).
// The stable overloads that don't use the unstable type should have NO cfg gate.
// Only the overload using UnstableFrame should get #[cfg(web_sys_unstable_apis)].
[Constructor()]
interface TextureLike {
};

[Constructor()]
interface WebGLLike {
  [Throws]
  undefined texUpload(TextureLike texture, long x, long y);
  [Throws]
  undefined texUpload(TextureLike texture, double dx, double dy);
  [Throws]
  undefined texUpload(TextureLike texture, DOMString data);
  [Throws]
  undefined texUpload(TextureLike texture, UnstableFrame frame);
};

// Test case for Clipboard.read() - stable has optional unstable param.
// Stable: read() should exist and NOT be cfg-gated
// Unstable: read_with_unstable_options() should be unstable
// The key is that read() has NO unstable override (different signature),
// only read_with_unstable_options() is unstable (additive).
[Constructor()]
interface TestOptionalUnstableArg {
  // The optional param references an unstable type (defined in unstable webidl)
  // So stable generates read() only, unstable adds read_with_unstable_options()
  Promise<DOMString> read(optional UnstableOptions options = {});
};
