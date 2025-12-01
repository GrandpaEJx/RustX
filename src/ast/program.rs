use super::node::Node;

#[derive(Debug, Clone)]
pub struct Program {
    pub statements: Vec<Node>,
}

impl Program {
    pub fn new() -> Self {
        Program {
            statements: Vec::new(),
        }
    }
    
    pub fn add_statement(&mut self, stmt: Node) {
        self.statements.push(stmt);
    }
}
