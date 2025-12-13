// Example demonstrating template string literals

name = "Alice"
age = 25
city = "New York"

// Basic template string
greeting = `Hello, my name is {name}!`
print(greeting)

// Multiple variables
info = `My name is {name} and I am {age} years old`
print(info)

// Template string with all variables
full_info = `Name: {name}, Age: {age}, City: {city}`
print(full_info)

// Template strings in expressions
message = `User {name} from {city} is {age} years old`
print(message)

// Using template strings directly
print(`Welcome, {name}!`)
print(`You are {age} years old`)

// Template strings with calculations
x = 10
y = 20
print(`{x} + {y} = ...`) // Note: expressions inside {} not yet supported, only variable names

// Return the final message
`{name} lives in {city}`
