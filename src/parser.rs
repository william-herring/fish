use logos::{Lexer, Logos};
use crate::lexer::Token;

struct Parser<'source> {
    lexer: Lexer<'source, Token>,
    current: Option<(Token, std::ops::Range<usize>)>,
}