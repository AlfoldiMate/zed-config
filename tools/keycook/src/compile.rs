//! AST → ordered Zed keymap sections, with checks.

use crate::ast::*;
use crate::catalog::Catalog;
use crate::keys;
use crate::manifest::Manifest;
use crate::parser::{self, Diagnostic, Severity};
use crate::predicate::{split_top, Predicate};
use regex::Regex;
use serde_json::Value;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Bound {
    pub key: String,
    pub value: Value,
    pub span: Span,
    /// true when the compiler inserted it (auto null)
    pub synthetic: bool,
    /// true when it came from a spliced `@body`; a later binding in the same block overrides it silently
    pub from_body: bool,
}

#[derive(Debug, Clone)]
pub struct Section {
    /// Zed predicate ("" for no context)
    pub context: String,
    /// Header path for the comment line, e.g. `Workspace > Pane > Editor / mode == full`
    pub path: String,
    pub bindings: Vec<Bound>,
    pub span: Span,
    /// indices of enclosing sections, nearest last
    pub ancestors: Vec<usize>,
}

impl Section {
    pub fn get(&self, key: &str) -> Option<&Bound> {
        self.bindings.iter().rev().find(|b| b.key == key)
    }
}

#[derive(Debug, Default)]
pub struct Output {
    pub sections: Vec<Section>,
    pub diags: Vec<Diagnostic>,
    pub items: Vec<Item>,
    pub aliases: Vec<(String, String)>,
    pub bodies: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct Options {
    pub auto_null: bool,
    /// Emit sections ordered by node depth (number of `>` segments, root first), stable otherwise,
    /// so a generic `Editor` block never shadows a `… > Editor` refinement written above it.
    pub sort: bool,
}

impl Default for Options {
    fn default() -> Self {
        Options { auto_null: true, sort: true }
    }
}

/// Number of node segments a predicate pins: `A > B > C` is 3, `Editor && x` is 1, "" is 0.
pub fn segments(ctx: &str) -> usize {
    if ctx.trim().is_empty() {
        return 0;
    }
    split_top(ctx, ">").len()
}

struct Ctx<'a> {
    manifest: &'a Manifest,
    catalog: &'a Catalog,
    opts: &'a Options,
    aliases: Vec<(String, String)>,
    bodies: HashMap<String, Vec<Item>>,
    sections: Vec<Section>,
    diags: Vec<Diagnostic>,
    use_depth: usize,
    in_body: usize,
}

#[derive(Clone)]
struct Frame {
    pred: Option<String>,
    /// predicate of the block enclosing this one (None at root or at top level)
    outer_pred: Option<String>,
    /// this block's own header, alias-expanded and normalised, with its leading `>` if any
    header: Option<String>,
    path: String,
    section: usize,
    prefix: Vec<String>,
    ancestors: Vec<usize>,
}

pub fn compile(src: &str, manifest: &Manifest, catalog: &Catalog, opts: &Options) -> Output {
    let (items, diags) = parser::parse(src);
    let mut cx = Ctx { manifest, catalog, opts, aliases: Vec::new(), bodies: HashMap::new(), sections: Vec::new(), diags, use_depth: 0, in_body: 0 };
    collect_aliases(&items, &mut cx.aliases);
    collect_bodies(&items, &mut cx.bodies, &mut cx.diags);
    // root section for bindings without any context
    cx.sections.push(Section { context: String::new(), path: "(no context)".into(), bindings: vec![], span: Span::new(0, 0), ancestors: vec![] });
    let root = Frame { pred: None, outer_pred: None, header: None, path: String::new(), section: 0, prefix: vec![], ancestors: vec![] };
    cx.walk(&items, &root);
    if opts.sort {
        cx.sort_by_specificity();
    }
    cx.post_checks();
    let aliases = cx.aliases.clone();
    let mut bodies: Vec<String> = cx.bodies.keys().cloned().collect();
    bodies.sort();
    Output { sections: cx.sections, diags: cx.diags, items, aliases, bodies }
}

fn collect_bodies(items: &[Item], out: &mut HashMap<String, Vec<Item>>, diags: &mut Vec<Diagnostic>) {
    for it in items {
        match it {
            Item::Body { name, items, span } => {
                if out.insert(name.clone(), items.clone()).is_some() {
                    diag(diags, *span, Severity::Error, format!("body `@{name}` is defined twice"));
                }
                collect_bodies(items, out, diags);
            }
            Item::Block(b) => collect_bodies(&b.items, out, diags),
            _ => {}
        }
    }
}

fn collect_aliases(items: &[Item], out: &mut Vec<(String, String)>) {
    for it in items {
        match it {
            Item::Alias { name, value, .. } => out.push((name.clone(), value.clone())),
            Item::Block(b) => collect_aliases(&b.items, out),
            _ => {}
        }
    }
}

fn diag(diags: &mut Vec<Diagnostic>, span: Span, severity: Severity, message: impl Into<String>) {
    diags.push(Diagnostic { span, message: message.into(), severity });
}

fn has_top_level(s: &str, op: &str) -> bool {
    split_top(s, op).len() > 1
}

impl<'a> Ctx<'a> {
    fn walk(&mut self, items: &[Item], frame: &Frame) {
        for it in items {
            match it {
                Item::Alias { .. } | Item::Body { .. } => {}
                Item::Use { name, span } => {
                    let Some(body) = self.bodies.get(name).cloned() else {
                        if self.aliases.iter().any(|(n, _)| n == name) {
                            diag(&mut self.diags, *span, Severity::Error, format!("`@{name}` is a context alias, not a body; use it as a header: `@{name} {{ … }}`"));
                        } else {
                            diag(&mut self.diags, *span, Severity::Error, format!("unknown body `@{name}`"));
                        }
                        continue;
                    };
                    if self.use_depth > 16 {
                        diag(&mut self.diags, *span, Severity::Error, format!("`@{name}` splices itself recursively"));
                        continue;
                    }
                    self.use_depth += 1;
                    self.in_body += 1;
                    self.walk(&body, frame);
                    self.in_body -= 1;
                    self.use_depth -= 1;
                }
                Item::Binding(b) => self.binding(b, frame),
                Item::Block(b) => match &b.header {
                    Header::Prefix(keys) => {
                        let mut f = frame.clone();
                        for k in keys {
                            if k.text.contains(' ') || keys::expand_range(&k.text).is_some() {
                                diag(&mut self.diags, k.span, Severity::Error, "a prefix header is one keystroke per token; quote sequences in bindings instead");
                            }
                            if let Some(e) = keys::check_keystroke(&k.text) {
                                diag(&mut self.diags, k.span, Severity::Error, e);
                            }
                            f.prefix.push(k.text.clone());
                        }
                        self.walk(&b.items, &f);
                    }
                    Header::Context(h) => {
                        if !frame.prefix.is_empty() {
                            diag(&mut self.diags, b.header_span, Severity::Error, "a context block cannot sit inside a prefix block");
                        }
                        // `| X { … }` widens the enclosing block's header: parent-of-parent && (header || X)
                        let (outer, header_text) = if let Some(alt) = h.trim().strip_prefix('|') {
                            let Some(own) = frame.header.as_deref() else {
                                diag(&mut self.diags, b.header_span, Severity::Error, "`| X` needs an enclosing context block whose header it widens");
                                continue;
                            };
                            let alt = self.normalize_header(alt, b.header_span);
                            let widened = match own.trim().strip_prefix('>') {
                                Some(core) => format!("> ({} || {})", core.trim(), alt.trim()),
                                None => format!("({} || {})", own.trim(), alt.trim()),
                            };
                            (frame.outer_pred.clone(), widened)
                        } else {
                            (frame.pred.clone(), self.normalize_header(h, b.header_span))
                        };
                        let pred = match self.compose(outer.as_deref(), &header_text, b.header_span) {
                            Some(p) => p,
                            None => continue,
                        };
                        let path = if frame.path.is_empty() { h.clone() } else { format!("{} / {}", frame.path, h) };
                        let mut ancestors = frame.ancestors.clone();
                        ancestors.push(frame.section);
                        let idx = self.sections.len();
                        self.sections.push(Section { context: pred.clone(), path: path.clone(), bindings: vec![], span: b.span, ancestors: ancestors.clone() });
                        let f = Frame { pred: Some(pred), outer_pred: outer, header: Some(header_text), path, section: idx, prefix: vec![], ancestors };
                        self.walk(&b.items, &f);
                    }
                },
            }
        }
    }

    /// Alias expansion plus `&`/`|` → `&&`/`||`.
    fn normalize_header(&mut self, header: &str, span: Span) -> String {
        let mut h = self.expand_aliases(header, span);
        h = h.replace("&&", "\u{1}").replace("||", "\u{2}");
        h = h.replace('&', "&&").replace('|', "||");
        h = h.replace('\u{1}', "&&").replace('\u{2}', "||");
        h.trim().to_string()
    }

    /// Build the Zed predicate for a normalised header under `parent`.
    fn compose(&mut self, parent: Option<&str>, header: &str, span: Span) -> Option<String> {
        let h = header.to_string();
        let pred = if let Some(rest) = h.trim().strip_prefix('>') {
            let rest = rest.trim();
            let Some(p) = parent else {
                diag(&mut self.diags, span, Severity::Error, "`>` needs an enclosing context block to be the ancestor");
                return None;
            };
            let p = if has_top_level(p, "||") { format!("({p})") } else { p.to_string() };
            let r = if has_top_level(rest, "||") { format!("({rest})") } else { rest.to_string() };
            format!("{p} > {r}")
        } else {
            match parent {
                None => h.trim().to_string(),
                Some(p) => {
                    let p = if has_top_level(p, "||") { format!("({p})") } else { p.to_string() };
                    let hh = if has_top_level(&h, "||") { format!("({})", h.trim()) } else { h.trim().to_string() };
                    format!("{p} && {hh}")
                }
            }
        };
        match Predicate::parse(&pred) {
            Ok(p) => {
                self.check_predicate(&p, &pred, span);
                Some(pred)
            }
            Err(e) => {
                diag(&mut self.diags, span, Severity::Error, format!("context does not parse: {e} (in `{pred}`)"));
                None
            }
        }
    }

    fn expand_aliases(&mut self, header: &str, span: Span) -> String {
        let re = Regex::new(r"@([A-Za-z_][A-Za-z0-9_-]*)").unwrap();
        let mut out = String::new();
        let mut last = 0;
        for m in re.captures_iter(header) {
            let whole = m.get(0).unwrap();
            let name = &m[1];
            out.push_str(&header[last..whole.start()]);
            match self.aliases.iter().find(|(n, _)| n == name) {
                Some((_, v)) => {
                    let needs_parens = v.contains('|') || v.contains('&') || v.contains('>');
                    if needs_parens {
                        out.push('(');
                        out.push_str(v);
                        out.push(')');
                    } else {
                        out.push_str(v);
                    }
                }
                None => {
                    diag(&mut self.diags, span, Severity::Error, format!("unknown alias `@{name}`"));
                    out.push_str(name);
                }
            }
            last = whole.end();
        }
        out.push_str(&header[last..]);
        out
    }

    fn check_predicate(&mut self, p: &Predicate, text: &str, span: Span) {
        // two different node names required on one node can never match
        let req = p.required_identifiers();
        let nodes: Vec<&String> = req.iter().filter(|id| self.catalog.is_node(id)).collect();
        let mut distinct: Vec<&String> = Vec::new();
        for n in nodes {
            if !distinct.contains(&n) {
                distinct.push(n);
            }
        }
        if distinct.len() >= 2 {
            diag(&mut self.diags, span, Severity::Error, format!("`{}` and `{}` are different nodes, so `{text}` never matches; nest with `> {}` instead", distinct[0], distinct[1], distinct[1]));
        }
        for (id, val) in p.mentioned() {
            if !self.catalog.known(&id) {
                diag(&mut self.diags, span, Severity::Hint, format!("`{id}` is not a known context identifier in Zed 1.21"));
            } else if let (Some(v), Some(vals)) = (val, self.catalog.attrs.get(&id)) {
                if !vals.is_empty() && !vals.contains(&v) {
                    diag(&mut self.diags, span, Severity::Warning, format!("`{id} == {v}`: known values are {}", vals.iter().map(|x| format!("`{x}`")).collect::<Vec<_>>().join(", ")));
                }
            }
        }
    }

    fn binding(&mut self, b: &Binding, frame: &Frame) {
        if b.keys.len() != 1 || b.values.len() != 1 {
            diag(&mut self.diags, b.span, Severity::Error, "`|` is not allowed in a binding: write one line per action");
            return;
        }
        if b.values[0].len() != 1 {
            diag(&mut self.diags, b.values_span, Severity::Error, format!("{} actions on one line: a binding is `keys: action`; write one line per action", b.values[0].len()));
            return;
        }
        self.bind_group(&b.keys[0], &b.values[0], frame, b.span);
    }

    fn bind_group(&mut self, keys: &[KeyTok], actions: &[ActionTok], frame: &Frame, _span: Span) {
        // expand ranges
        let mut expanded: Vec<(String, Option<String>, Span)> = Vec::new();
        let mut any_range = false;
        for k in keys {
            match keys::expand_range(&k.text) {
                Some(Ok(list)) => {
                    any_range = true;
                    for (key, val) in list {
                        expanded.push((key, Some(val), k.span));
                    }
                }
                Some(Err(e)) => diag(&mut self.diags, k.span, Severity::Error, e),
                None => expanded.push((k.text.clone(), None, k.span)),
            }
        }
        for (key, _, sp) in &expanded {
            for stroke in key.split(' ') {
                if let Some(e) = keys::check_keystroke(stroke) {
                    diag(&mut self.diags, *sp, Severity::Error, e);
                }
            }
        }
        let _ = any_range;
        for (key, range_val, kspan) in expanded.iter() {
            let act = &actions[0];
            let Some(value) = self.action_value(act, range_val.as_deref()) else { continue };
            let full = if frame.prefix.is_empty() { key.clone() } else { format!("{} {}", frame.prefix.join(" "), key) };
            let sec = &mut self.sections[frame.section];
            if let Some(prev) = sec.bindings.iter().position(|x| x.key == full) {
                let prev_val = sec.bindings[prev].value.clone();
                if sec.bindings[prev].from_body && self.in_body == 0 {
                    // overriding a spliced body binding is the point of splicing
                } else if prev_val == value {
                    diag(&mut self.diags, *kspan, Severity::Warning, format!("`{full}` is bound twice to the same action in this block"));
                } else {
                    diag(&mut self.diags, *kspan, Severity::Warning, format!("`{full}` rebinds a key already bound in this block; the earlier `{}` is dead", short(&prev_val)));
                }
                sec.bindings.remove(prev);
            }
            let from_body = self.in_body > 0;
            let sec = &mut self.sections[frame.section];
            sec.bindings.push(Bound { key: full, value, span: kspan.join(act.span()), synthetic: false, from_body });
        }
    }

    fn action_value(&mut self, act: &ActionTok, range_val: Option<&str>) -> Option<Value> {
        match act {
            ActionTok::Null { .. } => Some(Value::Null),
            ActionTok::Action { name, name_span, args, span } => {
                let mut name = name.clone();
                if self.manifest.get(&name).is_none() {
                    if let Some(new) = self.manifest.aliases.get(&name) {
                        diag(&mut self.diags, *name_span, Severity::Warning, format!("`{name}` is a deprecated alias of `{new}`"));
                        name = new.clone();
                    } else if !self.manifest.actions.is_empty() {
                        let hint = self.suggest(&name);
                        diag(&mut self.diags, *name_span, Severity::Error, format!("unknown action `{name}`{hint}"));
                        return None;
                    }
                }
                let arg_value = match (args, range_val) {
                    (Some(a), Some(v)) => match parser::parse_args(&keys::substitute_range(&a.raw, v)) {
                        Ok(x) => Some(x),
                        Err(e) => {
                            diag(&mut self.diags, a.span, Severity::Error, format!("bad arguments: {e}"));
                            return None;
                        }
                    },
                    (Some(a), None) => Some(a.value.clone()),
                    (None, _) => None,
                };
                if let Some(e) = self.manifest.check_args(&name, arg_value.as_ref()) {
                    diag(&mut self.diags, *span, Severity::Error, e);
                }
                if let Some(a) = self.manifest.get(&name) {
                    if let Some(d) = &a.deprecation {
                        diag(&mut self.diags, *name_span, Severity::Warning, format!("`{name}` is deprecated: {d}"));
                    }
                }
                Some(match arg_value {
                    None => Value::String(name),
                    Some(v) => Value::Array(vec![Value::String(name), v]),
                })
            }
        }
    }

    fn suggest(&self, name: &str) -> String {
        let lower = name.to_lowercase();
        let tail = lower.rsplit("::").next().unwrap_or(&lower).to_string();
        let mut cands: Vec<&String> = self.manifest.sorted_names.iter().filter(|n| n.to_lowercase().contains(&tail)).take(3).collect();
        if cands.is_empty() {
            cands = self.manifest.sorted_names.iter().filter(|n| n.to_lowercase().starts_with(&lower[..lower.len().min(6)])).take(3).collect();
        }
        if cands.is_empty() {
            String::new()
        } else {
            format!("; did you mean {}?", cands.iter().map(|c| format!("`{c}`")).collect::<Vec<_>>().join(", "))
        }
    }

    fn sort_by_specificity(&mut self) {
        let n = self.sections.len();
        let mut order: Vec<usize> = (0..n).collect();
        order.sort_by_key(|&i| segments(&self.sections[i].context));
        let mut remap = vec![0usize; n];
        for (new_i, &old_i) in order.iter().enumerate() {
            remap[old_i] = new_i;
        }
        let mut sections: Vec<Section> = order.iter().map(|&i| self.sections[i].clone()).collect();
        for s in &mut sections {
            for a in &mut s.ancestors {
                *a = remap[*a];
            }
        }
        self.sections = sections;
    }

    fn post_checks(&mut self) {
        let n = self.sections.len();
        for si in 0..n {
            // chord waits inside one section
            let keys: Vec<(String, bool, Span)> = self.sections[si].bindings.iter().map(|b| (b.key.clone(), b.value.is_null(), b.span)).collect();
            for (k, is_null, span) in &keys {
                if *is_null {
                    continue;
                }
                let toks: Vec<&str> = k.split(' ').collect();
                for i in 1..toks.len() {
                    let pre = toks[..i].join(" ");
                    if let Some(p) = keys.iter().find(|(pk, _, _)| pk == &pre) {
                        if !p.1 {
                            diag(&mut self.diags, *span, Severity::Warning, format!("`{k}` waits 1 s because `{pre}` is also bound in this block; bind `{pre}: null` or drop one"));
                        }
                    }
                }
            }
            // auto null: a chord's prefix bound (non-null) in an ancestor and not here
            let ancestors = self.sections[si].ancestors.clone();
            let mut to_add: Vec<(String, Span, String)> = Vec::new();
            for (k, is_null, span) in &keys {
                if *is_null {
                    continue;
                }
                let toks: Vec<&str> = k.split(' ').collect();
                for i in 1..toks.len() {
                    let pre = toks[..i].join(" ");
                    if self.sections[si].get(&pre).is_some() || to_add.iter().any(|(p, _, _)| p == &pre) {
                        continue;
                    }
                    for &a in ancestors.iter().rev() {
                        if let Some(b) = self.sections[a].get(&pre) {
                            if !b.value.is_null() {
                                to_add.push((pre.clone(), *span, self.sections[a].path.clone()));
                            }
                            break;
                        }
                    }
                }
            }
            for (pre, span, from) in to_add {
                if self.opts.auto_null {
                    diag(&mut self.diags, span, Severity::Info, format!("added `{pre}: null` here: `{pre}` is a leaf in `{from}` and would make this chord wait 1 s"));
                    self.sections[si].bindings.insert(0, Bound { key: pre, value: Value::Null, span, synthetic: true, from_body: false });
                } else {
                    diag(&mut self.diags, span, Severity::Warning, format!("`{pre}` is a leaf in `{from}`, so this chord waits 1 s; add `{pre}: null` here"));
                }
            }
            // redundancy: same key and value as an ancestor
            let own: Vec<(String, Value, Span)> = self.sections[si].bindings.iter().filter(|b| !b.synthetic).map(|b| (b.key.clone(), b.value.clone(), b.span)).collect();
            for (k, v, span) in own {
                let own_segments = segments(&self.sections[si].context);
                for &a in ancestors.iter().rev() {
                    if let Some(b) = self.sections[a].get(&k) {
                        // only a hint when both match at the same node: same segment count, or the
                        // root section, which matches at the leaf
                        // the root section is first in the file, so any later same-node section overrides it: never a safe removal
                        let same_node = !self.sections[a].context.is_empty() && segments(&self.sections[a].context) == own_segments;
                        if b.value == v && same_node {
                            diag(&mut self.diags, span, Severity::Hint, format!("`{k}` is already bound the same way in `{}`", self.sections[a].path));
                        }
                        break;
                    }
                }
            }
        }
    }
}

pub fn short(v: &Value) -> String {
    match v {
        Value::Null => "null".into(),
        Value::String(s) => s.clone(),
        Value::Array(a) => a.first().and_then(|x| x.as_str()).unwrap_or("?").to_string(),
        _ => v.to_string(),
    }
}

/// Effective map (context, key) → value, last write wins, for tests and `explain`.
pub fn effective(sections: &[Section]) -> HashMap<(String, String), Value> {
    let mut m = HashMap::new();
    for s in sections {
        for b in &s.bindings {
            m.insert((normalize_ws(&s.context), b.key.clone()), b.value.clone());
        }
    }
    m
}

pub fn normalize_ws(s: &str) -> String {
    s.split_whitespace().collect::<Vec<_>>().join(" ")
}
