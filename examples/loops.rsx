// Loop examples demonstrating while, for, and range

// Example 1: While loop
counter = 0
sum = 0
while counter < 5 {
    sum = sum + counter
    counter = counter + 1
}
print("While loop sum:", sum)

// Example 2: For loop with array
numbers = [1, 2, 3, 4, 5]
total = 0
for num in numbers {
    total = total + num
}
print("For loop total:", total)

// Example 3: Range function - range(end)
print("Range 0 to 5:")
for i in range(5) {
    print(i)
}

// Example 4: Range function - range(start, end)
print("Range 2 to 7:")
for i in range(2, 7) {
    print(i)
}

// Example 5: Range function - range(start, end, step)
print("Range 0 to 10, step 2:")
for i in range(0, 10, 2) {
    print(i)
}

// Example 6: Nested loops
print("Multiplication table (3x3):")
for i in range(1, 4) {
    for j in range(1, 4) {
        product = i * j
        print(i, "*", j, "=", product)
    }
}

// Example 7: Building an array with range
squares = []
for n in range(1, 6) {
    square = n * n
    // Note: array append not yet implemented, so we can't build arrays dynamically
    print("Square of", n, "is", square)
}

"Loops demonstration complete"
