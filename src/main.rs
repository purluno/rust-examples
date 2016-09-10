mod fib1;
mod fib2;
mod binsearch;
mod quicksort;
mod c_hello;

mod readfile;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let example: &str;
    if args.len() < 2 {
        example = "";
    } else {
        example = args[1].as_str();
    }
    match example {
        "fib1" => fib1::run(),
        "fib2" => fib2::run(),
        "binsearch" => binsearch::run(),
        "quicksort" => quicksort::run(),
        "c_hello" => c_hello::run(),
        _ => fib1::run()
    }
}
