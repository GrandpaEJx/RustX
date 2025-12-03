use crate::error::{Error, Result};

pub struct Transpiler;

impl Transpiler {
    pub fn new() -> Self {
        Transpiler
    }

    pub fn transpile(&self, code: &str) -> Result<String> {
        // Very basic transpiler for simple Rust expressions
        let mut output = String::new();
        let lines: Vec<&str> = code.lines().collect();

        for line in lines {
            let trimmed = line.trim();
            if trimmed.is_empty() {
                continue;
            }

            if trimmed.starts_with("let ") {
                // Convert let x: i32 = 5; to Int x = 5
                if let Some(eq_pos) = trimmed.find('=') {
                    let left = trimmed[4..eq_pos].trim();
                    let right = trimmed[eq_pos + 1..].trim().trim_end_matches(';');

                    // Parse type if present
                    let (name, var_type) = if let Some(colon_pos) = left.find(':') {
                        let name_str = left[..colon_pos].trim();
                        let type_str = left[colon_pos + 1..].trim();
                        let rsx_type = match type_str {
                            "i32" | "i64" => "Int",
                            "f32" | "f64" => "Float",
                            "bool" => "Bool",
                            "&str" | "String" => "Str",
                            _ => "Auto",
                        };
                        (name_str, rsx_type)
                    } else {
                        (left, "Auto")
                    };

                    output.push_str(&format!("{} {} = {}\n", var_type, name, right));
                }
            } else if trimmed.contains("println!") {
                // Convert println!("{}", x); to println(x)
                if let Some(start) = trimmed.find("println!(\"{}\", ") {
                    if let Some(end) = trimmed[start..].find(')') {
                        let var = &trimmed[start + 15..start + end];
                        output.push_str(&format!("println({})\n", var.trim()));
                    }
                } else if trimmed == "println!();" {
                    output.push_str("println()\n");
                }
            } else {
                // For other lines, try to convert basic expressions
                output.push_str(&format!("{}\n", trimmed));
            }
        }

        Ok(output)
    }
}