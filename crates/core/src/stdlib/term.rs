use crate::value::Value;
// use std::sync::Arc;

pub fn red(args: Vec<Value>) -> Result<Value, String> {
    colorize(args, "31")
}

pub fn green(args: Vec<Value>) -> Result<Value, String> {
    colorize(args, "32")
}

pub fn yellow(args: Vec<Value>) -> Result<Value, String> {
    colorize(args, "33")
}

pub fn blue(args: Vec<Value>) -> Result<Value, String> {
    colorize(args, "34")
}

pub fn magenta(args: Vec<Value>) -> Result<Value, String> {
    colorize(args, "35")
}

pub fn cyan(args: Vec<Value>) -> Result<Value, String> {
    colorize(args, "36")
}

pub fn white(args: Vec<Value>) -> Result<Value, String> {
    colorize(args, "37")
}

pub fn bg_red(args: Vec<Value>) -> Result<Value, String> {
    colorize(args, "41")
}

pub fn bg_green(args: Vec<Value>) -> Result<Value, String> {
    colorize(args, "42")
}

pub fn bg_yellow(args: Vec<Value>) -> Result<Value, String> {
    colorize(args, "43")
}

pub fn bg_blue(args: Vec<Value>) -> Result<Value, String> {
    colorize(args, "44")
}

pub fn bg_magenta(args: Vec<Value>) -> Result<Value, String> {
    colorize(args, "45")
}

pub fn bg_cyan(args: Vec<Value>) -> Result<Value, String> {
    colorize(args, "46")
}

pub fn bg_white(args: Vec<Value>) -> Result<Value, String> {
    colorize(args, "47")
}

pub fn bold(args: Vec<Value>) -> Result<Value, String> {
    colorize(args, "1")
}

pub fn dim(args: Vec<Value>) -> Result<Value, String> {
    colorize(args, "2")
}

pub fn italic(args: Vec<Value>) -> Result<Value, String> {
    colorize(args, "3")
}

pub fn underline(args: Vec<Value>) -> Result<Value, String> {
    colorize(args, "4")
}

fn colorize(args: Vec<Value>, code: &str) -> Result<Value, String> {
    if args.len() != 1 {
        return Err("Color function expects 1 argument (text)".to_string());
    }
    
    let text = args[0].to_string();
    Ok(Value::String(format!("\x1b[{}m{}\x1b[0m", code, text)))
}

pub fn clear(args: Vec<Value>) -> Result<Value, String> {
    if !args.is_empty() {
        return Err("term.clear expects 0 arguments".to_string());
    }
    // Clear screen and move cursor to top-left
    print!("\x1b[2J\x1b[1;1H");
    Ok(Value::Null)
}
