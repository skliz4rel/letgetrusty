use std::fmt;
use std::ops::Add;

//associated type allows only one type per implementation
//Generic type allows multiple types per implementation
//Use associated type when you are sure it only goign to be used by one type. Orderwise convert to generics
//Generic allows function overloading.

fn associated_type() {
    trait MyIterator {
        type Item;

        fn next(&mut self) -> Option<Self::Item>;
    }

    struct Counter;

    impl MyIterator for Counter {
        type Item = u32;

        fn next(&mut self) -> Option<Self::Item> {
            Some(0)
        }
    }
}

fn generic_type() {
    pub trait Iterator<T> {
        fn next(&mut self) -> Option<T>;
    }

    struct Counter {}

    //generics allows mutiple implementation

    //u32 implementation

    impl Iterator<u32> for Counter {
        fn next(&mut self) -> Option<u32> {
            Some(0)
        }
    }

    //u16 implementation

    impl Iterator<u16> for Counter {
        fn next(&mut self) -> Option<u16> {
            Some(0)
        }
    }
}

fn overloading_in_traits() {
    #[derive(Debug, PartialEq)]
    struct Point {
        x: i32,
        y: i32,
    }

    //So we are overloading a default trait method for this particular custom
    //struct Point that we just created. Overloading built in implementation
    impl Add for Point {
        type Output = Point;

        fn add(self, other: Point) -> Point {
            Point {
                x: self.x + other.x,
                y: self.y + other.y,
            }
        }
    }

    //Run a test
    //Normally add operation work for numbers now we created an overload to work on a struct Point obj.

    assert_eq!(
        Point { x: 1, y: 0 } + Point { x: 2, y: 3 },
        Point { x: 3, y: 3 }
    );
}

fn overloading_in_traits1() {
    struct Millimeters(u32);

    struct Meters(u32);

    impl Add<Meters> for Millimeters {
        type Output = Millimeters;

        fn add(self, other: Meters) -> Millimeters {
            Millimeters(self.0 + (other.0 * 1000))
        }
    }
}

fn implement_same_method_diff_traits() {
    trait Pilot {
        fn fly(&self);
    }

    trait Wizard {
        fn fly(&self);
    }

    struct Human;

    impl Human {
        fn fly(&self) {
            println!("Waving arms from Human ")
        }
    }

    impl Pilot for Human {
        fn fly(&self) {
            println!("This is your captain speaking");
        }
    }

    impl Wizard for Human {
        fn fly(&self) {
            println!("Up!");
        }
    }

    //calling each implementation below.

    let person: Human = Human;

    Pilot::fly(&person);
    Wizard::fly(&person);
    person.fly();
}

fn main() {
    associated_type();

    generic_type();

    overloading_in_traits();

    overloading_in_traits1();

    implement_same_method_diff_traits();

    super_traits();

    impl_traits_for_unrelated_types();
}

fn super_traits() {
    trait OutlinePrint: fmt::Display {
        fn outline_println(&self) {
            let output: String = self.to_string();

            let len: usize = output.len();

            println!("{}", "*".repeat(len + 4));
            println!("*{}*", " ".repeat(len + 2));
            println!("* {} *", output);
            println!("*{}*", " ".repeat(len + 2));
            println!("{}", "*".repeat(len + 4));
        }
    }

    struct Point {
        x: u32,
        y: u32,
    }

    impl OutlinePrint for Point {}
}

fn impl_traits_for_unrelated_types() {
    use std::fmt;

    struct Wrapper(Vec<String>);

    impl fmt::Display for Wrapper {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "[{}]", self.0.join(", "))
        }
    }

    //call to action
    let w: Wrapper = Wrapper(vec![String::from("hello"), String::from("world")]);
    println!("w = {}", w);
}
