use crate::value::Value;
use std::collections::HashMap;

pub fn parse(args: Vec<Value>) -> Result<Value, String> {
    if args.len() != 1 {
        return Err("json.parse expects 1 argument".to_string());
    }

    match &args[0] {
        Value::String(s) => {
            let v: serde_json::Value = serde_json::from_str(s).map_err(|e| e.to_string())?;
            json_to_value(v)
        }
        _ => Err("json.parse expects a string".to_string()),
    }
}

pub fn stringify(args: Vec<Value>) -> Result<Value, String> {
    if args.len() != 1 {
        return Err("json.stringify expects 1 argument".to_string());
    }

    // Convert Value to serde_json::Value
    let v = value_to_json(&args[0]);
    let s = serde_json::to_string(&v).map_err(|e| e.to_string())?;
    Ok(Value::String(s))
}

fn json_to_value(v: serde_json::Value) -> Result<Value, String> {
    match v {
        serde_json::Value::Null => Ok(Value::Null),
        serde_json::Value::Bool(b) => Ok(Value::Bool(b)),
        serde_json::Value::Number(n) => {
            if let Some(i) = n.as_i64() {
                Ok(Value::Int(i))
            } else if let Some(f) = n.as_f64() {
                Ok(Value::Float(f))
            } else {
                Ok(Value::Int(0)) // Should not happen for valid JSON numbers usually
            }
        }
        serde_json::Value::String(s) => Ok(Value::String(s)),
        serde_json::Value::Array(arr) => {
            let mut items = Vec::new();
            for item in arr {
                items.push(json_to_value(item)?);
            }
            Ok(Value::Array(items))
        }
        serde_json::Value::Object(obj) => {
            let mut map = HashMap::new();
            for (k, v) in obj {
                map.insert(k, json_to_value(v)?);
            }
            Ok(Value::Map(map))
        }
    }
}

fn value_to_json(v: &Value) -> serde_json::Value {
    match v {
        Value::Null => serde_json::Value::Null,
        Value::Bool(b) => serde_json::Value::Bool(*b),
        Value::Int(i) => serde_json::Value::Number(serde_json::Number::from(*i)),
        Value::Float(f) => {
            // Handle NaN/Infinity? JSON doesn't support them.
            if let Some(n) = serde_json::Number::from_f64(*f) {
                serde_json::Value::Number(n)
            } else {
                serde_json::Value::Null
            }
        }
        Value::String(s) => serde_json::Value::String(s.clone()),
        Value::Array(arr) => serde_json::Value::Array(arr.iter().map(value_to_json).collect()),
        Value::Map(map) => {
            let mut obj = serde_json::Map::new();
            for (k, v) in map {
                obj.insert(k.clone(), value_to_json(v));
            }
            serde_json::Value::Object(obj)
        }
        _ => serde_json::Value::Null, // Functions etc cannot be serialized
    }
}
