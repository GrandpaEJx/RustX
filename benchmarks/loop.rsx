print("Running RustX Loop(1M)...")
sum = 0
i = 0
start = time.now()
while i < 1000000 {
    sum = sum + i
    i = i + 1
}
end = time.now()
print("Result:", sum)
print("Time:", end - start)
