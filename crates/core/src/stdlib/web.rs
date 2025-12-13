use crate::value::Value;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use actix_web::{web, App, HttpResponse, HttpServer, Responder};

#[derive(Clone)]
struct Route {
    method: String,
    path: String,
    handler: Value,
}

struct AppConfig {
    routes: Vec<Route>,
}

pub fn app(_args: Vec<Value>) -> Result<Value, String> {
    let config = Arc::new(Mutex::new(AppConfig {
        routes: Vec::new(),
    }));
    
    let mut map = HashMap::new();
    
    // app.get(path, handler)
    let config_clone = Arc::clone(&config);
    map.insert("get".to_string(), Value::NativeFunction(Arc::new(move |args| {
        if args.len() != 2 { return Err("app.get expects 2 arguments: path, handler".to_string()); }
        let path = args[0].to_string();
        let handler = args[1].clone();
        
        let mut cfg = config_clone.lock().map_err(|e| e.to_string())?;
        cfg.routes.push(Route {
            method: "GET".to_string(),
            path,
            handler,
        });
        Ok(Value::Null)
    })));
    
    // app.post(path, handler)
    let config_clone = Arc::clone(&config);
    map.insert("post".to_string(), Value::NativeFunction(Arc::new(move |args| {
        if args.len() != 2 { return Err("app.post expects 2 arguments: path, handler".to_string()); }
        let path = args[0].to_string();
        let handler = args[1].clone();
        
        let mut cfg = config_clone.lock().map_err(|e| e.to_string())?;
        cfg.routes.push(Route {
             method: "POST".to_string(),
             path,
             handler,
        });
        Ok(Value::Null)
    })));
    
    // app.listen(port)
    let config_clone = Arc::clone(&config);
    map.insert("listen".to_string(), Value::NativeFunction(Arc::new(move |args| {
        if args.len() != 1 { return Err("app.listen expects 1 argument: port".to_string()); }
        let port = args[0].as_int()? as u16;
        
        let routes = {
            let cfg = config_clone.lock().map_err(|e| e.to_string())?;
            cfg.routes.clone()
        };
        
        println!("Server starting on port {}", port);
        
        // Start Actix System
        actix_web::rt::System::new().block_on(async move {
            HttpServer::new(move || {
                let mut app = App::new();
                
                for route in &routes {
                    let handler_val = route.handler.clone();
                    let route_method = route.method.clone();
                    
                    let actix_handler = move |req_body: String| {
                        let h = handler_val.clone();
                         async move {
                            let mut args = Vec::new();
                            // Pass body directly for simplicity
                             args.push(Value::String(req_body));

                            // Use .call helper or manual match (since strict type usage inside interpreter crate)
                            // But web.rs is inside core, so it has access to Value::call if pub?
                            // Checked value.rs: call is pub.
                            // BUT call returns Result.
                            
                            let result = h.call(args);
                            
                            match result {
                                Ok(v) => {
                                    match v {
                                        Value::String(s) => HttpResponse::Ok().body(s),
                                        Value::Map(_) | Value::Array(_) | Value::Int(_) | Value::Float(_) | Value::Bool(_) | Value::Null => {
                                            let json_val = to_serde_json(&v);
                                            HttpResponse::Ok().json(json_val)
                                        },
                                        _ => HttpResponse::Ok().body(format!("{}", v)),
                                    }
                                }
                                Err(e) => HttpResponse::InternalServerError().body(e),
                            }
                        }
                    };
                    
                    if route_method == "GET" {
                         app = app.route(&route.path, web::get().to(actix_handler));
                    } else if route_method == "POST" {
                         app = app.route(&route.path, web::post().to(actix_handler));
                    }
                }
                app
            })
            .bind(("127.0.0.1", port)).unwrap() // Unwrap for V1 simplicity
            .run()
            .await
        }).map_err(|e| e.to_string())?;
        
        Ok(Value::Null)
    })));
    
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
        },
        Value::Bool(b) => serde_json::Value::Bool(*b),
        Value::String(s) => serde_json::Value::String(s.clone()),
        Value::Array(arr) => {
            let vec: Vec<serde_json::Value> = arr.iter().map(to_serde_json).collect();
            serde_json::Value::Array(vec)
        },
        Value::Map(map) => {
            let mut m = serde_json::Map::new();
            for (k, v) in map {
                m.insert(k.clone(), to_serde_json(v));
            }
            serde_json::Value::Object(m)
        },
        _ => serde_json::Value::String(format!("{:?}", v)),
    }
}


pub fn json(args: Vec<Value>) -> Result<Value, String> {
    if args.is_empty() { return Err("json expect 1 arg".to_string()); }
    crate::stdlib::json::stringify(args)
}
