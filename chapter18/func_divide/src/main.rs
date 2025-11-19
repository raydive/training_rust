
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let p = Point { x: 1, y: 2 };
    let sum = hoge(&p);
    let Point { x, y } = p;
    println!("{}", sum);
    println!("x: {}, y: {}", x, y);
}

// 関数の引数で分解できる
fn hoge(Point {x, y}: &Point) -> i32 {
    x + y
}