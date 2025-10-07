pub mod ast;
pub mod lexer;
pub mod parser;

use ast::MathBlock;
use parser::Parser;

pub fn parse_math_block(input: &str, display_style: bool) -> Result<MathBlock, String> {
    let mut parser = Parser::new(input);
    let expr = parser.parse_expression()?;
    
    Ok(MathBlock {
        expr,
        display_style,
    })
}

pub fn parse_inline_math(input: &str) -> Result<MathBlock, String> {
    parse_math_block(input, false)
}

pub fn parse_display_math(input: &str) -> Result<MathBlock, String> {
    parse_math_block(input, true)
}

// 从Markdown文本中提取并解析数学公式
pub fn parse_markdown_math(markdown: &str) -> Vec<MathBlock> {
    let mut math_blocks = Vec::new();
    let markdown_chars: Vec<char> = markdown.chars().collect();
    let mut i = 0;
    
    while i < markdown_chars.len() {
        if markdown_chars[i] == '$' {
            // 检查是否是显示模式($$)
            let display_style = i + 1 < markdown_chars.len() && markdown_chars[i + 1] == '$';
            
            // 确定结束分隔符和偏移量
            let (start_offset, end_delimiter) = if display_style {
                (2, "$$")
            } else {
                (1, "$")
            };
            
            // 查找结束分隔符
            let start_index = i + start_offset;
            let mut end_index = None;
            
            for j in start_index..markdown_chars.len() {
                if markdown_chars[j] == end_delimiter.chars().next().unwrap() {
                    if display_style {
                        // 对于显示模式，需要检查下一个字符是否也是$
                        if j + 1 < markdown_chars.len() && markdown_chars[j + 1] == '$' {
                            end_index = Some(j);
                            break;
                        }
                    } else {
                        // 对于内联模式，找到$即可
                        end_index = Some(j);
                        break;
                    }
                }
            }
            
            // 如果找到了结束分隔符，则提取内容
            if let Some(end_idx) = end_index {
                let math_content: String = markdown_chars[start_index..end_idx].iter().collect();
                if let Ok(math_block) = parse_math_block(&math_content, display_style) {
                    math_blocks.push(math_block);
                }
                
                // 更新索引位置
                i = if display_style {
                    end_idx + 2  // 跳过$$
                } else {
                    end_idx + 1  // 跳过$
                };
            } else {
                i += 1;
            }
        } else {
            i += 1;
        }
    }
    
    math_blocks
}