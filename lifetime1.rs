use std::fmt::Display;

fn main() {
    let str1: String = String::from("first string");
    let str2: String = String::from("second string");
    let mut result: &str = "Anna";

    result = longest_with_an_annoucement(str1.as_str(), str2.as_str(), result);

    println!("Ths is the result {}", result);
}

fn longest_with_an_annoucement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
where
    T: Display,
{
    println!("Announcement! {}", ann);

    if x.len() > y.len() {
        x
    } else {
        y
    }
}
