use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, LitStr};

/// Evaluates RustX code and returns the result
/// 
/// # Example
/// ```ignore
/// use rustx_macros::rx;
/// 
/// let result: i64 = rx! { "10 + 20" };
/// assert_eq!(result, 30);
/// ```
#[proc_macro]
pub fn rx(input: TokenStream) -> TokenStream {
    let source = parse_macro_input!(input as LitStr);
    let source_str = source.value();

    // Generate code that executes the RustX script at runtime
    let expanded = quote! {
        {
            use rustx_core::{Interpreter, Lexer, Parser, Value};
            
            fn execute_rustx(source: &str) -> Value {
                let mut lexer = Lexer::new(source);
                let tokens = lexer.tokenize()
                    .expect("Failed to tokenize RustX code");
                let mut parser = Parser::new(tokens);
                let ast = parser.parse()
                    .expect("Failed to parse RustX code");
                let mut interpreter = Interpreter::new();
                interpreter.eval_program(ast)
                    .expect("Failed to execute RustX code")
            }
            
            fn convert_value<T: FromRustX>(value: Value) -> T {
                T::from_rustx(value)
            }
            
            trait FromRustX: Sized {
                fn from_rustx(value: Value) -> Self;
            }
            
            impl FromRustX for i64 {
                fn from_rustx(value: Value) -> Self {
                    match value {
                        Value::Int(n) => n,
                        Value::Float(f) => f as i64,
                        _ => panic!("Cannot convert {:?} to i64", value),
                    }
                }
            }
            
            impl FromRustX for f64 {
                fn from_rustx(value: Value) -> Self {
                    match value {
                        Value::Float(f) => f,
                        Value::Int(n) => n as f64,
                        _ => panic!("Cannot convert {:?} to f64", value),
                    }
                }
            }
            
            impl FromRustX for String {
                fn from_rustx(value: Value) -> Self {
                    match value {
                        Value::String(s) => s,
                        _ => panic!("Cannot convert {:?} to String", value),
                    }
                }
            }
            
            impl FromRustX for bool {
                fn from_rustx(value: Value) -> Self {
                    match value {
                        Value::Bool(b) => b,
                        _ => panic!("Cannot convert {:?} to bool", value),
                    }
                }
            }
            
            let value = execute_rustx(#source_str);
            convert_value(value)
        }
    };

    TokenStream::from(expanded)
}

/// Alias for rx! macro
/// 
/// # Example
/// ```ignore
/// use rustx_macros::rsx;
/// 
/// let result: i64 = rsx! { "5 * 6" };
/// assert_eq!(result, 30);
/// ```
#[proc_macro]
pub fn rsx(input: TokenStream) -> TokenStream {
    rx(input)
}
