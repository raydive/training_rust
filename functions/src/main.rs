fn main() {
    println!("Hello, world!");

    another_function();
    another_function1(1);
}

// snake caseで
fn another_function() {
    println!("Another function.");  // 別の関数
}

fn another_function1(x: i32) {
    println!("The value of x is: {}", x);   // xの値は{}です
}