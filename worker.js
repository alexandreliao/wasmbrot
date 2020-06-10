"use strict";

import init, { Wasmbrot } from "./wasmbrot.js";

async function run() {
  const wasm = await init();
  memory = wasm.memory;
}

onmessage = function(msg) {
};
