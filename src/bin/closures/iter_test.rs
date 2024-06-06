fn main() {
    iter_test1();
    iter_test2();
    iter_test3();
    iter_test4();
    iter_test5();

    iter_test6();
    iter_test7();
}

fn iter_test1() {
    let v1 = vec![1, 2, 3];
    let v1_iter = v1.iter();

    for val in v1_iter {
        println!("{}", val);
    }
}

fn iter_test2() {
    let v1 = vec![1, 2, 3];

    let mut v1_iter = v1.iter();

    assert_eq!(v1_iter.next(), Some(&1));
    assert_eq!(v1_iter.next(), Some(&2));
    assert_eq!(v1_iter.next(), Some(&3));
}

fn iter_test3() {
    let v1 = vec![1, 2, 3];

    let v1_iter = v1.iter();

    // sum会获取迭代器的所有权，调用后不允许再调用
    let total: i32 = v1_iter.sum();

    assert_eq!(total, 6);
}

fn iter_test4() {
    let v1 = vec![1, 2, 3];
    
    // 调用collect才进行消费
    let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();

    assert_eq!(v2, vec![2, 3, 4]);
}

#[derive(PartialEq, Debug)]
struct Shoe {
    size: u32,
    style: String,
}

fn shoe_in_my_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    // 过滤
    shoes.into_iter()
        .filter(|s| s.size == shoe_size)
        .collect()
}

fn iter_test5() {
    let shoes = vec![
        Shoe { size: 10, style: String::from("sneaker") },
        Shoe { size: 13, style: String::from("sandal") },
        Shoe { size: 10, style: String::from("boot") },
    ];

    let in_my_size = shoe_in_my_size(shoes, 10);

    assert_eq!(
        in_my_size,
        vec![
            Shoe { size: 10, style: String::from("sneaker") },
            Shoe { size: 10, style: String::from("boot") },
        ]
    );

}

struct Counter {
    count: u32    
}

impl Counter {
    fn new() -> Counter {
        Counter {
            count: 0
        }
    }
}

/**
 * 实现自己的迭代器
 */
impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        self.count += 1;

        if self.count > 5 {
            None
        } else {
            Some(self.count)
        }
    }
}

fn iter_test6() {

    let counter = Counter::new();
    
    for num in counter.into_iter() {
        println!("{}", num);
    }
}

fn iter_test7() {
    let sum: u32 = Counter::new()
        .zip(Counter::new().skip(1))
        .map(|(a, b)| a *b)
        .filter(|x| x % 3 == 0)
        .sum();
    assert_eq!(18, sum);
}