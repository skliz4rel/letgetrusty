trait Animal {
    fn speak(&self);
}

struct Dog;
struct Cat;

impl Animal for Dog {
    fn speak(&self) {
        println!("Woof");
    }
}

impl Animal for Cat {
    fn speak(&self) {
        println!("Meon");
    }
}

//call to action
fn main() {
    let dog: Box<dyn Animal> = Box::new(Dog);
    let cat: Box<dyn Animal> = Box::new(Cat);

    dog.speak();
    cat.speak();

    //USING REFERENCE INSTEAD OF BOX
    let dog1 = Dog;
    make_animal(&dog1);

    //This works because Animal is object-safe.

    // What makes a trait object-safe?
    // 1. Methods don’t use Self in return position
    // fn clone(&self) -> Self; (NOT ALLOWED)

    // fn clone_box(&self) -> Box<dyn Trait>; (ALLOWED)

    // 2. Methods don’t have generic type parameters
    // fn do_something<T>(&self, value: T);  (NOT ALLOWED)
    // fn new() -> Self where Self: Sized;

    // 3. Self: Sized methods are excluded
    // fn new() -> Self where Self: Sized; (ALLOWED)

    // NOT ALLOWED
    // trait BadTrait {
    //     fn create() -> Self;
    // }
}

fn make_animal(a: &dyn Animal) {
    a.speak();
}
