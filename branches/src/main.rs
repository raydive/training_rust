// fn main() {
//     let number = 3;

//     if number < 5 {
//         println!("condition was true");       // 条件は真でした
//     } else {
//         println!("condition was false");      // 条件は偽でした
//     }
// }
// https://doc.rust-jp.rs/book-ja/ch03-05-control-flow.html

// fn main() {
//     let number = 6;

//     if number % 4 == 0 {
//         // 数値は4で割り切れます
//         println!("number is divisible by 4");
//     } else if number % 3 == 0 {
//         // 数値は3で割り切れます
//         println!("number is divisible by 3");
//     } else if number % 2 == 0 {
//         // 数値は2で割り切れます
//         println!("number is divisible by 2");
//     } else {
//         // 数値は4、3、2で割り切れません
//         println!("number is not divisible by 4, 3, or 2");
//     }
// }

// fn main() {
//     let condition = true;
//     let number = if condition {
//         5
//     } else {
//         6
//     };

//     // numberの値は、{}です
//     println!("The value of number is: {}", number);
// }

// fn main() {
//     let mut number = 3;

//     while number != 0 {
//         println!("{}!", number);

//         number = number - 1;
//     }

//     // 発射！
//     println!("LIFTOFF!!!");
// }


// fn main() {
//     let a = [10, 20, 30, 40, 50];
//     let mut index = 0;

//     while index < 5 {
//         // 値は{}です
//         println!("the value is: {}", a[index]);

//         index = index + 1;
//     }
// }
fn main() {
    let a = [10, 20, 30, 40, 50];

    for element in a.iter() {
        // 値は{}です
        println!("the value is: {}", element);
    }
}
