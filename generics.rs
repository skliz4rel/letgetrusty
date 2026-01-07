fn main() {
    let numbers = vec![23, 4, 5, 1000, 6, 66, 565];

    let largest = get_largest(numbers.clone());
    println!("The largest number in our list is {}", largest);

    let largest = get_largest_generic::<i32>(numbers.clone());
    println!("The largest number in our list is {}", largest);

    structs_meth();
}

fn get_largest(list: Vec<i32>) -> i32 {
    let mut largest = list[0];

    for i in list {
        if i > largest {
            largest = i;
        }
    }

    largest
}

fn get_largest_generic<T: PartialOrd + Copy>(list: Vec<T>) -> T {
    let mut largest = list[0];

    for i in list {
        if i > largest {
            largest = i;
        }
    }

    largest
}

struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

impl Point<f64> {
    fn y(&self) -> f64 {
        self.y
    }
}

struct Point1<T, U> {
    x: T,
    y: U,
}
impl<T, U> Point1<T, U> {
    fn mixup<V, W>(self, other: Point1<V, W>) -> Point1<T, W> {
        Point1 {
            x: self.x,
            y: other.y,
        }
    }
}

fn structs_meth() {
    let p1 = Point { x: 1.0, y: 3.0 };
    println!("x = {}", p1.x());
    println!("y = {}", p1.y());

    let p2 = Point1 { x: 2, y: 3.2 };
    println!("Point1: x={}, y={}", p2.x, p2.y);

    let p3: Point1<&str, char> = Point1 { x: "Hello", y: 'c' };

    let p4 = p2.mixup(p3);

    println!("x = {} and y = {}", p4.x, p4.y);
}
