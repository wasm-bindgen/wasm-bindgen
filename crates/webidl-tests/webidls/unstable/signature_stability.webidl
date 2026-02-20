enum SignatureStabilityMode {
  "fast",
  "safe"
};

dictionary SignatureStabilityOptions {
  SignatureStabilityMode mode = "safe";
};

partial interface SignatureStability {
  DOMString process(SignatureStabilityOptions options);
};

// Unstable partial mixin that overrides the stable method with different types.
// The stable put_image_data(f64, f64) should be gated with #[cfg(not(web_sys_unstable_apis))]
// The unstable put_image_data(i32, i32) should be gated with #[cfg(web_sys_unstable_apis)]
// The stable method name should NOT be changed to put_image_data_with_f64_and_f64
partial interface mixin CanvasImageDataMixin {
  // Spec-correct method uses long (i32) parameters
  undefined putImageData(ImageDataLike imagedata, long dx, long dy);
};

// Unstable version without [Throws] - same Rust parameter types but different return type
partial interface GeolocationLike {
  undefined getCurrentPosition(PositionCallback successCallback);
};

// Unstable dictionary type for TestOptionalUnstableArg.read()
dictionary UnstableOptions {
  DOMString mode;
};

// Unstable type referenced by stable WebGLLike.texUpload() overload
[Constructor()]
interface UnstableFrame {
};
