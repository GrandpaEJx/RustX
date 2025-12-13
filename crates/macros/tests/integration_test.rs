use rustx_macros::{rx, rsx};

#[test]
fn test_rx_macro_integer() {
    let result: i64 = rx! { "10 + 20" };
    assert_eq!(result, 30);
}

#[test]
fn test_rx_macro_string() {
    let result: String = rx! { "\"Hello, \" + \"World!\"" };
    assert_eq!(result, "Hello, World!");
}

#[test]
fn test_rx_macro_boolean() {
    let result: bool = rx! { "5 > 3" };
    assert!(result);
}

#[test]
fn test_rsx_macro_alias() {
    let result: i64 = rsx! { "5 * 6" };
    assert_eq!(result, 30);
}

#[test]
fn test_rx_macro_complex() {
    let result: i64 = rx! { "
        x = 10
        y = 20
        x + y
    " };
    assert_eq!(result, 30);
}

#[test]
fn test_rx_macro_function() {
    let result: i64 = rx! { "
        fn double(n) => n * 2
        double(21)
    " };
    assert_eq!(result, 42);
}
