use logos::Logos;

#[derive(Logos, Debug, PartialEq)]
#[logos(skip r"[ \t\n\f]+")]
pub enum Token {
    #[token(";")]
    Delim,

    #[token("(")]
    LeftParen,
    #[token(")")]
    RightParen,

    #[token("if")]
    IF,
    #[token("else")]
    ELSE,

    #[token("{")]
    LeftBrace,
    #[token("}")]
    RightBrace,

    #[token("+")]
    PLUS,
    #[token("-")]
    MINUS,
    #[token("*")]
    STAR,
    #[token("/")]
    SLASH,

    #[regex("[a-zA-Z]+[a-zA-Z0-9]*", |sl| sl.slice().to_string())]
    Id(String),

    #[regex("\"[^\"]*\"", |sl| sl.slice().to_string())]
    String(String),

    #[regex("[0-9]+(.[0-9]+)?", |n| n.slice().parse::<f64>().ok())]
    Number(f64),

}
