# RustX Quick Reference

One-page reference for RustX syntax and features.

## Basics

```rust
// Variables
x = 42
name = "Alice"

// Comments
// Single line
/* Multi-line */

// Print
print("Hello")
print("Value:", x)
```

## Data Types

```rust
// Numbers
int = 42
float = 3.14

// Strings
str = "hello"
template = `Hello, {name}!`

// Arrays
arr = [1, 2, 3]
mixed = [1, "two", 3.0]

// Maps
obj = {name: "Alice", age: 25}
```

## Control Flow

```rust
// If/Else
if x > 10 {
    print("big")
} else {
    print("small")
}

// While
while x < 10 {
    x = x + 1
}

// For
for i in range(5) {
    print(i)
}

for item in [1, 2, 3] {
    print(item)
}
```

## Functions

```rust
// Function definition
fn add(a, b) {
    return a + b
}

// Implicit return
fn multiply(a, b) {
    a * b
}

// Arrow function
fn square(x) => x * x
```

## Standard Library

```rust
// Import modules (v0.5.0+)
use json
use http
use os
use time
use web
use fs
use term

// JSON
obj = json.parse(`{"key": "value"}`)
str = json.stringify({name: "Alice"})

// HTTP
response = http.get("https://api.example.com")
response = http.post("https://api.example.com", data)

// OS
home = os.env("HOME")
args = os.args()

// Time
now = time.now()
time.sleep(1000)  // milliseconds

// File System
content = fs.read("file.txt")
fs.write("file.txt", "content")
exists = fs.exists("file.txt")
fs.remove("file.txt")

// Terminal
print(term.red("Error"))
print(term.green("Success"))
print(term.bold("Important"))
term.clear()

// Web Server
app = web.app()
app.get("/", fn(req) {
    return web.json({message: "OK"})
})
app.listen(3000)
```

## Methods

```rust
// String
"hello".upper()        // "HELLO"
"WORLD".lower()        // "world"
"  x  ".trim()         // "x"
"a,b,c".split(",")     // ["a", "b", "c"]

// Array
[1, 2, 3].len()        // 3
arr.push(4)            // Add to end
arr.pop()              // Remove from end

// Number
3.14.round()           // 3
3.14.floor()           // 3
3.14.ceil()            // 4
(-42).abs()            // 42
```

## CLI Commands

```bash
# Run (interpreter)
rustx script.rsx

# Build (JIT - faster)
rustx script.rsx -o binary

# Transpile to Rust
rustx script.rsx --rs output.rs

# Check syntax
rustx check script.rsx

# REPL
rustx repl

# Help
rustx --help
```

## Performance Tips

```rust
// Use explicit imports (faster!)
use json  // ✅ Good
// vs auto-import ❌ (slower)

// Type inference helps
x = 42          // Optimized to native i64
y = 10
z = x + y       // Pure native math!

// Template strings
msg = `Hello, {name}!`  // ✅ Good
msg = "Hello, " + name  // ❌ Slower
```

## Common Patterns

```rust
// Read JSON file
use fs
use json

content = fs.read("data.json")
data = json.parse(content)

// HTTP + JSON
use http
use json

response = http.get("https://api.github.com/users/octocat")
user = json.parse(response)
print(user.login)

// Error handling
use fs

if fs.exists("config.json") {
    config = fs.read("config.json")
} else {
    print("Config not found!")
}
```

## Keyboard Shortcuts (REPL)

- `Ctrl+C` - Cancel input / Exit
- `Ctrl+D` - Exit REPL
- `↑` / `↓` - History navigation
- `Tab` - (future: auto-complete)

## Links

- Documentation: [GUIDE.md](GUIDE.md)
- Examples: [examples/](examples/)
- Benchmarks: [benchmarks/LANG_COMPARISON.md](benchmarks/LANG_COMPARISON.md)
- Contributing: [CONTRIBUTING.md](CONTRIBUTING.md)
