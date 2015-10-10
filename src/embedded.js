var ffi = require("ffi")

var lib = ffi.Library("target/release/libembedded_rust", {
  "process": ["void", []]
});

lib.process();
