use std::ops::Deref;
use std::rc::Rc; // ✅ fixed

// ---------------- Box-based list ----------------
enum List {
    Cons(i32, Box<List>),
    Nil,
}

// ---------------- Rc-based list ----------------
enum List1 {
    Cons(i32, Rc<List1>),
    Nil,
}

// Aliasing to avoid name collisions
use List::{Cons as BoxCons, Nil as BoxNil};
use List1::{Cons as RcCons, Nil as RcNil};

fn main() {
    // Box list
    let list = BoxCons(
        1,
        Box::new(BoxCons(2, Box::new(BoxCons(3, Box::new(BoxNil))))),
    );

    let b: Box<i32> = Box::new(5);
    println!("b = {}", b);

    dereference_op();
    reference_counting_smartpointer();
}

// ---------------- Deref example ----------------
struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

fn dereference_op() {
    let x: i32 = 5;
    let y: &i32 = &x;
    let z: Box<i32> = Box::new(x);
    let b: MyBox<i32> = MyBox::new(x);

    assert_eq!(x, *y);
    assert_eq!(x, *z);
    assert_eq!(x, *b);

    let m: MyBox<String> = MyBox::new(String::from("Reference coercion"));
    hello(&m); // Deref coercion: &MyBox<String> -> &str

    drop_trait();
}

// ---------------- Deref coercion ----------------
fn hello(name: &str) {
    println!("Hello, {}!", name);
}

// ---------------- Drop trait ----------------
fn drop_trait() {
    struct CustomSmartPointer {
        data: String,
    }

    impl Drop for CustomSmartPointer {
        fn drop(&mut self) {
            println!("Dropping CustomSmartPointer with data `{}`", self.data);
        }
    }

    let _c = CustomSmartPointer {
        data: String::from("my stuff"),
    };

    let _d = CustomSmartPointer {
        data: String::from("other stuff"),
    };

    println!("CustomSmartPointers created");
}

// ---------------- Rc example ----------------
fn reference_counting_smartpointer() {
    let a: Rc<List1> = Rc::new(RcCons(
        1,
        Rc::new(RcCons(2, Rc::new(RcCons(3, Rc::new(RcNil))))),
    ));

    println!("count after creating a = {}", Rc::strong_count(&a));

    let b: Rc<List1> = Rc::new(RcCons(3, Rc::clone(&a)));
    println!("count after creating b = {}", Rc::strong_count(&a));

    let c: Rc<List1> = Rc::new(RcCons(4, Rc::clone(&a)));
    println!("count after creating c = {}", Rc::strong_count(&a));
}
