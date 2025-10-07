use markdown_formula_parser::{parse_inline_math, parse_display_math, parse_markdown_math};

fn main() {
    println!("Markdown Formula Parser Examples:\n");
    
    // 示例1: 简单算术表达式
    let expr1 = "2 + 3 * 4";
    match parse_inline_math(expr1) {
        Ok(ast) => {
            println!("Expression: {}", expr1);
            println!("AST: {:#?}", ast.expr);
            println!("LaTeX: {}\n", ast.to_string());
        }
        Err(e) => {
            println!("Error parsing '{}': {}\n", expr1, e);
        }
    }
    
    // 示例2: 分数和上下标
    let expr2 = "x^2 + \\frac{1}{2}";
    match parse_inline_math(expr2) {
        Ok(ast) => {
            println!("Expression: {}", expr2);
            println!("AST: {:#?}", ast.expr);
            println!("LaTeX: {}\n", ast.to_string());
        }
        Err(e) => {
            println!("Error parsing '{}': {}\n", expr2, e);
        }
    }
    
    // 示例3: 复杂表达式
    let expr3 = "\\sqrt[3]{8} = 2";
    match parse_display_math(expr3) {
        Ok(ast) => {
            println!("Expression: {}", expr3);
            println!("AST: {:#?}", ast.expr);
            println!("LaTeX: {}\n", ast.to_string());
        }
        Err(e) => {
            println!("Error parsing '{}': {}\n", expr3, e);
        }
    }
    
    // 示例4: 从Markdown文本中提取公式
    let markdown = r#"
这是一个包含数学公式的Markdown文档。

行内公式：$E = mc^2$ 和 $\frac{1}{2}$。

块级公式：
$$
\int_{-\infty}^{\infty} e^{-x^2} dx = \sqrt{\pi}
$$

另一个行内公式：$a^2 + b^2 = c^2$。
    "#;
    
    println!("Parsing Markdown text:");
    let math_blocks = parse_markdown_math(markdown);
    for (i, block) in math_blocks.iter().enumerate() {
        println!("Formula {}: {}", i + 1, block.to_string());
        println!("Display style: {}", block.display_style);
        println!("---");
    }
}