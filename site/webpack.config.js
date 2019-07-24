const path = require('path');
// const WasmPackPlugin = require("@wasm-tool/wasm-pack-plugin");

module.exports = {
  mode: "development",
  entry: "./index.js",
  output: {
    path: path.resolve(__dirname, "dist"),
    filename: "index.js",
  },
  plugins: [
    // new WasmPackPlugin({
    //   crateDirectory: path.resolve(__dirname, "../"),
    //   outName: "hello_wasm",
    //   watchDirectories: [
    //     path.resolve(__dirname, "../src")
    //   ]
    // }),
  ]
};
