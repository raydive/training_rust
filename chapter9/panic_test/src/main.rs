/*
$ cargo run                                                                                                            [git][main] -? 
    Finished dev [unoptimized + debuginfo] target(s) in 0.00s
     Running `target/debug/panic_test`
thread 'main' panicked at 'crash and burn', src/main.rs:2:5
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
*/
fn main() {
    panic!("crash and burn");  //クラッシュして炎上
}
