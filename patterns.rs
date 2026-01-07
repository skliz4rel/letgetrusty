fn main() {
    patterns_with_enums();

    pattern_with_if_state();

    pattern_with_while();

    pattern_with_for();

    pattern_with_tuple();

    let point: (i32, i32) = (3, 5);
    print_cordinates(&point);

    pattern_types();

    valid_patterns();
}

fn patterns_with_enums() {
    #[derive(Debug)]
    enum Language {
        English,
        Spanish,
        Russian,
        Japanese,
    }

    let language: Language = Language::English;

    match language {
        Language::English => println!("Hello World!"),
        Language::Spanish => println!("Halo Mondo!"),
        Language::Russian => println!("Rusian"),
        lang => println!("Language {:?} is not supported ", lang),
        // _ => println!("Not a supported language."),This woudl also work. it just you wold not be able to pick the value entered by the user
    }
}

fn pattern_with_if_state() {
    let authorization_status: Option<&str> = None;
    let is_admin: bool = false;
    let group_id: Result<u8, _> = "34".parse();

    if let Some(status) = authorization_status {
        println!("Autorization status {}", status);
    } else if is_admin {
        println!("Authorization status: admin")
    } else if let Ok(group_id) = group_id {
        if group_id > 30 {
            println!("Authorization status is priviledge");
        } else {
            println!("Authorization status is basic")
        }
    }
}

fn pattern_with_while() {
    let mut stack: Vec<i32> = Vec::new();

    stack.push(1);
    stack.push(2);
    stack.push(3);

    //THe while loop would stop as soon as pop() returns a None when the vector is empty
    while let Some(top) = stack.pop() {
        println!("{}", top);
    }
}

fn pattern_with_for() {
    let v: Vec<char> = vec!['a', 'b', 'c'];

    for (index, value) in v.iter().enumerate() {
        println!("{} is at index {}", value, index);
    }
}

fn pattern_with_tuple() {
    let (x, y, z) = (1, 2, 3);

    if x == 1 {
        println!("Value is equal to 1")
    }
}

fn print_cordinates(&(x, y): &(i32, i32)) {
    println!("current location : ({} {})", x, y);
}

fn pattern_types() {
    //Irrefutable patterns,,,,, This are patterns that always match
    let x: i32 = 5;

    //Refutable
    let x: Option<&str> = None;
    if let Some(x) = x {
        println!("{}", x);
    };

    //Can only accept irrefutable patterns:
    //function paramters
    //let statements
    //for loops
}

fn valid_patterns() {
    //example 1
    let x: i32 = 1;

    match x {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        _ => println!("anything"),
    }

    //example 2
    let x: Option<i32> = Some(5);
    let y: i32 = 10;

    match x {
        Some(50) => println!("Got 50"),
        Some(y) => println!("Matched, y = {:?}", y),
        _ => println!("Default case. x={:?}", x),
    }

    //example 3
    let x = 4;

    match x {
        1 | 2 | 3 => println!("number is either 1,2,3"),
        4 => println!("This is four"),
        5 => println!("This is five"),
        _ => println!("not found"),
    }

    //example 4, matching range of vlaues

    let x: i32 = 5;

    match x {
        1..=5 => println!("one through five"),
        _ => println!("something else"),
    }

    let x: char = 'c';

    match x {
        'a'..='j' => println!(" From a to j"),
        'k'..='z' => println!("from k to z"),
        _ => println!("someting else"),
    }

    //deconstructing
    struct Point {
        x: i32,
        y: i32,
    }

    let p: Point = Point { x: 0, y: 7 };

    let Point { x: a, y: b } = p;

    assert_eq!(0, a);
    assert_eq!(7, b);

    //matching with enums
    enum Color {
        RGB(i32, i32, i32),
        HSV(i32, i32, i32),
    }

    enum Message {
        ChangeColor(Color),
    }

    let msg: Message = Message::ChangeColor(Color::HSV(0, 160, 255));

    match msg {
        Message::ChangeColor(Color::RGB(r, g, b)) => {
            println!("Change color: red {},green {}, and blue {}", r, g, b)
        }
        Message::ChangeColor(Color::HSV(h, s, v)) => {
            println!("Change color: hue {}, saturation {}, and value {}", h, s, v);
        }
        _ => (),
    }

    //Another sameple
    struct Point1 {
        x: i32,
        y: i32,
    }
    let ((feet, inches), Point1 { x, y }) = ((3, 10), Point1 { x: 3, y: -10 });

    //Another sample
    let mut setting_value = Some(5);
    let new_setting_value = Some(1);

    match (&setting_value, &new_setting_value) {
        (Some(_), Some(_)) => {
            println!("Can't overwrite an existing customized value");
        }
        _ => {
            setting_value = new_setting_value;
        }
    }

    println!("setting value is {:?}", setting_value);

    //Another sample

    let origin = Point { x: 0, y: 0 };

    match origin {
        Point { x, .. } => println!("x is {}", x),
    }

    //example
    let numbers = (1, 2, 3, 4, 5, 6, 7);

    match numbers {
        (first, .., last) => {
            println!("Some numbers {}, {}", first, last);
        }
    }

    //example
    let x: i32 = 4;
    let y: bool = false;

    match x {
        4 | 5 | 6 if y => println!("yes"),
        _ => println!("no"),
    }

    //example

    enum Message1 {
        Hello { id: i32 },
    }

    let msg: Message1 = Message1::Hello { id: 5 };

    match msg {
        Message1::Hello {
            id: id_variable @ 3..=7,
        } => println!("Found an id in range: {}", id_variable),

        Message1::Hello { id: 10..=12 } => {
            println!("Found an id in another range");
        }

        Message1::Hello { id } => {
            println!("Found some other id: {}", id);
        }
    }
}
