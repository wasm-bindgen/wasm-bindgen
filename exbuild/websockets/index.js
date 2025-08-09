import init from './pkg/websockets.js';

window.addEventListener('load', async () => {
    await init({ module_or_path: './pkg/websockets_bg.wasm' });
});
