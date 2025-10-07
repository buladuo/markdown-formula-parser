use markdown_formula_parser::parse_inline_math;

fn main() {
    println!("=== 分数和根式测试 ===\n");

    let examples = vec![
        // 基本分数
        "\\frac{1}{2}",
        "\\frac{a}{b}",
        "\\frac{1+2}{3+4}",
        "\\frac{\\frac{1}{2}}{3}",
        "\\frac{1}{\\frac{2}{3}}",
        
        // 根式
        "\\sqrt{2}",
        "\\sqrt{x+y}",
        "\\sqrt{\\frac{1}{2}}",
        "\\sqrt[3]{8}",
        "\\sqrt[n]{x}",
        "\\sqrt[\\frac{1}{2}]{x}",
        
        // 分数和根式组合
        "\\frac{\\sqrt{2}}{2}",
        "\\sqrt{\\frac{a+b}{c+d}}",
        "\\frac{1}{\\sqrt[3]{x+1}}",
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