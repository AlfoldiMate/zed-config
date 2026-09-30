//! Formatter: 2-space indent by block depth, one binding per line, and within one block every
//! direct binding's action starts in the same column. Comments and blank lines are kept.

/// Split a line into (code, trailing comment) with quotes respected.
fn split_comment(line: &str) -> (&str, &str) {
    let b = line.as_bytes();
    let mut in_str = false;
    let mut i = 0;
    while i < b.len() {
        let c = b[i];
        if in_str {
            if c == b'\\' {
                i += 2;
                continue;
            }
            if c == b'"' {
                in_str = false;
            }
        } else if c == b'"' {
            in_str = true;
        } else if c == b'/' && i + 1 < b.len() && b[i + 1] == b'/' {
            return (&line[..i], &line[i..]);
        }
        i += 1;
    }
    (line, "")
}

/// Position just after the key separator `:` of a binding (quote-aware), if the line is a binding.
fn key_end(code: &str) -> Option<usize> {
    let b = code.as_bytes();
    let mut in_str = false;
    let mut i = 0;
    let mut in_word = false;
    while i < b.len() {
        let c = b[i];
        if in_str {
            if c == b'\\' {
                i += 2;
                continue;
            }
            if c == b'"' {
                in_str = false;
                // a quoted key directly followed by ':'
                if i + 1 < b.len() && b[i + 1] == b':' && (i + 2 >= b.len() || b[i + 2] != b':') {
                    return Some(i + 2);
                }
            }
            i += 1;
            continue;
        }
        if c == b'"' {
            in_str = true;
            i += 1;
            continue;
        }
        if c == b'(' {
            return None; // arguments before any key: not a binding line we understand
        }
        if c == b':' {
            let next_colon = i + 1 < b.len() && b[i + 1] == b':';
            let prev_colon = i > 0 && b[i - 1] == b':';
            if !next_colon && !prev_colon && in_word {
                return Some(i + 1);
            }
            if next_colon {
                // `::` belongs to an action name: no key separator seen before it means this is not a binding
                return None;
            }
        }
        in_word = !c.is_ascii_whitespace();
        i += 1;
    }
    None
}

#[derive(Debug)]
enum Kind {
    Other,
    Open,
    Close,
    Binding { key: String, action: String },
}

pub fn format(src: &str) -> String {
    let lines: Vec<&str> = src.split('\n').collect();
    let mut kinds = Vec::with_capacity(lines.len());
    let mut depths = Vec::with_capacity(lines.len());
    let mut blocks = Vec::with_capacity(lines.len());
    let mut comments = Vec::with_capacity(lines.len());
    let mut depth = 0usize;
    let mut block_stack = vec![0usize];
    let mut next_block = 1usize;
    for line in &lines {
        let (code, comment) = split_comment(line);
        let code = code.trim();
        comments.push(comment.trim_end().to_string());
        let kind = if code.is_empty() {
            Kind::Other
        } else if code == "}" {
            Kind::Close
        } else if code.ends_with('{') {
            Kind::Open
        } else if code.contains('{') && code.ends_with('}') {
            Kind::Other // inline block, kept verbatim
        } else if code.starts_with('@') && (code.contains('=') || !code.contains(':')) {
            Kind::Other
        } else if let Some(k) = key_end(code) {
            let key = code[..k].trim().to_string();
            let action = code[k..].trim().to_string();
            if action.is_empty() {
                Kind::Other
            } else {
                Kind::Binding { key, action }
            }
        } else {
            Kind::Other
        };
        match kind {
            Kind::Close => {
                depth = depth.saturating_sub(1);
                if block_stack.len() > 1 {
                    block_stack.pop();
                }
                depths.push(depth);
                blocks.push(*block_stack.last().unwrap());
            }
            Kind::Open => {
                depths.push(depth);
                blocks.push(*block_stack.last().unwrap());
                depth += 1;
                block_stack.push(next_block);
                next_block += 1;
            }
            _ => {
                depths.push(depth);
                blocks.push(*block_stack.last().unwrap());
            }
        }
        kinds.push(kind);
    }
    // column per block
    let mut width: Vec<usize> = vec![0; next_block];
    for (i, k) in kinds.iter().enumerate() {
        if let Kind::Binding { key, .. } = k {
            let w = key.chars().count();
            if w > width[blocks[i]] {
                width[blocks[i]] = w;
            }
        }
    }
    let mut out = String::with_capacity(src.len() + 64);
    for (i, line) in lines.iter().enumerate() {
        let indent = "  ".repeat(depths[i]);
        let (code, _) = split_comment(line);
        let code = code.trim();
        let comment = &comments[i];
        let rendered = match &kinds[i] {
            Kind::Binding { key, action } => {
                let pad = width[blocks[i]] - key.chars().count();
                let mut s = format!("{indent}{key}{} {action}", " ".repeat(pad));
                if !comment.is_empty() {
                    s.push_str("  ");
                    s.push_str(comment);
                }
                s
            }
            _ => {
                if code.is_empty() {
                    if comment.is_empty() {
                        String::new()
                    } else {
                        format!("{indent}{comment}")
                    }
                } else if comment.is_empty() {
                    format!("{indent}{code}")
                } else {
                    format!("{indent}{code}  {comment}")
                }
            }
        };
        out.push_str(&rendered);
        if i + 1 < lines.len() {
            out.push('\n');
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn aligns_per_block() {
        let src = "Editor {\n escape:  editor::Cancel // c\n  cmd-shift-left shift-home: editor::SelectToBeginningOfLine(a, b)\n    mode == full {\n  x: a::B\n   \"g g\": c::D\n  }\n  g: { d: e::F }\n}\n";
        let want = "Editor {\n  escape:                    editor::Cancel  // c\n  cmd-shift-left shift-home: editor::SelectToBeginningOfLine(a, b)\n  mode == full {\n    x:     a::B\n    \"g g\": c::D\n  }\n  g: { d: e::F }\n}\n";
        assert_eq!(format(src), want);
        assert_eq!(format(want), want);
    }

    #[test]
    fn leaves_aliases_and_bodies() {
        let src = "@m = a | b\n@n = {\n  j: x::Y\n}\n@m {\n  @n\n  \":\": z::W\n}\n";
        assert_eq!(format(src), src);
    }
}
