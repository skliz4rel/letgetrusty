//3 closure traints
//Fn captures the variable and it environment immutably
//FnMut captures the variable and it environment mutably
//FnOnce specifies the closure takes ownership of it environment taking ownership of it variables

fn add_one(x: i32) -> i32 {
    x + 1
}

//This method below accepts a closure trait bound
fn do_twice<T>(f: T, arg: i32) -> i32
where
    T: Fn(i32) -> i32,
{
    f(arg) + f(arg)
}

fn main() {
    let answer: i32 = do_twice(add_one, 5);

    println!("The answer is: {}", answer);

    example();

    example2();
}

fn example() {
    let list_of_numbers: Vec<i32> = vec![1, 2, 3];
    let list_of_strings: Vec<String> = list_of_numbers
        .iter()
        .map(|i: &i32| i.to_string())
        .collect();

    println!("{:?}", list_of_strings);
}

fn example2() {
    #[derive(Debug)]
    enum Status {
        Value(u32),
        Stop,
    }

    let list_of_statuses: Vec<Status> = (0u32..20).map(Status::Value).collect();
    println!("{:?}", list_of_statuses);
}

//returning closures from functions

fn returns_closure(a: i32) -> Box<dyn Fn(i32) -> i32> {
    if a > 0 {
        Box::new(move |b| a + b)
    } else {
        Box::new(move |b| a - b)
    }
}
