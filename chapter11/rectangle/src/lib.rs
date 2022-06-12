#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };

        assert!(larger.can_hold(&smaller));
    }

    #[test]
    fn smaller_cannot_hold_larger() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };

        assert!(!smaller.can_hold(&larger))
    }


    #[test]
    fn it_adds_two() {
        assert_eq!(4, add_two(2));
    }

    /*
    ---- tests::greeting_contains_name stdout ----
thread 'tests::greeting_contains_name' panicked at 'Greeting did not contain name, value was `Hello!`', src/lib.rs:42:9
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

     */
    #[test]
    fn greeting_contains_name() {
        let result = greeting("Carol");
        assert!(result.contains("Carol"),
                //挨拶(greeting)は名前を含んでいません。その値は`{}`でした
                "Greeting did not contain name, value was `{}`",
                result);
    }

    #[test]
//    #[should_panic]
    #[should_panic(expected = "Guess value must be less than or equal to 100")]
    fn greater_than_100() {
        Guess::new(200);
    }

    #[test]
    fn add_two_and_two() {
        assert_eq!(4, add_two(2));
    }

    #[test]
    fn add_three_and_two() {
        assert_eq!(5, add_two(3));
    }

    #[test]
    fn one_hundred() {
        assert_eq!(102, add_two(100));
    }

    #[test]
#[ignore]
fn expensive_test() {
    // 実行に1時間かかるコード
    // code that takes an hour to run
}
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        //self.width > other.width && self.height > other.height
        self.width < other.width && self.height > other.height
    }
}

pub fn add_two(a: i32) -> i32 {
    //a + 3
    a + 2
}

pub fn greeting(name: &str) -> String {
//    format!("Hello {}!", name)
    String::from("Hello!")
}

pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        //if value < 1 || value > 100 {
        // if value < 1 {
        //     //予想値は1から100の間でなければなりませんが、{}でした。
        //     panic!("Guess value must be between 1 and 100, got {}.", value);
        // }

        if value < 1 {
            // panic!(
            //     //予想値は1以上でなければなりませんが、{}でした。
            //     "Guess value must be greater than or equal to 1, got {}.",
            //     value
            // );
            panic!(
                //予想値は100以下でなければなりませんが、{}でした。
                "Guess value must be less than or equal to 100, got {}.",
                value
            );
        } else if value > 100 {
            // panic!(
            //     //予想値は100以下でなければなりませんが、{}でした。
            //     "Guess value must be less than or equal to 100, got {}.",
            //     value
            // );
            panic!(
                //予想値は1以上でなければなりませんが、{}でした。
                "Guess value must be greater than or equal to 1, got {}.",
                value
            );
        }

        Guess { value }
    }
}