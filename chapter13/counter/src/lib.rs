struct Counter {
    count: u32,
}

impl Counter {
    fn new() -> Counter {
        Counter { count: 0 }
    }
}

impl Iterator for Counter {
    type Item = u32; // このイテレータが要素として返す型

    fn next(&mut self) -> Option<Self::Item> {
        self.count += 1;

        if self.count < 6 {
            Some(self.count) // Someでラップされた値を返す
        } else {
            None // Noneを返すとイテレータは停止する
        }
    }
}

#[test]
fn calling_next_directly() {
    let mut counter = Counter::new();

    assert_eq!(counter.next(), Some(1)); // 1
    assert_eq!(counter.next(), Some(2)); // 2
    assert_eq!(counter.next(), Some(3)); // 3
    assert_eq!(counter.next(), Some(4)); // 4
    assert_eq!(counter.next(), Some(5)); // 5
    assert_eq!(counter.next(), None);
}

#[test]
fn using_other_iterator_trait_methods() {
    // zipは4組しか生成しないことに注意してください; 理論的な5番目の組の(5, None)は、 入力イテレータのどちらかがNoneを返したら、zipはNoneを返却するため、決して生成されることはありません。
    let sum = Counter::new().zip(Counter::new().skip(1)) // (1, 2), (2, 3), (3, 4), (4, 5)
        .map(|(a, b)| a * b) 
        .filter(|x| x % 3 == 0) 
        .sum::<u32>(); // 18

    assert_eq!(18, sum);
}