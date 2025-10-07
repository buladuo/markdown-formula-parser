use markdown_formula_parser::parse_inline_math;

fn main() {
    println!("=== 绝对值测试 ===\n");
    
    // 测试简单绝对值
    let expr1 = "|x|";
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
    
    // 测试带运算的绝对值
    let expr2 = "|x| + |y|";
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
    
    // 测试范数
    let expr3 = "||x||";
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
    
    // 测试复杂表达式中的绝对值
    let expr4 = "|x + y|";
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
    
    // 测试分数中的绝对值
    let expr5 = "|\\frac{x}{y}|";
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
    
    // 测试绝对值与幂运算结合
    let expr6 = "|x|^2";
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
    
    // 测试绝对值与下标结合
    let expr7 = "|x_1|";
    match parse_inline_math(expr7) {
        Ok(ast) => {
            println!("表达式: {}", expr7);
            println!("AST: {:#?}", ast.expr);
            println!("LaTeX: {}\n", ast.to_string());
        }
        Err(e) => {
            println!("解析错误 '{}': {}\n", expr7, e);
        }
    }
    
    // 测试多个绝对值相乘
    let expr8 = "|a| |b| |c|";
    match parse_inline_math(expr8) {
        Ok(ast) => {
            println!("表达式: {}", expr8);
            println!("AST: {:#?}", ast.expr);
            println!("LaTeX: {}\n", ast.to_string());
        }
        Err(e) => {
            println!("解析错误 '{}': {}\n", expr8, e);
        }
    }
    
    // 测试绝对值与函数结合
    let expr9 = "|\\sin(x)|";
    match parse_inline_math(expr9) {
        Ok(ast) => {
            println!("表达式: {}", expr9);
            println!("AST: {:#?}", ast.expr);
            println!("LaTeX: {}\n", ast.to_string());
        }
        Err(e) => {
            println!("解析错误 '{}': {}\n", expr9, e);
        }
    }
    
    // 测试复杂的绝对值表达式
    let expr10 = "|x^2 + y^2|";
    match parse_inline_math(expr10) {
        Ok(ast) => {
            println!("表达式: {}", expr10);
            println!("AST: {:#?}", ast.expr);
            println!("LaTeX: {}\n", ast.to_string());
        }
        Err(e) => {
            println!("解析错误 '{}': {}\n", expr10, e);
        }
    }
    
    // 测试绝对值与减法结合
    let expr11 = "|x| - |y|";
    match parse_inline_math(expr11) {
        Ok(ast) => {
            println!("表达式: {}", expr11);
            println!("AST: {:#?}", ast.expr);
            println!("LaTeX: {}\n", ast.to_string());
        }
        Err(e) => {
            println!("解析错误 '{}': {}\n", expr11, e);
        }
    }
    
    // 测试绝对值与乘法结合
    let expr12 = "|x * y|";
    match parse_inline_math(expr12) {
        Ok(ast) => {
            println!("表达式: {}", expr12);
            println!("AST: {:#?}", ast.expr);
            println!("LaTeX: {}\n", ast.to_string());
        }
        Err(e) => {
            println!("解析错误 '{}': {}\n", expr12, e);
        }
    }
    
    // 测试绝对值与除法结合
    let expr13 = "|\\frac{a}{b}|";
    match parse_inline_math(expr13) {
        Ok(ast) => {
            println!("表达式: {}", expr13);
            println!("AST: {:#?}", ast.expr);
            println!("LaTeX: {}\n", ast.to_string());
        }
        Err(e) => {
            println!("解析错误 '{}': {}\n", expr13, e);
        }
    }
    
    // 测试绝对值与根式结合
    let expr14 = "|\\sqrt{x}|";
    match parse_inline_math(expr14) {
        Ok(ast) => {
            println!("表达式: {}", expr14);
            println!("AST: {:#?}", ast.expr);
            println!("LaTeX: {}\n", ast.to_string());
        }
        Err(e) => {
            println!("解析错误 '{}': {}\n", expr14, e);
        }
    }
}