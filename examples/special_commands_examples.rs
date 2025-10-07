use markdown_formula_parser::{parse_inline_math, parse_display_math};

fn main() {
    println!("=== 特殊命令和符号测试 ===\n");
    
    // 测试 \vec 命令和点乘运算符
    let expr1 = "\\vec{a} \\cdot \\vec{b}";
    match parse_inline_math(expr1) {
        Ok(ast) => {
            println!("表达式: {}", expr1);
            println!("AST: {:#?}", ast.expr);
            println!("LaTeX: {}\n", ast.to_string());
        }
        Err(e) => {
            println!("解析错误 '{}': {}\n", expr1, e);
        }
    }
    
    // 测试复杂微积分表达式
    let expr2 = "\\frac{d}{dx}[\\int_{a}^{x} f(t) dt] = f(x)";
    match parse_display_math(expr2) {
        Ok(ast) => {
            println!("表达式: {}", expr2);
            println!("AST: {:#?}", ast.expr);
            println!("LaTeX: {}\n", ast.to_string());
        }
        Err(e) => {
            println!("解析错误 '{}': {}\n", expr2, e);
        }
    }
    
    // 测试 \cdot 运算符
    let expr3 = "a \\cdot b";
    match parse_inline_math(expr3) {
        Ok(ast) => {
            println!("表达式: {}", expr3);
            println!("AST: {:#?}", ast.expr);
            println!("LaTeX: {}\n", ast.to_string());
        }
        Err(e) => {
            println!("解析错误 '{}': {}\n", expr3, e);
        }
    }
    
    // 测试更复杂的表达式
    let expr4 = "|\\vec{a}| |\\vec{b}| \\cos(\\theta)";
    match parse_inline_math(expr4) {
        Ok(ast) => {
            println!("表达式: {}", expr4);
            println!("AST: {:#?}", ast.expr);
            println!("LaTeX: {}\n", ast.to_string());
        }
        Err(e) => {
            println!("解析错误 '{}': {}\n", expr4, e);
        }
    }
    
    // 测试绝对值
    let expr5 = "|x| + |y| = |z|";
    match parse_inline_math(expr5) {
        Ok(ast) => {
            println!("表达式: {}", expr5);
            println!("AST: {:#?}", ast.expr);
            println!("LaTeX: {}\n", ast.to_string());
        }
        Err(e) => {
            println!("解析错误 '{}': {}\n", expr5, e);
        }
    }
    
    // 测试混合表达式
    let expr6 = "\\vec{a} \\cdot \\vec{b} = |\\vec{a}| |\\vec{b}| \\cos(\\theta)";
    match parse_inline_math(expr6) {
        Ok(ast) => {
            println!("表达式: {}", expr6);
            println!("AST: {:#?}", ast.expr);
            println!("LaTeX: {}\n", ast.to_string());
        }
        Err(e) => {
            println!("解析错误 '{}': {}\n", expr6, e);
        }
    }
}