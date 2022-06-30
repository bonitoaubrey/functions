fn main() {
    let x = plus_one(3);

    println!("The value of x is: {x}.");
}

fn plus_one(x: i8) -> i8 {
    x + x
}
