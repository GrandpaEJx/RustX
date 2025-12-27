use crate::value::Value;
use std::collections::HashMap;

pub fn env(args: Vec<Value>) -> Result<Value, String> {
    if args.is_empty() {
        // Return all env vars as map
        let mut map = HashMap::new();
        for (k, v) in std::env::vars() {
            map.insert(k, Value::String(v));
        }
        return Ok(Value::Map(map));
    }

    let key = match &args[0] {
        Value::String(s) => s,
        _ => return Err("os.env expects a string key".to_string()),
    };

    match std::env::var(key) {
        Ok(v) => Ok(Value::String(v)),
        Err(_) => Ok(Value::Null),
    }
}

pub fn args(_args: Vec<Value>) -> Result<Value, String> {
    let args: Vec<Value> = std::env::args().map(Value::String).collect();
    Ok(Value::Array(args))
}
