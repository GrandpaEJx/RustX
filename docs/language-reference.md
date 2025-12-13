# RustX Language Reference

## Table of Contents

- [Variables](#variables)
- [Data Types](#data-types)
- [Operators](#operators)
- [Control Flow](#control-flow)
- [Functions](#functions)
- [Arrays](#arrays)
- [Template Strings](#template-strings)
- [Comments](#comments)

## Variables

Variables are declared by simple assignment. No keywords needed.

```rustx
x = 10
name = "Alice"
active = true
```

Variables are dynamically typed and can be reassigned:

```rustx
x = 42        // x is an integer
x = "hello"   // now x is a string
```

## Data Types

### Integer

```rustx
age = 25
count = -10
```

### Float

```rustx
pi = 3.14159
temperature = -5.5
```

### String

```rustx
name = "Alice"
message = "Hello, World!"
```

### Boolean

```rustx
active = true
completed = false
```

### Array

```rustx
numbers = [1, 2, 3, 4, 5]
mixed = [1, "two", 3.0, true]
nested = [[1, 2], [3, 4]]
```

### Map (Dictionary)

```rustx
person = {
    "name": "Alice",
    "age": 25,
    "city": "NYC"
}
```

### Null

```rustx
value = null
```

## Operators

### Arithmetic

```rustx
a + b    // Addition
a - b    // Subtraction
a * b    // Multiplication
a / b    // Division
a % b    // Modulo (remainder)
```

### Comparison

```rustx
a == b   // Equal
a != b   // Not equal
a < b    // Less than
a > b    // Greater than
a <= b   // Less than or equal
a >= b   // Greater than or equal
```

### Logical

```rustx
a && b   // Logical AND
a || b   // Logical OR
!a       // Logical NOT
```

### Assignment

```rustx
x = 10       // Assign
x = x + 5    // Update
```

## Control Flow

### If Expression

If is an expression, meaning it returns a value:

```rustx
age = 20
status = if age >= 18 {
    "Adult"
} else {
    "Minor"
}
```

Else-if chains:

```rustx
score = 85
grade = if score >= 90 {
    "A"
} else if score >= 80 {
    "B"
} else if score >= 70 {
    "C"
} else {
    "F"
}
```

### While Loop

```rustx
counter = 0
while counter < 5 {
    print(counter)
    counter = counter + 1
}
```

### For Loop

Iterate over arrays:

```rustx
fruits = ["apple", "banana", "cherry"]
for fruit in fruits {
    print(fruit)
}
```

Iterate over ranges:

```rustx
for i in range(5) {
    print(i)  // 0, 1, 2, 3, 4
}

for i in range(2, 7) {
    print(i)  // 2, 3, 4, 5, 6
}

for i in range(0, 10, 2) {
    print(i)  // 0, 2, 4, 6, 8
}
```

## Functions

### Basic Function

```rustx
fn greet(name) {
    print(`Hello, {name}!`)
}

greet("Alice")
```

### Function with Return Value

The last expression is automatically returned:

```rustx
fn add(a, b) {
    a + b
}

result = add(10, 20)  // result = 30
```

### Arrow Functions

Short syntax for simple functions:

```rustx
fn double(x) => x * 2
fn square(x) => x * x

print(double(5))   // 10
print(square(4))   // 16
```

### Recursive Functions

```rustx
fn factorial(n) {
    if n <= 1 {
        1
    } else {
        n * factorial(n - 1)
    }
}

print(factorial(5))  // 120
```

## Arrays

### Creating Arrays

```rustx
numbers = [1, 2, 3, 4, 5]
empty = []
```

### Accessing Elements

```rustx
numbers = [10, 20, 30, 40]
first = numbers[0]      // 10
second = numbers[1]     // 20
last = numbers[-1]      // 40 (negative indexing)
```

### Array Functions

```rustx
items = [1, 2, 3]

// Add element
push(items, 4)          // items = [1, 2, 3, 4]

// Remove last element
last = pop(items)       // last = 4, items = [1, 2, 3]

// Get length
size = len(items)       // 3
```

## Template Strings

Template strings use backticks and support variable interpolation:

```rustx
name = "Alice"
age = 25

// Basic interpolation
message = `Hello, {name}!`

// Multiple variables
info = `{name} is {age} years old`

// Direct usage
print(`Welcome, {name}!`)
```

**Note:** Currently only variable names are supported inside `{}`, not expressions.

## Method Chaining

RustX supports method chaining using the dot operator, allowing for more fluent and readable code:

### String Methods

```rustx
text = "hello world"
upper_text = text.upper()           // "HELLO WORLD"
lower_text = text.lower()           // "hello world"

// Chaining multiple methods
clean = "  MESSY  ".trim().lower()  // "messy"

// Split with chaining
words = "a,b,c".split(",")          // ["a", "b", "c"]
```

### Array Methods

```rustx
numbers = [1, 2, 3, 4, 5]
size = numbers.len()                // 5

// Direct on literals
count = [10, 20, 30].len()          // 3
```

### Math Methods

```rustx
pi = 3.14159
rounded = pi.round()                // 3
floored = pi.floor()                // 3
ceiled = pi.ceil()                  // 4

negative = -42
positive = negative.abs()           // 42
```

### Complex Chaining

You can chain multiple methods together:

```rustx
// Process CSV data
data = "  Apple,Banana,Cherry  "
processed = data.trim().lower().split(",")
// Result: ["apple", "banana", "cherry"]

// String manipulation
result = "hello".upper().len()      // 5
```

### Available Methods

**String Methods:**

- `.upper()` - Convert to uppercase
- `.lower()` - Convert to lowercase
- `.trim()` - Remove whitespace from both ends
- `.split(delimiter)` - Split into array

**Collection Methods:**

- `.len()` - Get length (works on strings, arrays, maps)

**Number Methods:**

- `.abs()` - Absolute value
- `.floor()` - Round down
- `.ceil()` - Round up
- `.round()` - Round to nearest integer

## Comments

### Single-line Comments

```rustx
// This is a single-line comment
x = 10  // Comment after code
```

### Multi-line Comments

```rustx
/*
This is a multi-line comment
It can span multiple lines
*/
x = 10
```

## Blocks

Blocks create new scopes:

```rustx
x = 10
{
    y = 20
    print(x + y)  // 30
}
// y is not accessible here
```

## Expression vs Statement

In RustX, most things are expressions (they return values):

```rustx
// If is an expression
result = if true { 10 } else { 20 }

// Blocks are expressions
value = {
    x = 10
    y = 20
    x + y  // Last expression is returned
}

// Functions are expressions
add = fn(a, b) => a + b
```

## Best Practices

1. **Use meaningful variable names**

   ```rustx
   // Good
   user_age = 25

   // Avoid
   x = 25
   ```

2. **Keep functions small and focused**

   ```rustx
   fn calculate_total(items) {
       total = 0
       for item in items {
           total = total + item
       }
       total
   }
   ```

3. **Use template strings for readability**

   ```rustx
   // Good
   print(`User {name} is {age} years old`)

   // Avoid
   print("User " + name + " is " + age + " years old")
   ```

4. **Comment complex logic**
   ```rustx
   // Calculate factorial using recursion
   fn factorial(n) {
       if n <= 1 { 1 } else { n * factorial(n - 1) }
   }
   ```

---

## See Also

- [Getting Started](getting-started.md) - Installation and setup
- [Built-in Functions](built-in-functions.md) - Function reference
- [Examples Guide](examples-guide.md) - Practical examples

---

**Navigation:** [📚 Docs Home](README.md) | [⬅️ Getting Started](getting-started.md) | [➡️ Built-in Functions](built-in-functions.md)
