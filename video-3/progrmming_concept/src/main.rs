fn main() {
    println!("Hello, world!");
    let mut x = 5; // immutable variable    
    println!("The value of x is: {}", x);
    x = 6; // error: cannot assign twice to immutable variable
    const MAX_POINTS: u32 = 100_000; // immutable variable
    let sum = sum (5, 6);
    println!("The value of sum is: {}", sum);
    let a = [1, 2, 3, 4, 5];
    for element in a.iter() {
        println!("the value is: {}", element);
    }   
}


fn sum( a : i32, b : i32) -> i32 {
    a + b
}