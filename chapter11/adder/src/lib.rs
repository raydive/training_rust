#[cfg(test)]
mod tests {
    #[test]
    fn exploration() {
        assert_eq!(2 + 2, 4)
    }

    // #[test]
    // fn another() {
    //     //このテストを失敗させる
    //     panic!("Make this test fail");
    // }
}

pub fn add_two(a: i32) -> i32 {
    internal_adder(a, 2)
}

fn internal_adder(a: i32, b: i32) -> i32 {
    a + b
}
