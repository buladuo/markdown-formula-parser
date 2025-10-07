use markdown_formula_parser::parse_inline_math;

fn main() {
    println!("=== 简单绝对值测试 ===\n");

    let test_cases = vec![
        "|x|",
        "|x| + |y|",
        "|a| |b|",
    ];

    for case in test_cases {
        match parse_inline_math(case) {
            Ok(ast) => {
                println!("表达式: {}", case);
                println!("AST: {:#?}", ast);
                // println!("LaTeX: {}\n", ast.to_latex());
                println!();
            }
            Err(e) => {
                println!("解析错误 '{}': {}\n", case, e);
            }
        }
    }
}