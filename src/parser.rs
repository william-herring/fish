use logos_nom_bridge::Tokens;
use nom::combinator::map;
use crate::lexer::Token;

type Input<'source> = Tokens<'source, Token>;

// Abstract Syntax Trees
#[derive(Debug, PartialEq, Eq)]
enum Expr {
    Literal(i64),
    Binary {
        op: Op,
        left: Box<Expr>,
        right: Box<Expr>,
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Op {
    Add,
    Sub,
}

pub fn parse_expression(input: Input<'_>) -> nom::IResult<Input<'_>, Expr> {

}