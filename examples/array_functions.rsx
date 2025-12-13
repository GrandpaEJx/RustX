// Test Array Functions

print("=== Map ===")
arr = [1, 2, 3, 4, 5]
fn double(x) { return x * 2 }
doubled = map(arr, double)
print("Original:", arr)
print("Doubled (function):", doubled)

print("Doubled (method):", arr.map(double))

print("\n=== Filter ===")
fn is_even(x) { return x % 2 == 0 }
evens = filter(arr, is_even)
print("Evens (function):", evens)
print("Evens (method):", arr.filter(is_even))

print("\n=== Reduce ===")
fn sum(acc, x) { return acc + x }
total = reduce(arr, sum, 0)
print("Sum (function):", total)
print("Sum (method):", arr.reduce(sum, 0))

// Test optional initial value (reduce without initial uses first element)
total_no_init = reduce(arr, sum)
print("Sum (no init):", total_no_init)

print("\n=== Reverse ===")
reversed = reverse(arr)
print("Reversed (function):", reversed)
// Note: reverse and sort currently mutate if we aren't careful? 
// Wait, my implementation returns NEW array for reverse?
// Let's check logic_reverse: "items.reverse(); Ok(Value::Array(items))"
// It takes `Value::Array(a)` which clones the vectors if passed by value match?
// eval_expr returns a Value. Cloning it?
// `items = match arr { Value::Array(a) => a ... }` moves the vector out of the value?
// Or clones it? Value is Clone.
// If I pass a variable `arr`, `eval_expr(Ident)` returns a CLONE of the value in env.
// So `reverse(arr)` reverses the CLONE. `arr` remains untouched? YES.
// EXCEPT: `builtin_sort` implementation:
// `arr.sort_by(...)`.
// So it returns a new sorted array.
// But wait, `builtin_push` MUTATES the array in implementation via `env.update`.
// `builtin_reverse` implementation:
// `let mut arr = match self.eval_expr(...)?`.
// It evaluates the expression. If it's a variable, it gets a COPY.
// Then it reverses the copy and returns it.
// So usage `reversed = reverse(arr)` works. `arr` is unchanged.
// Usage `arr.reverse()` returns reversed copy. `arr` unchanged.
// This is "functional" style (immutable).
// `push` and `pop` were mutating because they explicitly did `env.update`.
// `reverse` and `sort` I implemented DO NOT do `env.update`.
// So they are non-mutating.
// Example verifying this:
print("Reversed (method):", arr.reverse())
print("Original after reverse:", arr) // Should be [1,2,3,4,5]

print("\n=== Sort ===")
mixed = [5, 2, 8, 1, 9]
sorted = sort(mixed)
print("Original:", mixed)
print("Sorted (function):", sorted)

mixed2 = [3, 1, 4, 1, 5]
print("Sorted (method):", mixed2.sort())
print("Original mixed2:", mixed2) 

print("\n=== Chaining ===")
// (1..5).map(double).filter(is_even) -> [2, 4, 6, 8, 10] -> filter -> [2, 4, 6, 8, 10] (all even)
// Let's do something interesting
// [1, 2, 3, 4, 5] -> map(double) -> [2, 4, 6, 8, 10] -> filter(>5)
fn gt_five(x) { return x > 5 }
result = arr.map(double).filter(gt_five)
print("Chained (double -> value > 5):", result)
