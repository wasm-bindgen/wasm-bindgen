// Test callbacks with various signatures

// Simple void callback (no params, no return)
callback VoidCallback = undefined ();

// Callback with single parameter
callback NumberCallback = undefined (long value);

// Callback with return value
callback StringTransformer = DOMString (DOMString input);

// Callback with multiple parameters
callback BinaryOp = long (long a, long b);

// Callback with complex types
callback ObjectCallback = undefined (object data);

// Callback with sequence
callback SequenceCallback = sequence<long> (sequence<DOMString> input);

// Test interface using these callbacks
[Constructor()]
interface TestCallbacks {
    // Test void callback
    undefined invokeVoidCallback(VoidCallback callback);

    // Test callback with parameter
    undefined invokeNumberCallback(NumberCallback callback, long value);

    // Test callback with return
    DOMString invokeStringTransformer(StringTransformer callback, DOMString input);

    // Test callback with multiple params
    long invokeBinaryOp(BinaryOp callback, long a, long b);

    // Test object callback
    undefined invokeObjectCallback(ObjectCallback callback, object data);

    // Test sequence callback
    sequence<long> invokeSequenceCallback(SequenceCallback callback, sequence<DOMString> input);
};
