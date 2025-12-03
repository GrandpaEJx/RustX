use crate::error::Result;
use crate::runtime::Value;
use super::Interpreter;

impl Interpreter {
    pub fn format_string(&self, format: &str, args: &[Value]) -> Result<String> {
        let mut result = String::new();
        let mut arg_index = 0;
        let mut chars = format.chars().peekable();

        while let Some(ch) = chars.next() {
            if ch == '{' {
                if let Some(_ch2) = chars.next() {
                    if let Some(arg) = args.get(arg_index) {
                        result.push_str(&arg.to_string());
                        arg_index += 1;
                    }
                }
            } else {
                result.push(ch);
            }
        }

        Ok(result)
    }

    pub fn interpolate_string(&self, template: &str, values: &[Value]) -> Result<String> {
        let mut result = String::new();
        let mut chars = template.chars().peekable();

        while let Some(ch) = chars.next() {
            if ch == '{' {
                if let Some(_ch2) = chars.next() {
                    // For now, just use the first value or empty string
                    if let Some(value) = values.first() {
                        result.push_str(&value.to_string());
                    }
                }
            } else {
                result.push(ch);
            }
        }

        Ok(result)
    }
}