use std::collections::HashMap;
use std::collections::HashSet;

fn main() {
    vectors();
    vectors_with_enums();
    strings();
    hashmaps();
    hashMap_sample();
    hashsets();
}

fn vectors() {
    let v: Vec<i32> = vec![1, 2, 3, 4, 5, 6];
    let mut v1: Vec<i32> = Vec::new();
    v1.push(1);
    v1.push(2);
    v1.push(3);

    println!("{:?}", v1);

    let second: &i32 = &v1[1];

    println!("Second element is {}", second);

    //access elements in vector
    let first: &Option<&i32> = &v1.get(0);

    match first {
        Some(value) => println!("First value in the vector list is {}", value),
        None => println!("First value is not present, it empty"),
    }

    //udpating the values inthe  vector

    for i in &mut v1 {
        *i += 50;
    }

    for i in &v1 {
        print!("{}, ", i);
    }
}

fn vectors_with_enums() {
    enum Values {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        Values::Int(5),
        Values::Float(1.0),
        Values::Text(String::from("Jide Akindejoye")),
    ];

    match &row[0] {
        Values::Int(value) => println!("This is the value {}", value),
        _ => println!("This is not an integer"),
    }
}

fn strings() {
    //collections of uft8 encoded bytes
    let mut s: String = String::new();
    let mut s1: String = "hello".to_string();
    let mut s2: String = String::from("hello friend");

    //U can use the format method or macro to concatenate and print
    format!(" {} {}", s1, s2); //it wouold not take owership

    //adding to strings
    s.push_str("rice");
    s.push_str("beans");

    s.push('a'); //adding characters

    let s3 = s + &s2;

    println!("s3 value = {}", s3);

    //reading the bytes of a string
    println!("Bytes of the string s3 ");
    for b in s3.bytes() {
        print!("{} ", b);
    }

    println!("Chars of the string s3 ");
    for b in s3.chars() {
        print!("{} ", b);
    }
}

fn hashmaps() {
    let mut h: HashMap<String, i32> = HashMap::new();
    h.insert(String::from("name"), 30);
    h.insert(String::from("nam"), 20);

    //inserting only when the key is empty
    h.entry(String::from("uncle")).or_insert(40);
    h.entry(String::from("sister")).or_insert(50);

    let value: Option<&i32> = h.get("name");

    match value {
        Some(v) => println!("hashmap value {}", v),
        None => println!("value was not found"),
    }
}

fn hashMap_sample() {
    let text: String = String::from("hello world wonderful world");

    let mut map: HashMap<&str, i32> = HashMap::new();

    //["hello", "world", "wonderful", "world"]

    for word in text.split_whitespace() {
        let count: &mut i32 = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);
}

fn hashsets() {
    let mut numbers: HashSet<i32> = HashSet::new();

    //insert values
    numbers.insert(1);
    numbers.insert(2);
    numbers.insert(3);
    numbers.insert(4);
    numbers.insert(4); //This is going to be ignored

    //check is existence
    if numbers.contains(&2) {
        println!("2 is in the set")
    }

    //Iterate
    for n in &numbers {
        println!("{}", n);
    }

    println!("Set size: {}", numbers.len());

    // let a: HashSet<i32> = [1, 2, 3].into_iter().collect();
    // let b: HashSet<i32> = [3, 4, 5].into_iter().collect();

    // let union1: HashSet<i32> = a.union(&b).copied().collect();
    // let intersection: HashSet<i32> = a.intersection(&b).copied().collect();
    // let difference: HashSet<i32> = a.difference(&b).copied().collect();

    // println!("{:?}", union1); // {1, 2, 3, 4, 5}
    // println!("{:?}", intersection); // {3}
    // println!("{:?}", difference); // {1, 2}
}
