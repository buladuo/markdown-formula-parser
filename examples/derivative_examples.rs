use markdown_formula_parser::{parse_inline_math, parse_display_math};

fn main() {
    println!("=== 导数表达式测试 ===\n");
    
    // 测试基本导数表达式
    let expr1 = "\\frac{d}{dx} x^2";
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
    
    // 测试带方括号的导数表达式
    let expr2 = "\\frac{d}{dx}[x^2]";
    match parse_inline_math(expr2) {
        Ok(ast) => {
            println!("表达式: {}", expr2);
            println!("AST: {:#?}", ast.expr);
            println!("LaTeX: {}\n", ast.to_string());
        }
        Err(e) => {
            println!("解析错误 '{}': {}\n", expr2, e);
        }
    }
    
    // 测试完整的微积分基本定理表达式
    let expr3 = "\\frac{d}{dx}[\\int_{a}^{x} f(t) dt] = f(x)";
    match parse_display_math(expr3) {
        Ok(ast) => {
            println!("表达式: {}", expr3);
            println!("AST: {:#?}", ast.expr);
            println!("LaTeX: {}\n", ast.to_string());
        }
        Err(e) => {
            println!("解析错误 '{}': {}\n", expr3, e);
        }
    }
    
    // 测试简单导数计算结果
    let expr4 = "\\frac{d}{dx}[x^2] = 2x";
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
}