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

    // 乘除法: multiplicative ( (* | / | \cdot) multiplicative )*
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
                    // 特殊处理导数运算符 \frac{d}{dx}
                    if let MathExpr::Variable(ref left) = expr {
                        if left == "d" {
                            if let MathExpr::Variable(ref right_var) = right {
                                if right_var.starts_with("d") && right_var.len() > 1 {
                                    let variable = &right_var[1..];
                                    return Ok(MathExpr::Variable(format!("\\frac{{d}}{{d{}}}", variable)));
                                }
                            }
                        }
                    }
                    expr = MathExpr::Fraction {
                        numerator: Box::new(expr),
                        denominator: Box::new(right),
                    };
                }
                Token::CDot => {
                    self.tokens.next();
                    let right = self.parse_power()?;
                    expr = MathExpr::BinaryOp {
                        left: Box::new(expr),
                        operator: BinaryOperator::DotProduct,
                        right: Box::new(right),
                    };
                }
                _ => break,
            }
        }

        Ok(expr)
    }

    // 幂运算: power (^ power)*
    fn parse_power(&mut self) -> Result<MathExpr, String> {
        let mut expr = self.parse_factor()?;

        while self.consume(&Token::Caret) {
            let right = self.parse_unary()?;  // 使用parse_unary而不是parse_power来避免无限递归
            expr = MathExpr::BinaryOp {
                left: Box::new(expr),
                operator: BinaryOperator::Power,
                right: Box::new(right),
            };
        }

        Ok(expr)
    }

    // 一元运算: (+ | - | !) unary | factor
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

        self.parse_factor()
    }

    // 因子: primary (primary)* 用于处理连续表达式，如隐式乘法
    fn parse_factor(&mut self) -> Result<MathExpr, String> {
        let mut expr = self.parse_primary()?;

        // 处理连续的表达式（隐式乘法）
        loop {
            let peeked = match self.tokens.peek() {
                Some(t) => t,
                None => break,
            };

            // 只有当接下来是明确可构成乘法的元素时才继续
            // 注意：Pipe符号可以触发隐式乘法，因为它可能是另一个绝对值的开始
            match peeked {
                Token::Number(_) | Token::Identifier(_) | Token::LParen | Token::Backslash | Token::LBracket | Token::Pipe => {}
                _ => break,
            }

            // 特别注意：如果当前expr已经是Superscript或Subscript，需要防止错误连接
            // 但我们仍然允许如 sin^2 x 这样的隐式乘法

            let right = self.parse_primary()?;
            expr = MathExpr::BinaryOp {
                left: Box::new(expr),
                operator: BinaryOperator::Multiply,
                right: Box::new(right),
            };
        }

        Ok(expr)
    }

    // 基本元素: number | identifier | function | subscript | superscript | parentheses | absolute value
    fn parse_primary(&mut self) -> Result<MathExpr, String> {
        let mut expr = {
            let token = self.tokens.next().ok_or("Unexpected end of input")?;

            match token {
                Token::Number(n) => MathExpr::Number(n),
                Token::Identifier(name) => {
                    // 检查是否是函数调用或者带导数符号的变量
                    if self.consume(&Token::LParen) {
                        self.parse_function_call(name)?
                    } else if self.consume(&Token::Prime) {
                        // 处理导数符号 f'
                        MathExpr::Variable(format!("{}'", name))
                    } else {
                        MathExpr::Variable(name)
                    }
                }
                Token::LParen => {
                    let expr = self.parse_expression()?;
                    self.expect(&Token::RParen)?;
                    MathExpr::Parenthesized(Box::new(expr))
                }
                Token::LBracket => {
                    let expr = self.parse_expression()?;
                    self.expect(&Token::RBracket)?;
                    MathExpr::Parenthesized(Box::new(expr))
                }
                Token::Pipe => {
                    // 处理绝对值 |...|
                    return self.parse_absolute_value();
                }
                Token::Begin => {
                    // 直接处理矩阵环境
                    return self.parse_matrix_environment();
                }
                Token::Backslash => {
                    self.parse_command()?
                }
                Token::Minus => {
                    // 处理负号
                    let expr = self.parse_unary()?;
                    return Ok(MathExpr::UnaryOp {
                        operator: UnaryOperator::Minus,
                        expr: Box::new(expr),
                    });
                }
                _ => return Err(format!("Unexpected token: {:?}", token)),
            }
        };

        // 处理连续的下标和上标
        loop {
            if self.consume(&Token::Underscore) {
                expr = self.parse_subscript(expr)?;
            } else if self.consume(&Token::Caret) {
                expr = self.parse_superscript(expr)?;
            } else {
                break;
            }
        }

        // 处理导数符号
        if self.consume(&Token::Prime) {
            expr = MathExpr::Variable(format!("{}'", expr.to_string()));
        }

        Ok(expr)
    }

    fn parse_function_call(&mut self, name: String) -> Result<MathExpr, String> {
        let args = self.parse_function_call_args()?;
        Ok(MathExpr::FunctionCall { name, args })
    }

    fn parse_function_call_args(&mut self) -> Result<Vec<MathExpr>, String> {
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
        Ok(args)
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
        
        if let Token::Begin = token {
            return self.parse_matrix_environment();
        }
        
        if let Token::Identifier(ref cmd) = token {
            if cmd == "frac" {
                self.expect(&Token::LCurly)?;
                let numerator = self.parse_expression()?;
                self.expect(&Token::RCurly)?;
                self.expect(&Token::LCurly)?;
                let denominator = self.parse_expression()?;
                self.expect(&Token::RCurly)?;
                
                // 特殊处理导数运算符 \frac{d}{dx}
                if let MathExpr::Variable(ref num) = numerator {
                    if num == "d" {
                        if let MathExpr::Variable(ref den) = denominator {
                            if den.starts_with("d") && den.len() > 1 {
                                // 这是一个导数运算符，检查后面是否有方括号表达式
                                let variable = den[1..].to_string();
                                // 检查后面是否跟着方括号表达式
                                if let Some(Token::LBracket) = self.tokens.peek() {
                                    self.tokens.next(); // 消耗 [
                                    let inner_expr = self.parse_expression()?;
                                    self.expect(&Token::RBracket)?; // 消耗 ]
                                    
                                    return Ok(MathExpr::Derivative {
                                        variable,
                                        expression: Box::new(inner_expr),
                                    });
                                } else {
                                    // 没有方括号，返回导数运算符
                                    return Ok(MathExpr::Variable(format!("\\frac{{d}}{{d{}}}", variable)));
                                }
                            }
                        }
                    }
                }
                
                return Ok(MathExpr::Fraction {
                    numerator: Box::new(numerator),
                    denominator: Box::new(denominator),
                });
            }
        }
        
        let mut expr = if let Token::Identifier(cmd) = token {
            match cmd.as_str() {
                "sqrt" => {
                    if self.consume(&Token::LBracket) {
                        let index = self.parse_expression()?;
                        self.expect(&Token::RBracket)?;
                        self.expect(&Token::LCurly)?;
                        let radicand = self.parse_expression()?;
                        self.expect(&Token::RCurly)?;
                        
                        MathExpr::Root {
                            radicand: Box::new(radicand),
                            index: Some(Box::new(index)),
                        }
                    } else {
                        self.expect(&Token::LCurly)?;
                        let radicand = self.parse_expression()?;
                        self.expect(&Token::RCurly)?;
                        
                        MathExpr::Root {
                            radicand: Box::new(radicand),
                            index: None,
                        }
                    }
                }
                "vec" => {
                    // 处理 \vec{...} 命令
                    self.expect(&Token::LCurly)?;
                    let expr = self.parse_expression()?;
                    self.expect(&Token::RCurly)?;
                    MathExpr::Variable(format!("\\vec{{{}}}", expr.to_string()))
                }
                "int" => {
                    // 处理积分符号
                    MathExpr::Variable("\\int".to_string())
                }
                "left" => {
                    // 处理 \left 命令
                    // 简单跳过，因为我们主要关注数学表达式的结构
                    if let Some(next_token) = self.tokens.next() {
                        match next_token {
                            Token::LBracket => MathExpr::Variable("[".to_string()),
                            Token::LParen => MathExpr::Variable("(".to_string()),
                            _ => MathExpr::Variable(format!("\\left{}", 
                                match next_token {
                                    Token::Identifier(s) => s,
                                    _ => format!("{:?}", next_token)
                                }
                            ))
                        }
                    } else {
                        MathExpr::Variable("\\left".to_string())
                    }
                }
                "right" => {
                    // 处理 \right 命令
                    // 简单跳过，因为我们主要关注数学表达式的结构
                    if let Some(next_token) = self.tokens.next() {
                        match next_token {
                            Token::RBracket => MathExpr::Variable("]".to_string()),
                            Token::RParen => MathExpr::Variable(")".to_string()),
                            _ => MathExpr::Variable(format!("\\right{}", 
                                match next_token {
                                    Token::Identifier(s) => s,
                                    _ => format!("{:?}", next_token)
                                }
                            ))
                        }
                    } else {
                        MathExpr::Variable("\\right".to_string())
                    }
                }
                _ => {
                    // 检查是否是函数调用
                    if self.consume(&Token::LParen) {
                        let args = self.parse_function_call_args()?;
                        return Ok(MathExpr::FunctionCall {
                            name: format!("\\{}", cmd),
                            args,
                        });
                    } else {
                        MathExpr::Variable(format!("\\{}", cmd))
                    }
                }
            }
        } else {
            return Err("Expected command name after backslash".to_string());
        };

        // 处理命令后的下标和上标
        loop {
            if self.consume(&Token::Underscore) {
                expr = self.parse_subscript(expr)?;
            } else if self.consume(&Token::Caret) {
                expr = self.parse_superscript(expr)?;
            } else {
                break;
            }
        }

        Ok(expr)
    }

    // 解析矩阵环境
    fn parse_matrix_environment(&mut self) -> Result<MathExpr, String> {
        self.expect(&Token::LCurly)?;
        
        // 获取矩阵类型
        let matrix_type_token = self.tokens.next().ok_or("Expected matrix type")?;
        let matrix_type = match matrix_type_token {
            Token::Matrix => "matrix",
            Token::PMatrix => "pmatrix",
            Token::BMatrix => "bmatrix",
            Token::VMatrix => "vmatrix",
            Token::VMatrixDouble => "Vmatrix",
            _ => return Err("Expected matrix type".to_string()),
        };
        
        self.expect(&Token::RCurly)?;
        
        // 解析矩阵内容
        let mut rows: Vec<Vec<MathExpr>> = Vec::new();
        let mut current_row: Vec<MathExpr> = Vec::new();
        
        loop {
            // 检查是否是结束标记
            if let Some(Token::End) = self.tokens.peek() {
                // 消耗结束标记
                self.tokens.next(); // end
                self.expect(&Token::LCurly)?;
                
                // 检查结束标记类型是否匹配
                let end_type_token = self.tokens.next().ok_or("Expected matrix type in end command")?;
                let end_type = match end_type_token {
                    Token::Matrix => "matrix",
                    Token::PMatrix => "pmatrix",
                    Token::BMatrix => "bmatrix",
                    Token::VMatrix => "vmatrix",
                    Token::VMatrixDouble => "Vmatrix",
                    _ => return Err("Expected matrix type in end command".to_string()),
                };
                
                if end_type != matrix_type {
                    return Err(format!("Mismatched matrix environment: expected {}, found {}", matrix_type, end_type));
                }
                
                self.expect(&Token::RCurly)?;
                
                // 将最后一行添加到矩阵中（如果有）
                if !current_row.is_empty() {
                    rows.push(current_row);
                }
                
                return Ok(MathExpr::Matrix { 
                    rows,
                    matrix_type: matrix_type.to_string()
                });
            }
            
            // 解析矩阵元素
            let expr = self.parse_expression()?;
            current_row.push(expr);
            
            // 检查下一个符号
            match self.tokens.peek() {
                Some(Token::Ampersand) => {
                    // 遇到 & 符号，表示当前单元格结束，继续同一行的下一个元素
                    self.tokens.next(); // 消耗 &
                }
                Some(Token::DoubleBackslash) => {
                    // 遇到 \\ 符号，表示当前行结束
                    self.tokens.next(); // 消耗 \\
                    rows.push(current_row);
                    current_row = Vec::new();
                }
                Some(Token::End) => {
                    // 可能是结束标记，继续循环让上面的检查处理
                    continue;
                }
                _ => {
                    // 其他情况，继续解析
                    continue;
                }
            }
        }
    }

    // 解析绝对值表达式
    fn parse_absolute_value(&mut self) -> Result<MathExpr, String> {
        // 检查是否是范数（双竖线）
        if let Some(Token::Pipe) = self.tokens.peek() {
            // 连续两个Pipe，这是范数
            self.tokens.next(); // 消耗第二个Pipe
            let inner_expr = self.parse_expression_until_pipe()?;
            self.expect(&Token::Pipe)?; // 消费第三个Pipe
            self.expect(&Token::Pipe)?; // 消费第四个Pipe
            // 范数用norm函数表示
            return Ok(MathExpr::FunctionCall {
                name: "norm".to_string(),
                args: vec![inner_expr],
            });
        }
        
        // 单个绝对值
        let inner_expr = self.parse_expression_until_pipe()?;
        self.expect(&Token::Pipe)?; // 消费结束的Pipe
        
        Ok(MathExpr::FunctionCall {
            name: "abs".to_string(),
            args: vec![inner_expr],
        })
    }
    
    // 解析表达式直到遇到Pipe符号（不包括Pipe）
    fn parse_expression_until_pipe(&mut self) -> Result<MathExpr, String> {
        let mut expr = self.parse_equality_until_pipe()?;

        while self.consume(&Token::Equals) {
            // 检查下一个符号是否是Pipe
            if self.check(&Token::Pipe) {
                break;
            }
            let right = self.parse_equality_until_pipe()?;
            expr = MathExpr::BinaryOp {
                left: Box::new(expr),
                operator: BinaryOperator::Equals,
                right: Box::new(right),
            };
        }

        Ok(expr)
    }
    
    // 等式: expression = expression，但遇到Pipe时停止
    fn parse_equality_until_pipe(&mut self) -> Result<MathExpr, String> {
        let mut expr = self.parse_additive_until_pipe()?;

        while let Some(token) = self.tokens.peek() {
            // 遇到Pipe时停止
            if let Token::Pipe = token {
                break;
            }
            
            match token {
                Token::Equals => {
                    self.tokens.next();
                    // 检查下一个符号是否是Pipe
                    if self.check(&Token::Pipe) {
                        break;
                    }
                    let right = self.parse_additive_until_pipe()?;
                    expr = MathExpr::BinaryOp {
                        left: Box::new(expr),
                        operator: BinaryOperator::Equals,
                        right: Box::new(right),
                    };
                }
                _ => break,
            }
        }

        Ok(expr)
    }
    
    // 加减法: additive ( (+ | -) additive )*，但遇到Pipe时停止
    fn parse_additive_until_pipe(&mut self) -> Result<MathExpr, String> {
        let mut expr = self.parse_multiplicative_until_pipe()?;

        while let Some(token) = self.tokens.peek() {
            // 遇到Pipe时停止
            if let Token::Pipe = token {
                break;
            }
            
            match token {
                Token::Plus => {
                    self.tokens.next();
                    let right = self.parse_multiplicative_until_pipe()?;
                    expr = MathExpr::BinaryOp {
                        left: Box::new(expr),
                        operator: BinaryOperator::Add,
                        right: Box::new(right),
                    };
                }
                Token::Minus => {
                    self.tokens.next();
                    let right = self.parse_multiplicative_until_pipe()?;
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
    
    // 乘除法: multiplicative ( (* | / | \cdot) multiplicative )*，但遇到Pipe时停止
    fn parse_multiplicative_until_pipe(&mut self) -> Result<MathExpr, String> {
        let mut expr = self.parse_power_until_pipe()?;

        while let Some(token) = self.tokens.peek() {
            // 遇到Pipe时停止
            if let Token::Pipe = token {
                break;
            }
            
            match token {
                Token::Asterisk => {
                    self.tokens.next();
                    let right = self.parse_power_until_pipe()?;
                    expr = MathExpr::BinaryOp {
                        left: Box::new(expr),
                        operator: BinaryOperator::Multiply,
                        right: Box::new(right),
                    };
                }
                Token::Slash => {
                    self.tokens.next();
                    let right = self.parse_power_until_pipe()?;
                    // 特殊处理导数运算符 \frac{d}{dx}
                    if let MathExpr::Variable(ref left) = expr {
                        if left == "d" {
                            if let MathExpr::Variable(ref right_var) = right {
                                if right_var.starts_with("d") && right_var.len() > 1 {
                                    let variable = &right_var[1..];
                                    return Ok(MathExpr::Variable(format!("\\frac{{d}}{{d{}}}", variable)));
                                }
                            }
                        }
                    }
                    expr = MathExpr::Fraction {
                        numerator: Box::new(expr),
                        denominator: Box::new(right),
                    };
                }
                Token::CDot => {
                    self.tokens.next();
                    let right = self.parse_power_until_pipe()?;
                    expr = MathExpr::BinaryOp {
                        left: Box::new(expr),
                        operator: BinaryOperator::DotProduct,
                        right: Box::new(right),
                    };
                }
                _ => break,
            }
        }

        Ok(expr)
    }
    
    // 幂运算: power (^ power)*，但遇到Pipe时停止
    fn parse_power_until_pipe(&mut self) -> Result<MathExpr, String> {
        let mut expr = self.parse_factor_until_pipe()?;

        while self.consume(&Token::Caret) {
            // 检查下一个符号是否是Pipe
            if self.check(&Token::Pipe) {
                break;
            }
            
            let right = self.parse_unary()?;  // 使用parse_unary而不是parse_power来避免无限递归
            expr = MathExpr::BinaryOp {
                left: Box::new(expr),
                operator: BinaryOperator::Power,
                right: Box::new(right),
            };
        }

        Ok(expr)
    }
    
    // 因子: primary (primary)* 用于处理连续表达式，如隐式乘法，但遇到Pipe时停止
    fn parse_factor_until_pipe(&mut self) -> Result<MathExpr, String> {
        let mut expr = self.parse_primary_until_pipe()?;

        // 处理连续的表达式（隐式乘法）
        loop {
            let peeked = match self.tokens.peek() {
                Some(t) => t,
                None => break,
            };

            // 遇到Pipe时停止
            if let Token::Pipe = peeked {
                break;
            }

            // 只有当接下来是明确可构成乘法的元素时才继续
            // 注意：Pipe符号可以触发隐式乘法，因为它可能是另一个绝对值的开始
            match peeked {
                Token::Number(_) | Token::Identifier(_) | Token::LParen | Token::Backslash | Token::LBracket | Token::Pipe => {}
                _ => break,
            }

            // 特别注意：如果当前expr已经是Superscript或Subscript，需要防止错误连接
            // 但我们仍然允许如 sin^2 x 这样的隐式乘法

            let right = self.parse_primary_until_pipe()?;
            expr = MathExpr::BinaryOp {
                left: Box::new(expr),
                operator: BinaryOperator::Multiply,
                right: Box::new(right),
            };
        }

        Ok(expr)
    }
    
    // 基本元素: number | identifier | function | subscript | superscript | parentheses | absolute value
    // 但遇到Pipe时停止
    fn parse_primary_until_pipe(&mut self) -> Result<MathExpr, String> {
        let mut expr = {
            let token = self.tokens.next().ok_or("Unexpected end of input")?;
            
            // 遇到Pipe时停止
            if let Token::Pipe = token {
                return Err("Unexpected Pipe".to_string());
            }

            match token {
                Token::Number(n) => MathExpr::Number(n),
                Token::Identifier(name) => {
                    // 检查是否是函数调用或者带导数符号的变量
                    if self.consume(&Token::LParen) {
                        self.parse_function_call_args_until_pipe().map(|args| MathExpr::FunctionCall { name, args })?
                    } else if self.consume(&Token::Prime) {
                        // 处理导数符号 f'
                        MathExpr::Variable(format!("{}'", name))
                    } else {
                        MathExpr::Variable(name)
                    }
                }
                Token::LParen => {
                    let expr = self.parse_expression_until_pipe()?;
                    self.expect(&Token::RParen)?;
                    MathExpr::Parenthesized(Box::new(expr))
                }
                Token::LBracket => {
                    let expr = self.parse_expression_until_pipe()?;
                    self.expect(&Token::RBracket)?;
                    MathExpr::Parenthesized(Box::new(expr))
                }
                Token::Pipe => {
                    // 处理嵌套绝对值 |...|
                    let inner_expr = self.parse_expression_until_pipe()?;
                    self.expect(&Token::Pipe)?;  // 消耗结束的Pipe符号
                    return Ok(MathExpr::FunctionCall {
                        name: "abs".to_string(),
                        args: vec![inner_expr],
                    });
                }
                Token::Begin => {
                    // 直接处理矩阵环境
                    return self.parse_matrix_environment();
                }
                Token::Backslash => {
                    self.parse_command_until_pipe()?
                }
                Token::Minus => {
                    // 处理负号
                    let expr = self.parse_unary()?;
                    return Ok(MathExpr::UnaryOp {
                        operator: UnaryOperator::Minus,
                        expr: Box::new(expr),
                    });
                }
                _ => return Err(format!("Unexpected token: {:?}", token)),
            }
        };

        // 处理连续的下标和上标
        loop {
            // 检查是否遇到Pipe
            if self.check(&Token::Pipe) {
                break;
            }
            
            if self.consume(&Token::Underscore) {
                // 处理下标
                let subscript = if self.check(&Token::LCurly) {
                    self.tokens.next(); // 消耗 {
                    let expr = self.parse_expression_until_pipe()?;
                    self.expect(&Token::RCurly)?;
                    expr
                } else {
                    // 不带花括号的简写形式，但仍需正确处理Pipe边界
                    self.parse_expression_until_pipe()?
                };
                
                expr = MathExpr::Subscript {
                    base: Box::new(expr),
                    subscript: Box::new(subscript),
                };
            } else if self.consume(&Token::Caret) {
                // 处理上标
                let superscript = if self.check(&Token::LCurly) {
                    self.tokens.next(); // 消耗 {
                    let expr = self.parse_expression_until_pipe()?;
                    self.expect(&Token::RCurly)?;
                    expr
                } else {
                    // 不带花括号的简写形式，但仍需正确处理Pipe边界
                    self.parse_expression_until_pipe()?
                };
                
                expr = MathExpr::Superscript {
                    base: Box::new(expr),
                    superscript: Box::new(superscript),
                };
            } else {
                break;
            }
        }

        // 处理导数符号
        if self.consume(&Token::Prime) {
            expr = MathExpr::Variable(format!("{}'", expr.to_string()));
        }

        Ok(expr)
    }
    
    fn parse_command_until_pipe(&mut self) -> Result<MathExpr, String> {
        let token = self.tokens.next().ok_or("Expected command name after backslash")?;
        
        if let Token::Begin = token {
            return self.parse_matrix_environment();
        }
        
        if let Token::Identifier(ref cmd) = token {
            if cmd == "frac" {
                self.expect(&Token::LCurly)?;
                let numerator = self.parse_expression_until_pipe()?;
                self.expect(&Token::RCurly)?;
                self.expect(&Token::LCurly)?;
                let denominator = self.parse_expression_until_pipe()?;
                self.expect(&Token::RCurly)?;
                
                // 特殊处理导数运算符 \frac{d}{dx}
                if let MathExpr::Variable(ref num) = numerator {
                    if num == "d" {
                        if let MathExpr::Variable(ref den) = denominator {
                            if den.starts_with("d") && den.len() > 1 {
                                // 这是一个导数运算符，检查后面是否有方括号表达式
                                let variable = den[1..].to_string();
                                // 检查后面是否跟着方括号表达式
                                if let Some(Token::LBracket) = self.tokens.peek() {
                                    self.tokens.next(); // 消耗 [
                                    let inner_expr = self.parse_expression_until_pipe()?;
                                    self.expect(&Token::RBracket)?; // 消耗 ]
                                    
                                    return Ok(MathExpr::Derivative {
                                        variable,
                                        expression: Box::new(inner_expr),
                                    });
                                } else {
                                    // 没有方括号，返回导数运算符
                                    return Ok(MathExpr::Variable(format!("\\frac{{d}}{{d{}}}", variable)));
                                }
                            }
                        }
                    }
                }
                
                return Ok(MathExpr::Fraction {
                    numerator: Box::new(numerator),
                    denominator: Box::new(denominator),
                });
            }
        }
        
        let mut expr = if let Token::Identifier(cmd) = token {
            match cmd.as_str() {
                "sqrt" => {
                    if self.consume(&Token::LBracket) {
                        let index = self.parse_expression_until_pipe()?;
                        self.expect(&Token::RBracket)?;
                        self.expect(&Token::LCurly)?;
                        let radicand = self.parse_expression_until_pipe()?;
                        self.expect(&Token::RCurly)?;
                        
                        MathExpr::Root {
                            radicand: Box::new(radicand),
                            index: Some(Box::new(index)),
                        }
                    } else {
                        self.expect(&Token::LCurly)?;
                        let radicand = self.parse_expression_until_pipe()?;
                        self.expect(&Token::RCurly)?;
                        
                        MathExpr::Root {
                            radicand: Box::new(radicand),
                            index: None,
                        }
                    }
                }
                "vec" => {
                    // 处理 \vec{...} 命令
                    self.expect(&Token::LCurly)?;
                    let expr = self.parse_expression_until_pipe()?;
                    self.expect(&Token::RCurly)?;
                    MathExpr::Variable(format!("\\vec{{{}}}", expr.to_string()))
                }
                "int" => {
                    // 处理积分符号
                    MathExpr::Variable("\\int".to_string())
                }
                "left" => {
                    // 处理 \left 命令
                    // 简单跳过，因为我们主要关注数学表达式的结构
                    if let Some(next_token) = self.tokens.next() {
                        match next_token {
                            Token::LBracket => MathExpr::Variable("[".to_string()),
                            Token::LParen => MathExpr::Variable("(".to_string()),
                            _ => MathExpr::Variable(format!("\\left{}", 
                                match next_token {
                                    Token::Identifier(s) => s,
                                    _ => format!("{:?}", next_token)
                                }
                            ))
                        }
                    } else {
                        MathExpr::Variable("\\left".to_string())
                    }
                }
                "right" => {
                    // 处理 \right 命令
                    // 简单跳过，因为我们主要关注数学表达式的结构
                    if let Some(next_token) = self.tokens.next() {
                        match next_token {
                            Token::RBracket => MathExpr::Variable("]".to_string()),
                            Token::RParen => MathExpr::Variable(")".to_string()),
                            _ => MathExpr::Variable(format!("\\right{}", 
                                match next_token {
                                    Token::Identifier(s) => s,
                                    _ => format!("{:?}", next_token)
                                }
                            ))
                        }
                    } else {
                        MathExpr::Variable("\\right".to_string())
                    }
                }
                _ => {
                    // 检查是否是函数调用
                    if self.consume(&Token::LParen) {
                        let args = self.parse_function_call_args_until_pipe()?;
                        return Ok(MathExpr::FunctionCall {
                            name: format!("\\{}", cmd),
                            args,
                        });
                    } else {
                        MathExpr::Variable(format!("\\{}", cmd))
                    }
                }
            }
        } else {
            return Err("Expected command name after backslash".to_string());
        };

        // 处理命令后的下标和上标
        loop {
            // 检查是否遇到Pipe
            if self.check(&Token::Pipe) {
                break;
            }
            
            if self.consume(&Token::Underscore) {
                expr = self.parse_subscript(expr)?;
            } else if self.consume(&Token::Caret) {
                expr = self.parse_superscript(expr)?;
            } else {
                break;
            }
        }

        Ok(expr)
    }
    
    fn parse_function_call_args_until_pipe(&mut self) -> Result<Vec<MathExpr>, String> {
        let mut args = Vec::new();

        if !self.check(&Token::RParen) {
            loop {
                let arg = self.parse_expression_until_pipe()?;
                args.push(arg);

                if !self.consume(&Token::Comma) {
                    break;
                }
                
                // 检查是否遇到Pipe
                if self.check(&Token::Pipe) {
                    break;
                }
            }
        }

        self.expect(&Token::RParen)?;
        Ok(args)
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