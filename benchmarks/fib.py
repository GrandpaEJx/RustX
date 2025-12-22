import time
def fib(n):
    if n < 2: return n
    return fib(n-1) + fib(n-2)

print("Running Python Fib(30)...")
start = time.time()
res = fib(30)
end = time.time()
print(f"Result: {res}")
print(f"Time: {end - start:.6f}s")
