fn main() {
    println!("Hello, world!");
    let v1 = vec![1, 2, 3, 4, 5];
    let v1_iter = v1.iter();
    for val in v1_iter {
        println!("Got: {}", val);
    }
}


#[test]
fn test_iter() {
    let v1 = vec![1, 2, 3];
    let mut mut_iter = v1.into_iter();
    assert_eq!(mut_iter.next(), Some(1));
    assert_eq!(mut_iter.next(), Some(2));
    assert_eq!(mut_iter.next(), Some(3));
    assert_eq!(mut_iter.next(), None);

}

// Adapters method takes an iterator and returns another iterator
#[test]
fn iterator_sum() {
    let v1 = vec![1, 2, 3];
    let v1_iter = v1.iter();
    let total:i32 = v1_iter.sum();
    assert_eq!(total, 6);

}
#[derive(Debug, PartialEq)]
struct Shoe {
    size: u32,
    style: String,
}

fn shoes_in_my_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    shoes.into_iter().filter(|s| s.size == shoe_size).collect()
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn filters_by_size() {
        let shoes = vec![
            Shoe { size: 10, style: String::from("sneaker") },
            Shoe { size: 13, style: String::from("sandal") },
            Shoe { size: 10, style: String::from("boot") },
        ];
        let in_my_size = shoes_in_my_size(shoes, 10);
        assert_eq!(
            in_my_size,
            vec![
                Shoe { size: 10, style: String::from("sneaker") },
                Shoe { size: 10, style: String::from("boot") },
            ]
        );
    }
}


