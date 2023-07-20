use crate::List::{Cons, Nil};
use std::rc::Rc;

enum List {
    Cons(i32, Rc<List>),
    Nil,
}

/*
count after creating list = 1
count after creating list = 2
count after creating list = 3
*/
fn main() {
    // これでコンパイル通る
    let list = Rc::new(Cons(1, Rc::new(Cons(2, Rc::new(Cons(3, Rc::new(Nil)))))));

    println!("count after creating list = {}", Rc::strong_count(&list));
    let b = Cons(4, Rc::clone(&list));
    println!("count after creating list = {}", Rc::strong_count(&list));
    let c = Cons(5, Rc::clone(&list));
    println!("count after creating list = {}", Rc::strong_count(&list));
}
