# keycook

A hierarchical keymap DSL for Zed, compiled to `keymap.json`, with a language server and a Zed
extension. Zed only: the DSL's vocabulary *is* Zed's context vocabulary, nesting is `&&`, and the
compiler's checks come from Zed's own action manifest and predicate grammar.

```
keycook build keymap.kc            # → keymap.json next to it
keycook check keymap.kc            # diagnostics only
keycook fmt keymap.kc              # 2-space indent, actions aligned per block (--check, --stdout)
keycook actions GoTo               # search the action manifest
keycook lsp                        # language server on stdio (used by the Zed extension)
```

## The language

```
@modal = vim_mode == helix_normal | vim_mode == helix_select     // alias for a header
@nav = {                        // a body: bindings and blocks, spliced by name
  j k: menu::SelectNext menu::SelectPrevious
}
ProjectPanel {
  @nav                          // splice; a later line in the same block overrides silently
  j: vim::MenuSelectNext
}

Editor {                        // context block: header is a Zed predicate fragment
  escape: editor::Cancel        // key: action
  cmd-c ctrl-c: editor::Copy    // several keys, one action; that is the only list form
  "g g": vim::StartOfDocument   // a quoted key with spaces is a sequence
  1..9: vim::Number($)          // range; $ is the value ($-1, $+1 for integers)
  mode == full {                // nested: Editor && mode == full
    @modal {                    // alias: Editor && mode == full && (vim_mode == … || …)
      d: vim::HelixDelete
      space: {                  // prefix block: chords `space f`, `space e`
        f: file_finder::Toggle
        e: project_panel::ToggleFocus
      }
      g: { "r n": null }        // null unbinds
    }
  }
}
Picker > Editor { … }           // `>` is Zed's descendant operator
Workspace {
  > Pane {                      // a `>` header extends the chain: Workspace > Pane
    | Dock {                    // a `|` header widens the enclosing header: Workspace > (Pane || Dock)
      > Terminal { … }          // Workspace > (Pane || Dock) > Terminal
    }
  }
}
```

Rules the compiler enforces or applies:

- **Bodies versus aliases.** `@x = predicate` is a header alias, used as `@x { … }`. `@x = { … }`
  is a body, used as a bare `@x` line; it can hold blocks too, which nest relative to the use site.
  Bodies share keys across parents that have nothing in common (panels at different depths); `|`
  in a header shares a subtree under alternative ancestors.
- **Nesting is `&&` on one node.** Two node names in one block (`Pane { Editor { … } }`) are an
  error; use `> Editor`. Aliases and `|` groups are parenthesised for you.
- **Later wins at the same node**, exactly as in Zed. Sections are emitted in source order but sorted
  by node depth (number of `>` segments, root first), so a generic `Editor` block never shadows a
  `… > Editor` refinement written above it. Within one node, order your blocks: put `mode == full`
  before `showing_completions` if the latter must win. `--source-order` disables the sort.
- **Prefix nulls.** If a chord's prefix is a leaf in an enclosing block (bare `space` in
  `VimControl`), `prefix: null` is inserted so the chord does not wait one second.
  `--no-auto-null` turns that into a warning.
- **Actions and arguments** are validated against `actions.json` (`zed --dump-all-actions`,
  embedded for Zed 1.21.0; `--manifest` overrides). Deprecated aliases are rewritten.
- **Contexts** are parsed with a port of gpui's predicate grammar and checked against
  `contexts.json`: unknown identifiers are hints, unknown attribute values are warnings.
- **Keystrokes** are checked for Zed's spelling (`escape` not `esc`, `alt-` not `opt-`).
- Warnings: a key rebound in the same block, a chord that waits on a sibling leaf. Hints: a binding
  identical to one in an enclosing same-node block.

## Zed integration

1. `cargo install --path tools/keycook` puts `keycook` on PATH.
2. In Zed run `zed: install dev extension` and pick `tools/keycook/zed-extension`. It registers the
   `keycook` language for `.kc` files, the tree-sitter grammar in `tools/keycook/tree-sitter-keycook`
   (built from this repository at the commit named in `extension.toml`), and `keycook lsp` as the
   language server.
3. Open `keymap.kc`: diagnostics inline; completion is slot-aware: at the start of a line it offers
   named keys and modifiers (`ctrl-` then narrows to `ctrl-enter`, `ctrl-tab`, …) plus context
   identifiers, aliases and bodies; after `key:` it offers actions with docs and argument
   signatures; inside `(` the argument names; after a complete action nothing. Hover on actions and
   context identifiers, the block tree in the outline, and `format document` runs the formatter.
4. Build with `keycook build keymap.kc`; Zed reloads `keymap.json` on save.

## Tests

`cargo test` runs the unit tests and two golden tests: `default-macos.kc` and `vim.kc` (mechanical
transcriptions of Zed 1.21.0's keymaps) must compile to exactly the effective bindings of
`tools/upstream/*.json`. That is the proof that the DSL can express everything Zed's own files do.

## Layout

```
src/parser.rs     tokenizer + parser → ast.rs
src/predicate.rs  port of crates/gpui/src/keymap/context.rs (parse, eval, depth_of)
src/compile.rs    AST → sections, checks, sort, auto-null
src/emit.rs       keymap.json with comments
src/manifest.rs   actions.json loader, argument validation, signatures
src/catalog.rs    contexts.json: nodes, flags, attributes
src/keys.rs       keystroke syntax, ranges
src/lsp.rs        tower-lsp server
zed-extension/    Zed extension (language + language server)
tree-sitter-keycook/  grammar for highlighting and structure
```
