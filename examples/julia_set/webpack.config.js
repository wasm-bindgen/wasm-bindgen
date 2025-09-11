const path = require('path');
const HtmlWebpackPlugin = require('html-webpack-plugin');
const CopyWebpackPlugin = require('copy-webpack-plugin');
const WasmPackPlugin = require("@wasm-tool/wasm-pack-plugin");

module.exports = {
    entry: './index.js',
    output: {
        path: path.resolve(__dirname, '..', 'dist', 'julia_set'),
        filename: 'index.js',
    },
    plugins: [
        new HtmlWebpackPlugin({
            template: 'index.html'
        }),
        new WasmPackPlugin({
            crateDirectory: __dirname
        }),
        new CopyWebpackPlugin({
            patterns: [{ from: 'styles', to: 'styles' }]
        }),
    ],
    mode: 'development',
    experiments: {
        asyncWebAssembly: true
    }
};
