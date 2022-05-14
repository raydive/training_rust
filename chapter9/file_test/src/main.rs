/*
$ cargo run                                                                                                            [git][main] -? 
   Compiling file_test v0.1.0 (/Users/arata_n/src/training_rust/chapter9/file_test)
warning: unused variable: `f`
 --> src/main.rs:6:9
  |
6 |     let f = match f {
  |         ^ help: if this is intentional, prefix it with an underscore: `_f`
  |
  = note: `#[warn(unused_variables)]` on by default

warning: `file_test` (bin "file_test") generated 1 warning
    Finished dev [unoptimized + debuginfo] target(s) in 0.22s
     Running `target/debug/file_test`
thread 'main' panicked at 'There was a problem opening the file: Os { code: 2, kind: NotFound, message: "No such file or directory" }', src/main.rs:10:13
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
[~/src/training_rust/chapter9/file_test]
$ touch hello.txt                                                                                                      [git][main] -? 
[~/src/training_rust/chapter9/file_test]
$ cargo run                                                                                                            [git][main] -? 
warning: unused variable: `f`
 --> src/main.rs:6:9
  |
6 |     let f = match f {
  |         ^ help: if this is intentional, prefix it with an underscore: `_f`
  |
  = note: `#[warn(unused_variables)]` on by default

warning: `file_test` (bin "file_test") generated 1 warning
    Finished dev [unoptimized + debuginfo] target(s) in 0.00s
     Running `target/debug/file_test`
*/
use std::fs::File;

fn main() {
    let f = File::open("hello.txt");

    let f = match f {
        Ok(file) => file,
        Err(error) => {
            // ファイルを開く際に問題がありました
            panic!("There was a problem opening the file: {:?}", error)
        },
    };
}