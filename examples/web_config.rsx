rust {
    // Force JIT
}

print("🚀 Starting Configurable Web Server...")

// Test Config: 2 workers, debug true
let app = web.app({
    "host": "127.0.0.1",
    "workers": 2,
    "debug": false
})

fn get_handler(body) {
    print("[GET] / received")
    return web.json({"method": "GET"})
}

fn put_handler(body) {
    print("[PUT] /update received")
    return web.json({"method": "PUT", "body": body})
}

fn delete_handler(body) {
    print("[DELETE] /remove received")
    return web.json({"method": "DELETE"})
}

app.get("/", get_handler)
app.put("/update", put_handler)
app.delete("/remove", delete_handler)

let port = 8081
print("Server listening on port:", port)
app.listen(port)
