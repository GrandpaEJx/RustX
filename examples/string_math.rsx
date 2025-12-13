// Example demonstrating string and math functions

print("=== STRING FUNCTIONS ===\n")

// split() - split string into array
text = "apple,banana,cherry"
fruits = split(text, ",")
print("split('apple,banana,cherry', ',') =", fruits)

// join() - join array into string
numbers = [1, 2, 3, 4, 5]
joined = join(numbers, " - ")
print("join([1,2,3,4,5], ' - ') =", joined)

// trim() - remove whitespace
messy = "  hello world  "
clean = trim(messy)
print("trim('  hello world  ') = '", clean, "'", sep="")

// upper() and lower()
message = "Hello World"
print("upper('Hello World') =", upper(message))
print("lower('Hello World') =", lower(message))

print("\n=== MATH FUNCTIONS ===\n")

// abs() - absolute value
print("abs(-42) =", abs(-42))
print("abs(3.14) =", abs(3.14))
print("abs(-7.5) =", abs(-7.5))

// min() and max()
print("min(10, 20) =", min(10, 20))
print("max(10, 20) =", max(10, 20))
print("min(3.14, 2.71) =", min(3.14, 2.71))

// floor(), ceil(), round()
pi = 3.14159
print("floor(3.14159) =", floor(pi))
print("ceil(3.14159) =", ceil(pi))
print("round(3.14159) =", round(pi))

print("floor(7.9) =", floor(7.9))
print("ceil(7.1) =", ceil(7.1))
print("round(7.5) =", round(7.5))

print("\n=== COMBINING FUNCTIONS ===\n")

// Process a CSV-like string
data = "10,20,30,40,50"
values = split(data, ",")
print("Original:", data)
print("Split:", values)
print("Length:", len(values))

// String manipulation pipeline
name = "  RUSTX LANGUAGE  "
processed = lower(trim(name))
print("Processed name:", processed)

// Math operations
a = -15
b = 25
print("Numbers:", a, "and", b)
print("Absolute values:", abs(a), "and", abs(b))
print("Min:", min(abs(a), abs(b)))
print("Max:", max(abs(a), abs(b)))

// Result
"All functions working!"
