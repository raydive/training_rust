/*
$ RUST_BACKTRACE=1 cargo run                                                                                           [git][main] -? 
   Compiling trace_test v0.1.0 (/Users/arata_n/src/training_rust/chapter9/trace_test)
    Finished dev [unoptimized + debuginfo] target(s) in 0.21s
     Running `target/debug/trace_test`
thread 'main' panicked at 'index out of bounds: the len is 3 but the index is 99', src/main.rs:4:5
stack backtrace:
   0: rust_begin_unwind
             at /rustc/7737e0b5c4103216d6fd8cf941b7ab9bdbaace7c/library/std/src/panicking.rs:584:5
   1: core::panicking::panic_fmt
             at /rustc/7737e0b5c4103216d6fd8cf941b7ab9bdbaace7c/library/core/src/panicking.rs:143:14
   2: core::panicking::panic_bounds_check
             at /rustc/7737e0b5c4103216d6fd8cf941b7ab9bdbaace7c/library/core/src/panicking.rs:85:5
   3: <usize as core::slice::index::SliceIndex<[T]>>::index
             at /rustc/7737e0b5c4103216d6fd8cf941b7ab9bdbaace7c/library/core/src/slice/index.rs:189:10
   4: core::slice::index::<impl core::ops::index::Index<I> for [T]>::index
             at /rustc/7737e0b5c4103216d6fd8cf941b7ab9bdbaace7c/library/core/src/slice/index.rs:15:9
   5: <alloc::vec::Vec<T,A> as core::ops::index::Index<I>>::index
             at /rustc/7737e0b5c4103216d6fd8cf941b7ab9bdbaace7c/library/alloc/src/vec/mod.rs:2531:9
   6: trace_test::main
             at ./src/main.rs:4:5
   7: core::ops::function::FnOnce::call_once
             at /rustc/7737e0b5c4103216d6fd8cf941b7ab9bdbaace7c/library/core/src/ops/function.rs:227:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
*/
fn main() {
    let v = vec![1, 2, 3];

    v[99];
}
