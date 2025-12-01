#[derive(Debug, Clone)]
pub enum VarType {
    Str,
    Int,
    Bool,
    Float,
    Auto,
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
