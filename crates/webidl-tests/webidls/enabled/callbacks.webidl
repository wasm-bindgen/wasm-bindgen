callback interface CallbackInterface1 {
  undefined foo();
};

callback interface CallbackInterface2 {
  undefined foo();
  undefined bar();
};

[Constructor()]
interface TakeCallbackInterface {
  undefined a(CallbackInterface1 arg);
  undefined b(CallbackInterface2 arg);
};


// Test case for optional callbacks (mimics decodeAudioData pattern)
callback SuccessCallback = undefined (DOMString result);
callback ErrorCallback = undefined (DOMString error);

[Constructor()]
interface TestOptionalCallbacks {
  Promise<DOMString> doWork(
    DOMString input,
    optional SuccessCallback successCallback,
    optional ErrorCallback errorCallback
  );
};
