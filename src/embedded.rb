require "ffi"

module EmbeddedExample
  extend FFI::Library
  ffi_lib "target/release/libembedded_rust.so"
  attach_function :process, [], :void
end

EmbeddedExample.process
