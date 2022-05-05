// https://doc.rust-jp.rs/book-ja/ch03-05-control-flow.html
fn main() {
    let cel_temp = 60.0;
    println!("摂氏 {}", cel_temp);

    let fahren_temp = cel_temp * 1.8 + 32.0;
    println!("華氏 {}", fahren_temp);
}
