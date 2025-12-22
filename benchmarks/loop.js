console.log("Running Node Loop(1M)...");
let sum = 0;
let i = 0;
const start = process.hrtime.bigint();
while (i < 1000000) {
    sum = sum + i;
    i = i + 1;
}
const end = process.hrtime.bigint();
const dura_ns = Number(end - start);
console.log("Result:", sum);
console.log("Time:", (dura_ns / 1e9).toFixed(6) + "s");
