// Example demonstrating method chaining

// String methods
text = "hello world"
print(text.upper())                    // HELLO WORLD
print(text.lower())                    // hello world (already lowercase)

loud = "QUIET DOWN".lower()
print(loud)                            // quiet down

// Template strings with method chaining
name = "alice"
greeting = `hello, {name}`.upper()
print(greeting)                        // HELLO, ALICE

// Chaining multiple methods
messy = "  HELLO WORLD  "
clean = messy.trim().lower()
print(clean)                           // hello world

// String split with chaining
csv = "apple,banana,cherry"
fruits = csv.split(",")
print(fruits)                          // [apple, banana, cherry]

// Array length with chaining
numbers = [1, 2, 3, 4, 5]
size = numbers.len()
print(`Array has {size} elements`)     // Array has 5 elements

// Direct method calls on literals
print("RustX".lower())                 // rustx
print([1, 2, 3].len())                 // 3

// Math methods
pi = 3.14159
print(pi.round())                      // 3
print(pi.floor())                      // 3
print(pi.ceil())                       // 4

negative = -42
print(negative.abs())                  // 42

// Complex chaining
data = "  Apple,Banana,Cherry  "
processed = data.trim().lower().split(",")
print(processed)                       // [apple, banana, cherry]

// Method chaining in expressions
result = "hello".upper().len()
print(`Length of uppercase: {result}`) // Length of uppercase: 5

// Final result
"Method chaining works!".upper()
