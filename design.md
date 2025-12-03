# RustX Advanced Features Design

## Overview

This document outlines the design for advanced language features in RustX: inheritance system, module system, imports, and additional language constructs.

## 1. Inheritance System

### 1.1 Class Definitions

**Syntax:**
```
class ClassName {
    // Fields
    Type fieldName

    // Methods
    fn methodName(param: Type) -> ReturnType {
        // body
    }
}

class ChildClass extends ParentClass {
    // Additional fields and methods
}
```

**Examples:**
```
class Animal {
    Str name
    Int age

    fn speak() {
        println("Animal speaks")
    }

    fn get_age() -> Int {
        return this.age
    }
}

class Dog extends Animal {
    Str breed

    fn speak() {
        println("{this.name} barks!")
    }

    fn fetch(item: Str) {
        println("{this.name} fetches {item}")
    }
}
```

### 1.2 Object Instantiation

**Syntax:**
```
ClassName obj = new ClassName()
obj.field = value
obj.method()
```

**Examples:**
```
Dog myDog = new Dog()
myDog.name = "Buddy"
myDog.age = 3
myDog.breed = "Golden Retriever"
myDog.speak()        // Buddy barks!
myDog.fetch("ball")  // Buddy fetches ball
```

### 1.3 Implementation Details

**AST Nodes:**
- `ClassDecl { name, fields: Vec<Field>, methods: Vec<FunctionDecl>, parent: Option<String> }`
- `ObjectInstantiation { class_name }`
- `MethodCall { object, method_name, arguments }`
- `FieldAccess { object, field_name }`
- `This` (identifier for self-reference)

**Runtime:**
- `Value::Object { class_name, fields: HashMap<String, Value>, methods: HashMap<String, Function> }`
- Method resolution: check class, then parent classes
- Field access: direct lookup in object fields

## 2. Module System

### 2.1 Module Definitions

**Syntax:**
```
module module_name {
    // Classes, functions, variables
    class MyClass { ... }
    fn my_function() { ... }
    Str global_var = "value"
}
```

**Examples:**
```
module animals {
    class Cat {
        Str name
        fn meow() { println("{this.name} meows") }
    }

    fn create_cat(name: Str) -> Cat {
        Cat cat = new Cat()
        cat.name = name
        return cat
    }
}

module utils {
    fn print_hello(name: Str) {
        println("Hello {name}!")
    }
}
```

### 2.2 Import System

**Syntax:**
```
import module_name
import module_name.item_name
import module_name as alias
```

**Examples:**
```
import animals
import utils.print_hello
import animals as pet_module

Cat myCat = animals.create_cat("Whiskers")
utils.print_hello("World")
print_hello("Direct import")
pet_module.create_cat("Fluffy")
```

### 2.3 Namespaced Access

**Syntax:**
```
module_name::item_name
```

**Examples:**
```
animals::create_cat("Test")
utils::print_hello("Test")
```

### 2.4 Implementation Details

**AST Nodes:**
- `ModuleDecl { name, body: Vec<Node> }`
- `ImportStmt { module, item: Option<String>, alias: Option<String> }`

**Runtime:**
- Module environment: `HashMap<String, Environment>`
- Import resolution: copy items from module to current environment
- Namespaced access: traverse module hierarchy

## 3. Additional Language Features

### 3.1 Arrays

**Syntax:**
```
Type[] array_name = [value1, value2, value3]
array_name[index] = new_value
Type value = array_name[index]
```

**Examples:**
```
Int[] numbers = [1, 2, 3, 4, 5]
numbers[0] = 10
println(numbers[2])  // 3

Str[] names = ["Alice", "Bob", "Charlie"]
names[1] = "Dave"
```

### 3.2 Control Flow

#### If/Else Statements
**Syntax:**
```
if (condition) {
    // statements
} else if (condition) {
    // statements
} else {
    // statements
}
```

**Examples:**
```
if (x > 10) {
    println("x is greater than 10")
} else if (x > 5) {
    println("x is greater than 5")
} else {
    println("x is small")
}
```

#### While Loops
**Syntax:**
```
while (condition) {
    // statements
}
```

**Examples:**
```
Int i = 0
while (i < 10) {
    println(i)
    i = i + 1
}
```

#### For Loops
**Syntax:**
```
for (item in array) {
    // statements using item
}
```

**Examples:**
```
Int[] numbers = [1, 2, 3, 4, 5]
for (num in numbers) {
    println(num)
}
```

### 3.3 Implementation Details

**AST Nodes:**
- `ArrayLiteral { elements: Vec<Node> }`
- `ArrayAccess { array, index }`
- `IfStmt { condition, then_branch, else_branch: Option<Node> }`
- `WhileStmt { condition, body }`
- `ForStmt { iterator, iterable, body }`

**Runtime:**
- `Value::Array(Vec<Value>)`
- Control flow evaluation with proper scoping
- Iterator protocol for for loops

## 4. Parser Extensions

### 4.1 New Keywords
- `class`, `extends`, `new`, `this`
- `module`, `import`, `as`
- `if`, `else`, `while`, `for`, `in`

### 4.2 New Tokens
- `CLASS`, `EXTENDS`, `NEW`, `THIS`
- `MODULE`, `IMPORT`, `AS`
- `IF`, `ELSE`, `WHILE`, `FOR`, `IN`
- `LBRACKET`, `RBRACKET` for arrays
- `DOUBLE_COLON` for namespacing

## 5. Interpreter Extensions

### 5.1 Environment Hierarchy
- Global environment
- Module environments
- Function scopes
- Block scopes

### 5.2 Method Resolution
1. Check object class methods
2. Check parent class methods (recursive)
3. Check global functions

### 5.3 Import Resolution
1. Load module if not loaded
2. Copy requested items to current environment
3. Handle name conflicts with aliases

## 6. Transpiler Updates

### 6.1 Class Transpilation
```
class Dog extends Animal {
    Str breed
    fn speak() { println("Woof") }
}
```
↓
```
struct Dog {
    name: String,
    age: i64,
    breed: String,
}

impl Dog {
    fn speak(&self) {
        println!("Woof");
    }
}
```

### 6.2 Module Transpilation
```
module animals {
    class Dog { ... }
}
```
↓
```
mod animals {
    struct Dog { ... }
}
```

## 7. Error Handling

### 7.1 Inheritance Errors
- Circular inheritance
- Undefined parent class
- Method signature mismatch

### 7.2 Module Errors
- Module not found
- Import conflicts
- Undefined symbols

### 7.3 Runtime Errors
- Null pointer access
- Array bounds errors
- Type mismatches

## 8. Testing Strategy

### 8.1 Unit Tests
- AST node creation
- Parser correctness
- Interpreter evaluation

### 8.2 Integration Tests
- Complete programs with inheritance
- Module interactions
- Import resolution

### 8.3 Example Programs
- Animal hierarchy
- Math utilities module
- Game with objects and arrays