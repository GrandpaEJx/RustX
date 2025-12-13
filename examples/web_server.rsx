rust {}
print("Starting Web Server Example...")

fn home(req) {
    print("Home request")
    return web.json({ "message": "Welcome to RustX Web Framework!", "status": "ok" })
}

fn echo(req) {
    print("Received echo request:", req)
    return web.json(req)
}

let server = web.app()
server.get("/", home)
server.post("/echo", echo)

print("Listening on 8080...")
server.listen(8080)
