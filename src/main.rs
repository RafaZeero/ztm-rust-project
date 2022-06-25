fn add(num_one: i32, num_two: i32) -> i32 {
    return num_one + num_two;
}

fn main() {
    let mut total = add(10, 45);
    let mut free_shipping = false;

    if total > 50 {
        println!("You qualify for free shipping!!");
        free_shipping = true;
    } else if total > 20 {
        println!("Add more items and you will have free shipping.");
    } else {
        println!("No free shipping.");
    }

    match free_shipping {
        //exhaustive variables
        true => total = total + 0,
        false => total = total + 10,
    }

    // or it can be written like:

    // total = match free_shipping {
    //     //exhaustive variables
    //     true => total + 0,
    //     false => total + 10,
    // };

    // to print wild cards
    match total {
        1 => println!("1"),
        2 => println!("2"),
        _ => println!("No match found."),
    }

    println!("Total: {:?}", total);

    let items: [i32; 5] = [1, 2, 3, 4, 5];
    println!("Items: {:?}", items);
}
