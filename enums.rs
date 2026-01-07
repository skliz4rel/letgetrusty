enum IPAddress {
    V4(u8, u8, u8, u8),
    V6(String),
}

#[derive(Debug)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

#[derive(Debug)]
enum Message {
    Warn(String, String),
    Success(String),
    Error(String, String, String),
}

impl Message {
    fn display(&self) {
        match self {
            Message::Warn(_, _) => {
                println!("Warn enums {:#?}", self);
            }
            Message::Error(_, _, _) => {
                println!("Error enums {:#?}", self);
            }
            Message::Success(_) => {
                println!("Success enums {:#?}", self);
            }
        }
    }
}

fn main() {
    //opiton enums
    let x: i32 = 5;
    let y: Option<i32> = Some(10);
    let c: Option<i32> = None;

    let sum: i32 = x + y.unwrap_or(0);

    let warn_message: Message = Message::Warn(
        String::from("Title"),
        String::from("Warn body of java message"),
    );

    warn_message.display();

    //uses the optional enums
    let five: Option<i32> = Some(5);
    let six: Option<i32> = Some(6);
    let no_value = None;

    let result = plus_one(five);

    println!("Display the value {}", result.unwrap_or(0));

    let result = plus_one(six);

    println!("Display the value {}", result.unwrap_or(0));

    let result = plus_one(no_value);

    println!("Display the value {}", result.unwrap_or(0));

    handle_one_op_in_enums();

    optional_with_if();
}

fn match_expression(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn handle_one_op_in_enums() {
    println!("Choosing the handle only one item in an enums");

    enum Names {
        Jide,
        Gbemi,
        Bolaji,
    }

    let value: Names = Names::Jide;

    match value {
        Names::Jide => println!("This is jide the programmer"),
        _ => (), //Do nothing if others was selected
    }
}

fn optional_with_if() {
    let number: Option<i32> = Some(3);

    if let Some(3) = number {
        println!("three, using the if let optional statement");
    }
}
