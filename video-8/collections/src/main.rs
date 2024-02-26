use unicode_segmentation::UnicodeSegmentation;
use std::collections::HashMap;

fn main() {
    println!("Hello, world!");
    let a = [1,2, 3];
    println!("{:?}", a);
    let mut v:Vec<i32> = Vec::new();
    v.push(5);
    v.push(6);


    let mut v2 = vec![1, 2, 3, 4, 5];
    let third: &i32 = &v2[2];
  
    println!("The third element is {}", third);
    v2.push(9);
    match v2.get(2) {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element."),
    }

    // Strings are stored as a collection of UTF-8 encoded bytes
    let  s1 = String::new();
    let s2 = "initial contents";
    let s3 = s2.to_string();
    let s4 = String::from("initial contents");

    let hello1 = String::from("السلام عليكم");
    let hello2 = String::from("Dobrý den");
    let hello3 = String::from("Hello");
    let hello4 = String::from("שָׁלוֹם");
    let hello5 = String::from("नमस्ते");
    let hello6 = String::from("こんにちは");
    let hello7 = String::from("안녕하세요");
    println!("{}, {}, {}, {}, {}, {}, {}", hello1, hello2, hello3, hello4, hello5, hello6, hello7);
    let hello_str = format!("{}, {}, {}, {}, {}, {}, {}", hello1, hello2, hello3, hello4, hello5, hello6, hello7);
    println!("{}", hello_str);

    let hello_hindi = String::from("नमस्ते");
    for c in hello_hindi.chars() {
        println!("{}", c);
    }
    for b in hello_hindi.bytes() {
        println!("{}", b);
    }
    for g in "नमस्ते".graphemes(true) {
        println!("{}", g);
    }

    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    let team_name = String::from("Blue");
    let score = scores.get(&team_name);
    println!("{:?}", score);
    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }
    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Red")).or_insert(50);

    let text = "hello world wonderful world";
    let mut map = HashMap::new();
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{:?}", map);

}
