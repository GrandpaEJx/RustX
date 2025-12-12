// Simple loop test

// Test 1: While loop
counter = 0
sum = 0
while counter < 5 {
    sum = sum + counter
    counter = counter + 1
}

// Test 2: For loop with array
numbers = [1, 2, 3, 4, 5]
total = 0
for num in numbers {
    total = total + num
}

// Test 3: Range function
range_sum = 0
for i in range(5) {
    range_sum = range_sum + i
}

// Return final result
sum + total + range_sum
