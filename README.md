# embedded-rust

_Rust Inside Other Languages_ is the third project included in [The Rust Programming Language](https://doc.rust-lang.org/book/) book.

It's intended to show off one of Rust's greatest strengths: a lack of a substantial runtime.

> Rust has support for _FFI_ (foreign function interface) in both directions: it can call into C code easily, but crucially, it can also be called into as easily as C. Combined with Rust's lack of a garbage collector and low runtime requirements, this makes Rust a great candidate to embed inside of other languages when you need that extra oomph.

We're going to pick an example where Rust has a clear advantage over many other languages: numeric computing and threading.

Many languages place numbers on the heap, rather than on the stack. Especially in languages that focus on object-oriented programming and use garbage collection, heap allocation is the default.[...]

Second, many languages have a 'global interpreter lock' (_GIL_), which limits concurrency in many situations.[...]

To emphasize these two aspects, we're going to create a little project that uses them heavily. Since the focus of the example is to embed Rust into other languages, rather than the problem itself, we'll just use a toy example:

> Start ten threads. Inside each thread, count from one to five million. After all ten threads are finished, print out 'done!'.
