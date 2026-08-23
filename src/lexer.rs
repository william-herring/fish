use logos::Logos;

#[derive(Logos, Debug, PartialEq)]
#[logos(skip r"[ \t\n\f]+")] // Ignore whitespace
enum Token {
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

    #[regex("[a-zA-Z0-9]+")]
    Ident,
}