const path = require('path');
const HtmlWebpackPlugin = require('html-webpack-plugin');
const webpack = require('webpack');
const WasmPackPlugin = require("@wasm-tool/wasm-pack-plugin");
const MiniCssExtractPlugin = require("mini-css-extract-plugin");

module.exports = {
    entry: './index.js',
    output: {
        path: path.resolve(__dirname, '..', 'dist', 'todomvc'),
        filename: 'index.js',
    },
    plugins: [
        new HtmlWebpackPlugin({
            template: 'index.html'
        }),
        new MiniCssExtractPlugin(),
        new WasmPackPlugin({
            crateDirectory: __dirname
        }),
    ],
    mode: 'development',
    experiments: {
        asyncWebAssembly: true
    },
    module: {
        rules: [
            {
                test: /\.css$/,
                use: [MiniCssExtractPlugin.loader, 'css-loader'],
            }
        ]
    }
};
