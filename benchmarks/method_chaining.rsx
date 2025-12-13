// Method chaining benchmark

print("Method chaining benchmark...")

iterations = 10000
counter = 0

// Simple chaining
while counter < iterations {
    result = "hello world".upper().len()
    counter = counter + 1
}

print(`Completed {iterations} simple chain operations`)

// Complex chaining
counter = 0
while counter < iterations {
    data = "  Apple,Banana,Cherry  "
    processed = data.trim().lower().split(",")
    size = processed.len()
    
    counter = counter + 1
}

print(`Completed {iterations} complex chain operations`)

// Math method chaining
counter = 0
while counter < iterations {
    pi = 3.14159
    rounded = pi.round()
    floored = pi.floor()
    ceiled = pi.ceil()
    
    counter = counter + 1
}

print(`Completed {iterations} math method operations`)

print("Method chaining benchmark complete!")
