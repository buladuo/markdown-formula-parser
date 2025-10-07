use markdown_formula_parser::parse_inline_math;

fn main() {
    println!("=== 幂运算和上下标测试 ===\n");

    let examples = vec![
        // 上标测试
        "x^2",
        "x^{2}",
        "x^(2)",  // 这个应该会失败，因为我们不支持圆括号作为上标分组
        "x^2^3",
        "x^{2+3}",
        "e^{i\\pi}",
        
        // 下标测试
        "x_1",
        "x_{1}",
        "x_(1)",  // 这个也应该会失败
        "x_1_2",
        "x_{i+1}",
        
        // 组合测试
        "x_1^2",
        "x^2_1",
        "x_{i+1}^{n+1}",
        "A_{ij}^{kl}",
        
        // 复杂组合
        "\\sum_{i=1}^{n} x_i^2",
        "\\prod_{k=1}^{\\infty} a_k",
    ];

    for expr in examples {
        match parse_inline_math(expr) {
            Ok(ast) => {
                println!("表达式: {}", expr);
                println!("AST: {:#?}", ast.expr);
                println!("LaTeX: {}\n", ast.to_string());
            }
            Err(e) => {
                println!("解析错误 '{}': {}\n", expr, e);
            }
        }
    }
}