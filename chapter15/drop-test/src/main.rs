/*
$ cargo run                                                                                                                 [git][main] ? 
   Compiling drop-test v0.1.0 (/Users/arata_n/dev/training_rust/chapter15/drop-test)
error[E0040]: explicit use of destructor method
  --> src/main.rs:19:7
   |
19 |     c.drop();
   |     --^^^^--
   |     | |
   |     | explicit destructor calls not allowed
   |     help: consider using `drop` function: `drop(c)`
*/
struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}


fn main() {
    let c = CustomSmartPointer { data: String::from("my stuff")};
    println!("CustomSmartPointers created.");

    c.drop();
    println!("CustomSmartPointer dropped befre the end of main.");
}
