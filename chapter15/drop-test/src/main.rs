// CustomSmartPointers created.
// Dropping CustomSmartPointer with data `other stuff`!
// Dropping CustomSmartPointer with data `my stuff`!
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
    let b = CustomSmartPointer { data: String::from("other stuff")};

    println!("CustomSmartPointers created.");
}
