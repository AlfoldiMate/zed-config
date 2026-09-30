//! Golden test: the transcribed upstream keymaps (`default-macos.kc`, `vim.kc`) must compile to
//! exactly the effective (context, key) → value map of the original JSON files.

use keycook::catalog::Catalog;
use keycook::compile::{self, normalize_ws, Options};
use keycook::manifest::Manifest;
use serde_json::Value;
use std::collections::HashMap;

fn strip_json_comments(text: &str) -> String {
    let b = text.as_bytes();
    let mut out = String::new();
    let mut i = 0;
    let mut in_str = false;
    while i < b.len() {
        let c = b[i] as char;
        if in_str {
            out.push(c);
            if c == '\\' && i + 1 < b.len() {
                out.push(b[i + 1] as char);
                i += 2;
                continue;
            }
            if c == '"' {
                in_str = false;
            }
            i += 1;
            continue;
        }
        if c == '"' {
            in_str = true;
            out.push(c);
            i += 1;
            continue;
        }
        if text[i..].starts_with("//") {
            while i < b.len() && b[i] != b'\n' {
                i += 1;
            }
            continue;
        }
        if text[i..].starts_with("/*") {
            i = text[i..].find("*/").map(|j| i + j + 2).unwrap_or(b.len());
            continue;
        }
        out.push(c);
        i += 1;
    }
    // trailing commas
    let re = regex::Regex::new(r",(\s*[\]}])").unwrap();
    re.replace_all(&out, "$1").to_string()
}

fn upstream(name: &str) -> HashMap<(String, String), Value> {
    let path = format!("{}/../upstream/{name}", env!("CARGO_MANIFEST_DIR"));
    let text = std::fs::read_to_string(&path).expect("upstream keymap present");
    let secs: Vec<Value> = serde_json::from_str(&strip_json_comments(&text)).expect("valid upstream json");
    let mut m = HashMap::new();
    for s in secs {
        let ctx = s.get("context").and_then(|c| c.as_str()).unwrap_or("").to_string();
        if let Some(b) = s.get("bindings").and_then(|b| b.as_object()) {
            for (k, v) in b {
                m.insert((normalize_ws(&ctx), k.clone()), v.clone());
            }
        }
    }
    m
}

fn check(kc: &str, json: &str) {
    let src = std::fs::read_to_string(format!("{}/{kc}", env!("CARGO_MANIFEST_DIR"))).expect("kc present");
    let out = compile::compile(&src, &Manifest::embedded(), &Catalog::embedded(), &Options { auto_null: false, sort: false });
    let errors: Vec<_> = out.diags.iter().filter(|d| d.severity == keycook::parser::Severity::Error).collect();
    assert!(errors.is_empty(), "compile errors in {kc}: {:#?}", errors);
    let got = compile::effective(&out.sections);
    // the compiler rewrites deprecated aliases to their current names; do the same to the upstream
    let manifest = Manifest::embedded();
    let rename = |v: Value| -> Value {
        match v {
            Value::String(s) => Value::String(manifest.aliases.get(&s).cloned().unwrap_or(s)),
            Value::Array(mut a) => {
                if let Some(Value::String(s)) = a.first().cloned() {
                    a[0] = Value::String(manifest.aliases.get(&s).cloned().unwrap_or(s));
                }
                Value::Array(a)
            }
            other => other,
        }
    };
    let want: HashMap<(String, String), Value> = upstream(json).into_iter().map(|(k, v)| (k, rename(v))).collect();
    let mut missing = Vec::new();
    for (k, v) in &want {
        match got.get(k) {
            Some(g) if g == v => {}
            Some(g) => missing.push(format!("{k:?}: got {g}, want {v}")),
            None => missing.push(format!("{k:?}: missing (want {v})")),
        }
    }
    let mut extra = Vec::new();
    for k in got.keys() {
        if !want.contains_key(k) {
            extra.push(format!("{k:?}"));
        }
    }
    assert!(missing.is_empty() && extra.is_empty(), "{kc} vs {json}\nmissing/different ({}):\n{}\nextra ({}):\n{}", missing.len(), missing.join("\n"), extra.len(), extra.join("\n"));
    assert_eq!(got.len(), want.len());
}

#[test]
fn default_macos_round_trips() {
    check("default-macos.kc", "default-macos.json");
}

#[test]
fn vim_round_trips() {
    check("vim.kc", "vim.json");
}
