fn add(num_one: i32, num_two: i32) -> i32 {
    return num_one + num_two;
}

fn main() {
    let foo = add(10, 2);
    // how to print a variable
    println!("first print: {}", foo);
    //how to print more than one variable
    println!("second print: {} {}", foo, true);
    // how to print one variable multiple times OR how to select which variable goes where
    println!("third print: {0} {0}", foo);
    // complex placeholder
    println!("fourth print: {:?}", foo);
}
