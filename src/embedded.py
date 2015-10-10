from ctypes import cdll

lib = cdll.LoadLibrary("target/release/libembedded_rust.so")
lib.process()
