use markdown_formula_parser::parse_markdown_math;

fn main() {
    println!("=== Markdown公式提取测试 ===\n");

    let markdown_examples = vec![
        // 基本文本与公式混合
        r#"这是行内公式 $E = mc^2$ 的示例。"#,
        
        // 多个行内公式
        r#"公式：$a^2 + b^2 = c^2$ 和 $\sin^2(x) + \cos^2(x) = 1$。"#,
        
        // 块级公式
        r#"这是一个块级公式：
$$
\int_{-\infty}^{\infty} e^{-x^2} dx = \sqrt{\pi}
$$"#,
        
        // 行内和块级混合
        r#"行内公式：$x = \frac{-b \pm \sqrt{b^2 - 4ac}}{2a}$。
        
块级公式：
$$
\sum_{n=1}^{\infty} \frac{1}{n^2} = \frac{\pi^2}{6}
$$"#,
        
        // 复杂的Markdown文档
        r#"# 数学公式示例

在物理学中，著名的质能方程是 $E = mc^2$。

另一个重要的方程是薛定谔方程：
$$
i\hbar\frac{\partial}{\partial t}\Psi(\vec{r},t) = \hat{H}\Psi(\vec{r},t)
$$

在几何学中，勾股定理表述为 $a^2 + b^2 = c^2$。"#,
        
        // 边界情况
        r#"空公式：$$$$(在块级公式中)和$$$(在行内公式中)。"#,
        
        // 不完整或错误的公式
        r#"不完整的公式：$E = mc^"#,
    ];

    for (i, markdown) in markdown_examples.iter().enumerate() {
        println!("测试案例 {}:", i + 1);
        println!("Markdown内容:\n{}\n", markdown);
        
        let math_blocks = parse_markdown_math(markdown);
        if math_blocks.is_empty() {
            println!("未找到数学公式。\n");
        } else {
            for (j, block) in math_blocks.iter().enumerate() {
                println!("公式 {}: {}", j + 1, block.to_string());
                println!("显示模式: {}", if block.display_style { "块级" } else { "行内" });
                println!("表达式 AST: {:#?}\n", block.expr);
            }
        }
        println!("---\n");
    }
}