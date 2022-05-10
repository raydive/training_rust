mod front_of_house {
    // ここのpub はないと他から見えない
    pub mod hosting {
        pub fn add_to_waitlist() {}
        fn cannot_call() {}
    }
}

pub fn eat_at_restaurant() {
    // Absolute path
    // 絶対パス
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path
    // 相対パス
    front_of_house::hosting::add_to_waitlist();

    use self::front_of_house::hosting;

    hosting::add_to_waitlist();

    /*
        Checking can_compile v0.1.0 (/Users/arata_n/src/training_rust/chapter7/can_compile)
        error[E0425]: cannot find function `add_to_waitlist` in this scope
        --> src/lib.rs:22:5
        |
        22 |     add_to_waitlist();
        |     ^^^^^^^^^^^^^^^ not found in this scope
        |
        help: consider importing this function
        |
        1  | use crate::front_of_house::hosting::add_to_waitlist;
        |
     */
    add_to_waitlist();

    /*
    $ cargo check                                                                                                                                                                 [git][main] ? 
    Checking can_compile v0.1.0 (/Users/arata_n/src/training_rust/chapter7/can_compile)
    error[E0603]: function `cannot_call` is private
    --> src/lib.rs:22:14
    |
    22 |     hosting::cannot_call();
    |              ^^^^^^^^^^^ private function
    |
    note: the function `cannot_call` is defined here
    --> src/lib.rs:5:9
    |
    5  |         fn cannot_call() {}
    |         ^^^^^^^^^^^^^^^^

    For more information about this error, try `rustc --explain E0603`.
    error: could not compile `can_compile` due to previous error
     */
    hosting::cannot_call();
}

fn main() {}
