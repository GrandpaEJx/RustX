// Array operations benchmark

print("Array operations benchmark...")

iterations = 10000
counter = 0

// Array creation and access
while counter < iterations {
    arr = [1, 2, 3, 4, 5]
    first = arr[0]
    last = arr[4]
    size = arr.len()
    
    counter = counter + 1
}

print(`Completed {iterations} array access operations`)

// Array push/pop
counter = 0
while counter < iterations {
    arr = []
    push(arr, 1)
    push(arr, 2)
    push(arr, 3)
    val = pop(arr)
    
    counter = counter + 1
}

print(`Completed {iterations} push/pop operations`)

// Array iteration
counter = 0
total = 0
while counter < 1000 {
    arr = range(100)
    for item in arr {
        total = total + item
    }
    counter = counter + 1
}

print(`Completed 1000 array iterations (100 items each)`)

print("Array benchmark complete!")
