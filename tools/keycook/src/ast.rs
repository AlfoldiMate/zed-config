//! Syntax tree of a `.kc` file.

use serde_json::Value;

/// Byte range in the source file.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Span {
    pub start: usize,
    pub end: usize,
}

impl Span {
    pub fn new(start: usize, end: usize) -> Self {
        Span { start, end }
    }
    pub fn join(self, other: Span) -> Span {
        Span::new(self.start.min(other.start), self.end.max(other.end))
    }
}

#[derive(Debug, Clone)]
pub enum Item {
    /// `@name = expression` — a named context fragment.
    Alias { name: String, value: String, span: Span },
    /// `HEADER { … }` — a context block, or `keys: { … }` — a prefix block.
    Block(Block),
    /// `keys: actions`
    Binding(Binding),
    /// `@name = { … }` — a named body of items, spliced wherever `@name` appears on its own line.
    Body { name: String, items: Vec<Item>, span: Span },
    /// `@name` on its own line inside a block: splice that body here.
    Use { name: String, span: Span },
}

#[derive(Debug, Clone)]
pub struct Block {
    pub header: Header,
    pub header_span: Span,
    pub items: Vec<Item>,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub enum Header {
    /// Raw Zed predicate fragment, exactly as written (may start with `>`).
    Context(String),
    /// Key prefix: the keystrokes before the colon, e.g. `["g"]` or `["cmd-k"]`.
    Prefix(Vec<KeyTok>),
}

/// One key token as written, before range expansion. `text` is unquoted.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct KeyTok {
    pub text: String,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub struct Binding {
    /// Groups separated by `|`; each group is a list of keys.
    pub keys: Vec<Vec<KeyTok>>,
    /// Groups separated by `|`; each group is a list of actions.
    pub values: Vec<Vec<ActionTok>>,
    pub span: Span,
    pub keys_span: Span,
    pub values_span: Span,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ActionTok {
    Null { span: Span },
    Action { name: String, name_span: Span, args: Option<Args>, span: Span },
}

impl ActionTok {
    pub fn span(&self) -> Span {
        match self {
            ActionTok::Null { span } => *span,
            ActionTok::Action { span, .. } => *span,
        }
    }
}

/// Parsed `( … )` argument list.
#[derive(Debug, Clone, PartialEq)]
pub struct Args {
    pub raw: String,
    pub span: Span,
    pub value: Value,
}
