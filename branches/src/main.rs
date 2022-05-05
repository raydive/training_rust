// fn main() {
//     let number = 3;

//     if number < 5 {
//         println!("condition was true");       // 条件は真でした
//     } else {
//         println!("condition was false");      // 条件は偽でした
//     }
// }
// https://doc.rust-jp.rs/book-ja/ch03-05-control-flow.html

fn main() {
    let number = 6;

    if number % 4 == 0 {
        // 数値は4で割り切れます
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        // 数値は3で割り切れます
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        // 数値は2で割り切れます
        println!("number is divisible by 2");
    } else {
        // 数値は4、3、2で割り切れません
        println!("number is not divisible by 4, 3, or 2");
    }
}
