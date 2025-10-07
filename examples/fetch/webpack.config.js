const path = require('path');
const HtmlWebpackPlugin = require('html-webpack-plugin');

module.exports = {
    entry: './index.js',
    output: {
        path: path.resolve(__dirname, '..', 'dist', 'fetch'),
        filename: 'index.js',
    },
    plugins: [
        new HtmlWebpackPlugin(),
    ],
    mode: 'development',
    experiments: {
        asyncWebAssembly: true
   }
};
