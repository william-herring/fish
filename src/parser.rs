use crate::lexer::Token;

// ASTs
#[derive(Debug, PartialEq, Eq)]
enum Expr {
    Literal(i64),
    Unary(Unary),
    Binary {
        op: Op,
        left: Box<Expr>,
        right: Box<Expr>,
    },
    Grouping {
        expr: Box<Expr>,
    },
    Operator(Op)
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Unary {
    Negate,
    Not,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Op {
    Add,
    Sub,
    Multiply,
    Divide,
    Equals,
    NotEquals,
    LessThan,
    GreaterThan,
    LessThanEquals,
    GreaterThanEquals,
    Modulo,
}


pub fn parse_expression(input: Token) ->  {

}