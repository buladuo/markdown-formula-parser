use markdown_formula_parser::parse_inline_math;

fn main() {
    println!("=== 基本算术表达式测试 ===\n");

    let examples = vec![
        "1 + 2",
        "3 - 4",
        "5 * 6",
        "7 / 8",
        "2 + 3 * 4",
        "(2 + 3) * 4",
        "1 + 2 + 3",
        "10 - 5 + 3",
        "2 * 3 * 4",
        "24 / 6 / 2",
        "2 + 3 * 4 - 5",
        "((2 + 3) * 4) - 5",
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