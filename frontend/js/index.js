// Note that a dynamic `import` statement here is required due to
// webpack/webpack#6615, but in theory `import { greet } from './pkg';`
// will work here one day as well!
((function(){  console.log('loading...');
  fetch('./../assets/engine.wasm')
  .then(response => response.arrayBuffer())
  .then(bytes => WebAssembly.instantiate(bytes, {}))
  .then(wasmContainer => {
    const {add,subtract,multiply} = wasmContainer.instance.exports;
    console.log('4 + 2 = ', add(4, 2));
    console.log('4 - 2 = ', subtract(4, 2));
    console.log('4 * 2 = ', multiply(4, 2));
  }).catch(err=>console.log(err));})())
