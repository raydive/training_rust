extern crate adder;

mod common;

/*
$ cargo test                                                                                                                                                             [git][main] -? 
   Compiling adder v0.1.0 (/Users/arata_n/dev/training_rust/chapter11/adder)
    Finished test [unoptimized + debuginfo] target(s) in 0.20s
     Running unittests src/lib.rs (target/debug/deps/adder-d053b13dba04ef5b)

running 1 test
test tests::exploration ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/integration_test.rs (target/debug/deps/integration_test-e4b0c26a0430228f)

running 1 test
test it_adds_two ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

   Doc-tests adder

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
 */

#[test]
fn it_adds_two() {
   common::setup1();
    assert_eq!(4, adder::add_two(2));
}
