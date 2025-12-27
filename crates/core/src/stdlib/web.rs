use crate::value::Value;
use actix_web::{web, App, HttpResponse, HttpServer};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

#[derive(Clone)]
struct Route {
    method: String,
    path: String,
    handler: Value,
}

struct AppConfig {
    routes: Vec<Route>,
    host: String,
    workers: usize,
    debug: bool,
}

pub fn app(args: Vec<Value>) -> Result<Value, String> {
    let mut host = "127.0.0.1".to_string();
    let mut workers = 1;
    let mut debug = false;

    if !args.is_empty() {
        if let Value::Map(m) = &args[0] {
            if let Some(Value::String(h)) = m.get("host") {
                host = h.clone();
            }
            if let Some(Value::Int(w)) = m.get("workers") {
                workers = *w as usize;
            }
            if let Some(Value::Bool(d)) = m.get("debug") {
                debug = *d;
            }
        }
    }

    let config = Arc::new(Mutex::new(AppConfig {
        routes: Vec::new(),
        host,
        workers,
        debug,
    }));

    let mut map = HashMap::new();

    // Helper to register methods
    let register =
        |method: &str, map: &mut HashMap<String, Value>, config: Arc<Mutex<AppConfig>>| {
            let method = method.to_string();
            map.insert(
                method.to_lowercase(),
                Value::NativeFunction(Arc::new(move |args| {
                    if args.len() != 2 {
                        return Err(format!(
                            "app.{} expects 2 arguments: path, handler",
                            method.to_lowercase()
                        ));
                    }
                    let path = args[0].to_string();
                    let handler = args[1].clone();

                    let mut cfg = config.lock().map_err(|e| e.to_string())?;
                    cfg.routes.push(Route {
                        method: method.clone(),
                        path,
                        handler,
                    });
                    Ok(Value::Null)
                })),
            );
        };

    register("GET", &mut map, Arc::clone(&config));
    register("POST", &mut map, Arc::clone(&config));
    register("PUT", &mut map, Arc::clone(&config));
    register("DELETE", &mut map, Arc::clone(&config));
    register("PATCH", &mut map, Arc::clone(&config));
    register("HEAD", &mut map, Arc::clone(&config));
    register("OPTIONS", &mut map, Arc::clone(&config));

    // app.listen(port, options?)
    let config_clone = Arc::clone(&config);
    map.insert(
        "listen".to_string(),
        Value::NativeFunction(Arc::new(move |args| {
            if args.is_empty() || args.len() > 4 {
                return Err(
                    "app.listen expects 1 to 4 arguments: port, [debug], [workers], [host]"
                        .to_string(),
                );
            }

            let port = args[0].as_int()? as u16;

            let (mut host, mut workers, mut debug, routes) = {
                let cfg = config_clone.lock().map_err(|e| e.to_string())?;
                (cfg.host.clone(), cfg.workers, cfg.debug, cfg.routes.clone())
            };

            if args.len() == 2 {
                match &args[1] {
                    Value::Map(m) => {
                        if let Some(Value::String(h)) = m.get("host") {
                            host = h.clone();
                        }
                        if let Some(Value::Int(w)) = m.get("workers") {
                            workers = *w as usize;
                        }
                        if let Some(Value::Bool(d)) = m.get("debug") {
                            debug = *d;
                        }
                    }
                    Value::Bool(d) => {
                        debug = *d;
                    }
                    _ => {
                        return Err(
                            "app.listen: Second argument must be an options map or debug boolean"
                                .to_string(),
                        )
                    }
                }
            } else if args.len() >= 3 {
                if let Value::Bool(d) = &args[1] {
                    debug = *d;
                } else {
                    return Err("app.listen: Second argument (debug) must be boolean".to_string());
                }

                if let Value::Int(w) = &args[2] {
                    workers = *w as usize;
                } else {
                    return Err("app.listen: Third argument (workers) must be integer".to_string());
                }

                if args.len() == 4 {
                    if let Value::String(h) = &args[3] {
                        host = h.clone();
                    } else {
                        return Err("app.listen: Fourth argument (host) must be string".to_string());
                    }
                }
            }

            if debug {
                println!(
                    "Server starting on {}:{} with {} workers (debug={})",
                    host, port, workers, debug
                );
            }

            // Start Actix System
            actix_web::rt::System::new()
                .block_on(async move {
                    HttpServer::new(move || {
                        let mut app = App::new();

                        for route in &routes {
                            let handler_val = route.handler.clone();
                            let route_method = route.method.clone();
                            // Capturing debug for logging if needed
                            let debug_val = debug;

                            let actix_handler = move |req_body: String| {
                                let h = handler_val.clone();
                                async move {
                                    let args = vec![
                                        Value::String(req_body),
                                        // Pass debug flag as second argument
                                        Value::Bool(debug_val),
                                    ];

                                    let result = h.call(args);

                                    match result {
                                        Ok(v) => match v {
                                            Value::String(s) => HttpResponse::Ok().body(s),
                                            Value::Map(_)
                                            | Value::Array(_)
                                            | Value::Int(_)
                                            | Value::Float(_)
                                            | Value::Bool(_)
                                            | Value::Null => {
                                                let json_val = to_serde_json(&v);
                                                HttpResponse::Ok().json(json_val)
                                            }
                                            _ => HttpResponse::Ok().body(format!("{}", v)),
                                        },
                                        Err(e) => HttpResponse::InternalServerError().body(e),
                                    }
                                }
                            };

                            let actix_route = match route_method.as_str() {
                                "GET" => web::get(),
                                "POST" => web::post(),
                                "PUT" => web::put(),
                                "DELETE" => web::delete(),
                                "PATCH" => web::patch(),
                                "HEAD" => web::head(),
                                _ => web::get(), // Default/Fallback
                            };

                            app = app.route(&route.path, actix_route.to(actix_handler));
                        }
                        app
                    })
                    .workers(workers)
                    .bind((host, port))
                    .unwrap()
                    .run()
                    .await
                })
                .map_err(|e| e.to_string())?;

            Ok(Value::Null)
        })),
    );

    Ok(Value::Map(map))
}

fn to_serde_json(v: &Value) -> serde_json::Value {
    match v {
        Value::Null => serde_json::Value::Null,
        Value::Int(n) => serde_json::Value::Number((*n).into()),
        Value::Float(f) => {
            if let Some(n) = serde_json::Number::from_f64(*f) {
                serde_json::Value::Number(n)
            } else {
                serde_json::Value::Null
            }
        }
        Value::Bool(b) => serde_json::Value::Bool(*b),
        Value::String(s) => serde_json::Value::String(s.clone()),
        Value::Array(arr) => {
            let vec: Vec<serde_json::Value> = arr.iter().map(to_serde_json).collect();
            serde_json::Value::Array(vec)
        }
        Value::Map(map) => {
            let mut m = serde_json::Map::new();
            for (k, v) in map {
                m.insert(k.clone(), to_serde_json(v));
            }
            serde_json::Value::Object(m)
        }
        _ => serde_json::Value::String(format!("{:?}", v)),
    }
}

pub fn json(args: Vec<Value>) -> Result<Value, String> {
    if args.is_empty() {
        return Err("json expect 1 arg".to_string());
    }
    crate::stdlib::json::stringify(args)
}
