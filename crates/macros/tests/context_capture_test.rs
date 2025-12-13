use rustx_macros::rx_with;

#[test]
fn test_rx_with_basic() {
    let x = 10;
    let y = 20;
    
    let result: i64 = rx_with! {
        vars: { x, y },
        code: "x + y"
    };
    
    assert_eq!(result, 30);
}

#[test]
fn test_rx_with_complex() {
    let a = 5;
    let b = 3;
    
    let result: i64 = rx_with! {
        vars: { a, b },
        code: "a * b + a"
    };
    
    assert_eq!(result, 20); // 5 * 3 + 5 = 20
}

#[test]
fn test_rx_with_string() {
    let name = "RustX".to_string();
    
    let result: String = rx_with! {
        vars: { name },
        code: "\"Hello, \" + name + \"!\""
    };
    
    assert_eq!(result, "Hello, RustX!");
}

#[test]
fn test_rx_with_multiple_types() {
    let count = 42;
    let message = "items".to_string();
    
    let result: String = rx_with! {
        vars: { count, message },
        code: "\"Found \" + type(count) + \" \" + message"
    };
    
    assert_eq!(result, "Found int items");
}

#[test]
fn test_rx_with_functions() {
    let x = 10;
    
    let result: i64 = rx_with! {
        vars: { x },
        code: "
            fn double(n) => n * 2
            double(x) + 5
        "
    };
    
    assert_eq!(result, 25); // double(10) + 5 = 25
}
