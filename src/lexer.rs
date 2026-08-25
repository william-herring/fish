use logos::Logos;

#[derive(Logos, Debug, PartialEq)]
#[logos(skip r"[ \t\n\f]+")] // Ignore whitespace
pub enum Token {
    #[token("fun")]
    Fun,
    #[token("let")]
    Let,

    #[token("int")]
    IntKey,
    #[token("str")]
    StrKey,
    #[token("float")]
    FloatKey,
    #[token("bool")]
    BoolKey,

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
    #[token("%=")]
    ModAssign,

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

    // Feel like there needs to be some way to make it not just grab from the middle of something
    // Right now if someone tries to put "9a" instead of an error they get
    // number 9, identifier a
    // Which I guess could be alright actually
    #[regex("[a-zA-Z_][a-zA-Z0-9_]*")]
    Ident,
    #[regex("[0-9]+\\.?[0-9]*")]
    Number,
}