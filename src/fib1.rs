pub fn run() {
    println!("fib(10) = {}", fib(10));
}

fn fib(n: i64) -> i64 {
    let mut a = 0;
    let mut b = 1;
    for _ in 0..n {
        let c = a + b;
        a = b;
        b = c;
    }
    a
}
