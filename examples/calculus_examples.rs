use markdown_formula_parser::{parse_inline_math, parse_display_math};

fn main() {
    println!("=== 微积分表达式测试 ===\n");
    
    // 测试基本导数表达式
    let expr1 = "\\frac{d}{dx} f(x)";
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
    
    // 测试基本积分表达式
    let expr2 = "\\int_{a}^{x} f(t) dt";
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
    
    // 测试微积分基本定理表达式
    let expr3 = "\\frac{d}{dx}\\left[\\int_{a}^{x} f(t) dt\\right] = f(x)";
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
    
    // 测试牛顿莱布尼茨公式
    let expr4 = "\\int_{a}^{b} f'(x) dx = f(b) - f(a)";
    match parse_display_math(expr4) {
        Ok(ast) => {
            println!("表达式: {}", expr4);
            println!("AST: {:#?}", ast.expr);
            println!("LaTeX: {}\n", ast.to_string());
        }
        Err(e) => {
            println!("解析错误 '{}': {}\n", expr4, e);
        }
    }
    
    // 测试导数运算符
    let expr5 = "\\frac{d}{dx}[x^2] = 2x";
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
}