const path = require('path');
const HtmlWebpackPlugin = require('html-webpack-plugin');
const CopyWebpackPlugin = require('copy-webpack-plugin');
const WasmPackPlugin = require("@wasm-tool/wasm-pack-plugin");

module.exports = {
    entry: './index.js',
    output: {
        path: path.resolve(__dirname, '..', 'dist', 'weather_report'),
        filename: 'index.js',
    },
    plugins: [
        new HtmlWebpackPlugin({
            template: './index.html'
        }),
        new WasmPackPlugin({
            crateDirectory: __dirname
        }),
        new CopyWebpackPlugin({
            patterns: [{ from: 'assets', to: 'assets' }]
        }),
    ],
    mode: 'development',
    experiments: {
        syncWebAssembly: true
    }
};
