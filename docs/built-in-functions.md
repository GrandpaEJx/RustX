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

## Standard Library Modules

RustX includes several built-in modules that provide extended functionality. To use these modules, import them at the top of your script using the `import` statement.

### Importing Modules

**Syntax:** `import module_name`

**Available Modules:**

- `web` - Web server and HTTP routing
- `json` - JSON parsing and serialization
- `time` - Time and date operations
- `http` - HTTP client requests
- `os` - Operating system and environment access

---

## Web Module

The `web` module enables you to build high-performance web servers and APIs directly in RustX.

### `web.app()`

Create a new web application instance.

**Returns:** App object with routing methods

**Example:**

```rustx
import web

let app = web.app()
```

---

### `app.get(path, handler)`

Register a GET route handler.

**Parameters:**

- `path` (string) - URL path (e.g., `"/"`, `"/users"`)
- `handler` (function) - Function that receives `(body, debug)` and returns a response

**Example:**

```rustx
fn home(body, debug) {
    return web.json({"message": "Welcome!"})
}

app.get("/", home)
```

---

### `app.post(path, handler)`

Register a POST route handler.

**Parameters:**

- `path` (string) - URL path
- `handler` (function) - Function that receives `(body, debug)` and returns a response

**Example:**

```rustx
fn create_user(body, debug) {
    let data = json.parse(body)
    return web.json({"created": data})
}

app.post("/users", create_user)
```

---

### `app.listen(port, debug, workers)`

Start the web server.

**Parameters:**

- `port` (int) - Port number to listen on
- `debug` (bool) - Enable debug logging
- `workers` (int) - Number of worker threads

**Example:**

```rustx
let port = 8080
let debug = false
let workers = 4

print("Server running on http://localhost:", port)
app.listen(port, debug, workers)
```

---

### `web.json(value)`

Convert a value to a JSON response.

**Parameters:**

- `value` (any) - Value to serialize to JSON

**Returns:** JSON response object

**Example:**

```rustx
fn api_response(body, debug) {
    return web.json({
        "status": "success",
        "data": [1, 2, 3],
        "count": 3
    })
}
```

---

### Complete Web Server Example

```rustx
rust {
    // Force JIT compilation
}

import web
import json

let app = web.app()

// Home route
fn home(body, debug) {
    return web.json({
        "name": "My API",
        "version": "1.0.0",
        "endpoints": ["/", "/echo", "/add"]
    })
}

// Echo POST data
fn echo(body, debug) {
    return web.json({"echoed": body})
}

// Add two numbers from JSON
fn add(body, debug) {
    let input = json.parse(body)
    let result = input["a"] + input["b"]
    return web.json({
        "a": input["a"],
        "b": input["b"],
        "sum": result
    })
}

app.get("/", home)
app.post("/echo", echo)
app.post("/add", add)

app.listen(8080, false, 1)
```

**Test with curl:**

```bash
curl http://localhost:8080/
curl -X POST http://localhost:8080/add \
  -H "Content-Type: application/json" \
  -d '{"a": 10, "b": 20}'
```

---

## JSON Module

Parse and serialize JSON data.

### `json.parse(string)`

Parse a JSON string into a RustX value.

**Parameters:**

- `string` (string) - JSON string to parse

**Returns:** Parsed value (map, array, string, number, bool, or null)

**Example:**

```rustx
import json

json_str = '{"name": "Alice", "age": 30, "active": true}'
data = json.parse(json_str)

print(data["name"])    // Alice
print(data["age"])     // 30
print(data["active"])  // true
```

---

### `json.stringify(value)`

Convert a RustX value to a JSON string.

**Parameters:**

- `value` (any) - Value to serialize

**Returns:** JSON string

**Example:**

```rustx
import json

user = {
    "name": "Bob",
    "age": 25,
    "hobbies": ["coding", "gaming"]
}

json_str = json.stringify(user)
print(json_str)
// {"name":"Bob","age":25,"hobbies":["coding","gaming"]}
```

---

## Time Module

Work with timestamps and delays.

### `time.now()`

Get the current Unix timestamp in seconds.

**Returns:** Integer timestamp

**Example:**

```rustx
import time

timestamp = time.now()
print("Current time:", timestamp)
// Current time: 1702567890
```

---

### `time.sleep(seconds)`

Pause execution for a specified duration.

**Parameters:**

- `seconds` (int or float) - Duration to sleep

**Example:**

```rustx
import time

print("Starting...")
time.sleep(2)
print("2 seconds later!")
```

---

## HTTP Module

Make HTTP client requests.

### `http.get(url)`

Send a GET request to a URL.

**Parameters:**

- `url` (string) - URL to request

**Returns:** Response body as string

**Example:**

```rustx
import http
import json

response = http.get("https://api.example.com/data")
data = json.parse(response)
print(data)
```

---

### `http.post(url, body)`

Send a POST request with a body.

**Parameters:**

- `url` (string) - URL to request
- `body` (string) - Request body

**Returns:** Response body as string

**Example:**

```rustx
import http
import json

payload = json.stringify({"key": "value"})
response = http.post("https://api.example.com/submit", payload)
print(response)
```

---

## OS Module

Access operating system information.

### `os.env(key)`

Get an environment variable value.

**Parameters:**

- `key` (string) - Environment variable name

**Returns:** String value or empty string if not set

**Example:**

```rustx
import os

home = os.env("HOME")
path = os.env("PATH")
api_key = os.env("API_KEY")

print("Home directory:", home)
```

---

### `os.args()`

Get command-line arguments passed to the script.

**Returns:** Array of strings (arguments)

**Example:**

```rustx
import os

args = os.args()
print("Script arguments:", args)

// Run with: rustx_lang script.rsx arg1 arg2
// Output: Script arguments: [arg1, arg2]
```

---

## See Also

- [Language Reference](language-reference.md) - Complete syntax guide
- [Getting Started](getting-started.md) - Installation and setup
- [Examples Guide](examples-guide.md) - More practical examples

---

**Navigation:** [📚 Docs Home](README.md) | [⬅️ Language Reference](language-reference.md) | [➡️ Examples Guide](examples-guide.md)
