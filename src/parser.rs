use crate::lexer::Token;

// ASTs
#[derive(Debug, PartialEq, Eq)]
enum Expr {
    Literal(i64),
    Unary {
        op: Unary,
        expr: Box<Expr>,
    },
    Binary {
        op: Op,
        left: Box<Expr>,
        right: Box<Expr>,
    },
    Grouping {
        expr: Box<Expr>,
    },
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

impl Op {
    fn precedence(self) -> u8 {
        match self {
            Op::Equals | Op::NotEquals => 1,

            Op::LessThan
            | Op::GreaterThan
            | Op::LessThanEquals
            | Op::GreaterThanEquals => 2,

            Op::Add | Op::Sub => 3,

            Op::Multiply
            | Op::Divide
            | Op::Modulo => 4,
        }
    }
}


pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}
impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, current: 0 }
    }

    pub fn parse_expression(&mut self) -> Result<Expr, String> {
        self.parse_precedence(0)
    }

    fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.current)
    }

    fn advance(&mut self) -> Option<Token> {
        let token = self.tokens.get(self.current)?.clone();
        self.current += 1;
        Some(token)
    }

    fn expect(&mut self, expected: Token) -> Result<(), String> {
        match self.advance() {
            Some(token) if token == expected => Ok(()),

            Some(token) => Err(String::from(format!("expected {:?}, got {:?}", expected, token))),

            None => Err(String::from("Unexpected end of input")),
        }
    }

    fn peek_binary_operator(&self) -> Option<Op> {
        match self.peek()? {
            Token::Plus => Some(Op::Add),
            Token::Minus => Some(Op::Sub),
            Token::Multiply => Some(Op::Multiply),
            Token::Divide => Some(Op::Divide),

            Token::EqualTo => Some(Op::Equals),
            Token::NotEqualTo => Some(Op::NotEquals),

            Token::LessThan => Some(Op::LessThan),
            Token::MoreThan => Some(Op::GreaterThan),
            Token::LessEqualTo => Some(Op::LessThanEquals),
            Token::MoreEqualTo => Some(Op::GreaterThanEquals),

            _ => None,
        }
    }

    fn parse_prefix(&mut self) -> Result<Expr, String> {
        let token = self.advance().ok_or(String::from("Unexpected EOF"))?;

        match token {
            Token::Int(value) => Ok(Expr::Literal(value)),

            Token::Minus => {
                let expr = self.parse_precedence(5)?;

                Ok(Expr::Unary {
                    op: Unary::Negate,
                    expr: Box::new(expr),
                })
            }

            Token::Not => {
                let expr = self.parse_precedence(5)?;

                Ok(Expr::Unary {
                    op: Unary::Not,
                    expr: Box::new(expr),
                })
            }

            Token::LeftParen => {
                let expr = self.parse_expression()?;

                self.expect(Token::RightParen)?;

                Ok(Expr::Grouping {
                    expr: Box::new(expr),
                })
            }

            _ => Err(String::from("Unexpected token")),
        }
    }

    fn parse_precedence(&mut self, min_precedence: u8) -> Result<Expr, String> {
        let mut left = self.parse_prefix()?;

        loop {
            let Some(op) = self.peek_binary_operator() else {
                break;
            };

            let precedence = op.precedence();

            if precedence < min_precedence {
                break;
            }

            self.advance();

            let right = self.parse_precedence(precedence + 1)?;

            left = Expr::Binary {
                op,
                left: Box::new(left),
                right: Box::new(right),
            };
        }

        Ok(left)
    }
}