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

/// Evaluates RustX code with access to Rust variables
///
/// # Example
/// ```ignore
/// use rustx_macros::rx_with;
///
/// let x = 10;
/// let y = 20;
/// let result: i64 = rx_with! {
///     vars: { x, y },
///     code: "x + y * 2"
/// };
/// assert_eq!(result, 50);
/// ```
#[proc_macro]
pub fn rx_with(input: TokenStream) -> TokenStream {
    use syn::punctuated::Punctuated;
    use syn::{
        parse::{Parse, ParseStream},
        Ident, Token,
    };

    struct RxWithInput {
        vars: Vec<Ident>,
        code: LitStr,
    }

    impl Parse for RxWithInput {
        fn parse(input: ParseStream) -> syn::Result<Self> {
            // Parse "vars:"
            input.parse::<Ident>()?; // "vars"
            input.parse::<Token![:]>()?;

            // Parse variable list in braces
            let content;
            syn::braced!(content in input);
            let vars: Punctuated<Ident, Token![,]> =
                content.parse_terminated(Ident::parse, Token![,])?;
            let vars: Vec<Ident> = vars.into_iter().collect();

            input.parse::<Token![,]>()?;

            // Parse "code:"
            input.parse::<Ident>()?; // "code"
            input.parse::<Token![:]>()?;

            // Parse code string
            let code: LitStr = input.parse()?;

            Ok(RxWithInput { vars, code })
        }
    }

    let rx_input = parse_macro_input!(input as RxWithInput);
    let source_str = rx_input.code.value();
    let var_idents = &rx_input.vars;

    // Generate variable conversion code (executed in outer scope)
    let var_conversions = var_idents.iter().map(|var| {
        let temp_name = syn::Ident::new(&format!("__rustx_var_{}", var), var.span());
        quote! {
            let #temp_name: Value = ToRustXValue::to_rustx_value(&#var);
        }
    });

    // Generate variable injection code (uses converted values)
    let var_injections = var_idents.iter().map(|var| {
        let temp_name = syn::Ident::new(&format!("__rustx_var_{}", var), var.span());
        quote! {
            interpreter.env.set(
                stringify!(#var).to_string(),
                #temp_name.clone()
            );
        }
    });

    let expanded = quote! {
        {
            use rustx_core::{Interpreter, Lexer, Parser, Value};

            trait ToRustXValue {
                fn to_rustx_value(&self) -> Value;
            }

            impl ToRustXValue for i64 { fn to_rustx_value(&self) -> Value { Value::Int(*self) } }
            impl ToRustXValue for i32 { fn to_rustx_value(&self) -> Value { Value::Int(*self as i64) } }
            impl ToRustXValue for f64 { fn to_rustx_value(&self) -> Value { Value::Float(*self) } }
            impl ToRustXValue for bool { fn to_rustx_value(&self) -> Value { Value::Bool(*self) } }
            impl ToRustXValue for String { fn to_rustx_value(&self) -> Value { Value::String(self.clone()) } }
            impl ToRustXValue for &str { fn to_rustx_value(&self) -> Value { Value::String(self.to_string()) } }

            // Convert variables to RustX values in outer scope
            #(#var_conversions)*

            let mut lexer = Lexer::new(#source_str);
            let tokens = lexer.tokenize()
                .expect("Failed to tokenize RustX code");
            let mut parser = Parser::new(tokens);
            let ast = parser.parse()
                .expect("Failed to parse RustX code");
            let mut interpreter = Interpreter::new();

            // Inject variables
            #(#var_injections)*

            let result_value = interpreter.eval_program(ast)
                .expect("Failed to execute RustX code");

            // Convert result based on expected type
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

            // Convert result based on expected type
            FromRustX::from_rustx(result_value)
        }
    };

    TokenStream::from(expanded)
}
