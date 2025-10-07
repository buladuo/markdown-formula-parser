use markdown_formula_parser::{parse_inline_math, parse_display_math};

fn main() {
    println!("=== 负号处理测试 ===\n");
    
    // 测试简单的负号表达式
    let expr1 = "-x";
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
    
    // 测试负数
    let expr2 = "-5";
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
    
    // 测试负指数
    let expr3 = "e^{-t}";
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
    
    // 测试复杂负指数
    let expr4 = "t^{z-1}";
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
    
    // 测试完整表达式
    let expr5 = "\\Gamma(z) = \\int_{0}^{\\infty} t^{z-1} e^{-t} dt";
    match parse_display_math(expr5) {
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