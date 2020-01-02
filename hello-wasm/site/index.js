// publishしたhello-wasmを使用する
const js = import("./node_modules/@tetsu5555/hello-wasm/hello_wasm.js");
js.then(js => {
  js.greet("WebAssembly");
});