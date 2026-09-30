//! The context catalogue: node names, their flags and attributes. Used for completion, hover,
//! and the "two node names on one node" check.

use serde::Deserialize;
use std::collections::{HashMap, HashSet};

pub const EMBEDDED: &str = include_str!("../contexts.json");

#[derive(Debug, Deserialize)]
struct Raw {
    nodes: Vec<RawNode>,
    #[serde(default)]
    flag_docs: HashMap<String, String>,
}

#[derive(Debug, Deserialize)]
struct RawNode {
    name: String,
    #[serde(default)]
    doc: String,
    #[serde(default)]
    flags: Vec<String>,
    #[serde(default)]
    attrs: HashMap<String, Vec<String>>,
}

#[derive(Debug, Clone)]
pub struct Node {
    pub name: String,
    pub doc: String,
    pub flags: Vec<String>,
    pub attrs: HashMap<String, Vec<String>>,
}

#[derive(Debug, Default)]
pub struct Catalog {
    pub nodes: Vec<Node>,
    pub node_names: HashSet<String>,
    pub flags: HashSet<String>,
    pub attrs: HashMap<String, Vec<String>>,
    pub flag_docs: HashMap<String, String>,
}

impl Catalog {
    pub fn embedded() -> Catalog {
        let raw: Raw = serde_json::from_str(EMBEDDED).expect("contexts.json is valid");
        let mut c = Catalog { flag_docs: raw.flag_docs, ..Default::default() };
        for n in raw.nodes {
            c.node_names.insert(n.name.clone());
            for f in &n.flags {
                c.flags.insert(f.clone());
            }
            for (k, v) in &n.attrs {
                let e = c.attrs.entry(k.clone()).or_default();
                for x in v {
                    if !e.contains(x) {
                        e.push(x.clone());
                    }
                }
            }
            c.nodes.push(Node { name: n.name, doc: n.doc, flags: n.flags, attrs: n.attrs });
        }
        c
    }

    pub fn is_node(&self, id: &str) -> bool {
        self.node_names.contains(id)
    }

    pub fn known(&self, id: &str) -> bool {
        self.node_names.contains(id) || self.flags.contains(id) || self.attrs.contains_key(id)
    }

    pub fn doc_for(&self, id: &str) -> Option<String> {
        if let Some(n) = self.nodes.iter().find(|n| n.name == id) {
            let mut s = format!("**{}** — node", n.name);
            if !n.doc.is_empty() {
                s.push_str(&format!(". {}", n.doc));
            }
            if !n.flags.is_empty() {
                s.push_str(&format!("\n\nflags: `{}`", n.flags.join("`, `")));
            }
            if !n.attrs.is_empty() {
                let mut keys: Vec<_> = n.attrs.keys().cloned().collect();
                keys.sort();
                s.push_str(&format!("\n\nattributes: `{}`", keys.join("`, `")));
            }
            return Some(s);
        }
        if let Some(d) = self.flag_docs.get(id) {
            return Some(format!("**{id}** — flag. {d}"));
        }
        if self.flags.contains(id) {
            let on: Vec<_> = self.nodes.iter().filter(|n| n.flags.iter().any(|f| f == id)).map(|n| n.name.as_str()).collect();
            return Some(format!("**{id}** — flag on `{}`", on.join("`, `")));
        }
        if let Some(vals) = self.attrs.get(id) {
            return Some(format!("**{id}** — attribute. values: `{}`", vals.join("`, `")));
        }
        None
    }
}
