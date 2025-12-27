use crate::value::Value;
use std::fs;
use std::io::Write;

pub fn read(args: Vec<Value>) -> Result<Value, String> {
    if args.len() != 1 {
        return Err("fs.read expects 1 argument (path)".to_string());
    }

    let path = match &args[0] {
        Value::String(s) => s,
        _ => return Err("fs.read expects a string path".to_string()),
    };

    match fs::read_to_string(path) {
        Ok(content) => Ok(Value::String(content)),
        Err(e) => Err(format!("Failed to read file '{}': {}", path, e)),
    }
}

pub fn write(args: Vec<Value>) -> Result<Value, String> {
    if args.len() != 2 {
        return Err("fs.write expects 2 arguments (path, content)".to_string());
    }

    let path = match &args[0] {
        Value::String(s) => s,
        _ => return Err("fs.write expects a string path".to_string()),
    };

    let content = match &args[1] {
        Value::String(s) => s,
        _ => return Err("fs.write expects a string content".to_string()),
    };

    match fs::write(path, content) {
        Ok(_) => Ok(Value::Null),
        Err(e) => Err(format!("Failed to write file '{}': {}", path, e)),
    }
}

pub fn append(args: Vec<Value>) -> Result<Value, String> {
    if args.len() != 2 {
        return Err("fs.append expects 2 arguments (path, content)".to_string());
    }

    let path = match &args[0] {
        Value::String(s) => s,
        _ => return Err("fs.append expects a string path".to_string()),
    };

    let content = match &args[1] {
        Value::String(s) => s,
        _ => return Err("fs.append expects a string content".to_string()),
    };

    let mut file = fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(path)
        .map_err(|e| format!("Failed to open file '{}': {}", path, e))?;

    write!(file, "{}", content)
        .map_err(|e| format!("Failed to append to file '{}': {}", path, e))?;

    Ok(Value::Null)
}

pub fn exists(args: Vec<Value>) -> Result<Value, String> {
    if args.len() != 1 {
        return Err("fs.exists expects 1 argument (path)".to_string());
    }

    let path = match &args[0] {
        Value::String(s) => s,
        _ => return Err("fs.exists expects a string path".to_string()),
    };

    Ok(Value::Bool(std::path::Path::new(path).exists()))
}

pub fn remove(args: Vec<Value>) -> Result<Value, String> {
    if args.len() != 1 {
        return Err("fs.remove expects 1 argument (path)".to_string());
    }

    let path = match &args[0] {
        Value::String(s) => s,
        _ => return Err("fs.remove expects a string path".to_string()),
    };

    match fs::remove_file(path) {
        Ok(_) => Ok(Value::Null),
        Err(e) => Err(format!("Failed to remove file '{}': {}", path, e)),
    }
}
