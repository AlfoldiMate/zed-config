//! Formatter.
//!
//! - 2-space indent by block depth
//! - inline blocks `header { a: x  b: y }` are expanded to one item per line
//! - within one block, every direct binding's action starts in the same column, and every trailing
//!   comment of that block starts in the same column after the longest action
//! - comment-only lines, blank lines, aliases and bodies are kept

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

/// Whitespace-separated tokens; quotes and `name(...)` argument groups stay intact; `{` and `}`
/// are always their own tokens.
fn tokens(s: &str) -> Vec<String> {
    let chars: Vec<char> = s.chars().collect();
    let mut out = Vec::new();
    let mut i = 0;
    while i < chars.len() {
        let c = chars[i];
        if c.is_whitespace() {
            i += 1;
            continue;
        }
        if c == '{' || c == '}' {
            out.push(c.to_string());
            i += 1;
            continue;
        }
        let start = i;
        let mut in_str = false;
        let mut depth = 0usize;
        while i < chars.len() {
            let d = chars[i];
            if in_str {
                if d == '\\' {
                    i += 2;
                    continue;
                }
                if d == '"' {
                    in_str = false;
                }
                i += 1;
                continue;
            }
            if d == '"' {
                in_str = true;
                i += 1;
                continue;
            }
            if depth > 0 {
                if d == '(' {
                    depth += 1;
                } else if d == ')' {
                    depth -= 1;
                }
                i += 1;
                continue;
            }
            if d == '(' && i > start {
                depth = 1;
                i += 1;
                continue;
            }
            if d.is_whitespace() || d == '{' || d == '}' {
                break;
            }
            i += 1;
        }
        out.push(chars[start..i].iter().collect());
    }
    out
}

fn is_key_tok(t: &str) -> bool {
    (t.ends_with(':') && !t.ends_with("::") && t.len() > 1) || t == "\":\":"
}

/// Expand the tokens of an inline block body into lines (without indentation).
fn expand_tokens(toks: &[String]) -> Vec<String> {
    let mut lines = Vec::new();
    let mut i = 0;
    let mut keys: Vec<String> = Vec::new();
    while i < toks.len() {
        let t = &toks[i];
        if t == "{" || t == "}" {
            i += 1;
            continue;
        }
        if is_key_tok(t) {
            keys.push(t.clone());
            i += 1;
            if i < toks.len() && toks[i] == "{" {
                // nested block: collect to the matching brace
                let mut depth = 1;
                let mut j = i + 1;
                while j < toks.len() && depth > 0 {
                    if toks[j] == "{" {
                        depth += 1;
                    } else if toks[j] == "}" {
                        depth -= 1;
                    }
                    j += 1;
                }
                let inner = expand_tokens(&toks[i + 1..j.saturating_sub(1)]);
                lines.push(format!("{} {{", keys.join(" ")));
                for l in inner {
                    lines.push(format!("  {l}"));
                }
                lines.push("}".to_string());
                keys.clear();
                i = j;
            } else if i < toks.len() {
                lines.push(format!("{} {}", keys.join(" "), toks[i]));
                keys.clear();
                i += 1;
            } else {
                lines.push(keys.join(" "));
                keys.clear();
            }
        } else {
            keys.push(t.clone());
            i += 1;
        }
    }
    if !keys.is_empty() {
        lines.push(keys.join(" "));
    }
    lines
}

/// Pass 1: expand `header { … }` written on one line into several lines.
fn expand_inline(src: &str) -> Vec<String> {
    let mut out = Vec::new();
    for line in src.split('\n') {
        let (code, comment) = split_comment(line);
        let code_t = code.trim();
        let indent: String = line.chars().take_while(|c| *c == ' ' || *c == '\t').collect();
        let open = code_t.find('{');
        if let Some(o) = open {
            if code_t.ends_with('}') && o + 1 < code_t.len() - 1 {
                let header = code_t[..o].trim_end();
                let body = &code_t[o + 1..code_t.len() - 1];
                if !body.trim().is_empty() {
                    let mut first = format!("{indent}{header} {{");
                    if !comment.trim().is_empty() {
                        first.push_str("  ");
                        first.push_str(comment.trim());
                    }
                    out.push(first);
                    for l in expand_tokens(&tokens(body)) {
                        out.push(format!("{indent}  {l}"));
                    }
                    out.push(format!("{indent}}}"));
                    continue;
                }
            }
        }
        out.push(line.to_string());
    }
    out
}

/// Position just after the key separator `:` of a binding (quote-aware), if the line is a binding.
fn key_end(code: &str) -> Option<usize> {
    let b = code.as_bytes();
    let mut in_str = false;
    let mut i = 0;
    let mut word_len = 0usize; // length of the current unquoted word
    while i < b.len() {
        let c = b[i];
        if in_str {
            if c == b'\\' {
                i += 2;
                continue;
            }
            if c == b'"' {
                in_str = false;
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
        if c == b'(' && word_len > 0 {
            return None; // `name(args`: an action before any key separator, not a binding
        }
        if c == b':' {
            let next_colon = i + 1 < b.len() && b[i + 1] == b':';
            if next_colon {
                return None;
            }
            if word_len > 0 {
                return Some(i + 1);
            }
        }
        if c.is_ascii_whitespace() {
            word_len = 0;
        } else {
            word_len += 1;
        }
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
    let lines = expand_inline(src);
    let mut kinds = Vec::with_capacity(lines.len());
    let mut depths = Vec::with_capacity(lines.len());
    let mut blocks = Vec::with_capacity(lines.len());
    let mut codes = Vec::with_capacity(lines.len());
    let mut comments = Vec::with_capacity(lines.len());
    let mut depth = 0usize;
    let mut block_stack = vec![0usize];
    let mut next_block = 1usize;
    for line in &lines {
        let (code, comment) = split_comment(line);
        let code = code.trim().to_string();
        comments.push(comment.trim().to_string());
        let kind = if code.is_empty() {
            Kind::Other
        } else if code == "}" {
            Kind::Close
        } else if code.ends_with('{') {
            Kind::Open
        } else if code.starts_with('@') && (code.contains('=') || !code.contains(':')) {
            Kind::Other
        } else if let Some(k) = key_end(&code) {
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
        codes.push(code);
    }
    // per block: key column width and action width (for comment alignment)
    let mut key_w: Vec<usize> = vec![0; next_block];
    let mut act_w: Vec<usize> = vec![0; next_block];
    for (i, k) in kinds.iter().enumerate() {
        if let Kind::Binding { key, action } = k {
            let b = blocks[i];
            key_w[b] = key_w[b].max(key.chars().count());
            if !comments[i].is_empty() {
                act_w[b] = act_w[b].max(action.chars().count());
            }
        }
    }
    // comment column applies to the longest commented action in the block; uncommented longer
    // actions do not push it out
    let mut out = String::with_capacity(src.len() + 64);
    for i in 0..lines.len() {
        let indent = "  ".repeat(depths[i]);
        let code = &codes[i];
        let comment = &comments[i];
        let rendered = match &kinds[i] {
            Kind::Binding { key, action } => {
                let b = blocks[i];
                let mut s = format!("{indent}{key}{} {action}", " ".repeat(key_w[b] - key.chars().count()));
                if !comment.is_empty() {
                    let pad = act_w[b].saturating_sub(action.chars().count());
                    s.push_str(&" ".repeat(pad + 2));
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
        out.push_str(rendered.trim_end());
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
    fn aligns_per_block_and_comments() {
        let src = "Editor {\n escape:  editor::Cancel // c\n  cmd-shift-left shift-home: editor::SelectToBeginningOfLine(a, b)   // long\n    mode == full {\n  x: a::B\n   \"g g\": c::D\n  }\n  (: vim::SentenceBackward\n}\n";
        let want = "Editor {\n  escape:                    editor::Cancel                         // c\n  cmd-shift-left shift-home: editor::SelectToBeginningOfLine(a, b)  // long\n  mode == full {\n    x:     a::B\n    \"g g\": c::D\n  }\n  (:                         vim::SentenceBackward\n}\n";
        assert_eq!(format(src), want);
        assert_eq!(format(want), want);
    }

    #[test]
    fn expands_inline_blocks() {
        let src = "g: { .: pane::Reveal }  // ours\n!Terminal { cmd-n: x::New   f5: null }\nspace: { w: { h: a::L  j: a::D } q: a::Q }\n";
        let want = "g: {  // ours\n  .: pane::Reveal\n}\n!Terminal {\n  cmd-n: x::New\n  f5:    null\n}\nspace: {\n  w: {\n    h: a::L\n    j: a::D\n  }\n  q: a::Q\n}\n";
        assert_eq!(format(src), want);
        assert_eq!(format(want), want);
    }

    #[test]
    fn leaves_aliases_and_bodies() {
        let src = "@m = a | b\n@n = {\n  j: x::Y\n}\n@m {\n  @n\n  \":\": z::W\n}\n";
        assert_eq!(format(src), src);
    }
}
