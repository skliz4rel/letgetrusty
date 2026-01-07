fn main() {
    let v1: Vec<i32> = vec![1, 2, 3, 4];

    let iterator = v1.iter();

    for value in iterator {
        println!("Got in {}", value);
    }

    let total: i32 = v1.iter().sum();

    println!("This is the total of the list {}", total);

    //using iterator with consumers
    let new_list: Vec<i32> = v1.iter().map(|x| x + 1).collect(); //map is an provider and collect is a consumer
    println!("This new list {:?}", new_list);

    //testing the Shoe iterator sample below
    let shoelist = vec![
        Shoe {
            size: 10,
            style: String::from("Full cover shoes"),
        },
        Shoe {
            size: 20,
            style: String::from("Half shoes"),
        },
        Shoe {
            size: 6,
            style: String::from("Full cover shoes"),
        },
    ];

    let filter_list = shoes_in_my_size(shoelist, 10);

    println!("{}", filter_list.len());
}

#[derive(Debug)]
pub struct Shoe {
    size: u32,
    style: String,
}

fn shoes_in_my_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    shoes.into_iter().filter(|s| s.size == shoe_size).collect()
}

#[test]
fn iterator_test() {
    let v: Vec<i32> = vec![1, 2, 3, 4];

    let v1_iter = v.iter();

    asserteq!(v1_iter.next(), Some(&1));
    asserteq!(v1_iter.next(), Some(&2));
    asserteq!(v1_iter.next(), Some(&3));
    asserteq!(v1_iter.next(), Some(&4));
    asserteq!(v1_iter.next(), None);
}
