(function() {
    var elem = document.querySelector('#output');
    window.mergedLibrary = {};

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

    // Symbol attribute (decorator) suffixes: `$name__deps`, `$name__postset`,
    // `$name__export`, `$name__force` are attributes of `$name`, not library
    // symbols in their own right.
    var DECORATORS = ['__deps', '__postset', '__export', '__force'];
    function isDecorator(key) {
        return DECORATORS.some(function(d) { return key.endsWith(d); });
    }

    // Defer test execution to allow library_bindgen.js to finish evaluating
    setTimeout(function() {
        try {
            if (typeof window.mergedLibrary.$initBindgen !== 'function') {
                throw new Error("$initBindgen not found in the merged library.");
            }
            // Execute the initialization (assigns `wasm`, runs start).
            window.mergedLibrary.$initBindgen();
            // Each clean export is a hoisted `$<name>` library symbol carrying
            // `$<name>__export: true` (and `__force`). Under emscripten that
            // makes it a named ESM export (instance mode) and, via the symbol's
            // `Module['<name>'] = <name>` __postset, a `Module` property
            // (factory mode). Discover the exports from the `__export`
            // attributes and simulate the Module attachment directly from the
            // symbols rather than evaluating postset source.
            window.exportedSymbols = new Set();
            for (const key of Object.keys(window.mergedLibrary)) {
                if (!key.startsWith('$') || isDecorator(key)) continue;
                if (window.mergedLibrary[key + '__export'] === true) {
                    window.exportedSymbols.add(key.slice(1));
                }
            }
            for (const name of window.exportedSymbols) {
                window.Module[name] = window.mergedLibrary['$' + name];
            }
        } catch (e) {
            elem.textContent += 'test setup failed: ' + e;
            return;
        }

        function testInitBindgenForced() {
            // `$initBindgen` roots the library graph: it must be `__force`d
            // and its `__deps` must carry the global helper symbols
            // (previously kept via `extraLibraryFuncs`).
            if (window.mergedLibrary.$initBindgen__force !== true) {
                return { status: false, e: 'test result: $initBindgen is not __force: true' };
            }
            const deps = window.mergedLibrary.$initBindgen__deps || [];
            const required = ['$addOnInit', '$CLOSURE_DTORS', '$WASM_VECTOR_LEN'];
            for (const value of required) {
                if (!deps.includes(value)) {
                    return { status: false, e: `test result: ${value} not found in $initBindgen__deps` };
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
            // The hoisted exports must carry both attributes so emscripten
            // emits them as named ESM exports under -sMODULARIZE=instance.
            for (const name of ['hello', 'Interval']) {
                if (!window.exportedSymbols.has(name)) {
                    return { status: false, e: `test result: ${name} does not carry __export: true` };
                }
                if (window.mergedLibrary['$' + name + '__force'] !== true) {
                    return { status: false, e: `test result: ${name} does not carry __force: true` };
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

        const tests = [testInitBindgenForced(), testModuleExports()];
        for (const res of tests) {
            if (!res.status) {
                elem.textContent += res.e;
                return;
            }
        }       
        elem.textContent +='test result: ok';
    }, 50); // Small delay pushes this to the end of the event loop

})();
