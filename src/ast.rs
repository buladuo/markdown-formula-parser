#[derive(Debug, Clone, PartialEq)]
pub enum MathExpr {
    // 基本元素
    Number(f64),
    Variable(String),
    
    // 二元运算
    BinaryOp {
        left: Box<MathExpr>,
        operator: BinaryOperator,
        right: Box<MathExpr>,
    },
    
    // 一元运算
    UnaryOp {
        operator: UnaryOperator,
        expr: Box<MathExpr>,
    },
    
    // 函数调用
    FunctionCall {
        name: String,
        args: Vec<MathExpr>,
    },
    
    // 上下标
    Subscript {
        base: Box<MathExpr>,
        subscript: Box<MathExpr>,
    },
    
    Superscript {
        base: Box<MathExpr>,
        superscript: Box<MathExpr>,
    },
    
    // 分数
    Fraction {
        numerator: Box<MathExpr>,
        denominator: Box<MathExpr>,
    },
    
    // 根号
    Root {
        radicand: Box<MathExpr>,
        index: Option<Box<MathExpr>>,
    },
    
    // 括号
    Parenthesized(Box<MathExpr>),
    
    // 矩阵
    Matrix {
        rows: Vec<Vec<MathExpr>>,
        matrix_type: String,  // 存储矩阵类型 (matrix, pmatrix, bmatrix, vmatrix, Vmatrix)
    },
    
    // 导数运算符
    Derivative {
        variable: String,
        expression: Box<MathExpr>,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub enum BinaryOperator {
    Add,
    Subtract,
    Multiply,
    DotProduct, // 点乘运算符
    Divide,
    Power,
    Equals,
}

#[derive(Debug, Clone, PartialEq)]
pub enum UnaryOperator {
    Plus,
    Minus,
    Factorial,
}

#[derive(Debug, Clone, PartialEq)]
pub struct MathBlock {
    pub expr: MathExpr,
    pub display_style: bool, // true for block ($$), false for inline ($)
}

impl MathExpr {
    pub fn to_string(&self) -> String {
        match self {
            MathExpr::Number(n) => format!("{}", n),
            MathExpr::Variable(v) => {
                // 特殊处理vec命令和导数运算符
                if v.starts_with("\\vec{") {
                    v.clone()
                } else if v.starts_with("\\frac{d}{") && v.ends_with("}") {
                    // 导数运算符
                    v.clone()
                } else {
                    v.clone()
                }
            },
            MathExpr::BinaryOp { left, operator, right } => {
                format!("({} {} {})", left.to_string(), operator.to_string(), right.to_string())
            }
            MathExpr::UnaryOp { operator, expr } => {
                match operator {
                    UnaryOperator::Plus => format!("+{}", expr.to_string()),
                    UnaryOperator::Minus => format!("-{}", expr.to_string()),
                    UnaryOperator::Factorial => format!("{}!", expr.to_string()),
                }
            }
            MathExpr::FunctionCall { name, args } => {
                // 特殊处理绝对值
                if name == "abs" && args.len() == 1 {
                    format!("|{}|", args[0].to_string())
                } else {
                    let args_str = args.iter()
                        .map(|arg| arg.to_string())
                        .collect::<Vec<_>>()
                        .join(", ");
                    format!("{}({})", name, args_str)
                }
            }
            MathExpr::Subscript { base, subscript } => {
                format!("{}[{}]", base.to_string(), subscript.to_string())
            }
            MathExpr::Superscript { base, superscript } => {
                format!("{}^{{{}}}", base.to_string(), superscript.to_string())
            }
            MathExpr::Fraction { numerator, denominator } => {
                format!("\\frac{{{}}}{{{}}}", numerator.to_string(), denominator.to_string())
            }
            MathExpr::Root { radicand, index } => {
                if let Some(idx) = index {
                    format!("\\sqrt[{}]{{{}}}", idx.to_string(), radicand.to_string())
                } else {
                    format!("\\sqrt{{{}}}", radicand.to_string())
                }
            }
            MathExpr::Parenthesized(expr) => {
                format!("({})", expr.to_string())
            }
            MathExpr::Matrix { rows, matrix_type } => {
                let rows_str = rows.iter()
                    .map(|row| {
                        row.iter()
                            .map(|cell| cell.to_string())
                            .collect::<Vec<_>>()
                            .join(" & ")
                    })
                    .collect::<Vec<_>>()
                    .join(" \\\\ ");
                format!("\\begin{{{}}} {} \\end{{{}}}", matrix_type, rows_str, matrix_type)
            }
            MathExpr::Derivative { variable, expression } => {
                format!("\\frac{{d}}{{d{}}}({})", variable, expression.to_string())
            }
        }
    }
}

impl BinaryOperator {
    pub fn to_string(&self) -> &str {
        match self {
            BinaryOperator::Add => "+",
            BinaryOperator::Subtract => "-",
            BinaryOperator::Multiply => "*",
            BinaryOperator::DotProduct => "\\cdot",
            BinaryOperator::Divide => "/",
            BinaryOperator::Power => "^",
            BinaryOperator::Equals => "=",
        }
    }
}

impl UnaryOperator {
    pub fn to_string(&self) -> &str {
        match self {
            UnaryOperator::Plus => "+",
            UnaryOperator::Minus => "-",
            UnaryOperator::Factorial => "!",
        }
    }
}

impl MathBlock {
    pub fn to_string(&self) -> String {
        if self.display_style {
            format!("$$ {} $$", self.expr.to_string())
        } else {
            format!("$ {} $", self.expr.to_string())
        }
    }
}