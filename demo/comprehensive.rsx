// Comprehensive RustX Example
// This file demonstrates most language features

println("=== RustX Comprehensive Demo ===\n")

// Variable declarations with different types
Str name = "RustX Scripting"
Int age = 5
Bool isAwesome = true
Float version = 1.0

printf("Language: {name}\n")
printf("Version: {version}\n")
printf("Is awesome: {isAwesome}\n")
printf("Age: {age} years\n\n")

// Arithmetic operations
Int a = 15
Int b = 4

println("Arithmetic Operations:")
printf("a = {a}, b = {b}\n")
printf("a + b = {a + b}\n")
printf("a - b = {a - b}\n")
printf("a * b = {a * b}\n")
printf("a / b = {a / b}\n\n")

// Float operations
Float pi = 3.14159
Float radius = 5.0

println("Float Operations:")
printf("Circle with radius {radius}:\n")
printf("Circumference = {2 * pi * radius}\n")
printf("Area = {pi * radius * radius}\n\n")

// Boolean logic
Bool isAdult = age >= 18
Bool isYoung = age < 30

println("Boolean Logic:")
printf("Is adult: {isAdult}\n")
printf("Is young: {isYoung}\n")
printf("Is young adult: {isAdult && isYoung}\n\n")

// String operations
Str greeting = "Hello"
Str target = "World"

println("String Examples:")
printf("Concatenation: {greeting + ' ' + target}\n")
printf("Length of '{name}': {name.length()}\n\n")

// Complex expressions
Int result = (a + b) * (a - b) / 2
printf("Complex expression result: {result}\n")

// Nested expressions in printf
printf("Math is fun: {((10 + 5) * 2) / 3}\n")

println("\n=== Demo Complete ===")