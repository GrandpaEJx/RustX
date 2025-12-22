import "fs" as fs

print("Testing Filesystem Module")

// 1. Write File
print("Writing to test.txt...")
fs.write("test.txt", "Hello from RustX FS!")
if fs.exists("test.txt") {
    print("PASS: File created")
} else {
    print("FAIL: File not created")
}

// 2. Read File
let content = fs.read("test.txt")
print(`Content: {content}`)
if content == "Hello from RustX FS!" {
    print("PASS: Read correct content")
} else {
    print("FAIL: Read incorrect content")
}

// 3. Append File
print("Appending to test.txt...")
fs.append("test.txt", "\nAppended line")
let new_content = fs.read("test.txt")
print(`New Content:\n{new_content}`)

// 4. Remove File
print("Removing test.txt...")
fs.remove("test.txt")
if fs.exists("test.txt") {
    print("FAIL: File still exists")
} else {
    print("PASS: File removed")
}
