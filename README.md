## Rust eviroment setup
```bash
# install rust
$ curl https://sh.rustup.rs -sSf | sh
# install wasm-pack
$ cargo install wasm-pack
```

## Build and Run
```bash
$ wasm-pack build
$ cd site
$ npm i
$ npm start
```

## Useful links
* https://developer.mozilla.org/en-US/docs/WebAssembly/Rust_to_wasm
* https://rustwasm.github.io/wasm-bindgen/examples/paint.html
* https://rustwasm.github.io/wasm-bindgen/api/wasm_bindgen/index.html

## Try out / Learn more
* https://github.com/wasm-tool/wasm-pack-plugin#readme (wasm-pack in webpack to hot reload)
* https://github.com/koute/stdweb (wasm_bindgen library alternative)
