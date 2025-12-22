rust {
    fn run() -> Result<Value, String> {
        let n: i64 = 30;
        Ok(Value::Int(fib(n)))
    }

    fn fib(n: i64) -> i64 {
        if n < 2 { return n; }
        fib(n-1) + fib(n-2)
    }
}

print("Running RustX JIT Fib(30)...")
start = time.now()
res = run()
end = time.now()
print("Result:", res)
print("Time:", end - start)
