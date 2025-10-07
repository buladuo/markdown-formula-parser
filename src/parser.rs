use crate::ast::{MathExpr, BinaryOperator, UnaryOperator};
use crate::lexer::{Token, Lexer};
use std::iter::Peekable;

pub struct Parser<'a> {
    tokens: Peekable<Lexer<'a>>,
}

impl<'a> Parser<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            tokens: Lexer::new(input).peekable(),
        }
    }

    pub fn parse_expression(&mut self) -> Result<MathExpr, String> {
        self.parse_equality()
    }

    // 等式: expression = expression
    fn parse_equality(&mut self) -> Result<MathExpr, String> {
        let mut expr = self.parse_additive()?;

        while self.consume(&Token::Equals) {
            let right = self.parse_additive()?;
            expr = MathExpr::BinaryOp {
                left: Box::new(expr),
                operator: BinaryOperator::Equals,
                right: Box::new(right),
            };
        }

        Ok(expr)
    }

    // 加减法: additive ( (+ | -) additive )*
    fn parse_additive(&mut self) -> Result<MathExpr, String> {
        let mut expr = self.parse_multiplicative()?;

        while let Some(token) = self.tokens.peek() {
            match token {
                Token::Plus => {
                    self.tokens.next();
                    let right = self.parse_multiplicative()?;
                    expr = MathExpr::BinaryOp {
                        left: Box::new(expr),
                        operator: BinaryOperator::Add,
                        right: Box::new(right),
                    };
                }
                Token::Minus => {
                    self.tokens.next();
                    let right = self.parse_multiplicative()?;
                    expr = MathExpr::BinaryOp {
                        left: Box::new(expr),
                        operator: BinaryOperator::Subtract,
                        right: Box::new(right),
                    };
                }
                _ => break,
            }
        }

        Ok(expr)
    }

    // 乘除法: multiplicative ( (* | /) multiplicative )*
    fn parse_multiplicative(&mut self) -> Result<MathExpr, String> {
        let mut expr = self.parse_power()?;

        while let Some(token) = self.tokens.peek() {
            match token {
                Token::Asterisk => {
                    self.tokens.next();
                    let right = self.parse_power()?;
                    expr = MathExpr::BinaryOp {
                        left: Box::new(expr),
                        operator: BinaryOperator::Multiply,
                        right: Box::new(right),
                    };
                }
                Token::Slash => {
                    self.tokens.next();
                    let right = self.parse_power()?;
                    expr = MathExpr::Fraction {
                        numerator: Box::new(expr),
                        denominator: Box::new(right),
                    };
                }
                _ => break,
            }
        }

        Ok(expr)
    }

    // 幂运算: power (^ power)*
    fn parse_power(&mut self) -> Result<MathExpr, String> {
        let mut expr = self.parse_unary()?;

        while self.consume(&Token::Caret) {
            let right = self.parse_unary()?;
            expr = MathExpr::BinaryOp {
                left: Box::new(expr),
                operator: BinaryOperator::Power,
                right: Box::new(right),
            };
        }

        Ok(expr)
    }

    // 一元运算: (+ | - | !) unary | primary
    fn parse_unary(&mut self) -> Result<MathExpr, String> {
        if let Some(token) = self.tokens.peek() {
            match token {
                Token::Plus => {
                    self.tokens.next();
                    let expr = self.parse_unary()?;
                    return Ok(MathExpr::UnaryOp {
                        operator: UnaryOperator::Plus,
                        expr: Box::new(expr),
                    });
                }
                Token::Minus => {
                    self.tokens.next();
                    let expr = self.parse_unary()?;
                    return Ok(MathExpr::UnaryOp {
                        operator: UnaryOperator::Minus,
                        expr: Box::new(expr),
                    });
                }
                Token::Exclamation => {
                    self.tokens.next();
                    let expr = self.parse_unary()?;
                    return Ok(MathExpr::UnaryOp {
                        operator: UnaryOperator::Factorial,
                        expr: Box::new(expr),
                    });
                }
                _ => {}
            }
        }

        self.parse_primary()
    }

    // 基本元素: number | identifier | function | subscript | superscript | parentheses
    fn parse_primary(&mut self) -> Result<MathExpr, String> {
        let token = self.tokens.next().ok_or("Unexpected end of input")?;

        match token {
            Token::Number(n) => Ok(MathExpr::Number(n)),
            Token::Identifier(name) => {
                // 检查是否是函数调用
                if self.consume(&Token::LParen) {
                    self.parse_function_call(name)
                } else {
                    let mut expr = MathExpr::Variable(name);
                    
                    // 处理下标
                    if self.consume(&Token::Underscore) {
                        expr = self.parse_subscript(expr)?;
                    }
                    
                    // 处理上标
                    if self.consume(&Token::Caret) {
                        expr = self.parse_superscript(expr)?;
                    }
                    
                    Ok(expr)
                }
            }
            Token::LParen => {
                let expr = self.parse_expression()?;
                self.expect(&Token::RParen)?;
                Ok(MathExpr::Parenthesized(Box::new(expr)))
            }
            Token::Backslash => {
                self.parse_command()
            }
            _ => Err(format!("Unexpected token: {:?}", token)),
        }
    }

    fn parse_function_call(&mut self, name: String) -> Result<MathExpr, String> {
        let mut args = Vec::new();

        if !self.check(&Token::RParen) {
            loop {
                let arg = self.parse_expression()?;
                args.push(arg);

                if !self.consume(&Token::Comma) {
                    break;
                }
            }
        }

        self.expect(&Token::RParen)?;
        Ok(MathExpr::FunctionCall { name, args })
    }

    fn parse_subscript(&mut self, base: MathExpr) -> Result<MathExpr, String> {
        // 支持两种形式：带花括号 x_{sub} 和不带花括号 x_sub
        let subscript = if self.check(&Token::LCurly) {
            self.tokens.next(); // 消耗 {
            let expr = self.parse_expression()?;
            self.expect(&Token::RCurly)?;
            expr
        } else {
            // 不带花括号的简写形式
            self.parse_unary()?
        };
        
        Ok(MathExpr::Subscript {
            base: Box::new(base),
            subscript: Box::new(subscript),
        })
    }

    fn parse_superscript(&mut self, base: MathExpr) -> Result<MathExpr, String> {
        // 支持两种形式：带花括号 x^{sup} 和不带花括号 x^sup
        let superscript = if self.check(&Token::LCurly) {
            self.tokens.next(); // 消耗 {
            let expr = self.parse_expression()?;
            self.expect(&Token::RCurly)?;
            expr
        } else {
            // 不带花括号的简写形式
            self.parse_unary()?
        };
        
        Ok(MathExpr::Superscript {
            base: Box::new(base),
            superscript: Box::new(superscript),
        })
    }

    fn parse_command(&mut self) -> Result<MathExpr, String> {
        let token = self.tokens.next().ok_or("Expected command name after backslash")?;
        
        if let Token::Identifier(cmd) = token {
            match cmd.as_str() {
                "frac" => {
                    self.expect(&Token::LCurly)?;
                    let numerator = self.parse_expression()?;
                    self.expect(&Token::RCurly)?;
                    self.expect(&Token::LCurly)?;
                    let denominator = self.parse_expression()?;
                    self.expect(&Token::RCurly)?;
                    
                    Ok(MathExpr::Fraction {
                        numerator: Box::new(numerator),
                        denominator: Box::new(denominator),
                    })
                }
                "sqrt" => {
                    if self.consume(&Token::LBracket) {
                        let index = self.parse_expression()?;
                        self.expect(&Token::RBracket)?;
                        self.expect(&Token::LCurly)?;
                        let radicand = self.parse_expression()?;
                        self.expect(&Token::RCurly)?;
                        
                        Ok(MathExpr::Root {
                            radicand: Box::new(radicand),
                            index: Some(Box::new(index)),
                        })
                    } else {
                        self.expect(&Token::LCurly)?;
                        let radicand = self.parse_expression()?;
                        self.expect(&Token::RCurly)?;
                        
                        Ok(MathExpr::Root {
                            radicand: Box::new(radicand),
                            index: None,
                        })
                    }
                }
                _ => Ok(MathExpr::Variable(format!("\\{}", cmd))),
            }
        } else {
            Err("Expected command name after backslash".to_string())
        }
    }

    // 工具函数
    fn consume(&mut self, expected: &Token) -> bool {
        if self.check(expected) {
            self.tokens.next();
            true
        } else {
            false
        }
    }

    fn check(&mut self, expected: &Token) -> bool {
        self.tokens.peek() == Some(expected)
    }

    fn expect(&mut self, expected: &Token) -> Result<(), String> {
        if self.consume(expected) {
            Ok(())
        } else {
            Err(format!("Expected {:?}, found {:?}", expected, self.tokens.peek()))
        }
    }
}