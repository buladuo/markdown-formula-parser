# Markdown Formula Parser

一个用 Rust 编写的数学公式解析器，支持从 Markdown 文本中提取和解析 LaTeX 风格的数学表达式。

## 项目架构

### 模块结构

```
src/
├── ast.rs       # 抽象语法树定义
├── lexer.rs     # 词法分析器
├── parser.rs    # 递归下降解析器
├── lib.rs       # 库接口和主要功能
└── main.rs      # 示例和演示程序
```

### 核心组件

#### 1. 词法分析器 (Lexer)
- 使用 [Logos](https://crates.io/crates/logos) 库实现
- 将输入文本分解为标记 (Token)
- 支持数字、标识符、运算符、括号、特殊符号等

#### 2. 解析器 (Parser)
- 实现递归下降解析算法
- 按照运算符优先级解析表达式
- 支持复杂的数学表达式结构

#### 3. 抽象语法树 (AST)
- 定义数学表达式的内部表示
- 支持多种数学结构和运算

## 算法说明

### 递归下降解析器

本项目使用递归下降解析算法，这是一种自顶向下的解析方法：

1. **分层解析**：按照运算符优先级组织解析函数
2. **递归调用**：高优先级函数调用低优先级函数
3. **前瞻预测**：基于下一个标记决定解析路径

解析函数层次结构：
```
parse_expression (顶层)
└── parse_equality (=)
    └── parse_additive (+, -)
        └── parse_multiplicative (*, /, \cdot)
            └── parse_power (^)
                └── parse_factor (隐式乘法)
                    └── parse_unary (+, -, !)
                        └── parse_primary (基本元素)
```

### 运算符优先级

解析器严格按照数学运算符优先级进行解析：
1. 括号和基本元素 (最高优先级)
2. 幂运算 (^)
3. 一元运算 (+, -, !)
4. 乘除法 (*, /, \cdot)
5. 加减法 (+, -)
6. 等式 (=) (最低优先级)

## AST 结构

抽象语法树 (AST) 是解析结果的内部表示形式，定义在 `MathExpr` 枚举中：

```rust
#[derive(Debug, Clone, PartialEq)]
pub enum MathExpr {
    // 基本元素
    Number(f64),                           // 数字
    Variable(String),                      // 变量或标识符
    
    // 二元运算
    BinaryOp {
        left: Box<MathExpr>,               // 左操作数
        operator: BinaryOperator,          // 运算符
        right: Box<MathExpr>,              // 右操作数
    },
    
    // 一元运算
    UnaryOp {
        operator: UnaryOperator,           // 运算符
        expr: Box<MathExpr>,               // 操作数
    },
    
    // 函数调用
    FunctionCall {
        name: String,                      // 函数名
        args: Vec<MathExpr>,               // 参数列表
    },
    
    // 上下标
    Subscript {
        base: Box<MathExpr>,               // 基础表达式
        subscript: Box<MathExpr>,          // 下标
    },
    
    Superscript {
        base: Box<MathExpr>,               // 基础表达式
        superscript: Box<MathExpr>,        // 上标
    },
    
    // 分数
    Fraction {
        numerator: Box<MathExpr>,          // 分子
        denominator: Box<MathExpr>,        // 分母
    },
    
    // 根号
    Root {
        radicand: Box<MathExpr>,           // 被开方数
        index: Option<Box<MathExpr>>,      // 根指数（可选）
    },
    
    // 括号
    Parenthesized(Box<MathExpr>),          // 括号表达式
    
    // 矩阵
    Matrix {
        rows: Vec<Vec<MathExpr>>,          // 矩阵行数据
        matrix_type: String,               // 矩阵类型
    },
    
    // 导数运算符
    Derivative {
        variable: String,                  // 变量
        expression: Box<MathExpr>,         // 表达式
    },
}
```

### 运算符定义

```rust
// 二元运算符
pub enum BinaryOperator {
    Add,         // +
    Subtract,    // -
    Multiply,    // *
    DotProduct,  // \cdot
    Divide,      // /
    Power,       // ^
    Equals,      // =
}

// 一元运算符
pub enum UnaryOperator {
    Plus,        // +
    Minus,       // -
    Factorial,   // !
}
```

### AST 示例

输入表达式: `x^2 + \frac{1}{2}`

对应的 AST 结构:
```
BinaryOp {
  left: Superscript {
    base: Variable("x"),
    superscript: Number(2.0)
  },
  operator: Add,
  right: Fraction {
    numerator: Number(1.0),
    denominator: Number(2.0)
  }
}
```

## 支持的功能

### 1. 基本算术运算
- 加法: `a + b`
- 减法: `a - b`
- 乘法: `a * b` 或 `a \cdot b`
- 除法: `a / b`
- 幂运算: `a^b`
- 等式: `a = b`

### 2. 一元运算
- 正号: `+a`
- 负号: `-a`
- 阶乘: `a!`

### 3. 函数调用
- 基本函数: `f(x, y, z)`
- 三角函数: `\sin(x)`, `\cos(x)`, `\tan(x)`
- 对数函数: `\log(x)`, `\ln(x)`
- 其他函数: `\sqrt{x}`, `\sqrt[n]{x}`

### 4. 分数和根式
- 分数: `\frac{a}{b}`
- 平方根: `\sqrt{x}`
- n次方根: `\sqrt[n]{x}`

### 5. 上下标
- 下标: `x_1`, `x_{ij}`
- 上标: `x^2`, `x^{ij}`

### 6. 绝对值和范数
- 绝对值: `|x|`
- 范数: `||x||`

### 7. 矩阵
- 支持多种矩阵环境:
  - `matrix`: 普通矩阵
  - `pmatrix`: 圆括号矩阵
  - `bmatrix`: 方括号矩阵
  - `vmatrix`: 行列式
  - `Vmatrix`: 双竖线矩阵

### 8. 特殊符号和命令
- 向量: `\vec{a}`
- 点乘: `\cdot`
- 积分: `\int`
- 导数: `\frac{d}{dx}`, `f'`

### 9. Markdown 公式解析
- 行内公式: `$...$`
- 块级公式: `$$...$$`

## 使用示例

### 基本用法

```rust
use markdown_formula_parser::{parse_inline_math, parse_display_math};

fn main() {
    // 解析行内公式
    let result = parse_inline_math("E = mc^2");
    match result {
        Ok(math_block) => {
            println!("AST: {:#?}", math_block.expr);
            println!("LaTeX: {}", math_block.to_string());
        }
        Err(e) => println!("解析错误: {}", e),
    }
    
    // 解析块级公式
    let result = parse_display_math("\\int_{-\\infty}^{\\infty} e^{-x^2} dx = \\sqrt{\\pi}");
    // ... 处理解析结果
}
```

### Markdown 解析

```rust
use markdown_formula_parser::parse_markdown_math;

fn main() {
    let markdown = r#"
这是一个包含数学公式的Markdown文档。

行内公式：$E = mc^2$ 和 $\frac{1}{2}$。

块级公式：
$$
\int_{-\infty}^{\infty} e^{-x^2} dx = \sqrt{\pi}
$$
"#;

    let math_blocks = parse_markdown_math(markdown);
    for (i, block) in math_blocks.iter().enumerate() {
        println!("公式 {}: {}", i + 1, block.to_string());
        println!("显示模式: {}", if block.display_style { "块级" } else { "行内" });
    }
}
```

## 运行示例

项目包含多个示例程序，可以通过以下命令运行：

```bash
# 运行绝对值测试
cargo run --example absolute_value_test

# 运行算术表达式测试
cargo run --example arithmetic_examples

# 运行微积分表达式测试
cargo run --example calculus_examples

# 运行复杂表达式测试
cargo run --example complex_examples

# 运行导数表达式测试
cargo run --example derivative_examples

# 运行分数和根式测试
cargo run --example fractions_roots_examples

# 运行函数调用测试
cargo run --example functions_examples

# 运行矩阵表达式测试
cargo run --example matrix_examples

# 运行负号处理测试
cargo run --example negative_examples

# 运行幂运算和上下标测试
cargo run --example power_subscript_examples

# 运行特殊命令和符号测试
cargo run --example special_commands_examples

# 运行Markdown解析测试
cargo run --example markdown_parsing_examples
```