use crate::List::{Cons, Nil};

enum List {
    Cons(i32, Box<List>),
    Nil,
}

fn main() {
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));

    let b = Cons(4, Box::new(list));
    let c = Cons(5, Box::new(list));
}

/*
error[E0382]: use of moved value: `list`
  --> src/main.rs:12:30
   |
9  |     let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
   |         ---- move occurs because `list` has type `List`, which does not implement the `Copy` trait
10 |
11 |     let b = Cons(4, Box::new(list));
   |                              ---- value moved here
12 |     let c = Cons(5, Box::new(list));
   |                              ^^^^ value used here after move

*/