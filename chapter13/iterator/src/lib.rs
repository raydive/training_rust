#[cfg(test)]
mod tests {
    #[test]
    fn iterator_demonstration() {
        let v1 = vec![1, 2, 3];

        let mut v1_iter = v1.iter(); // v1のイテレータを作成

        assert_eq!(v1_iter.next(), Some(&1));
        assert_eq!(v1_iter.next(), Some(&2));
        assert_eq!(v1_iter.next(), Some(&3));
        assert_eq!(v1_iter.next(), None);
    }

    #[test]
    fn iterator_sum() {
        let v1 = vec![1, 2, 3];
    
        let v1_iter = v1.iter();
    
        let total: i32 = v1_iter.sum();
    
        assert_eq!(total, 6);
    }

    #[test]
    fn vec_map() {
        let v1: Vec<i32> = vec![1, 2, 3];

        let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();

        assert_eq!(v2, vec![2, 3, 4]);
    }

    #[test]
    fn enum_test() {
        enum Event {
            Init { a :i32 },
            Next { b :i32, c :i32 },
            End,
        }
        
        let init = Event::Init {a:1};
        match init {
            Event::Init {a} => println!("init {}", a),
            Event::Next {b, c} => println!("next {} {}", b, c),
            Event::End => println!("end"),
        }
    }
}
