/* 
*  reference: wasm-bindgen `Hello World`
*  https://rustwasm.github.io/docs/wasm-bindgen/examples/hello-world.html
*  note: removed support for Edge as the type encoding was deprecated
*/

const path = require('path');
const HtmlWebpackPlugin = require('html-webpack-plugin');
const webpack = require('webpack');
const WasmPackPlugin = require("@wasm-tool/wasm-pack-plugin");

module.exports = {
    entry: './index.js',
    output: {
        path: path.resolve(__dirname, 'dist'),
        filename: 'index.js',
    },
    watch: true,
    plugins: [
        new HtmlWebpackPlugin(),
        new WasmPackPlugin({
            crateDirectory: path.resolve(__dirname, ".")
        }),
    ],
    mode: 'development'
};