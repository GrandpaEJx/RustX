# RustX Language Design Specification

## 1. Philosophy

**"Clean. Minimal. Seamless."**
RustX (Crate: `rustx-lang`) is designed to be a lightweight scripting language that lives comfortably inside Rust projects. It avoids boilerplate, favoring inference and minimal syntax.

## 2. Syntax

### 2.1 Comments

C-style comments are used.

```rustx-lang
// Single line comment
/* Multi-line
   comment */
```

### 2.2 Variables & Types

Dynamic typing. No keywords (`let`, `var`) required for declaration.

```rustx
x = 10              // Int
ratio = 3.14        // Float
name = "RustX"      // String
is_valid = true     // Bool
list = [1, 2, 3]    // Array
obj = { "x": 1 }    // Map
```

### 2.3 Blocks & Scope

Braces `{}` define blocks. Semicolons `;` are optional and inferred by newlines.

```rustx
{
    inner = 5
    // inner is dropped at end of block
}
```

### 2.4 Control Flow

Everything is an expression.

**If/Else:**

```rustx
status = if score > 90 { "A" } else { "B" }
```

**Loops:**

```rustx
// For-in loop
for item in list {
    print(item)
}

// While loop
while x > 0 {
    x = x - 1
}
```

### 2.5 Functions

Functions are first-class.

**Standard:**

```rustx
fn add(a, b) {
    if a < 0 { return 0 }
    a + b  // Implicit return
}
```

**Short (Arrow):**

```rustx
fn sub(a, b) => a - b
```

## 3. Rust Interop (The "Seamless" Feature)

### 3.1 Macros

The `rustx-lang` crate provides two main macros:

- `rx! { ... }`: Evaluates an expression/block and returns a value.
- `rsx! { ... }`: Alias/Variant (can be used for specific modes or side-effects).

### 3.2 Context Capture

Script code inside macros can "see" local Rust variables.

**Rust Side:**

```rust
let factor = 2;
let result: i64 = rx! {
    val = 10
    val * factor  // 'factor' is captured from Rust
};
```

**Type Conversion:**

- `Rust -> RustX`: Via `From/Into` traits (Primitives, String, Vec, HashMap).
- `RustX -> Rust`: The return value of the macro block is converted back to the requested Rust type.

## 4. Modules & Imports

### 4.1 File Extensions

- `.rsx`: Standard script file.
- `.rsl`: Rust Script Library (intended for imports).

### 4.2 Import System

Imports are simplistic.

```rustx
import "math.rsl" as math

result = math.calc(10)
```

## 5. Runtime Architecture

### 5.1 Values

The internal `Value` enum supports:

- `Null`
- `Int` (i64)
- `Float` (f64)
- `Bool`
- `String`
- `Array` (Vec<Value>)
- `Map` (HashMap<String, Value>)
- `Function`
- `RustFn` (Native extensions using `Arc<dyn Fn...>`)

### 5.2 Error Handling

Result-based `Rv<T>` (Result Value). No panics allowed. Errors propagate up to the host Rust application.
