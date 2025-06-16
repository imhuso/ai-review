use regex;

/// 智能处理 Telegram MarkdownV2 格式
///
/// 支持尽可能多的 Markdown 格式，同时确保消息能正确发送
pub fn process_telegram_markdown(text: &str) -> String {
    let mut result = text.to_string();

    // 第一步：保护代码块和行内代码
    let mut protected_segments = Vec::new();
    let mut segment_index = 0;

    // 保护代码块 ```language\ncode\n```
    result = protect_code_blocks(result, &mut protected_segments, &mut segment_index);

    // 保护行内代码 `code`
    result = protect_inline_code(result, &mut protected_segments, &mut segment_index);

    // 第二步：处理 Markdown 格式
    // 转换标准 Markdown 到 Telegram MarkdownV2
    result = convert_markdown_to_telegram(result);

    // 第三步：转义剩余的特殊字符
    result = escape_remaining_special_chars(result);

    // 第四步：恢复保护的代码
    for (i, segment) in protected_segments.into_iter().enumerate() {
        let placeholder = format!("CODEBLOCK{}", i);
        result = result.replace(&placeholder, &segment);
    }

    result
}

/// 保护代码块
fn protect_code_blocks(mut text: String, protected: &mut Vec<String>, index: &mut usize) -> String {
    while let Some(start) = text.find("```") {
        if let Some(end_start) = text[start + 3..].find("```") {
            let end_pos = start + 3 + end_start + 3;
            let code_block = text[start..end_pos].to_string();
            let placeholder = format!("CODEBLOCK{}", *index);
            protected.push(code_block);
            text.replace_range(start..end_pos, &placeholder);
            *index += 1;
        } else {
            break;
        }
    }
    text
}

/// 保护行内代码
fn protect_inline_code(mut text: String, protected: &mut Vec<String>, index: &mut usize) -> String {
    let mut start = 0;
    while let Some(pos) = text[start..].find('`') {
        let abs_pos = start + pos;
        if let Some(end_pos) = text[abs_pos + 1..].find('`') {
            let abs_end = abs_pos + 1 + end_pos;
            let inline_code = text[abs_pos..=abs_end].to_string();
            let placeholder = format!("CODEBLOCK{}", *index);
            protected.push(inline_code);
            text.replace_range(abs_pos..=abs_end, &placeholder);
            *index += 1;
            start = abs_pos + placeholder.len();
        } else {
            break;
        }
    }
    text
}

/// 转换标准 Markdown 到 Telegram MarkdownV2
fn convert_markdown_to_telegram(text: String) -> String {
    let mut result = text;

    // 处理标题 - 转换为引用格式，更明显
    // # Title -> >Title
    // ## Title -> >Title
    if let Ok(header_regex) = regex::Regex::new(r"^(#{1,6})\s+(.+)$") {
        let lines: Vec<&str> = result.split('\n').collect();
        let mut new_lines = Vec::new();

        for line in lines {
            if let Some(captures) = header_regex.captures(line) {
                let title = captures.get(2).unwrap().as_str();
                new_lines.push(format!(">{}", title));
            } else {
                new_lines.push(line.to_string());
            }
        }
        result = new_lines.join("\n");
    }

    // 处理粗体 **text** -> *text*
    if let Ok(bold_regex) = regex::Regex::new(r"\*\*([^*]+)\*\*") {
        while let Some(captures) = bold_regex.captures(&result) {
            let full_match = captures.get(0).unwrap().as_str();
            let content = captures.get(1).unwrap().as_str();
            result = result.replace(full_match, &format!("*{}*", content));
        }
    }

    // 处理引用 > text -> >text (保持不变，但确保格式正确)
    let lines: Vec<&str> = result.split('\n').collect();
    let mut new_lines = Vec::new();

    for line in lines {
        if line.trim_start().starts_with('>') {
            // 保持引用格式
            new_lines.push(line.to_string());
        } else {
            new_lines.push(line.to_string());
        }
    }
    result = new_lines.join("\n");

    result
}

/// 转义剩余的特殊字符
fn escape_remaining_special_chars(text: String) -> String {
    let mut result = text;

    // 需要转义的字符，但排除我们想保留的格式字符
    let chars_to_escape = [
        // 不转义 * 因为我们用它做粗体
        // 不转义 > 因为我们用它做引用
        // 不转义 ` 因为已经保护了代码
        '_', '[', ']', '(', ')', '~', '#', '+', '-', '=', '|', '{', '}', '.', '!',
    ];

    for &ch in &chars_to_escape {
        result = result.replace(ch, &format!("\\{}", ch));
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_markdown_processing() {
        let input = "# 标题\n\n**粗体文本**\n\n`代码`\n\n```rust\nfn main() {}\n```";
        let result = process_telegram_markdown(input);
        
        // 验证标题转换为引用
        assert!(result.contains(">标题"));
        // 验证粗体转换
        assert!(result.contains("*粗体文本*"));
        // 验证代码块保持不变
        assert!(result.contains("```rust\nfn main() {}\n```"));
    }

    #[test]
    fn test_special_char_escaping() {
        let input = "测试_下划线和[方括号]";
        let result = process_telegram_markdown(input);
        
        assert!(result.contains("测试\\_下划线和\\[方括号\\]"));
    }
}
