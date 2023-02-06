fn main() {
    println!("Hello, world!");

    another_function(5, 6);

    println!("5 + 1 is {}", plus_one(5))
}

fn another_function(x: i32, y: i32) {
    println!("The value if x is: {}", x);
    println!("The value if y is: {}", y);
}

fn plus_one(x: i32) -> i32 {
    return x + 1;
    //x + 1
}
