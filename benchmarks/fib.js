function fib(n) {
    if (n < 2) return n;
    return fib(n - 1) + fib(n - 2);
}

console.log("Running Node Fib(30)...");
const start = process.hrtime.bigint();
const res = fib(30);
const end = process.hrtime.bigint();
const dura_ns = Number(end - start);
console.log("Result:", res);
console.log("Time:", (dura_ns / 1e9).toFixed(6) + "s");
