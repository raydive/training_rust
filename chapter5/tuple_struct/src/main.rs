struct Color(i32, i32, i32);

fn main() {
    let c = Color(0,0,0);

    println!("{} {} {}", c.0, c.1, c.2);
}
