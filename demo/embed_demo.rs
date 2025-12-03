use rustx::{run_code, Interpreter, Value, Result};

fn main() -> Result<()> {
    // Simple embedding
    println!("Running embedded RSX code:");
    run_code(r#"
        Int x = 42
        println(x)
    "#)?;

    // Embedding with custom functions
    println!("\nRunning RSX with custom Rust function:");
    let mut interpreter = Interpreter::new();
    interpreter.register_function("double", |args| {
        if let Some(Value::Integer(n)) = args.first() {
            Ok(Value::Integer(n * 2))
        } else {
            Err(rustx::Error::RuntimeError("Expected integer".to_string()))
        }
    });

    let code = r#"
        Int x = 21
        Int y = double(x)
        println(y)
    "#;

    let mut parser = rustx::Parser::new(code.to_string());
    let program = parser.parse()?;
    interpreter.interpret(program)?;

    Ok(())
}