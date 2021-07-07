fn main() {
    fibonacci(0, 1, 6);
}

// Program computes fibonacci sequence with arbitrary seed values first and second
fn fibonacci(mut first: i32, mut second:i32, mut n: i32) {
    let mut next : i32;
    
    println!("{}", first);

    while n > 0 {
        next = first + second;
        println!("{}", next);
        second = first;
        first = next;
        n -= 1;
    }
}