// Working RustX Example
// This file demonstrates implemented language features

println("=== RustX Demo ===")

// Variable declarations
Str name = "RustX"
Int a = 15
Int b = 4
Float pi = 3.14
Float radius = 5.0

// Basic printing
printf("Language: {name}\n")
printf("a = {a}, b = {b}\n")

// Arithmetic (simple cases work)
Int sum = a + b
Int diff = a - b
Int prod = a * b

printf("Sum: {sum}\n")
printf("Difference: {diff}\n") 
printf("Product: {prod}\n")

// Float operations
printf("Pi: {pi}\n")
printf("Radius: {radius}\n")
printf("Area: {pi * radius * radius}\n")

// Complex expressions (when assigned to variables first)
Int complex = (a + b) * (a - b) / 2
printf("Complex result: {complex}\n")

// Direct arithmetic in print
print("Direct addition: ")
println(a + b)

println("=== Demo Complete ===")