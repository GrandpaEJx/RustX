use crate::value::Value;
use std::collections::HashMap;

pub fn get(args: Vec<Value>) -> Result<Value, String> {
    if args.len() != 1 {
        return Err("http.get expects 1 argument (url)".to_string());
    }
    
    let url = match &args[0] {
        Value::String(s) => s,
        _ => return Err("http.get expect a URL string".to_string()),
    };
    
    let resp = reqwest::blocking::get(url).map_err(|e| e.to_string())?;
    let text = resp.text().map_err(|e| e.to_string())?;
    
    // In future, return a Response object (Map). For now, return text body.
    // Or return map { status: ..., body: ... }
    let mut map = HashMap::new();
    map.insert("body".to_string(), Value::String(text));
    // map.insert("status".to_string(), Value::Int(resp.status().as_u16() as i64)); // resp consumed above
    
    Ok(Value::Map(map))
}

pub fn post(args: Vec<Value>) -> Result<Value, String> {
    if args.len() < 2 {
        return Err("http.post expects at least 2 arguments (url, body)".to_string());
    }
    
    let url = match &args[0] {
        Value::String(s) => s,
        _ => return Err("http.post expects a URL string".to_string()),
    };
    
    let client = reqwest::blocking::Client::new();
    let mut req = client.post(url);
    
    // Body: string or map (json)
    match &args[1] {
        Value::String(s) => {
            req = req.body(s.clone());
            req = req.header("Content-Type", "text/plain");
        }
        Value::Map(_) => {
            // Assume JSON
             let json_val = super::json::stringify(vec![args[1].clone()])?;
             if let Value::String(s) = json_val {
                 req = req.body(s);
                 req = req.header("Content-Type", "application/json");
             }
        }
        _ => return Err("http.post body must be string or map".to_string()),
    }
    
    let resp = req.send().map_err(|e| e.to_string())?;
    let text = resp.text().map_err(|e| e.to_string())?;
    
    let mut map = HashMap::new();
    map.insert("body".to_string(), Value::String(text));
    
    Ok(Value::Map(map))
}
