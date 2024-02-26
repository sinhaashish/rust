fn main() {
    println!("Hello, world!");
    let char_list = vec!['y', 'm', 'a', 'q'];
    let result = get_largest(&char_list);
    println!("The largest char is {}", result);
    let number_list = vec![34, 50, 25, 100, 65];
    let result = get_largest(&number_list);
    println!("The largest number is {}", result);
    let p1 = Point { x: 5, y: 10 };
    let p2 = Point { x: 5.0, y: 10.0 };
    let p3 = Point { x: 5, y: 10.0 };
    println!("p1.x {} p1.y= {}", p1.x, p1.y);
    println!("p2.x {} p2.y= {}", p2.x, p2.y);
    println!("p3.x {} p3.y= {}", p3.x, p3.y);
    let p1= Point { x: 5, y: 10 };
    let p2 = Point { x: "Hello", y: 'c' };
    let p3 = p1.mixup(p2);
    println!("p3.x {} p3.y= {}", p3.x, p3.y);
}


fn get_largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];
    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }
    largest
}

struct Point<T, U> {
    x: T,
    y: U,
}

impl<T, U> Point<T, U> {
    fn x(&self) -> &T {
        &self.x
    }
    fn y(&self) -> &U {
        &self.y
    }
}

impl<T, U> Point<T, U> {
    fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}
