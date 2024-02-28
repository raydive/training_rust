// Good
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

    drop(c);
    println!("CustomSmartPointer dropped befre the end of main.");
}