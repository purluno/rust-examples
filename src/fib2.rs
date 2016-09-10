pub fn run() {
    println!("fib(10) = {}", fib(10));
}

fn fib(n: i64) -> i64 {
    fn f(a: i64, b: i64, remains: i64) -> i64 {
        if remains <= 0 {
            a
        } else {
            f(b, a + b, remains - 1)
        }
    }
    f(0, 1, n)
}
