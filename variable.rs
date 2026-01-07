fn main() {
    let mut x: i32 = 5;
    let mut x = 6;

    const SUB_VALUE: u32 = 100_000_000;

    //scalar or primitive datatypes: int bool float char
    //compound datatypes

    let tuple: (i32, String) = (3, String::from("first"));
    let (i32, String) = tuple; //assigning by destruction

    //arrays
    let array: [i32; 2] = [0, 1];

    let value: i32 = array[0];

    let sum: i32 = my_function(1, 2);

    println!("{sum}");

    controls();
}

fn my_function(x: i32, y: i32) -> i32 {
    println!("Another function");

    let sum: i32 = x + y;

    sum
}

fn conditions() {
    let number = 3;

    if number > 0 {
    } else {
    }
}

fn controls() {
    let mut counter: i32 = 0;
    let result = loop {
        println!("!again");

        counter += 1;

        if counter == 10 {
            break counter;
        }
    };

    println!("{result}");

    while counter > 5 {
        println!("Pringing from a while loop {}", counter);
        counter -= 1;

        if counter == 5 {
            break;
        }
    }

    //forloop
    let a: [i32; 4] = [1, 2, 3, 4];

    //using for loopo with the iterator
    for element in a.iter() {
        println!("Value in the list {}", element);
    }

    //using for loop for printing ranges
    for number in 1..10 {
        print!("{number}, ");
    }
}
