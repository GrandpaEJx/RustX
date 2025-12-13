// Example demonstrating built-in functions

// len() - get length of arrays, strings, maps
numbers = [1, 2, 3, 4, 5]
print("Array:", numbers)
print("Length:", len(numbers))

text = "Hello, RustX!"
print("String:", text)
print("Length:", len(text))

// type() - get type of values
print("\nType checking:")
print("type(42) =", type(42))
print("type(3.14) =", type(3.14))
print("type(\"hello\") =", type("hello"))
print("type(true) =", type(true))
print("type([1,2,3]) =", type([1,2,3]))

// push() - add elements to array
print("\nArray manipulation:")
fruits = ["apple", "banana"]
print("Initial:", fruits)

push(fruits, "cherry")
print("After push:", fruits)

push(fruits, "date")
print("After another push:", fruits)
print("Length now:", len(fruits))

// pop() - remove last element
print("\nPopping elements:")
last = pop(fruits)
print("Popped:", last)
print("Array now:", fruits)

last = pop(fruits)
print("Popped:", last)
print("Array now:", fruits)

// Combining functions
print("\nCombining functions:")
items = []
for i in range(1, 6) {
    push(items, i * 10)
}
print("Built array:", items)
print("Length:", len(items))
print("Type:", type(items))

// Final result
items
