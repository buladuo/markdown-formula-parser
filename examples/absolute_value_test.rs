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
}