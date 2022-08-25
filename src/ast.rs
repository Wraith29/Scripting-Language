#[derive(Debug)]
pub enum ComparisonOperator {
    Equal,
    NotEqual,
    GreaterEqual,
    LesserEqual
}


#[derive(Debug)]
pub enum BinaryOperator {
    Plus,
    Minus
}

#[derive(Debug)]
pub enum Node {
    Declaration {
        target: String,
        value: Box<Node>
    },
    Assignment {
        target: Box<Node>,
        value: Box<Node>
    },
    Variable {
        name: String
    },
    IntLiteral {
        value: i64
    },
    While {
        condition: Box<Node>,
        body: Box<Vec<Node>>
    },
    ComparisonOp {
        lhs: Box<Node>,
        rhs: Box<Node>,
        op: ComparisonOperator
    },
    BinaryOp {
        lhs: Box<Node>,
        rhs: Box<Node>,
        op: BinaryOperator
    }
}

#[derive(Debug)]
pub struct Ast {
    pub nodes: Vec<Node>
}

impl Ast {
    pub fn new() -> Ast {
        Ast { nodes: vec![] }
    }
}