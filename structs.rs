struct User {
    name: String,
    email: String,
    signed_in: u64,
    active: bool,
    username: String,
    password: String,
}

#[derive(Debug)]
struct Rect {
    width: i32,
    height: i32,
}

impl Rect {
    fn area(&self) -> i32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rect) -> bool {
        self.width >= other.width && self.height >= other.height
    }

    //static  methods
    fn square(size: i32) -> Rect {
        Rect {
            width: size,
            height: size,
        }
    }
}

fn main() {
    let Value: (i32, String) = (1, "hello".to_string()); //this is a tuple

    let mut user: User = User {
        username: String::from("skliz4rel"),
        email: String::from("skliz4rel@gmail.com"),
        name: String::from("Jide Akindejoye"),
        signed_in: 1,
        active: false,
        password: String::from("password"),
    };

    user.email = String::from("skliz4rel@yahoo.com");

    let user2: User = buildUser(
        String::from("gbemi@yahoo.com"),
        String::from("skliz4rel"),
        String::from("bode"),
    );

    let user3: User = User {
        email: String::from("a@gmail.com"),
        ..user2 //get other fields from this struct
    };

    //Tuple struct
    struct Color(i32, i32, i32);

    struct Point(i32, i32, i32);

    //Unlike struct
    struct Uni;

    let rect: Rect = Rect {
        width: 100,
        height: 200,
    };

    println!("Our Rectangle is {:?}", rect);

    let rect_area: i32 = rect.area();
    println!("Area of our rectangle {rect_area}");

    let rect1: Rect = Rect {
        width: 50,
        height: 60,
    };

    let rect2: Rect = Rect {
        width: 5000,
        height: 6000,
    };

    let rect3: Rect = Rect::square(100);

    println!(
        "This rectangle can contain the rect1 {}",
        rect.can_hold(&rect1)
    );

    println!(
        "This rectangle can contain the rect2 {}",
        rect.can_hold(&rect2)
    );

    println!("This square was created from the Rectangle {:#?}", &rect3);
}

fn buildUser(email: String, username: String, name: String) -> User {
    let user1: User = User {
        email: email,
        username: username,
        password: String::from("password"),
        name: name,
        signed_in: 1,
        active: false,
    };

    user1
}
