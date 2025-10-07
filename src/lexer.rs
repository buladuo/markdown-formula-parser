use logos::Logos;

#[derive(Logos, Debug, Clone, PartialEq)]
pub enum Token {
    // 数字
    #[regex(r"-?\d+\.?\d*", |lex| lex.slice().parse().ok())]
    Number(f64),
    
    // 标识符和函数名
    #[regex(r"[a-zA-Z][a-zA-Z0-9]*", |lex| lex.slice().to_string())]
    Identifier(String),
    
    // 运算符
    #[token("+")]
    Plus,
    #[token("-")]
    Minus,
    #[token("*")]
    Asterisk,
    #[token("/")]
    Slash,
    #[token("^")]
    Caret,
    #[token("!")]
    Exclamation,
    #[token("=")]
    Equals,
    
    // 括号
    #[token("(")]
    LParen,
    #[token(")")]
    RParen,
    #[token("[")]
    LBracket,
    #[token("]")]
    RBracket,
    #[token("{")]
    LCurly,
    #[token("}")]
    RCurly,
    
    // 特殊符号
    #[token("_")]
    Underscore,
    #[token(",")]
    Comma,
    #[token("\\")]
    Backslash,
    
    // 空白（跳过）
    #[regex(r"[ \t\n\r]+", logos::skip)]
    Whitespace,
}

pub struct Lexer<'a> {
    inner: logos::Lexer<'a, Token>,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            inner: Token::lexer(input),
        }
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next().and_then(|result| result.ok())
    }
}