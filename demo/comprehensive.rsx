// Comprehensive RustX Example
// This file demonstrates all implemented language features

println("=== RustX Comprehensive Demo ===")

// Variable declarations with different types
Str name = "RustX Scripting"
Int age = 5
Bool isAwesome = true
Float version = 1.0

printf("Language: {name}\n")
printf("Version: {version}\n")
printf("Is awesome: {isAwesome}\n")
printf("Age: {age} years\n")

// Arithmetic operations
Int a = 15
Int b = 4

println("\nArithmetic Operations:")
printf("a = {a}, b = {b}\n")
printf("a + b = {a + b}\n")
printf("a - b = {a - b}\n")
printf("a * b = {a * b}\n")
printf("a / b = {a / b}\n")

// Float operations
Float pi = 3.14159
Float radius = 5.0

println("\nFloat Operations:")
printf("Circle with radius {radius}:\n")
printf("Circumference = {2 * pi * radius}\n")
printf("Area = {pi * radius * radius}\n")

// Complex expressions that work
Int result = (a + b) * (a - b) / 2
printf("\nComplex expression result: {result}\n")

// Nested expressions in printf
printf("Math is fun: {((10 + 5) * 2) / 3}\n")

println("\n=== Demo Complete ===")