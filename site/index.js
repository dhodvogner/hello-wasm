const js = import("../pkg/hello_wasm.js");

js.then(js => {
  js.greet("WebAssembly");
}).catch(error => {
  console.error(error);
});
