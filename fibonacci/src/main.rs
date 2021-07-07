fn main() {
    fibonacci(0, 1, 6);
}


fn fibonacci(first: i32, second:i32, mut n: i32) {
    let mut p1 : i32 = first;
    let mut p2 : i32 = second;
    let mut next : i32;
    
    println!("{}", p1);

    while n > 0 {
        next = p1 + p2;
        println!("{}", next);
        p2 = p1;
        p1 = next;
        n -= 1;
    }
}