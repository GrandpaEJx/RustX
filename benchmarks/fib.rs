use std::time::Instant;

fn fib(n: i64) -> i64 {
    if n < 2 { return n; }
    return fib(n-1) + fib(n-2);
}

fn main() {
    println!("Running Rust Fib(30)...");
    let start = Instant::now();
    let res = fib(30);
    let duration = start.elapsed();
    println!("Result: {}", res);
    println!("Time: {:.6}", duration.as_secs_f64());
}
