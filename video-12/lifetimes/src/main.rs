fn main() {
    //let r :&i32;
    /* The below code errors out as r refers to x , whose lifetime is within the block
    and r is being used outside the block.
     */
    // {
    //     let x = 5;
    //     r = &x;
    // }
    //println!("r: {}", r);

    let string1 = String::from("abcd");
    let string2 = String::from("xyz");
    let result = compare2(string1.as_str(), string2.as_str());
    println!("The longest string is {}", result);

}


/*   |
19 | pub fn compare1(x: &str, y: &str) -> &str {
   |                    ----     ----     ^ expected named lifetime parameter
   |
   = help: this function's return type contains a borrowed value, but the signature does not say whether it is borrowed from `x` or `y`
help: consider introducing a named lifetime parameter
   |
19 | pub fn compare1<'a>(x: &'a str, y: &'a str) -> &'a str {
   |                ++++     ++          ++          ++

For more information about this error, try `rustc --explain E0106`.
error: could not compile `lifetimes` (bin "lifetimes") due to 1 previous error */

// pub fn compare1(x: &str, y: &str) -> &str {
//     if x.len() > y.len() {
//         x
//     } else {
//         y
//     }
// }


pub fn compare2<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// 1. Each parameter that is a reference gets its own lifetime parameter.
// 2. If there is exactly one input lifetime parameter, that lifetime is assigned to all output lifetime parameters.
// 3. If there are multiple input lifetime parameters, but one of them is &self or &mut self, the lifetime of self is assigned to all output lifetime parameters.

fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
    where T: Display
{
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}