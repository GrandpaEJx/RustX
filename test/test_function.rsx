fn greet(name: &String) -> String {
    printf("Hello from RustX function! hello {}", name)
    return name
}

let message = "World"
println("Testing function support")
println(greet(name=message))
