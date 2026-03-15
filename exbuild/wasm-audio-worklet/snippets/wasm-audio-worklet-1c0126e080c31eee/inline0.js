
export function createWorkletModuleUrl() {
    // This inline module is at: snippets/<crate>-<hash>/inline0.js
    // Main module is at: wasm_audio_worklet.js (2 levels up)
    const bindgenUrl = new URL('../../wasm_audio_worklet.js', import.meta.url).href;
    return URL.createObjectURL(new Blob([`
        import * as bindgen from '${bindgenUrl}';

        registerProcessor('WasmProcessor', class WasmProcessor extends AudioWorkletProcessor {
            constructor(options) {
                super();
                let [module, memory, handle] = options.processorOptions;
                bindgen.initSync({ module, memory });
                this.processor = bindgen.WasmAudioProcessor.unpack(handle);
            }
            process(inputs, outputs) {
                return this.processor.process(outputs[0][0]);
            }
        });
    `], { type: 'text/javascript' }));
}
