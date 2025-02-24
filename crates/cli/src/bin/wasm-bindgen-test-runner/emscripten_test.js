(function() {{
    var elem = document.querySelector('#output');
    window.extraLibraryFuncs = [];
    window.addToLibrary = function(LibraryWbg) {
        window.wasmExports = {__wbindgen_start:() => {}};
        window.cachedTextEncoder = {encodeInto:() => {}};
        window.Module = {};

        try {
            LibraryWbg.$initBindgen();
        } catch (e) {
            elem.innerText = 'test setup failed: ' + e;
        }

        function testExtraLibraryFuncs () {
            ['$initBindgen', '$addOnInit', '$CLOSURE_DTORS', '$getStringFromWasm0'].forEach((value) => {
                if (!extraLibraryFuncs.includes(value)) {
                    return { status: false, e: `test result: ${value} not found`};
                }
            });
            return {status: true, e: 'test result: ok'};
        }

        function testLibraryWbg () {
            if (typeof Module.hello !== 'function') {
                return {status: false, e:'test result: hello() is not found'};
            }
            if (typeof Module.Interval !== 'function') {
                return {status: false, e:'test result: Interval is not found'};
            }

            const keys = Object.keys(LibraryWbg);
            const testNames = ['clearInterval', 'setInterval', 'log'];
            
            for (const name of testNames) {
              const regex = new RegExp(`^__wbg_${name}`);
              const res = keys.find(key => regex.test(key));
              if (!res) {
                return {status: false, e:`test result: ${name} not found`};
              }
            }
            return {status: true, e:'test result: ok'};     
        }

        const tests = [testExtraLibraryFuncs(), testLibraryWbg()];
        for (const res of tests) {
            if (!res.status) {
                elem.innerText = res.e;
                return;
            }
        }       
        elem.innerText = 'test result: ok';

    };
}}());