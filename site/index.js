const js = import("./node_modules/@davesabra/hello-wasm/hello_wasm.js");
js.then(js => {
  js.greet("WebAssembly");
});