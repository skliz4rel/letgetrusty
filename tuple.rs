fn main() {
    let dimension: (i32, i32) = (100, 200);

    let result: i32 = area(dimension);

    println!("THis is the result {}", result);
}

fn area(dimension: (i32, i32)) -> i32 {
    dimension.0 * dimension.1
}
