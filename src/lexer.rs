use logos::Logos;

#[derive(Logos, Debug, Clone, PartialEq)]
pub enum Token {
    // 数字（正数）
    #[regex(r"\d+\.?\d*", |lex| lex.slice().parse().ok())]
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
    #[token("\\cdot")]
    CDot,
    #[token("'")]
    Prime, // 导数符号
    
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
    #[token("|")]
    Pipe, // 绝对值符号
    
    // 矩阵相关符号
    #[token("&")]
    Ampersand,
    #[token(";")]
    Semicolon,
    #[token("\\\\")]
    DoubleBackslash,
    
    // 环境命令
    #[token("\\begin")]
    Begin,
    #[token("\\end")]
    End,
    
    // 矩阵类型
    #[token("matrix")]
    Matrix,
    #[token("pmatrix")]
    PMatrix,
    #[token("bmatrix")]
    BMatrix,
    #[token("vmatrix")]
    VMatrix,
    #[token("Vmatrix")]
    VMatrixDouble,
    
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

#[derive(Clone)]
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