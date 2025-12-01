#[derive(Debug, Clone)]
pub enum VarType {
    Str,
    Int,
    Bool,
    Float,
    Auto,
    Ref(Box<VarType>), // For reference types like &str
    Void,               // For functions with no return value
}

#[derive(Debug, Clone)]
pub enum BinaryOperator {
    Add,
    Subtract,
    Multiply,
    Divide,
    Equals,
    NotEquals,
}
