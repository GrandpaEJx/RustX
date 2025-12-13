# RustX Language

A clean, minimal scripting language that seamlessly integrates with Rust.

## 🚀 Quick Start

### Installation

```bash
# Clone the repository
git clone https://github.com/yourusername/RustX.git
cd RustX

# Build the project
cargo build --release

# Run the CLI
cargo run --bin rustx_lang
```

### Your First RustX Script

Create a file called `hello.rsx`:

```rustx
// This is a comment
name = "World"
print("Hello,", name, "!")
```

Run it:

```bash
cargo run --bin rustx_lang -- hello.rsx
```

Output: `Hello, World !`

---

## 📖 Language Basics

### Variables

No keywords needed - just assign values:

```rustx
x = 10              // Integer
pi = 3.14           // Float
name = "RustX"      // String
active = true       // Boolean
items = [1, 2, 3]   // Array
```

### Arithmetic

```rustx
a = 10 + 5          // 15
b = 20 - 8          // 12
c = 3 * 4           // 12
d = 15 / 3          // 5
e = 17 % 5          // 2 (remainder)
```

### Functions

**Simple function:**

```rustx
fn greet(name) {
    print("Hello,", name)
}

greet("Alice")      // Hello, Alice
```

**Function with return value:**

```rustx
fn add(a, b) {
    a + b           // Last expression is returned
}

result = add(10, 20)
print(result)       // 30
```

**Arrow function (short form):**

```rustx
fn double(x) => x * 2

print(double(5))    // 10
```

### Control Flow

**If expressions:**

```rustx
age = 18

status = if age >= 18 {
    "Adult"
} else {
    "Minor"
}

print(status)       // Adult
```

**While loops:**

```rustx
counter = 0
while counter < 5 {
    print(counter)
    counter = counter + 1
}
// Prints: 0 1 2 3 4
```

**For loops:**

```rustx
numbers = [1, 2, 3, 4, 5]

for num in numbers {
    print(num * 2)
}
// Prints: 2 4 6 8 10
```

**Range function:**

```rustx
// range(end) - from 0 to end-1
for i in range(5) {
    print(i)        // 0 1 2 3 4
}

// range(start, end) - from start to end-1
for i in range(2, 7) {
    print(i)        // 2 3 4 5 6
}

// range(start, end, step)
for i in range(0, 10, 2) {
    print(i)        // 0 2 4 6 8
}
```

### Arrays

```rustx
// Create an array
fruits = ["apple", "banana", "cherry"]

// Access elements (0-indexed)
first = fruits[0]       // "apple"
last = fruits[2]        // "cherry"

// Loop through array
for fruit in fruits {
    print(fruit)
}
```

### Comments

```rustx
// Single line comment

/* Multi-line
   comment */
```

---

## 🎯 Examples

### Example 1: Calculate Sum

```rustx
numbers = [10, 20, 30, 40, 50]
total = 0

for num in numbers {
    total = total + num
}

print("Total:", total)  // Total: 150
```

### Example 2: Factorial Function

```rustx
fn factorial(n) {
    if n <= 1 {
        1
    } else {
        n * factorial(n - 1)
    }
}

result = factorial(5)
print("5! =", result)   // 5! = 120
```

### Example 3: FizzBuzz

```rustx
for i in range(1, 16) {
    if i % 15 == 0 {
        print("FizzBuzz")
    } else if i % 3 == 0 {
        print("Fizz")
    } else if i % 5 == 0 {
        print("Buzz")
    } else {
        print(i)
    }
}
```

---

## 🛠️ CLI Usage

### Interactive REPL

Start the REPL (Read-Eval-Print Loop):

```bash
cargo run --bin rustx_lang repl
```

**REPL Commands:**

- `:help` - Show help
- `:exit` - Exit REPL
- `:clear` - Clear screen
- `Ctrl+D` - Exit
- `Up/Down arrows` - Navigate command history

### Run a Script

```bash
cargo run --bin rustx_lang -- script.rsx
```

### Debug Flags

**View AST (Abstract Syntax Tree):**

```bash
cargo run --bin rustx_lang -- --ast script.rsx
```

**View Tokens:**

```bash
cargo run --bin rustx_lang -- --tokens script.rsx
```

**Measure Execution Time:**

```bash
cargo run --bin rustx_lang -- --time script.rsx
```

**Combine flags:**

```bash
cargo run --bin rustx_lang -- --ast --time script.rsx
```

---

## 🔧 Using RustX in Rust Code

### Basic Usage

```rust
use rustx_macros::rx;

fn main() {
    // Execute RustX code and get the result
    let result: i64 = rx! { "10 + 20" };
    println!("Result: {}", result);  // Result: 30
}
```

### With Variables

```rust
use rustx_macros::rx;

fn main() {
    let answer: i64 = rx! { "
        fn double(n) => n * 2
        double(21)
    " };

    println!("Answer: {}", answer);  // Answer: 42
}
```

### Type Conversion

The `rx!` macro automatically converts RustX values to Rust types:

```rust
let num: i64 = rx! { "42" };                    // Integer
let pi: f64 = rx! { "3.14" };                   // Float
let msg: String = rx! { "\"Hello\"" };          // String
let flag: bool = rx! { "true" };                // Boolean
```

---

## 📚 Built-in Functions

### `print(...)`

Print values to console:

```rustx
print("Hello")              // Hello
print("Sum:", 10 + 5)       // Sum: 15
print(1, 2, 3)              // 1 2 3
```

### `range(start?, end, step?)`

Generate a sequence of numbers:

```rustx
range(5)            // [0, 1, 2, 3, 4]
range(2, 7)         // [2, 3, 4, 5, 6]
range(0, 10, 2)     // [0, 2, 4, 6, 8]
```

---

## 🎓 Learning Path

1. **Start Simple**: Try the examples in `examples/basic.rsx`
2. **Learn Loops**: Check out `examples/loops.rsx`
3. **Try Recursion**: Explore `examples/recursion.rsx`
4. **Use the REPL**: Experiment interactively
5. **Read the Code**: Browse `crates/core/src/` to understand internals

---

## 🐛 Troubleshooting

### Script doesn't run

**Problem:** `cargo run --bin rustx_lang -- script.rsx` fails

**Solution:** Make sure the file exists and has `.rsx` extension

### Infinite loop

**Problem:** Script hangs forever

**Solution:** Check your loop conditions. Use `Ctrl+C` to stop.

### Syntax error

**Problem:** "Unexpected token" error

**Solution:** Check for:

- Missing braces `{}`
- Unclosed strings `""`
- Typos in keywords (`fn`, `if`, `while`, `for`)

---

## 📂 Project Structure

```
RustX/
├── crates/
│   ├── core/           # Language core (lexer, parser, interpreter)
│   ├── macros/         # rx! and rsx! macros
│   └── cli/            # Command-line interface
├── examples/           # Example .rsx scripts
│   ├── basic.rsx
│   ├── loops.rsx
│   ├── loops_simple.rsx
│   └── recursion.rsx
└── README.md          # This file
```

---

## 🤝 Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Run tests: `cargo test`
5. Submit a pull request

---

## 📄 License

MIT License - see LICENSE file for details

---

## 🔗 Resources

- **Examples**: Check the `examples/` directory
- **Tests**: See `crates/core/src/*/tests`
- **Design Doc**: Read `design.md` for language philosophy

---

## ❓ FAQ

**Q: What file extension should I use?**  
A: Use `.rsx` for RustX scripts

**Q: Can I use RustX in production?**  
A: RustX is experimental. Use at your own risk.

**Q: How do I debug my script?**  
A: Use `--ast` and `--tokens` flags to see how your code is parsed

**Q: Is there a standard library?**  
A: Currently only `print()` and `range()` are built-in. More coming soon!

**Q: Can I import other files?**  
A: Import system is planned but not yet implemented

---

## 💡 Tips

- **Start small**: Write simple scripts first
- **Use the REPL**: Great for testing expressions
- **Check examples**: Learn from working code
- **Read errors**: Error messages tell you what's wrong
- **Experiment**: Try things out - that's how you learn!

---

Happy coding with RustX! 🚀
