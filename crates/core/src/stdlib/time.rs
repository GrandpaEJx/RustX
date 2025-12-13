use crate::value::Value;
use std::{thread, time};
use chrono::Utc;

pub fn now(_args: Vec<Value>) -> Result<Value, String> {
    // Return timestamp in seconds (float)
    let now = Utc::now();
    let ts = now.timestamp() as f64 + now.timestamp_subsec_nanos() as f64 / 1_000_000_000.0;
    Ok(Value::Float(ts))
}

pub fn sleep(args: Vec<Value>) -> Result<Value, String> {
    if args.len() != 1 {
        return Err("time.sleep expects 1 argument (ms)".to_string());
    }
    
    let ms = match &args[0] {
        Value::Int(i) => *i as u64,
        Value::Float(f) => *f as u64,
        _ => return Err("time.sleep expects a number".to_string()),
    };
    
    thread::sleep(time::Duration::from_millis(ms));
    Ok(Value::Null)
}
