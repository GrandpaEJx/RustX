rust {
    // Force JIT
}

import web
import time
import json

let debug = false

let app = web.app()

// Home Route
fn home(body, debug) {
    let response = {
        "name": "RustX API",
        "version": "1.0.0",
        "status": "running",
        "endpoints": [
            "/echo",
            "/time",
            "/add"
        ]
    }
    return web.json(response)
}

// Time Route
fn current_time(body, debug) {
    let now = time.now()
    return web.json({
        "timestamp": now
    })
}

// Echo Route
fn echo(body, debug) {
    return web.json({
        "your_data": body
    })
}

// Calc Route (Add) - parses JSON body
// Body: {"a": 10, "b": 20}
fn add(body, debug) {
    let input = json.parse(body) 
    
    let a = input["a"]
    let b = input["b"]
    
    let result = a + b
    
    return web.json({
        "received": input,
        "result": result
    })
}

app.get("/", home)
app.get("/time", current_time)
app.post("/echo", echo)
app.post("/add", add)

let port = 8080
let workers = 1
print("Server listening on http://localhost:", port)
app.listen(port, debug, workers)
