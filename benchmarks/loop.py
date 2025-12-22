import time
print("Running Python Loop(1M)...")
sum = 0
i = 0
start = time.time()
while i < 1000000:
    sum = sum + i
    i = i + 1
end = time.time()
print(f"Result: {sum}")
print(f"Time: {end - start:.6f}s")
