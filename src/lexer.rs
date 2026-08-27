use logos::Logos;

#[derive(Logos, Clone, Debug, PartialEq)]
#[logos(skip r"[ \t\n\f]+")] // Ignore whitespace
pub enum Token {
    #[token("fun")]
    Fun,
    // let, what do you even do?
    #[token("let")]
    Let,
    
    #[token("true")]
    True,
    #[token("false")]
    False,
    
    #[token("and")]
    And,
    #[token("not")]
    Not,
    #[token("in")]
    In,
    #[token("use")]
    Use,
    
    #[token("if")]
    If,
    #[token("else")]
    Else,
    #[token("for")]
    For,
    #[token("break")]
    Break,
    #[token("continue")]
    Continue,

    #[token("print")]
    Print,
    
    
    #[token("int")]
    IntKey,
    #[token("str")]
    StrKey,
    #[token("float")]
    FloatKey,
    #[token("bool")]
    BoolKey,
    #[token("void")]
    VoidKey,

    #[token("->")]
    RangeOp,

    #[token("=")]
    Assign,
    #[token("+=")]
    AddAssign,
    #[token("-=")]
    SubAssign,
    #[token("*=")]
    MulAssign,
    #[token("/=")]
    DivAssign,
    #[token("//=")]
    IntDivAssign,
    #[token("%=")]
    ModAssign,
    #[token("++")]
    Add1Assign,
    #[token("--")]
    Sub1Assign,

    #[token("==")]
    EqualTo,
    #[token("!=")]
    NotEqualTo,
    #[token("<=")]
    LessEqualTo,
    #[token(">=")]
    MoreEqualTo,
    #[token("<")]
    LessThan,
    #[token(">")]
    MoreThan,

    #[token("(")]
    LeftParen,
    #[token(")")]
    RightParen,
    #[token("{")]
    LeftCurly,
    #[token("}")]
    RightCurly,
    #[token("[")]
    LeftSquare,
    #[token("]")]
    RightSquare,    
    #[token(",")]
    Comma,
    
    #[token("+")]
    Plus,
    #[token("-")]
    Minus,
    #[token("*")]
    Multiply,
    #[token("//")]
    IntDivide,
    #[token("/")]
    Divide,
    #[token("!")]
    Factorial,

    #[regex("[a-zA-Z_][a-zA-Z0-9_]*")]
    Ident,
    #[regex("[0-9]+\\.?[0-9]*")]
    Number,
    #[regex("\".*\"", allow_greedy=true)]
    #[regex("'.*\'", allow_greedy=true)]
    String,
    
    #[token(".")]
    Dot,
}