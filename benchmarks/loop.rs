use std::time::Instant;

fn main() {
    println!("Running Rust Loop(1M)...");
    let mut sum: i64 = 0;
    let mut i: i64 = 0;
    let start = Instant::now();
    while i < 1_000_000 {
        sum += i;
        i += 1;
    }
    let duration = start.elapsed();
    println!("Result: {}", sum);
    println!("Time: {:.6}", duration.as_secs_f64());
}
