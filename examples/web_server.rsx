use crate "actix-web" = "4"

rust {
    use actix_web::{get, App, HttpServer, Responder};

    #[get("/")]
    async fn hello() -> impl Responder {
        "Hello from RustX Web Server!"
    }

    #[actix_web::main]
    async fn run_actix() -> std::io::Result<()> {
        println!("Server running locally at http://127.0.0.1:8080");
        HttpServer::new(|| {
            App::new().service(hello)
        })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
    }

    fn start_server() -> Result<Value, String> {
        match run_actix() {
            Ok(_) => Ok(Value::Null),
            Err(e) => Err(e.to_string())
        }
    }
}

print("Initializing Actix Web Server...")
start_server()
