//! Language server: diagnostics, completion, hover, document symbols for `.kc` files.

use crate::ast::{Header, Item};
use crate::catalog::Catalog;
use crate::compile::{self, Options};
use crate::keys;
use crate::manifest::{summarize, Manifest};
use crate::parser::Severity;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tower_lsp::jsonrpc::Result;
use tower_lsp::lsp_types::*;
use tower_lsp::{Client, LanguageServer, LspService, Server};

pub struct LineIndex {
    starts: Vec<usize>,
    text: String,
}

impl LineIndex {
    pub fn new(text: &str) -> Self {
        let mut starts = vec![0];
        for (i, b) in text.bytes().enumerate() {
            if b == b'\n' {
                starts.push(i + 1);
            }
        }
        LineIndex { starts, text: text.to_string() }
    }
    pub fn position(&self, offset: usize) -> Position {
        let offset = offset.min(self.text.len());
        let line = match self.starts.binary_search(&offset) {
            Ok(l) => l,
            Err(l) => l - 1,
        };
        let start = self.starts[line];
        let col = self.text[start..offset].encode_utf16().count();
        Position::new(line as u32, col as u32)
    }
    pub fn offset(&self, pos: Position) -> usize {
        let line = (pos.line as usize).min(self.starts.len() - 1);
        let start = self.starts[line];
        let end = self.starts.get(line + 1).copied().unwrap_or(self.text.len());
        let line_text = &self.text[start..end];
        let mut units = 0u32;
        for (i, ch) in line_text.char_indices() {
            if units >= pos.character {
                return start + i;
            }
            units += ch.len_utf16() as u32;
        }
        end
    }
    pub fn line_text(&self, line: u32) -> &str {
        let line = (line as usize).min(self.starts.len() - 1);
        let start = self.starts[line];
        let end = self.starts.get(line + 1).copied().unwrap_or(self.text.len());
        self.text[start..end].trim_end_matches('\n')
    }
    pub fn range(&self, span: crate::ast::Span) -> Range {
        Range::new(self.position(span.start), self.position(span.end))
    }
}

pub struct Backend {
    client: Client,
    manifest: Arc<Manifest>,
    catalog: Arc<Catalog>,
    docs: RwLock<HashMap<Url, String>>,
}

impl Backend {
    async fn refresh(&self, uri: Url) {
        let text = self.docs.read().await.get(&uri).cloned().unwrap_or_default();
        let out = compile::compile(&text, &self.manifest, &self.catalog, &Options::default());
        let idx = LineIndex::new(&text);
        let diags = out
            .diags
            .iter()
            .map(|d| Diagnostic {
                range: idx.range(d.span),
                severity: Some(match d.severity {
                    Severity::Error => DiagnosticSeverity::ERROR,
                    Severity::Warning => DiagnosticSeverity::WARNING,
                    Severity::Info => DiagnosticSeverity::INFORMATION,
                    Severity::Hint => DiagnosticSeverity::HINT,
                }),
                source: Some("keycook".into()),
                message: d.message.clone(),
                ..Default::default()
            })
            .collect();
        self.client.publish_diagnostics(uri, diags, None).await;
    }
}

/// Where the cursor is on a line, for completion.
#[derive(Debug, PartialEq)]
enum Slot {
    /// inside `(`…`)` of the named action
    Args(String),
    /// after a key token on this line: an action name (partial text)
    Action(String),
    /// start of a line or in a header: context identifiers, aliases, keys
    Header(String),
}

fn slot_at(line: &str, col: usize) -> Slot {
    let before: String = line.chars().take(col).collect();
    // inside an argument list?
    if let Some(open) = before.rfind('(') {
        if !before[open..].contains(')') {
            let name_start = before[..open].rfind(char::is_whitespace).map(|i| i + 1).unwrap_or(0);
            return Slot::Args(before[name_start..open].to_string());
        }
    }
    // after a key token (word ending with a single ':' followed by whitespace)?
    let mut after_key = false;
    let mut last_word = String::new();
    for w in before.split_whitespace() {
        if w.ends_with(':') && !w.ends_with("::") && w.len() > 1 {
            after_key = true;
            last_word.clear();
        } else {
            last_word = w.to_string();
        }
    }
    let partial = if before.ends_with(char::is_whitespace) { String::new() } else { last_word };
    if after_key {
        Slot::Action(partial)
    } else {
        Slot::Header(partial)
    }
}

fn word_at(line: &str, col: usize) -> Option<String> {
    let chars: Vec<char> = line.chars().collect();
    if col > chars.len() {
        return None;
    }
    let is_w = |c: char| c.is_alphanumeric() || matches!(c, '_' | ':' | '-' | '@');
    let mut s = col;
    while s > 0 && is_w(chars[s - 1]) {
        s -= 1;
    }
    let mut e = col;
    while e < chars.len() && is_w(chars[e]) {
        e += 1;
    }
    if s == e {
        return None;
    }
    Some(chars[s..e].iter().collect::<String>().trim_end_matches(':').to_string())
}

#[tower_lsp::async_trait]
impl LanguageServer for Backend {
    async fn initialize(&self, _: InitializeParams) -> Result<InitializeResult> {
        Ok(InitializeResult {
            capabilities: ServerCapabilities {
                text_document_sync: Some(TextDocumentSyncCapability::Kind(TextDocumentSyncKind::FULL)),
                completion_provider: Some(CompletionOptions { trigger_characters: Some(vec![":".into(), "(".into(), " ".into(), "@".into(), ",".into()]), resolve_provider: Some(false), ..Default::default() }),
                hover_provider: Some(HoverProviderCapability::Simple(true)),
                document_symbol_provider: Some(OneOf::Left(true)),
                ..Default::default()
            },
            server_info: Some(ServerInfo { name: "keycook".into(), version: Some(env!("CARGO_PKG_VERSION").into()) }),
        })
    }

    async fn initialized(&self, _: InitializedParams) {
        self.client.log_message(MessageType::INFO, "keycook ready").await;
    }

    async fn shutdown(&self) -> Result<()> {
        Ok(())
    }

    async fn did_open(&self, p: DidOpenTextDocumentParams) {
        self.docs.write().await.insert(p.text_document.uri.clone(), p.text_document.text);
        self.refresh(p.text_document.uri).await;
    }

    async fn did_change(&self, p: DidChangeTextDocumentParams) {
        if let Some(c) = p.content_changes.into_iter().last() {
            self.docs.write().await.insert(p.text_document.uri.clone(), c.text);
        }
        self.refresh(p.text_document.uri).await;
    }

    async fn did_close(&self, p: DidCloseTextDocumentParams) {
        self.docs.write().await.remove(&p.text_document.uri);
        self.client.publish_diagnostics(p.text_document.uri, vec![], None).await;
    }

    async fn completion(&self, p: CompletionParams) -> Result<Option<CompletionResponse>> {
        let uri = p.text_document_position.text_document.uri;
        let pos = p.text_document_position.position;
        let text = self.docs.read().await.get(&uri).cloned().unwrap_or_default();
        let idx = LineIndex::new(&text);
        let line = idx.line_text(pos.line);
        let col = idx.offset(pos) - idx.starts[pos.line as usize];
        let col_chars = line[..col.min(line.len())].chars().count();
        let slot = slot_at(line, col_chars);
        let mut items = Vec::new();
        match slot {
            Slot::Args(name) => {
                for (arg, ty) in self.manifest.arg_names(&name) {
                    items.push(CompletionItem { label: arg.clone(), kind: Some(CompletionItemKind::PROPERTY), detail: Some(ty), ..Default::default() });
                }
                if items.is_empty() {
                    if let Some(a) = self.manifest.get(&name) {
                        if let Some(s) = &a.schema {
                            items.push(CompletionItem { label: summarize(s), kind: Some(CompletionItemKind::VALUE), detail: Some("argument".into()), ..Default::default() });
                        }
                    }
                }
            }
            Slot::Action(partial) => {
                let ns = partial.split("::").next().unwrap_or("").to_string();
                items.push(CompletionItem { label: "null".into(), kind: Some(CompletionItemKind::KEYWORD), detail: Some("unbind this key here".into()), sort_text: Some("0".into()), ..Default::default() });
                for name in &self.manifest.sorted_names {
                    if !ns.is_empty() && !partial.contains("::") && !name.starts_with(&ns) && !name.contains(&partial) {
                        continue;
                    }
                    let a = &self.manifest.actions[name];
                    let sig = a.schema.as_ref().map(|s| format!("({})", summarize(s))).unwrap_or_default();
                    items.push(CompletionItem {
                        label: name.clone(),
                        kind: Some(CompletionItemKind::FUNCTION),
                        detail: Some(format!("{}{}", a.human_name, sig)),
                        documentation: if a.doc.is_empty() { None } else { Some(Documentation::MarkupContent(MarkupContent { kind: MarkupKind::Markdown, value: a.doc.clone() })) },
                        insert_text: Some(if a.schema.as_ref().map(|s| s.get("required").map(|r| !r.as_array().map(|x| x.is_empty()).unwrap_or(true)).unwrap_or(false)).unwrap_or(false) { format!("{name}($1)") } else { name.clone() }),
                        insert_text_format: Some(InsertTextFormat::SNIPPET),
                        ..Default::default()
                    });
                }
            }
            Slot::Header(_) => {
                let out = compile::compile(&text, &self.manifest, &self.catalog, &Options { auto_null: false, sort: true });
                for (name, value) in &out.aliases {
                    items.push(CompletionItem { label: format!("@{name}"), kind: Some(CompletionItemKind::CONSTANT), detail: Some(value.clone()), ..Default::default() });
                }
                for name in &out.bodies {
                    items.push(CompletionItem { label: format!("@{name}"), kind: Some(CompletionItemKind::SNIPPET), detail: Some("body: splice its bindings here".into()), ..Default::default() });
                }
                for n in &self.catalog.nodes {
                    items.push(CompletionItem { label: n.name.clone(), kind: Some(CompletionItemKind::CLASS), detail: Some(if n.doc.is_empty() { "node".into() } else { n.doc.clone() }), ..Default::default() });
                }
                let mut flags: Vec<&String> = self.catalog.flags.iter().collect();
                flags.sort();
                for f in flags {
                    items.push(CompletionItem { label: f.clone(), kind: Some(CompletionItemKind::FIELD), detail: self.catalog.flag_docs.get(f).cloned().or(Some("flag".into())), ..Default::default() });
                }
                let mut attrs: Vec<(&String, &Vec<String>)> = self.catalog.attrs.iter().collect();
                attrs.sort();
                for (k, vals) in attrs {
                    for v in vals {
                        items.push(CompletionItem { label: format!("{k} == {v}"), kind: Some(CompletionItemKind::ENUM_MEMBER), ..Default::default() });
                    }
                    if vals.is_empty() {
                        items.push(CompletionItem { label: format!("{k} == "), kind: Some(CompletionItemKind::ENUM_MEMBER), ..Default::default() });
                    }
                }
                for m in keys::MODIFIERS {
                    items.push(CompletionItem { label: format!("{m}-"), kind: Some(CompletionItemKind::OPERATOR), detail: Some("modifier".into()), ..Default::default() });
                }
                for k in keys::NAMED_KEYS {
                    items.push(CompletionItem { label: format!("{k}:"), kind: Some(CompletionItemKind::KEYWORD), detail: Some("key".into()), ..Default::default() });
                }
            }
        }
        Ok(Some(CompletionResponse::Array(items)))
    }

    async fn hover(&self, p: HoverParams) -> Result<Option<Hover>> {
        let uri = p.text_document_position_params.text_document.uri;
        let pos = p.text_document_position_params.position;
        let text = self.docs.read().await.get(&uri).cloned().unwrap_or_default();
        let idx = LineIndex::new(&text);
        let line = idx.line_text(pos.line);
        let col = idx.offset(pos) - idx.starts[pos.line as usize];
        let col_chars = line[..col.min(line.len())].chars().count();
        let Some(word) = word_at(line, col_chars) else { return Ok(None) };
        let md = if let Some(a) = self.manifest.get(&word).or_else(|| self.manifest.aliases.get(&word).and_then(|n| self.manifest.get(n))) {
            let mut s = format!("**{}** — {}\n\n", a.name, a.human_name);
            if !a.doc.is_empty() {
                s.push_str(&a.doc);
                s.push_str("\n\n");
            }
            match &a.schema {
                Some(schema) => s.push_str(&format!("argument: `{}`", summarize(schema))),
                None => s.push_str("no argument"),
            }
            if !a.aliases.is_empty() {
                s.push_str(&format!("\n\naliases: `{}`", a.aliases.join("`, `")));
            }
            Some(s)
        } else if let Some(name) = word.strip_prefix('@') {
            let out = compile::compile(&text, &self.manifest, &self.catalog, &Options { auto_null: false, sort: true });
            out.aliases.iter().find(|(n, _)| n == name).map(|(n, v)| format!("**@{n}** = `{v}`")).or_else(|| out.bodies.iter().find(|b| b.as_str() == name).map(|n| format!("**@{n}** — a body; `@{n}` on its own line splices its bindings, later lines override")))
        } else {
            self.catalog.doc_for(&word).or_else(|| {
                let w = word.trim_end_matches(':');
                if keys::check_keystroke(w).is_none() && !w.is_empty() {
                    Some(format!("keystroke `{w}`"))
                } else {
                    None
                }
            })
        };
        Ok(md.map(|value| Hover { contents: HoverContents::Markup(MarkupContent { kind: MarkupKind::Markdown, value }), range: None }))
    }

    async fn document_symbol(&self, p: DocumentSymbolParams) -> Result<Option<DocumentSymbolResponse>> {
        let text = self.docs.read().await.get(&p.text_document.uri).cloned().unwrap_or_default();
        let idx = LineIndex::new(&text);
        let out = compile::compile(&text, &self.manifest, &self.catalog, &Options { auto_null: false, sort: true });
        fn conv(items: &[Item], idx: &LineIndex) -> Vec<DocumentSymbol> {
            let mut v = Vec::new();
            for it in items {
                match it {
                    Item::Block(b) => {
                        let (name, kind) = match &b.header {
                            Header::Context(h) => (h.clone(), SymbolKind::NAMESPACE),
                            Header::Prefix(k) => (format!("{}:", k.iter().map(|x| x.text.clone()).collect::<Vec<_>>().join(" ")), SymbolKind::KEY),
                        };
                        #[allow(deprecated)]
                        v.push(DocumentSymbol { name, detail: None, kind, tags: None, deprecated: None, range: idx.range(b.span), selection_range: idx.range(b.header_span), children: Some(conv(&b.items, idx)) });
                    }
                    Item::Alias { name, value, span } => {
                        #[allow(deprecated)]
                        v.push(DocumentSymbol { name: format!("@{name}"), detail: Some(value.clone()), kind: SymbolKind::CONSTANT, tags: None, deprecated: None, range: idx.range(*span), selection_range: idx.range(*span), children: None });
                    }
                    Item::Body { name, items, span } => {
                        #[allow(deprecated)]
                        v.push(DocumentSymbol { name: format!("@{name} = {{ }}"), detail: Some("body".into()), kind: SymbolKind::STRUCT, tags: None, deprecated: None, range: idx.range(*span), selection_range: idx.range(*span), children: Some(conv(items, idx)) });
                    }
                    Item::Use { name, span } => {
                        #[allow(deprecated)]
                        v.push(DocumentSymbol { name: format!("@{name}"), detail: Some("splice".into()), kind: SymbolKind::EVENT, tags: None, deprecated: None, range: idx.range(*span), selection_range: idx.range(*span), children: None });
                    }
                    Item::Binding(_) => {}
                }
            }
            v
        }
        Ok(Some(DocumentSymbolResponse::Nested(conv(&out.items, &idx))))
    }
}

pub async fn run(manifest: Manifest, catalog: Catalog) {
    let stdin = tokio::io::stdin();
    let stdout = tokio::io::stdout();
    let (service, socket) = LspService::new(|client| Backend { client, manifest: Arc::new(manifest), catalog: Arc::new(catalog), docs: RwLock::new(HashMap::new()) });
    Server::new(stdin, stdout, socket).serve(service).await;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn slots() {
        assert_eq!(slot_at("  d: vim::Hel", 13), Slot::Action("vim::Hel".into()));
        assert_eq!(slot_at("  d: editor::Foo(sk", 19), Slot::Args("editor::Foo".into()));
        assert_eq!(slot_at("  mode == fu", 12), Slot::Header("fu".into()));
        assert_eq!(slot_at("  \":\": command", 14), Slot::Action("command".into()));
    }
}
