(function() {
    var elem = document.querySelector('#output');
    window.extraLibraryFuncs = [];
    window.mergedLibrary = {};
    // emscripten setting the library file self-registers into at compile time.
    window.EXPORTED_FUNCTIONS = new Set();

    window.wasmExports = {
        __wbindgen_start: () => {},
        __wbg_wasmbindgentestcontext_free: () => {},
        __wbg_interval_free: () => {}
    };
    window.cachedTextEncoder = { encodeInto: () => {} };
    window.cachedTextDecoder = { decode: () => {} };
    window.Module = {};

    window.addToLibrary = function(obj) {
        Object.assign(window.mergedLibrary, obj);
    };

    // Defer test execution to allow library_bindgen.js to finish evaluating
    setTimeout(function() {
        try {
            if (typeof window.mergedLibrary.$initBindgen !== 'function') {
                throw new Error("$initBindgen not found in the merged library.");
            }
            // emscripten emits each `$Name` library symbol as a module-scope
            // `var Name = <value>`. Simulate that so the hoisted exports and
            // their `__postset` wiring (which reference bare names) resolve.
            // A symbol's name may itself contain `__` (e.g. `__wbgtest_*` or a
            // namespaced `app__math__Calc`), so exclude only the `__deps` /
            // `__postset` decorator keys, which are suffixes.
            for (const key of Object.keys(window.mergedLibrary)) {
                const name = key.slice(1);
                if (
                    key.startsWith('$') &&
                    !key.endsWith('__deps') &&
                    !key.endsWith('__postset') &&
                    window[name] === undefined
                ) {
                    window[name] = window.mergedLibrary[key];
                }
            }
            // Execute the initialization (assigns `wasm`, runs start).
            window.mergedLibrary.$initBindgen();
            // Run each symbol's `__postset` — this is where exports attach to
            // `Module` (factory mode) and namespace roots are assembled. Skip
            // emscripten's own `$initBindgen__postset` (an `addOnInit(...)`
            // registration that isn't modelled by this harness).
            for (const key of Object.keys(window.mergedLibrary)) {
                if (key.endsWith('__postset') && key !== '$initBindgen__postset') {
                    (0, eval)(window.mergedLibrary[key]);
                }
            }
        } catch (e) {
            elem.textContent += 'test setup failed: ' + e;
            return;
        }

        function testExtraLibraryFuncs() {
            const required = ['$initBindgen', '$addOnInit', '$CLOSURE_DTORS', '$WASM_VECTOR_LEN'];
            for (const value of required) {
                if (!window.extraLibraryFuncs.includes(value)) {
                    return { status: false, e: `test result: ${value} not found in extraLibraryFuncs` };
                }
            }
            return { status: true, e: 'test result: ok' };
        }

        function testModuleExports() {
            // Validate that the exports were successfully mapped to the Module
            if (typeof Module.hello !== 'function') {
                return { status: false, e: 'test result: hello() is not found in Module' };
            }
            if (typeof Module.Interval !== 'function') {
                return { status: false, e: 'test result: Interval is not found in Module' };
            }
            // The hoisted exports must self-register so emscripten emits them as
            // named ESM exports under -sMODULARIZE=instance.
            for (const name of ['hello', 'Interval']) {
                if (!window.EXPORTED_FUNCTIONS.has(name)) {
                    return { status: false, e: `test result: ${name} not registered in EXPORTED_FUNCTIONS` };
                }
            }

            // Search the accumulated library object for the specific imports
            const keys = Object.keys(window.mergedLibrary);
            const testNames = ['clearInterval', 'setInterval', 'log'];
            
            for (const name of testNames) {
                const regex = new RegExp(`^__wbg_${name}`);
                const res = keys.find(key => regex.test(key));
                if (!res) {
                    return { status: false, e: `test result: ${name} not found in mergedLibrary` };
                }
            }
            return { status: true, e: 'test result: ok' };      
        }

        const tests = [testExtraLibraryFuncs(), testModuleExports()];
        for (const res of tests) {
            if (!res.status) {
                elem.textContent += res.e;
                return;
            }
        }       
        elem.textContent +='test result: ok';
    }, 50); // Small delay pushes this to the end of the event loop

})();