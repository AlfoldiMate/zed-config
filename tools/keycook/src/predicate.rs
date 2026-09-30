//! Port of Zed's `KeyBindingContextPredicate` (crates/gpui/src/keymap/context.rs).
//! Same grammar, same precedence, same evaluation, so checks here agree with Zed.

use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Predicate {
    Identifier(String),
    Equal(String, String),
    NotEqual(String, String),
    Descendant(Box<Predicate>, Box<Predicate>),
    Not(Box<Predicate>),
    And(Box<Predicate>, Box<Predicate>),
    Or(Box<Predicate>, Box<Predicate>),
}

const PRECEDENCE_CHILD: u32 = 1;
const PRECEDENCE_OR: u32 = 2;
const PRECEDENCE_AND: u32 = 3;
const PRECEDENCE_EQ: u32 = 4;
const PRECEDENCE_NOT: u32 = 5;

fn is_identifier_char(c: char) -> bool {
    c.is_alphanumeric() || c == '_' || c == '-'
}

fn is_vim_operator_char(c: char) -> bool {
    matches!(c, '>' | '<' | '~' | '"' | '?')
}

fn skip_whitespace(s: &str) -> &str {
    s.trim_start()
}

impl Predicate {
    pub fn parse(source: &str) -> Result<Predicate, String> {
        let source = skip_whitespace(source);
        let (pred, rest) = Self::parse_expr(source, 0)?;
        if let Some(next) = rest.chars().next() {
            return Err(format!("unexpected character '{next}'"));
        }
        Ok(pred)
    }

    fn parse_expr(mut source: &str, min_precedence: u32) -> Result<(Predicate, &str), String> {
        let (mut predicate, rest) = Self::parse_primary(source)?;
        source = rest;
        'parse: loop {
            for (op, prec, kind) in [(">", PRECEDENCE_CHILD, 0u8), ("&&", PRECEDENCE_AND, 1), ("||", PRECEDENCE_OR, 2), ("==", PRECEDENCE_EQ, 3), ("!=", PRECEDENCE_EQ, 4)] {
                if source.starts_with(op) && prec >= min_precedence {
                    source = skip_whitespace(&source[op.len()..]);
                    let (right, rest) = Self::parse_expr(source, prec + 1)?;
                    predicate = match kind {
                        0 => Predicate::Descendant(Box::new(predicate), Box::new(right)),
                        1 => Predicate::And(Box::new(predicate), Box::new(right)),
                        2 => Predicate::Or(Box::new(predicate), Box::new(right)),
                        3 | 4 => match (predicate, right) {
                            (Predicate::Identifier(l), Predicate::Identifier(r)) => {
                                if kind == 3 {
                                    Predicate::Equal(l, r)
                                } else {
                                    Predicate::NotEqual(l, r)
                                }
                            }
                            _ => return Err(format!("operands of {op} must be identifiers")),
                        },
                        _ => unreachable!(),
                    };
                    source = rest;
                    continue 'parse;
                }
            }
            break;
        }
        Ok((predicate, source))
    }

    fn parse_primary(mut source: &str) -> Result<(Predicate, &str), String> {
        let next = source.chars().next().ok_or_else(|| "unexpected end".to_string())?;
        match next {
            '(' => {
                source = skip_whitespace(&source[1..]);
                let (predicate, rest) = Self::parse_expr(source, 0)?;
                let stripped = rest.strip_prefix(')').ok_or_else(|| "expected a ')'".to_string())?;
                Ok((predicate, skip_whitespace(stripped)))
            }
            '!' => {
                let source = skip_whitespace(&source[1..]);
                let (predicate, source) = Self::parse_expr(source, PRECEDENCE_NOT)?;
                Ok((Predicate::Not(Box::new(predicate)), source))
            }
            c if is_identifier_char(c) => {
                let len = source.find(|c: char| !is_identifier_char(c) && !is_vim_operator_char(c)).unwrap_or(source.len());
                let (identifier, rest) = source.split_at(len);
                Ok((Predicate::Identifier(identifier.to_string()), skip_whitespace(rest)))
            }
            c if is_vim_operator_char(c) => {
                let (operator, rest) = source.split_at(1);
                Ok((Predicate::Identifier(operator.to_string()), skip_whitespace(rest)))
            }
            _ => Err(format!("unexpected character '{next}'")),
        }
    }

    /// Identifiers that must be present as flags or attribute keys on ONE node for this
    /// predicate to match there (the top-level `&&` chain, not under `!`, `||` or `>`).
    pub fn required_identifiers(&self) -> Vec<String> {
        let mut out = Vec::new();
        fn walk(p: &Predicate, out: &mut Vec<String>) {
            match p {
                Predicate::Identifier(n) => out.push(n.clone()),
                Predicate::Equal(k, _) => out.push(k.clone()),
                Predicate::And(l, r) => {
                    walk(l, out);
                    walk(r, out);
                }
                Predicate::Descendant(_, child) => walk(child, out),
                _ => {}
            }
        }
        walk(self, &mut out);
        out
    }

    /// Every identifier and attribute mentioned anywhere (for catalogue validation).
    pub fn mentioned(&self) -> Vec<(String, Option<String>)> {
        let mut out = Vec::new();
        fn walk(p: &Predicate, out: &mut Vec<(String, Option<String>)>) {
            match p {
                Predicate::Identifier(n) => out.push((n.clone(), None)),
                Predicate::Equal(k, v) | Predicate::NotEqual(k, v) => out.push((k.clone(), Some(v.clone()))),
                Predicate::Descendant(l, r) | Predicate::And(l, r) | Predicate::Or(l, r) => {
                    walk(l, out);
                    walk(r, out);
                }
                Predicate::Not(x) => walk(x, out),
            }
        }
        walk(self, &mut out);
        out
    }

    pub fn eval(&self, contexts: &[KeyContext]) -> bool {
        self.eval_inner(contexts, contexts)
    }

    pub fn depth_of(&self, contexts: &[KeyContext]) -> Option<usize> {
        for depth in (0..=contexts.len()).rev() {
            if self.eval_inner(&contexts[..depth], contexts) {
                return Some(depth);
            }
        }
        None
    }

    fn eval_inner(&self, contexts: &[KeyContext], all: &[KeyContext]) -> bool {
        let Some(context) = contexts.last() else { return false };
        match self {
            Predicate::Identifier(name) => context.contains(name),
            Predicate::Equal(l, r) => context.get(l).map(|v| v == r).unwrap_or(false),
            Predicate::NotEqual(l, r) => context.get(l).map(|v| v != r).unwrap_or(true),
            Predicate::Not(p) => {
                for i in 0..all.len() {
                    if p.eval_inner(&all[..=i], all) {
                        return false;
                    }
                }
                true
            }
            Predicate::Descendant(parent, child) => {
                for i in 0..contexts.len().saturating_sub(1) {
                    if parent.eval_inner(&contexts[..=i], all) {
                        let sub = &contexts[i + 1..];
                        return child.eval_inner(sub, sub);
                    }
                }
                false
            }
            Predicate::And(l, r) => l.eval_inner(contexts, all) && r.eval_inner(contexts, all),
            Predicate::Or(l, r) => l.eval_inner(contexts, all) || r.eval_inner(contexts, all),
        }
    }
}

/// One node of a focus chain: flags plus `key=value` attributes.
#[derive(Debug, Clone, Default)]
pub struct KeyContext {
    pub flags: HashSet<String>,
    pub attrs: HashMap<String, String>,
}

impl KeyContext {
    pub fn parse(s: &str) -> KeyContext {
        let mut c = KeyContext::default();
        for part in s.split_whitespace() {
            if let Some((k, v)) = part.split_once('=') {
                c.attrs.insert(k.to_string(), v.to_string());
            } else {
                c.flags.insert(part.to_string());
            }
        }
        c
    }
    pub fn contains(&self, k: &str) -> bool {
        self.flags.contains(k) || self.attrs.contains_key(k)
    }
    pub fn get(&self, k: &str) -> Option<&String> {
        self.attrs.get(k)
    }
}

/// Split `s` on a top-level operator (outside parentheses).
pub fn split_top(s: &str, op: &str) -> Vec<String> {
    let mut parts = Vec::new();
    let mut depth = 0i32;
    let mut cur = String::new();
    let bytes: Vec<char> = s.chars().collect();
    let opc: Vec<char> = op.chars().collect();
    let mut i = 0;
    while i < bytes.len() {
        let c = bytes[i];
        if c == '(' {
            depth += 1;
        } else if c == ')' {
            depth -= 1;
        }
        if depth == 0 && bytes[i..].starts_with(&opc) {
            parts.push(cur.trim().to_string());
            cur.clear();
            i += opc.len();
            continue;
        }
        cur.push(c);
        i += 1;
    }
    parts.push(cur.trim().to_string());
    parts
}

#[cfg(test)]
mod tests {
    use super::*;

    fn chain(parts: &[&str]) -> Vec<KeyContext> {
        parts.iter().map(|p| KeyContext::parse(p)).collect()
    }

    #[test]
    fn precedence_and_eval() {
        let p = Predicate::parse("VimControl && !menu || !Editor && !Terminal").unwrap();
        assert!(matches!(p, Predicate::Or(..)));
        let buf = chain(&["Workspace os=macos", "Pane", "Editor VimControl vim_mode=normal"]);
        assert_eq!(p.depth_of(&buf), Some(3));
        let panel = chain(&["Workspace", "Dock", "ProjectPanel menu not_editing"]);
        assert_eq!(p.depth_of(&panel), Some(3));
        let not_editor = Predicate::parse("!Editor").unwrap();
        assert_eq!(not_editor.depth_of(&buf), None);
        let desc = Predicate::parse("Workspace > Editor").unwrap();
        assert_eq!(desc.depth_of(&buf), Some(3));
        let bad = Predicate::parse("Workspace && vim_mode == normal").unwrap();
        assert_eq!(bad.depth_of(&buf), None);
        assert!(Predicate::parse("vim_operator == >").is_ok());
        assert!(Predicate::parse("a &&").is_err());
    }
}
