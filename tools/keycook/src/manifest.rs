//! The action manifest: output of `zed --dump-all-actions`.

use serde::Deserialize;
use serde_json::Value;
use std::collections::HashMap;

pub const EMBEDDED: &str = include_str!("../../actions.json");

#[derive(Debug, Deserialize)]
struct Raw {
    actions: Vec<RawAction>,
}

#[derive(Debug, Deserialize)]
struct RawAction {
    name: String,
    human_name: String,
    schema: Option<Value>,
    #[serde(default)]
    deprecated_aliases: Vec<String>,
    deprecation_message: Option<String>,
    documentation: Option<String>,
}

#[derive(Debug, Clone)]
pub struct Action {
    pub name: String,
    pub human_name: String,
    pub schema: Option<Value>,
    pub aliases: Vec<String>,
    pub deprecation: Option<String>,
    pub doc: String,
}

#[derive(Debug, Default)]
pub struct Manifest {
    pub actions: HashMap<String, Action>,
    /// deprecated alias → current name
    pub aliases: HashMap<String, String>,
    pub sorted_names: Vec<String>,
}

impl Manifest {
    pub fn parse(json: &str) -> anyhow::Result<Manifest> {
        let raw: Raw = serde_json::from_str(json)?;
        let mut m = Manifest::default();
        for a in raw.actions {
            for al in &a.deprecated_aliases {
                m.aliases.insert(al.clone(), a.name.clone());
            }
            m.sorted_names.push(a.name.clone());
            m.actions.insert(
                a.name.clone(),
                Action {
                    name: a.name,
                    human_name: a.human_name,
                    schema: a.schema,
                    aliases: a.deprecated_aliases,
                    deprecation: a.deprecation_message,
                    doc: a.documentation.unwrap_or_default(),
                },
            );
        }
        m.sorted_names.sort();
        Ok(m)
    }

    pub fn embedded() -> Manifest {
        Manifest::parse(EMBEDDED).expect("embedded actions.json is valid")
    }

    pub fn get(&self, name: &str) -> Option<&Action> {
        self.actions.get(name)
    }

    /// Argument names an object-schema action accepts, with a one-line type hint.
    pub fn arg_names(&self, name: &str) -> Vec<(String, String)> {
        let Some(a) = self.get(name) else { return vec![] };
        let Some(schema) = &a.schema else { return vec![] };
        let mut out = Vec::new();
        collect_props(schema, &mut out);
        out
    }

    /// Validate an argument value against the action's schema. Light: shape and property names.
    pub fn check_args(&self, name: &str, args: Option<&Value>) -> Option<String> {
        let a = self.get(name)?;
        match (&a.schema, args) {
            (None, None) => None,
            (None, Some(v)) if v.as_object().map(|o| o.is_empty()).unwrap_or(false) => None,
            (None, Some(_)) => Some(format!("`{name}` takes no arguments")),
            (Some(schema), None) => {
                if schema_requires_value(schema) {
                    Some(format!("`{name}` needs an argument: {}", summarize(schema)))
                } else {
                    None
                }
            }
            (Some(schema), Some(v)) => check_value(schema, v).map(|e| format!("`{name}`: {e}; expected {}", summarize(schema))),
        }
    }
}

fn collect_props(schema: &Value, out: &mut Vec<(String, String)>) {
    if let Some(props) = schema.get("properties").and_then(|p| p.as_object()) {
        for (k, v) in props {
            out.push((k.clone(), summarize(v)));
        }
    }
    for key in ["anyOf", "oneOf"] {
        if let Some(alts) = schema.get(key).and_then(|a| a.as_array()) {
            for alt in alts {
                collect_props(alt, out);
            }
        }
    }
}

fn schema_requires_value(schema: &Value) -> bool {
    // an object schema with all-optional properties can be omitted (Zed defaults it)
    if schema.get("type").and_then(|t| t.as_str()) == Some("object") {
        return schema.get("required").and_then(|r| r.as_array()).map(|r| !r.is_empty()).unwrap_or(false);
    }
    if schema.get("anyOf").is_some() || schema.get("oneOf").is_some() {
        return false;
    }
    true
}

fn check_value(schema: &Value, v: &Value) -> Option<String> {
    if let Some(alts) = schema.get("anyOf").or_else(|| schema.get("oneOf")).and_then(|a| a.as_array()) {
        if alts.iter().any(|alt| check_value(alt, v).is_none()) {
            return None;
        }
        return Some("no variant matches".into());
    }
    if let Some(en) = schema.get("enum").and_then(|e| e.as_array()) {
        return if en.contains(v) { None } else { Some(format!("`{v}` is not one of the allowed values")) };
    }
    if let Some(c) = schema.get("const") {
        return if c == v { None } else { Some(format!("expected `{c}`")) };
    }
    if schema.get("$ref").is_some() {
        return None; // shared definitions: not resolved in v1
    }
    let ty = schema.get("type");
    let types: Vec<String> = match ty {
        Some(Value::String(s)) => vec![s.clone()],
        Some(Value::Array(a)) => a.iter().filter_map(|x| x.as_str().map(String::from)).collect(),
        _ => vec![],
    };
    if types.is_empty() {
        return None;
    }
    let ok = types.iter().any(|t| match t.as_str() {
        "object" => v.is_object(),
        "string" => v.is_string(),
        "integer" => v.is_i64() || v.is_u64(),
        "number" => v.is_number(),
        "boolean" => v.is_boolean(),
        "array" => v.is_array(),
        "null" => v.is_null(),
        _ => true,
    });
    if !ok {
        return Some(format!("`{v}` has the wrong type"));
    }
    if v.is_object() && types.iter().any(|t| t == "object") {
        let props = schema.get("properties").and_then(|p| p.as_object());
        let additional = schema.get("additionalProperties").map(|a| a != &Value::Bool(false)).unwrap_or(true);
        if let Some(obj) = v.as_object() {
            for (k, val) in obj {
                match props.and_then(|p| p.get(k)) {
                    Some(ps) => {
                        if let Some(e) = check_value(ps, val) {
                            return Some(format!("argument `{k}`: {e}"));
                        }
                    }
                    None if !additional => return Some(format!("unknown argument `{k}`")),
                    None => {}
                }
            }
            if let Some(req) = schema.get("required").and_then(|r| r.as_array()) {
                for r in req {
                    if let Some(name) = r.as_str() {
                        if !obj.contains_key(name) {
                            return Some(format!("missing argument `{name}`"));
                        }
                    }
                }
            }
        }
    }
    None
}

/// Compact human-readable form of a schema, for hovers and messages.
pub fn summarize(schema: &Value) -> String {
    if let Some(r) = schema.get("$ref").and_then(|r| r.as_str()) {
        return r.rsplit('/').next().unwrap_or(r).to_string();
    }
    if let Some(c) = schema.get("const") {
        return c.to_string();
    }
    if let Some(en) = schema.get("enum").and_then(|e| e.as_array()) {
        return en.iter().map(|v| v.to_string()).collect::<Vec<_>>().join(" | ");
    }
    for key in ["anyOf", "oneOf"] {
        if let Some(alts) = schema.get(key).and_then(|a| a.as_array()) {
            return alts.iter().map(summarize).collect::<Vec<_>>().join(" | ");
        }
    }
    match schema.get("type") {
        Some(Value::String(t)) if t == "object" => {
            let props = schema.get("properties").and_then(|p| p.as_object());
            let req: Vec<&str> = schema.get("required").and_then(|r| r.as_array()).map(|r| r.iter().filter_map(|x| x.as_str()).collect()).unwrap_or_default();
            match props {
                Some(p) if !p.is_empty() => {
                    let items: Vec<String> = p
                        .iter()
                        .map(|(k, v)| {
                            let opt = if req.contains(&k.as_str()) { "" } else { "?" };
                            let default = v.get("default").filter(|d| !d.is_null()).map(|d| format!(" = {d}")).unwrap_or_default();
                            format!("{k}{opt}: {}{default}", summarize(v))
                        })
                        .collect();
                    format!("{{ {} }}", items.join(", "))
                }
                _ => "object".into(),
            }
        }
        Some(Value::String(t)) if t == "array" => format!("[{}, …]", schema.get("items").map(summarize).unwrap_or_else(|| "any".into())),
        Some(Value::String(t)) => t.clone(),
        Some(Value::Array(ts)) => ts.iter().filter_map(|t| t.as_str()).collect::<Vec<_>>().join(" | "),
        _ => "any".into(),
    }
}
