fn main() {}

fn sample1() {
    type kilometer = i32;
    let x: i32 = 10;
    let y: kilometer = 10;

    let result = x * y;

    println!("result is {}", result);
}

fn sample2() {
    type Thunk = Box<dyn Fn() + Send + 'static>;

    let f: Thunk = Box::new(|| println!("hi"));

    fn takes_long_type(_f: Thunk) {
        // do something with f
    }

    fn returns_long_type() -> Thunk {
        Box::new(|| println!("hello from returned thunk"))
    }

    takes_long_type(f);

    let g = returns_long_type();
    g();
}
