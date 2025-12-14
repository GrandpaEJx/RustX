# RustX Language Reference

## Table of Contents

- [Variables](#variables)
- [Data Types](#data-types)
- [Operators](#operators)
- [Control Flow](#control-flow)
- [Functions](#functions)
- [Arrays](#arrays)
- [Template Strings](#template-strings)
- [Method Chaining](#method-chaining)
- [Imports & Standard Library](#imports--standard-library)
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

## Imports & Standard Library

RustX includes a rich standard library with modules for web development, HTTP requests, JSON handling, and more.

### Import Statement

Use the `import` keyword to load standard library modules:

```rustx
import web
import json
import time
import http
import os
```

You can import multiple modules:

```rustx
import web
import json

let app = web.app()
```

### Available Standard Library Modules

- **`web`** - Build web servers and APIs
- **`json`** - Parse and serialize JSON data
- **time** - Work with timestamps and delays
- **`http`** - Make HTTP client requests
- **`os`** - Access environment variables and CLI arguments

### Web Server Example

```rustx
rust {
    // Force JIT compilation for native performance
}

import web
import json

let app = web.app()

fn home(body, debug) {
    return web.json({
        "status": "running",
        "version": "1.0.0"
    })
}

fn add_numbers(body, debug) {
    let data = json.parse(body)
    let result = data["a"] + data["b"]
    return web.json({"sum": result})
}

app.get("/", home)
app.post("/add", add_numbers)

app.listen(8080, false, 4)
```

### JSON Processing

```rustx
import json

// Parse JSON string
json_str = '{"name": "Alice", "age": 30}'
user = json.parse(json_str)
print(user["name"])  // Alice

// Serialize to JSON
data = {"items": [1, 2, 3], "count": 3}
output = json.stringify(data)
print(output)  // {"items":[1,2,3],"count":3}
```

### HTTP Requests

```rustx
import http
import json

// GET request
response = http.get("https://api.example.com/data")
data = json.parse(response)

// POST request
payload = json.stringify({"key": "value"})
result = http.post("https://api.example.com/submit", payload)
```

### Time Operations

```rustx
import time

// Get current timestamp
now = time.now()
print("Timestamp:", now)

// Sleep for 2 seconds
print("Waiting...")
time.sleep(2)
print("Done!")
```

### Environment & Arguments

```rustx
import os

// Get environment variables
home = os.env("HOME")
path = os.env("PATH")

// Get CLI arguments
args = os.args()
print("Script called with:", args)
```

For complete API documentation, see [Built-in Functions](built-in-functions.md#standard-library-modules).

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
