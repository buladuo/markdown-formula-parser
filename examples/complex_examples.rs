use markdown_formula_parser::parse_inline_math;

fn main() {
    println!("=== 复杂表达式测试 ===\n");

    let examples = vec![
        // 复杂的科学计算表达式
        "E = mc^2",
        "F = G\\frac{m_1 m_2}{r^2}",
        "e^{i\\pi} + 1 = 0",
        "\\sum_{k=1}^{n} k = \\frac{n(n+1)}{2}",
        "\\int_{0}^{\\infty} e^{-x^2} dx = \\frac{\\sqrt{\\pi}}{2}",
        
        // 多层嵌套表达式
        "\\sqrt{\\frac{\\sum_{i=1}^{n} x_i^2}{n}}",
        "\\frac{\\sin(\\theta) + \\cos(\\theta)}{\\sqrt{\\sin^2(\\theta) + \\cos^2(\\theta)}}",
        
        // 包含各种元素的复杂表达式
        "\\frac{d}{dx}[\\int_{a}^{x} f(t) dt] = f(x)",
        "\\vec{a} \\cdot \\vec{b} = |\\vec{a}| |\\vec{b}| \\cos(\\theta)",
        
        // 大型数学公式
        "\\zeta(s) = \\sum_{n=1}^{\\infty} \\frac{1}{n^s}",
        "\\Gamma(z) = \\int_{0}^{\\infty} t^{z-1} e^{-t} dt",
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