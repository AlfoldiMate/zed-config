# keycook: a keymap DSL that compiles to Zed

Concept for a small language in which a keymap is written once, structured the way one thinks about
it (views, modes, menus, meanings), and a Rust CLI that cooks it into `keymap.json`, the cheat sheet,
and a report. Status: superseded by the Zed-only design that was built as `tools/keycook` (see its README). Kept
for the reasoning; the generic scope/intent model here was simplified away.

## 1. Why the flat file is big and what "once" has to mean

Zed's keymap is a list of sections, each a context predicate plus key→action pairs, resolved by
"deepest matching node wins, then later in file wins". Three things make it long:

1. **No sharing across views.** `j` means "next" in a buffer, a list panel, a picker and a menu, but
   it has to be written for each, against each view's own predicate.
2. **State flags multiply contexts.** `vim_mode`, `menu`, `in_replace`, `showing_completions` are
   orthogonal, so near-identical sections pile up.
3. **No naming.** A binding is `key → action`; there is no place for "this is delete" or "this is the
   goto menu", so neither the file nor the popup can explain itself, and documentation is a second file
   that drifts.

The DSL fixes all three with one rule: **a binding is written at the most general scope where its
meaning holds, and the compiler decides how to express that for Zed.** There are exactly two ways to
express "inherited" in Zed, and the compiler must know both:

- **Hoist.** If the general scope has a Zed predicate that matches at every child (for example
  `!Editor && !Terminal` matches at every panel node), emit the binding once there. Zed's own
  precedence does the inheritance: a child scope that overrides the key emits a later section with a
  more specific predicate at the same depth, and later wins.
- **Materialize.** If no shared predicate exists (buffer normal mode and the explorer share nothing
  in Zed's tree), copy the binding into each concrete child section.

The source stays small either way; only the output grows, and nobody reads the output.

## 2. The model

```
Layer      base < modal < semantic < user       later layers override earlier ones
Scope      a named place, with a Zed predicate, in a tree of inheritance (not Zed's focus tree)
Mode       an axis on a scope: buffer.normal, buffer.select, buffer.insert, explorer.editing
Intent     a meaning with one action per scope: item.delete = HelixDelete | Trash | RestoreFile
Binding    key → action | intent | null, at one or more scopes, optionally under a menu
Menu       a prefix with a name and children; the compiler owns the null-leader and chord rules
Import     an upstream keymap file with keep/drop rules, taken by reference not copied
Guard      extra predicate fragment on a binding: when="showing_completions"
```

Scopes are the part that must be designed carefully. They form the tree you think in, and each has a
Zed predicate. Inheritance in the DSL is "child gets parent's bindings unless it overrides"; the
compiler translates that with hoist or materialize per binding.

## 3. Syntax

KDL is proposed as the surface syntax: node based, hierarchical, comments, a Rust crate exists with
span information for good errors, and it reads like a config rather than a program. The same model
could be TOML or a custom grammar; KDL avoids writing a parser.

```kdl
keycook target="zed" version="1.21"
manifest "tools/actions.json"          // zed --dump-all-actions; validates every action and argument
contexts "tools/contexts.kdl"          // the focus-tree catalogue the atlas uses; validates predicates
leader "space"

// ── scopes: your tree, each with the Zed predicate it stands for ─────────────
scope "buffer" ctx="Editor && mode == full" {
  mode "normal" ctx="vim_mode == helix_normal && !menu"
  mode "select" ctx="vim_mode == helix_select && !menu"
  mode "insert" ctx="vim_mode == insert"
  mode "modal"  is="normal select"                     // alias for the two modal modes
}
scope "list" ctx="!Editor && !Terminal" {               // abstract; hoist target for panels
  scope "explorer" ctx="ProjectPanel && not_editing"
  scope "outline"  ctx="OutlinePanel && not_editing"
  scope "git"      ctx="GitPanel && ChangesList && !GitBranchSelector && !GitRepositorySelector"
}
scope "picker"   ctx="Picker > Editor"
scope "terminal" ctx="Terminal"
scope "input"    ctx="Editor && mode != full"             // search fields, commit box, agent box

// ── intents: one meaning, an action per scope ────────────────────────────────
intent "nav.down" {
  buffer   "vim::Down" display_lines=#true
  list     "menu::SelectNext"
  picker   "menu::SelectNext"
}
intent "item.delete" {
  buffer   "vim::HelixDelete"
  explorer "project_panel::Trash"
  git      "git::RestoreFile"  doc="discard the change"
}
intent "item.rename" { explorer "project_panel::Rename" }   // no buffer meaning: no buffer binding

// ── bindings: once, at the widest scope where the meaning holds ──────────────
bind "j"      intent="nav.down"    in="buffer.modal list"
bind "ctrl-j" intent="nav.down"    in="picker"
bind "d"      intent="item.delete" in="buffer.modal list"     // list → explorer, git; outline has none, skipped
bind "r"      intent="item.rename" in="list"
bind "ctrl-s" "workspace::Save"    in="buffer input"          // plain action, no intent needed
bind "ctrl-h" "workspace::ActivatePaneLeft" in="buffer.modal list"

// ── menus: prefix trees with names; docs come from here ──────────────────────
menu "g" "goto" in="buffer.modal" {
  bind "d" "editor::GoToDefinition"     doc="definition"
  bind "y" "editor::GoToTypeDefinition" doc="type definition"
  bind "r" "editor::FindAllReferences"  doc="references"
  unbind "r n" "r r" "r i" "r a"                            // upstream chords that made g r wait
}
menu "space" "leader" in="buffer.modal list" {
  bind "f" "file_finder::Toggle"         doc="find file"
  bind "e" "project_panel::ToggleFocus"  doc="explorer"
  bind "." intent="view.context-menu"    doc="context menu"
  menu "u" "toggles" in="buffer.modal" {
    bind "w" "editor::ToggleSoftWrap" doc="wrap"
  }
}

// ── overrides: a child says something different ──────────────────────────────
scope "git" { bind "s" "git::ToggleStaged" doc="stage"; bind "c" "git::Commit" }

// ── imports: keep upstream pieces by rule, never by copy ─────────────────────
import "tools/upstream/vim.json" as vim layer="modal" {
  keep ctx=r"helix|vim_mode == (insert|waiting|operator|replace)|vim_operator"
  drop key=r"^ctrl-w( |$)|^ctrl-x( |$)|^ctrl-page"
}
import "tools/upstream/default-macos.json" as base layer="base" {
  keep ctx=r"^$|menu|Picker|SearchBar|Panel|Terminal|showing_|in_snippet|renaming|^Editor$"
  drop key=r"^(?:[a-z-]+-)?f\d+$|^cmd-k\b|^ctrl-cmd-"
  allow key="cmd-q cmd-w cmd-s cmd-v cmd-c cmd-x cmd-z cmd-a cmd-p cmd-shift-p cmd-,"   // every other cmd- is dropped
}
```

About one hundred and fifty lines like this describe everything the current 1300-binding file does,
because the upstream parts are referenced, the panels inherit, and the menus carry their own docs.

## 4. Semantics the compiler must get right

- **Resolution of `in=`.** A name is a scope, `scope.mode`, or a mode alias. An abstract scope with a
  predicate is a hoist target; without one it is a pure group and its children are materialized.
- **Intent expansion.** `bind k intent=I in=S` becomes, for each concrete scope C under S, `k → I[C]`
  if I has an action for C, else nothing. `required=#true` on the intent makes a missing scope an error.
- **Order of emitted sections.** Layers in order; inside a layer, scopes parent first, children after,
  so an override at the same Zed depth wins by being later. Imports go first inside their layer.
- **Menus.** A `menu` key is a prefix at every scope it is in. The compiler emits `key → null` for the
  bare prefix wherever the same key is also a leaf in an inherited or imported section (this is the
  bare-`space` rule, done automatically), and errors when the author binds a leaf and a chord on the
  same key in the same scope.
- **Unbind.** `unbind` emits `null` in that scope. `drop` inside an import removes before emission.
- **Guards.** `when="…"` is and-ed onto the scope predicate for that binding only, creating a derived
  section. Repeated guards on many bindings become one section.
- **Overload.** Same key bound twice at the same scope in the same layer is an error; across layers
  the later layer wins and the report says what it shadowed.

## 5. Checks, from the data we already have

The compiler carries two inputs beside the DSL, both already produced in this repo:

- **Action manifest** (`zed --dump-all-actions`): every action name exists, arguments match the JSON
  schema, deprecated aliases are rewritten to the new name.
- **Context catalogue** (the atlas focus tree): every predicate parses with the gpui grammar and
  matches at least one composable focus chain, otherwise "unreachable context". The predicate
  evaluator is a straight port of `crates/gpui/src/keymap/context.rs`, already done once in
  JavaScript for the atlas.

With those the compiler runs the same checks the atlas runs today, on every focus chain of the
catalogue: chord waits (a leaf that is also a prefix), shadowed bindings (never the winner anywhere),
duplicates in overlapping contexts, and intents with no realization in a scope that binds them.

## 6. Outputs

- `keymap.json`, sections annotated with a comment naming the DSL line that produced them.
- `KEYMAP.md`, generated from scopes, menus and `doc=` strings, so the guide cannot drift. Sections
  follow the scope tree; menus render as tables; imported bindings can be included per scope or listed
  as "from Zed".
- `report.txt`: counts per layer, dropped upstream bindings, warnings.
- `keycook explain <key> --at <scope>` prints the winning binding for a focus chain and everything it
  shadowed, the same answer the atlas gives, from the terminal.
- `keycook diff-upstream <new vim.json>` after a Zed upgrade: which kept upstream bindings changed or
  vanished, and which drop rules no longer match anything.

## 7. The Rust CLI

```
keycook/
  src/main.rs          clap: build | check | explain | doc | diff-upstream | watch
  src/dsl/             kdl parse → AST with spans (kdl crate), miette for errors with source excerpts
  src/model/           Scope tree, Mode, Intent, Binding, Menu, Import, Layer
  src/zed/predicate.rs port of gpui KeyBindingContextPredicate (parse, eval, depth_of)
  src/zed/keymap.rs    JSON-with-comments reader for upstream files, writer for keymap.json
  src/zed/manifest.rs  actions.json loader, schema validation of arguments
  src/compile/         resolve → expand → order → emit
  src/check/           chord waits, shadowing, unreachable, duplicates (uses the context catalogue)
  src/doc/             KEYMAP.md renderer
```

Crates: `kdl`, `miette`, `clap`, `serde_json`, `regex`, `jsonschema` (argument validation),
`notify` for `watch`. The predicate port is about two hundred lines; the atlas JavaScript is the
reference and its verification cases become the unit tests. Target size is a few thousand lines, most
of it the compiler and checks.

The intent table makes the model target-agnostic in principle: a `helix` or `neovim` backend would
only need its own predicate mapping and action names. That is not a goal, but it is a sign the model
is at the right altitude.

## 8. Migration path from what exists

1. Write `keymap.kdl` for the current overlay and the two import rule sets. The overlay is already
   structured by scope, so this is mostly transcription.
2. Build the compiler until `keycook build` reproduces today's generated `keymap.json` binding for
   binding (a golden test), then start using the checks.
3. Replace `tools/build-keymap.py` and the hand-maintained `KEYMAP.md` with the compiler's outputs.
4. Fold the intent matrix from `KEYMAP-DESIGN.md` into `intent` nodes as it gets filled.

## 9. Open questions

- **Scope predicates for modes in inputs.** Search fields and the commit box are editors in insert
  mode; whether `input` is a scope or a mode of `buffer` decides how `ctrl-s` and `escape` are written.
- **Hoist versus materialize choice.** Automatic (hoist when a predicate exists) is simplest, but an
  author may want `materialize=#true` to keep a panel independent of the shared `!Editor && !Terminal`
  section. Probably an attribute with a sensible default.
- **Imports and intents.** Upstream bindings have no intent. Whether to let an intent claim an upstream
  binding (`intent "nav.down" { list from=base "menu::SelectNext" }`) so docs can name it, or leave
  imports opaque.
- **Which-key labels.** Zed 1.21 cannot show group names in the popup; the `menu` names are for the
  generated docs only, until Zed exposes labels.
