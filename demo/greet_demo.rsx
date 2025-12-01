// Demonstrate the greet function with named parameters
fn greet(name: &String) -> String {
    printf("Greeting for: {}", name)
    return name
}

let message = "RustX Developer"
let title = "Senior"
println("Testing greet function with string interpolation")

// Test 1: Simple named parameter call
let result1 = greet(name = message)
println("Simple call result: ")
println(result1)

// Test 2: Nested function calls with named parameters
let result2 = greet(name = "World")
println("Nested call result: ")
println(result2)

// Test 3: Multiple variables
let hello = "Hello"
let world = "World"
let result3 = greet(name = world)
println("Multiple vars result: ")
println(result3)

printf("All tests completed successfully!")