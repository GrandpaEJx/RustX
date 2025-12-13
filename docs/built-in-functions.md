# Built-in Functions Reference

RustX comes with 15+ built-in functions for common operations.

## Core Functions

### `print(...)`

Print values to the console.

**Syntax:** `print(value1, value2, ...)`

**Examples:**

```rustx
print("Hello, World!")
print("Sum:", 10 + 20)
print(1, 2, 3, 4, 5)
```

---

### `range(end)` / `range(start, end)` / `range(start, end, step)`

Generate a sequence of numbers.

**Examples:**

```rustx
range(5)            // [0, 1, 2, 3, 4]
range(2, 7)         // [2, 3, 4, 5, 6]
range(0, 10, 2)     // [0, 2, 4, 6, 8]
range(10, 0, -1)    // [10, 9, 8, 7, 6, 5, 4, 3, 2, 1]
```

---

### `len(value)`

Get the length of an array, string, or map.

**Examples:**

```rustx
len([1, 2, 3])              // 3
len("Hello")                // 5
len({a: 1, b: 2})           // 2
```

---

### `type(value)`

Get the type name of a value as a string.

**Examples:**

```rustx
type(42)                    // "int"
type(3.14)                  // "float"
type("hello")               // "string"
type(true)                  // "bool"
type([1, 2, 3])             // "array"
type({a: 1})                // "map"
```

---

## Array Functions

### `push(array, value)`

Add an element to the end of an array (mutates the array).

**Examples:**

```rustx
items = [1, 2, 3]
push(items, 4)
print(items)                // [1, 2, 3, 4]
```

---

### `pop(array)`

Remove and return the last element from an array.

**Examples:**

```rustx
items = [1, 2, 3, 4]
last = pop(items)
print(last)                 // 4
print(items)                // [1, 2, 3]
```

---

## String Functions

### `split(string, delimiter)`

Split a string into an array of substrings.

**Examples:**

```rustx
text = "apple,banana,cherry"
fruits = split(text, ",")
print(fruits)               // [apple, banana, cherry]

sentence = "Hello World"
words = split(sentence, " ")
print(words)                // [Hello, World]
```

---

### `join(array, delimiter)`

Join an array of values into a single string.

**Examples:**

```rustx
words = ["Hello", "World"]
sentence = join(words, " ")
print(sentence)             // Hello World

numbers = [1, 2, 3, 4, 5]
result = join(numbers, "-")
print(result)               // 1-2-3-4-5
```

---

### `trim(string)`

Remove whitespace from both ends of a string.

**Examples:**

```rustx
text = "  hello world  "
clean = trim(text)
print(clean)                // hello world
```

---

### `upper(string)`

Convert a string to uppercase.

**Examples:**

```rustx
text = "hello world"
loud = upper(text)
print(loud)                 // HELLO WORLD
```

---

### `lower(string)`

Convert a string to lowercase.

**Examples:**

```rustx
text = "HELLO WORLD"
quiet = lower(text)
print(quiet)                // hello world
```

---

## Math Functions

### `abs(number)`

Get the absolute value of a number.

**Examples:**

```rustx
abs(-42)                    // 42
abs(3.14)                   // 3.14
abs(-7.5)                   // 7.5
```

---

### `min(a, b)`

Get the minimum of two numbers.

**Examples:**

```rustx
min(10, 20)                 // 10
min(3.14, 2.71)             // 2.71
min(-5, -10)                // -10
```

---

### `max(a, b)`

Get the maximum of two numbers.

**Examples:**

```rustx
max(10, 20)                 // 20
max(3.14, 2.71)             // 3.14
max(-5, -10)                // -5
```

---

### `floor(number)`

Round down to the nearest integer.

**Examples:**

```rustx
floor(3.14)                 // 3
floor(7.9)                  // 7
floor(-2.3)                 // -3
```

---

### `ceil(number)`

Round up to the nearest integer.

**Examples:**

```rustx
ceil(3.14)                  // 4
ceil(7.1)                   // 8
ceil(-2.3)                  // -2
```

---

### `round(number)`

Round to the nearest integer.

**Examples:**

```rustx
round(3.14)                 // 3
round(3.5)                  // 4
round(7.9)                  // 8
```

---

## Usage Examples

### String Processing

```rustx
// Parse CSV
data = "apple,banana,cherry"
fruits = split(data, ",")

for fruit in fruits {
    print(upper(fruit))
}
// Output: APPLE, BANANA, CHERRY
```

### Array Manipulation

```rustx
// Build a list dynamically
numbers = []
for i in range(1, 6) {
    push(numbers, i * 10)
}
print(numbers)              // [10, 20, 30, 40, 50]
```

### Math Operations

```rustx
// Find range of values
values = [15, 23, 8, 42, 16]
minimum = values[0]
maximum = values[0]

for val in values {
    minimum = min(minimum, val)
    maximum = max(maximum, val)
}

print(`Range: {minimum} to {maximum}`)
```

### Combined Example

```rustx
// Process user data
name = "  ALICE SMITH  "
age = 25.7

// Clean and format
clean_name = trim(name)
formatted_name = lower(clean_name)
rounded_age = round(age)

print(`User: {formatted_name}, Age: {rounded_age}`)
// Output: User: alice smith, Age: 26
```

---

## See Also

- [Language Reference](language-reference.md) - Complete syntax guide
- [Getting Started](getting-started.md) - Installation and setup
- [Examples Guide](examples-guide.md) - More practical examples

---

**Navigation:** [📚 Docs Home](README.md) | [⬅️ Language Reference](language-reference.md) | [➡️ Examples Guide](examples-guide.md)
