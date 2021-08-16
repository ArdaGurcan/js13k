import init from "./wasm_js13k.js";
import { greet } from "./wasm_js13k.js";

const runWasm = async () => {
  const helloWorld = await init("./wasm_js13k_bg.wasm");
};
runWasm();