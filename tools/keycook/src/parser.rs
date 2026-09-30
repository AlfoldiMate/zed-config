//! Tokenizer and parser for the `.kc` DSL.
//!
//! Grammar, informally:
//!
//! ```text
//! file      := item*
//! item      := alias | block | binding | newline
//! alias     := '@' name '=' text-to-end-of-line
//! block     := header '{' item* '}'
//! header    := context-text            (no key token before the '{', same line)
//!            | keys ':'                (a prefix block)
//! binding   := keys ':' actions        (ends at newline, '{', '}' or the next key token)
//! keys      := keytok+ ('|' keytok+)*
//! actions   := action+ ('|' action+)*
//! action    := 'null' | name ['(' args ')']
//! ```
//!
//! A word ending in a single `:` is a key token. Quoted words are keys or context text verbatim.

use crate::ast::*;
use serde_json::{Map, Value};

#[derive(Debug, Clone)]
pub struct Diagnostic {
    pub span: Span,
    pub message: String,
    pub severity: Severity,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Severity {
    Error,
    Warning,
    Info,
    Hint,
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum Tok {
    /// A word; `key` is true when it ended with a single `:` (stripped).
    Word { text: String, key: bool, quoted: bool, span: Span },
    Pipe(Span),
    LBrace(Span),
    RBrace(Span),
    /// `@name = value`
    Alias { name: String, value: String, span: Span },
    Newline(Span),
}

impl Tok {
    fn span(&self) -> Span {
        match self {
            Tok::Word { span, .. } | Tok::Pipe(span) | Tok::LBrace(span) | Tok::RBrace(span) | Tok::Newline(span) => *span,
            Tok::Alias { span, .. } => *span,
        }
    }
}

pub(crate) fn tokenize(src: &str, diags: &mut Vec<Diagnostic>) -> Vec<Tok> {
    let b = src.as_bytes();
    let mut toks = Vec::new();
    let mut i = 0;
    let n = b.len();
    let mut at_line_start = true;
    while i < n {
        let c = b[i];
        if c == b'\n' {
            toks.push(Tok::Newline(Span::new(i, i + 1)));
            i += 1;
            at_line_start = true;
            continue;
        }
        if c == b' ' || c == b'\t' || c == b'\r' {
            i += 1;
            continue;
        }
        if c == b'/' && i + 1 < n && b[i + 1] == b'/' {
            while i < n && b[i] != b'\n' {
                i += 1;
            }
            continue;
        }
        let looks_like_alias = c == b'@' && at_line_start && {
            let rest = &src[i + 1..];
            let name_len = rest.find(|ch: char| !(ch.is_alphanumeric() || ch == '_' || ch == '-')).unwrap_or(rest.len());
            name_len > 0 && rest[name_len..].trim_start().starts_with('=')
        };
        if looks_like_alias {
            let start = i;
            while i < n && b[i] != b'\n' {
                i += 1;
            }
            let line = &src[start..i];
            if let Some(eq) = line.find('=') {
                let name = line[1..eq].trim().to_string();
                let value = line[eq + 1..].trim().to_string();
                toks.push(Tok::Alias { name, value, span: Span::new(start, i) });
            } else {
                diags.push(Diagnostic { span: Span::new(start, i), message: "alias needs `@name = expression`".into(), severity: Severity::Error });
            }
            continue;
        }
        at_line_start = false;
        match c {
            b'{' => {
                toks.push(Tok::LBrace(Span::new(i, i + 1)));
                i += 1;
            }
            b'}' => {
                toks.push(Tok::RBrace(Span::new(i, i + 1)));
                i += 1;
            }
            b'|' if i + 1 >= n || b[i + 1].is_ascii_whitespace() => {
                toks.push(Tok::Pipe(Span::new(i, i + 1)));
                i += 1;
            }
            b'"' => {
                let start = i;
                i += 1;
                let mut text = String::new();
                let mut closed = false;
                while i < n {
                    let d = b[i];
                    if d == b'\\' && i + 1 < n {
                        text.push(b[i + 1] as char);
                        i += 2;
                        continue;
                    }
                    if d == b'"' {
                        i += 1;
                        closed = true;
                        break;
                    }
                    if d == b'\n' {
                        break;
                    }
                    text.push(d as char);
                    i += 1;
                }
                if !closed {
                    diags.push(Diagnostic { span: Span::new(start, i), message: "unterminated string".into(), severity: Severity::Error });
                }
                // a quoted key is followed directly by ':'
                let mut key = false;
                if i < n && b[i] == b':' && (i + 1 >= n || b[i + 1] != b':') {
                    key = true;
                    i += 1;
                }
                toks.push(Tok::Word { text, key, quoted: true, span: Span::new(start, i) });
            }
            _ => {
                let start = i;
                let mut depth = 0usize;
                let mut in_str = false;
                while i < n {
                    let d = b[i];
                    if in_str {
                        if d == b'\\' {
                            i += 2;
                            continue;
                        }
                        if d == b'"' {
                            in_str = false;
                        }
                        i += 1;
                        continue;
                    }
                    if depth > 0 {
                        if d == b'"' {
                            in_str = true;
                        } else if d == b'(' {
                            depth += 1;
                        } else if d == b')' {
                            depth -= 1;
                        } else if d == b'\n' {
                            break;
                        }
                        i += 1;
                        continue;
                    }
                    if d == b'(' && i > start {
                        depth = 1;
                        i += 1;
                        continue;
                    }
                    if d.is_ascii_whitespace() || d == b'{' || d == b'}' {
                        break;
                    }
                    i += 1;
                }
                if depth > 0 {
                    diags.push(Diagnostic { span: Span::new(start, i), message: "unclosed `(` in arguments".into(), severity: Severity::Error });
                }
                let mut text = &src[start..i];
                let mut key = false;
                if text.len() > 1 && text.ends_with(':') && !text.ends_with("::") {
                    key = true;
                    text = &text[..text.len() - 1];
                } else if text == ":" {
                    // a bare colon after a quoted key was consumed above; a bare colon here is a stray
                    diags.push(Diagnostic { span: Span::new(start, i), message: "stray `:`; the key `:` must be quoted as \":\"".into(), severity: Severity::Error });
                }
                toks.push(Tok::Word { text: text.to_string(), key, quoted: false, span: Span::new(start, i) });
            }
        }
    }
    toks
}

pub struct Parser<'a> {
    src: &'a str,
    toks: Vec<Tok>,
    pos: usize,
    pub diags: Vec<Diagnostic>,
}

pub fn parse(src: &str) -> (Vec<Item>, Vec<Diagnostic>) {
    let mut diags = Vec::new();
    let toks = tokenize(src, &mut diags);
    let mut p = Parser { src, toks, pos: 0, diags };
    let items = p.items(None);
    (items, p.diags)
}

impl<'a> Parser<'a> {
    fn peek(&self) -> Option<&Tok> {
        self.toks.get(self.pos)
    }

    fn err(&mut self, span: Span, msg: impl Into<String>) {
        self.diags.push(Diagnostic { span, message: msg.into(), severity: Severity::Error });
    }

    /// Parse items until `}` (when `close` is Some) or end of input.
    fn items(&mut self, close: Option<Span>) -> Vec<Item> {
        let mut items = Vec::new();
        loop {
            match self.peek().cloned() {
                None => {
                    if let Some(open) = close {
                        self.err(open, "unclosed `{`");
                    }
                    return items;
                }
                Some(Tok::Newline(_)) => {
                    self.pos += 1;
                }
                Some(Tok::RBrace(sp)) => {
                    self.pos += 1;
                    if close.is_none() {
                        self.err(sp, "unexpected `}`");
                        continue;
                    }
                    return items;
                }
                Some(Tok::Alias { name, value, span }) => {
                    self.pos += 1;
                    items.push(Item::Alias { name, value, span });
                }
                Some(Tok::LBrace(sp)) => {
                    self.pos += 1;
                    self.err(sp, "`{` needs a header: a context expression or `keys:`");
                    let inner = self.items(Some(sp));
                    items.extend(inner);
                }
                Some(Tok::Pipe(sp)) => {
                    self.pos += 1;
                    self.err(sp, "unexpected `|`");
                }
                Some(Tok::Word { .. }) => {
                    if let Some(item) = self.line_item() {
                        items.push(item);
                    }
                }
            }
        }
    }

    /// Looks at the rest of the current line to decide: context block, prefix block, or binding.
    fn line_item(&mut self) -> Option<Item> {
        // scan to end of line for a `{` that is not preceded by a completed binding
        let start = self.pos;
        let mut j = start;
        let mut first_key: Option<usize> = None;
        let mut lbrace: Option<usize> = None;
        while let Some(t) = self.toks.get(j) {
            match t {
                Tok::Newline(_) | Tok::RBrace(_) | Tok::Alias { .. } => break,
                Tok::LBrace(_) => {
                    lbrace = Some(j);
                    break;
                }
                Tok::Word { key: true, .. } if first_key.is_none() => first_key = Some(j),
                _ => {}
            }
            j += 1;
        }
        match (lbrace, first_key) {
            // `HEADER {` with no key token before it: a context block
            (Some(lb), None) => {
                let hs = self.toks[start].span().start;
                let he = self.toks[lb - 1].span().end;
                let header = self.src[hs..he].trim().to_string();
                let open = self.toks[lb].span();
                self.pos = lb + 1;
                let inner = self.items(Some(open));
                let end = self.toks.get(self.pos - 1).map(|t| t.span().end).unwrap_or(he);
                Some(Item::Block(Block { header: Header::Context(header), header_span: Span::new(hs, he), items: inner, span: Span::new(hs, end) }))
            }
            // `keys: {` — the key token is the last word before the brace
            (Some(lb), Some(k)) if k == lb - 1 => {
                let keys = self.key_tokens(start, k);
                let hs = self.toks[start].span().start;
                let he = self.toks[k].span().end;
                let open = self.toks[lb].span();
                self.pos = lb + 1;
                let inner = self.items(Some(open));
                let end = self.toks.get(self.pos - 1).map(|t| t.span().end).unwrap_or(he);
                Some(Item::Block(Block { header: Header::Prefix(keys), header_span: Span::new(hs, he), items: inner, span: Span::new(hs, end) }))
            }
            _ => self.binding(),
        }
    }

    /// Collect the key tokens from `from` up to and including the key token at `key_idx`.
    fn key_tokens(&mut self, from: usize, key_idx: usize) -> Vec<KeyTok> {
        let mut keys = Vec::new();
        for i in from..=key_idx {
            match &self.toks[i] {
                Tok::Word { text, span, .. } => keys.push(KeyTok { text: text.clone(), span: *span }),
                Tok::Pipe(sp) => {
                    let sp = *sp;
                    self.err(sp, "`|` is not allowed in a prefix header");
                }
                _ => {}
            }
        }
        keys
    }

    fn binding(&mut self) -> Option<Item> {
        // keys: words (with pipes) up to the key token
        let start_span = self.toks[self.pos].span();
        let mut groups: Vec<Vec<KeyTok>> = vec![Vec::new()];
        let mut found_key = false;
        while let Some(t) = self.peek().cloned() {
            match t {
                Tok::Word { text, key, span, .. } => {
                    self.pos += 1;
                    groups.last_mut().unwrap().push(KeyTok { text, span });
                    if key {
                        found_key = true;
                        break;
                    }
                }
                Tok::Pipe(_) => {
                    self.pos += 1;
                    groups.push(Vec::new());
                }
                _ => break,
            }
        }
        let keys_end = self.toks.get(self.pos - 1).map(|t| t.span().end).unwrap_or(start_span.end);
        let keys_span = Span::new(start_span.start, keys_end);
        if !found_key {
            self.err(keys_span, "expected `keys: action` (a key token ends with `:`) or a `header {` block");
            // skip to end of line
            while let Some(t) = self.peek() {
                if matches!(t, Tok::Newline(_) | Tok::RBrace(_) | Tok::LBrace(_)) {
                    break;
                }
                self.pos += 1;
            }
            return None;
        }
        if groups.iter().any(|g| g.is_empty()) {
            self.err(keys_span, "empty key group around `|`");
        }
        // actions: until newline, brace, next key token
        let mut values: Vec<Vec<ActionTok>> = vec![Vec::new()];
        let vstart = self.peek().map(|t| t.span().start).unwrap_or(keys_end);
        let mut vend = vstart;
        while let Some(t) = self.peek().cloned() {
            match t {
                Tok::Word { key: true, .. } | Tok::Newline(_) | Tok::RBrace(_) | Tok::LBrace(_) | Tok::Alias { .. } => break,
                Tok::Pipe(_) => {
                    self.pos += 1;
                    values.push(Vec::new());
                }
                Tok::Word { text, span, quoted, .. } => {
                    self.pos += 1;
                    vend = span.end;
                    if !quoted && text == "null" {
                        values.last_mut().unwrap().push(ActionTok::Null { span });
                    } else {
                        let tok = self.action(&text, span);
                        values.last_mut().unwrap().push(tok);
                    }
                }
            }
        }
        let values_span = Span::new(vstart, vend.max(vstart));
        if values.iter().all(|v| v.is_empty()) {
            self.err(keys_span, "missing action after `:` (use `null` to unbind)");
        }
        Some(Item::Binding(Binding { keys: groups, values, span: Span::new(start_span.start, vend.max(keys_end)), keys_span, values_span }))
    }

    fn action(&mut self, text: &str, span: Span) -> ActionTok {
        if let Some(open) = text.find('(') {
            let name = text[..open].to_string();
            let raw_all = &text[open..];
            if !raw_all.ends_with(')') {
                self.err(span, "arguments must end with `)`");
            }
            let raw = raw_all.trim_start_matches('(').trim_end_matches(')').to_string();
            let args_span = Span::new(span.start + open, span.end);
            let value = match parse_args(&raw) {
                Ok(v) => v,
                Err(e) => {
                    self.err(args_span, format!("bad arguments: {e}"));
                    Value::Null
                }
            };
            ActionTok::Action { name_span: Span::new(span.start, span.start + open), name, args: Some(Args { raw, span: args_span, value }), span }
        } else {
            ActionTok::Action { name: text.to_string(), name_span: span, args: None, span }
        }
    }
}

/// Split on top-level commas (outside quotes, brackets, braces, parens).
pub fn split_top_commas(s: &str) -> Vec<String> {
    let mut out = Vec::new();
    let mut cur = String::new();
    let mut depth = 0i32;
    let mut in_str = false;
    let mut chars = s.chars().peekable();
    while let Some(c) = chars.next() {
        if in_str {
            cur.push(c);
            if c == '\\' {
                if let Some(d) = chars.next() {
                    cur.push(d);
                }
            } else if c == '"' {
                in_str = false;
            }
            continue;
        }
        match c {
            '"' => {
                in_str = true;
                cur.push(c);
            }
            '[' | '{' | '(' => {
                depth += 1;
                cur.push(c);
            }
            ']' | '}' | ')' => {
                depth -= 1;
                cur.push(c);
            }
            ',' if depth == 0 => {
                out.push(cur.trim().to_string());
                cur.clear();
            }
            _ => cur.push(c),
        }
    }
    if !cur.trim().is_empty() {
        out.push(cur.trim().to_string());
    }
    out
}

fn is_ident(s: &str) -> bool {
    !s.is_empty() && s.chars().all(|c| c.is_alphanumeric() || c == '_')
}

/// `a, b=false, c="x", d=1, e={"k":1}` → object; a single JSON scalar/array → positional; empty → `{}`.
pub fn parse_args(raw: &str) -> Result<Value, String> {
    let raw = raw.trim();
    if raw.is_empty() {
        return Ok(Value::Object(Map::new()));
    }
    let parts = split_top_commas(raw);
    // positional: exactly one part that is not `name=value` and not a bare flag name
    if parts.len() == 1 {
        let p = &parts[0];
        let flag_name = is_ident(p) && p.chars().next().map(|c| c.is_alphabetic() || c == '_').unwrap_or(false) && !matches!(p.as_str(), "true" | "false" | "null");
        if !flag_name && !p.contains('=') {
            if let Ok(v) = serde_json::from_str::<Value>(p) {
                return Ok(v);
            }
            return Ok(Value::String(p.to_string()));
        }
    }
    let mut map = Map::new();
    for p in parts {
        if let Some(eq) = p.find('=') {
            let k = p[..eq].trim();
            let v = p[eq + 1..].trim();
            if !is_ident(k) {
                return Err(format!("`{k}` is not an argument name"));
            }
            let val = match serde_json::from_str::<Value>(v) {
                Ok(x) => x,
                Err(_) => Value::String(v.to_string()),
            };
            map.insert(k.to_string(), val);
        } else if is_ident(&p) {
            map.insert(p.clone(), Value::Bool(true));
        } else {
            return Err(format!("`{p}` is neither `name`, `name=value` nor a single JSON value"));
        }
    }
    Ok(Value::Object(map))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_bindings_and_blocks() {
        let src = "cmd-q: zed::Quit\nEditor {\n  a b: x::A x::B\n  g: { d: e::D }\n  mode == full { enter: e::Newline(a, b=false) }\n}\n";
        let (items, diags) = parse(src);
        assert!(diags.is_empty(), "{diags:?}");
        assert_eq!(items.len(), 2);
        match &items[1] {
            Item::Block(b) => {
                assert!(matches!(b.header, Header::Context(ref h) if h == "Editor"));
                assert_eq!(b.items.len(), 3);
            }
            _ => panic!(),
        }
    }

    #[test]
    fn args() {
        assert_eq!(parse_args("").unwrap(), serde_json::json!({}));
        assert_eq!(parse_args("0").unwrap(), serde_json::json!(0));
        assert_eq!(parse_args("\"ctrl-u\"").unwrap(), serde_json::json!("ctrl-u"));
        assert_eq!(parse_args("a, b=false").unwrap(), serde_json::json!({"a": true, "b": false}));
        assert_eq!(parse_args("severity={\"min\": \"hint\", \"max\": \"error\"}").unwrap(), serde_json::json!({"severity": {"min": "hint", "max": "error"}}));
        assert_eq!(parse_args("[\"ctrl-a\", \"\\u0001\"]").unwrap(), serde_json::json!(["ctrl-a", "\u{1}"]));
        assert_eq!(parse_args("save_intent=\"skip\"").unwrap(), serde_json::json!({"save_intent": "skip"}));
    }

    #[test]
    fn quoted_keys_and_pipes() {
        let src = "\":\": c::P\n\"cmd-\\\"\": e::X\nleft | shift-left: e::L | e::SL\n";
        let (items, diags) = parse(src);
        assert!(diags.is_empty(), "{diags:?}");
        assert_eq!(items.len(), 3);
        if let Item::Binding(b) = &items[2] {
            assert_eq!(b.keys.len(), 2);
            assert_eq!(b.values.len(), 2);
        } else {
            panic!()
        }
    }
}
