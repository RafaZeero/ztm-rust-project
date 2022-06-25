fn add(num_one: i32, num_two: i32) -> i32 {
    return num_one + num_two;
}

fn main() {
    let total = add(10, 45);

    if total > 50 {
        println!("You qualify for free shipping!!");
        println!("Total: {}", total - 10);
    } else {
        println!("Total: {:?}", total);
    }
}
