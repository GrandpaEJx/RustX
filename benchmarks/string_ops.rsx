// String operations benchmark

print("String operations benchmark...")

iterations = 10000
counter = 0

// String concatenation and manipulation
while counter < iterations {
    text = "Hello World"
    upper = text.upper()
    lower = text.lower()
    trimmed = "  test  ".trim()
    
    counter = counter + 1
}

print(`Completed {iterations} string operations`)

// Template string performance
counter = 0
while counter < iterations {
    name = "RustX"
    version = "0.2.0"
    message = `{name} version {version}`
    
    counter = counter + 1
}

print(`Completed {iterations} template string operations`)

// String splitting and joining
counter = 0
while counter < iterations {
    csv = "a,b,c,d,e"
    parts = csv.split(",")
    joined = join(parts, "-")
    
    counter = counter + 1
}

print(`Completed {iterations} split/join operations`)

print("String benchmark complete!")
