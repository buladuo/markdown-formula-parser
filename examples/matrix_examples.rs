use markdown_formula_parser::parse_inline_math;

fn main() {
    println!("=== 矩阵测试 ===\n");

    // 注意：当前实现可能不支持矩阵，这些测试用例可以用于未来扩展
    let examples = vec![
        "\\begin{matrix} a & b \\\\ c & d \\end{matrix}",
        "\\begin{matrix} 1 & 2 & 3 \\\\ 4 & 5 & 6 \\\\ 7 & 8 & 9 \\end{matrix}",
        "\\begin{matrix} x_1 \\\\ x_2 \\\\ x_3 \\end{matrix}",
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
    
    println!("注意：当前版本可能不支持矩阵语法，这些测试用例用于未来扩展。");
}