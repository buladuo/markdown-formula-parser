use markdown_formula_parser::parse_inline_math;

fn main() {
    println!("=== 函数调用测试 ===\n");

    let examples = vec![
        // 基本函数
        "\\sin(x)",
        "\\cos(x)",
        "\\tan(x)",
        "\\log(x)",
        "\\ln(x)",
        "\\exp(x)",
        
        // 带复杂参数的函数
        "\\sin(2x)",
        "\\cos(x + y)",
        "\\log(\\frac{1}{2})",
        "\\sin(\\sqrt{x})",
        
        // 多参数函数
        "\\max(a, b)",
        "\\min(x, y, z)",
        "\\gcd(a, b, c)",
        
        // 嵌套函数
        "\\sin(\\cos(x))",
        "\\log(\\sin(\\cos(x)))",
        
        // 函数与运算符组合
        "\\sin(x) + \\cos(y)",
        "\\log(x) * \\exp(y)",
        "\\sqrt{\\sin^2(x) + \\cos^2(x)}",
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