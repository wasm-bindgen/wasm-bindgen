import init from './websockets.js';

window.addEventListener('load', async () => {
    await init({ module_or_path: './websockets_bg.wasm' });
});
