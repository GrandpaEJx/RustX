fn greet(name: &String) -> String {
    return name
}

let message = "World"
println("Testing named parameters")
let result = greet(name = message)
println(result)