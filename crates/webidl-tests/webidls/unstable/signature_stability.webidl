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
