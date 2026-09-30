# Zed keymap contexts (Zed 1.21.0)

Every context identifier Zed 1.21.0 can put on the key-dispatch tree, where it comes from, and how the
`"context"` expression language matches and combines them. Sources: `crates/gpui/src/keymap/context.rs`
(grammar and evaluation), the `key_context` / `dispatch_context` functions of each view, `docs/src/key-bindings.md`
and `docs/src/vim.md` at tag `v1.21.0`. The companion file `ZED-ACTIONS.md` lists every action.

To see the live tree at any moment run `dev: open key context view` from the command palette. It prints
exactly the identifiers below, in the order the tree is nested, and also shows the key-equivalent
remapping for your keyboard layout.

## 1. How matching works

### The tree

Each focused element contributes one node. A node is a set of entries: bare **flags** (`Editor`, `menu`)
and **key=value attributes** (`mode=full`, `vim_mode=normal`). The tree is the chain of focused
ancestors, root first:

```
Workspace os=macos keyboard_layout=com.apple.keylayout.QWERTY
  Pane
    Editor mode=full extension=md vim_mode=normal vim_operator=none os=macos VimControl
```

```
Workspace os=macos
  Dock os=macos
    ProjectPanel menu not_editing os=macos
```

Only the *focused* chain exists. A hidden panel, or a visible one that does not have focus, is not in
the tree, which is why `left_dock == …` attributes exist on `Workspace` (see §2.1).

### Grammar of a `"context"` string

| Syntax | Meaning |
|---|---|
| `Name` | the node has flag `Name` (or has an attribute with that key, any value) |
| `key == value` | the node has attribute `key` with exactly `value` |
| `key != value` | the node has no such attribute, **or** it has a different value (a missing key counts as true) |
| `!X` | `X` matches **no node anywhere in the whole tree** (not just this node) |
| `X && Y`, `X \|\| Y` | boolean and / or, evaluated on the same node |
| `A > B` | some ancestor node matches `A` and the subtree under it matches `B` |
| `( … )` | grouping |

Identifier characters are letters, digits, `_` and `-`. The vim operator characters `>`, `<`, `~`, `"`, `?`
are also allowed inside an identifier so that `vim_operator == >` and `vim_operator == g~` parse. Both
sides of `==` / `!=` must be plain identifiers (no quotes, no spaces inside values).

Operator precedence, lowest to highest, from the parser constants:

1. `>` (ancestor)
2. `||`
3. `&&`
4. `==`, `!=`
5. `!`

So `A > B && C` means `A > (B && C)`, `A || B && C` means `A || (B && C)`, and `!A && B` means `(!A) && B`.
Use parentheses whenever an `||` sits next to an `&&`; the defaults do, e.g.
`"Editor && (showing_code_actions || showing_completions)"`.

### Which node an expression is tested on

An expression is matched at the **deepest node that satisfies it**. Zed walks from the leaf up:
`Editor && vim_mode == normal` is tested on the Editor node; `Workspace` on the Workspace node; a section
with no `"context"` matches at the leaf. That depth decides precedence (next section).

Attributes are only visible on the node that sets them. `Workspace && vim_mode == normal` can never match,
because `vim_mode` lives on the Editor node. Cross-level conditions need `>`:
`debugger_stopped > vim_mode == normal` (Workspace attribute, then Editor attribute).

`!X` is special: it scans every node in the tree, so `!Editor` means "no Editor is focused anywhere",
and `!menu` means "no list, completion popup or panel with the `menu` flag is in the chain". Before Zed
0.197 `!` looked at one node only; expressions written for older Zed may now behave differently.

`A > B` takes the **first node from the root** that matches `A` and then tests `B` against the subtree
below it. If that subtree does not match `B` the whole expression fails; Zed does not try a deeper
ancestor. Inside `B`, a `!X` only scans that subtree. With `Workspace > Pane > Editor` the expression
`Workspace > Editor` matches (ancestor, not parent), and `Pane > Editor` does too.

### Precedence between bindings

1. **Deeper wins.** A binding whose context matched at a deeper node beats one matched higher up.
   `Editor` beats `Pane` beats `Workspace`. A section without a context matches at the leaf, so it beats
   everything.
2. **Later wins at equal depth.** Sections lower in the file override earlier ones. Your `keymap.json`
   is loaded after the defaults, so a user section with the same context as a default section wins.
3. `null` follows the same rules and blanks the key at that depth and above.
4. Multi-key chords: when one binding is a prefix of another and both are live, Zed waits 1 second for
   the rest. Binding the prefix alone to `null` (or nothing) removes the timeout. This constant is not
   configurable in 1.21; the which-key popup only changes what is shown, not the wait.
5. Some actions are conditional and propagate to the next binding when they decline. A `null` at a
   deeper level stops that fallback.

### Recipes for combining

| Goal | Expression |
|---|---|
| any editor, including single-line inputs | `Editor` |
| only real code buffers | `Editor && mode == full` |
| code buffer, not a picker or search input | `Editor && mode == full && !menu` |
| vim normal or visual, no popup | `VimControl && !menu` (what `vim.json` uses) |
| vim normal only | `vim_mode == normal` (implies Editor) |
| insert mode, no completion list | `vim_mode == insert && !(showing_code_actions \|\| showing_completions)` |
| while an operator is pending (`d`, `c`, …) | `vim_mode == operator && vim_operator == d` |
| Helix modes | `helix_mode` or `vim_mode == helix_normal \|\| vim_mode == helix_select` |
| everywhere except editors and terminals | `!Editor && !Terminal` |
| all list-like panels | `Dock \|\| Workspace \|\| OutlinePanel \|\| ProjectPanel \|\| CollabPanel` |
| the explorer while not renaming | `ProjectPanel && not_editing` |
| the search bar's text input | `BufferSearchBar > Editor`; replace box: `BufferSearchBar && in_replace > Editor` |
| a picker's query input | `Picker > Editor`; a specific picker: `FileFinder > Picker > Editor` |
| the picker whether its list or input is focused | `FileFinder \|\| (FileFinder > Picker > Editor)` |
| git panel changes list | `GitPanel && ChangesList` |
| git commit message editor | `GitPanel && CommitEditor` or `CommitEditor > Editor` |
| agent input box | `AcpThread > Editor` |
| only on macOS / not on macOS | `os == macos`, `os != macos` (works on every node) |
| by file type | `Editor && extension == md` |
| when the debugger is paused, in vim normal | `debugger_stopped > vim_mode == normal` |
| while a snippet tabstop is active | `Editor && in_snippet && has_next_tabstop && !showing_completions` |
| when an edit prediction is showing | `Editor && edit_prediction` (and `edit_prediction_mode == eager` for the no-modifier variant) |

Tips:

- Put the most specific node first for readability (`GitPanel && ChangesList`), but order does not
  affect matching.
- Never add `!menu` to a binding for a panel that carries `menu` itself (ProjectPanel, OutlinePanel,
  CollabPanel, GitPanel list, ThreadsSidebar, KeymapEditor, Onboarding, the settings NavigationMenu).
- `Editor` alone also matches the search bar, pickers, the commit box and the agent input. Add
  `mode == full` (or `!menu`, or a `>` parent) to keep bindings out of them.
- A leader key that is also a printable character (space, comma) must be bound to `null` in every context
  where it is a chord prefix, or the chord will time out after a second. Vim mode already does this for `space`.

## 2. Catalog of contexts

Flags are shown bare, attributes as `key = values`. "Set by" names the view that adds them. An entry is
present only under the condition given.

### 2.1 Workspace level

Node `Workspace` (`crates/workspace/src/workspace.rs`), always the root.

| Entry | Present when |
|---|---|
| `Workspace` | always |
| `os = macos \| linux \| windows \| unknown` | always; also on every other node built with defaults (Pane, Editor, panels, pickers) |
| `keyboard_layout = <layout id>` | always, e.g. `com.apple.keylayout.QWERTY`, `com.apple.keylayout.ABC` |
| `debugger_running` | a debug thread is running or stepping |
| `debugger_stopped` | a debug thread is stopped at a breakpoint |
| `debugger_session` | any live session (running, stepping or stopped) |
| `left_dock = <panel key>` | the left dock is open; value is the active panel's key |
| `right_dock = <panel key>` | the right dock is open |
| `bottom_dock = <panel key>` | the bottom dock is open |

Panel keys used by the dock attributes: `ProjectPanel`, `GitPanel`, `OutlinePanel`, `TerminalPanel`,
`DebugPanel`, `CollaborationPanel`, `agent_panel`. Example: `Workspace && right_dock == ProjectPanel`.

### 2.2 Pane and dock nodes

| Node | Entries | Set by |
|---|---|---|
| `Pane` | `EmptyPane` when the pane has no item | `crates/workspace/src/pane.rs` |
| `Dock` | none | `crates/workspace/src/dock.rs`; sits between Workspace and a panel |
| `Pane` + `RunModal` | debugger "new session" modal | `crates/debugger_ui/src/new_process_modal.rs` |
| `Pane` + `GitPicker` | plus `GitBranchSelector` or `StashList` for the active tab | `crates/git_ui/src/git_picker.rs` |

### 2.3 Editor

Node `Editor` (`crates/editor/src/editor.rs`). Every text input in Zed is an Editor, so the attributes
below are what separates a buffer from a one-line field.

| Entry | Present when |
|---|---|
| `Editor` | always |
| `mode = full \| auto_height \| single_line \| minimap` | always. `full` is a code buffer; `auto_height` is a growing box (commit message, agent input, inline assistant); `single_line` is a search or picker field |
| `extension = <lowercase file extension>` | singleton buffer backed by a file with an extension (`md`, `rs`, `svg`, `csv` …) |
| `multibuffer` | the editor shows excerpts (search results, diagnostics, project diff) |
| `jupyter` | the `jupyter` editor setting is enabled |
| `renaming` | an LSP rename box is open |
| `inline_input` | an inline input (e.g. vim `:` prompt) is pending |
| `in_snippet` | a snippet is active; with `has_previous_tabstop` / `has_next_tabstop` |
| `menu` + `showing_completions` | the completion list is visible |
| `menu` + `showing_code_actions` | the code actions list is visible |
| `showing_signature_help` | signature help with more than one signature is open |
| `edit_prediction` and `copilot_suggestion` | an edit prediction (inline completion) is displayed; `copilot_suggestion` is the legacy alias |
| `edit_prediction_mode = eager \| subtle` | always; `subtle` when predictions need a modifier to accept |
| `in_leading_whitespace` | the cursor is in leading whitespace (used for `tab` accepting predictions) |
| `selection_mode` | vim/emacs style mark selection mode is on |
| `multiple_selections` | more than one cursor |
| `start_of_input` / `end_of_input` | single-line or auto-height editor, one empty selection, at the very start / end |
| `diffs_expanded` | at least one diff hunk is expanded inline |
| `in_preview` | the editor is a preview tab |
| `os = …` | always |

Addons extend the Editor node when the editor is focused:

| Entry | Set by |
|---|---|
| `agent_diff` | an editor whose buffer is under agent review (`crates/agent_ui/src/agent_diff.rs`) |
| `editor_agent_diff` | same, for the editor form (not the AgentDiff pane) |
| `use_modifier_to_send` | agent message editor, when `agent.use_modifier_to_send` is set |
| vim entries | see §2.4 |

### 2.4 Vim and Helix (added to the Editor node)

From `crates/vim/src/vim.rs`. These exist only with `vim_mode` or `helix_mode` enabled in settings, and
only on the Editor node.

| Entry | Values / present when |
|---|---|
| `vim_mode` | `normal`, `visual` (covers visual, visual line, visual block), `insert`, `replace`, `helix_normal`, `helix_select`, `operator` (an operator is pending and the next key completes it), `waiting` (waiting for an arbitrary character, e.g. after `f`, `t`, `r`, `m`, or after Helix `[` / `]`), `literal` (after `ctrl-v` in insert mode) |
| `vim_operator` | `none`, or the pending operator's id while `vim_mode` is `operator` or (for Helix `[`/`]`) `waiting`. Ids: `i`, `a` (text objects), `c`, `d`, `y`, `r`, `^K` (digraph), `^V` (literal), `f`, `t`, `F`, `T`, `s`, `S` (sneak), `ys`, `cs`, `ds` (surround), `m` (mark), `'`, `` ` `` (jump), `>`, `<`, `eq` (`=` auto-indent), `sh` (`!` shell), `gq`, `gR`, `cx`, `gU`, `gu`, `g~`, `g?`, `"` (register), `q`, `@`, `gc`, `gb`, `gw`, `helix_m`, `helix_next`, `helix_previous`, `helix_ms`, `helix_mr`, `helix_md` |
| `VimControl` | `vim_mode` is `normal`, `visual`, `operator`, `helix_normal` or `helix_select` (vim keys should work) |
| `VimCount` | a count has been typed (before an operator, or after one) |
| `helix_mode` | the underlying mode is a Helix mode, even while `vim_mode` reads `operator` or `waiting` |

Note the difference: `vim_mode == waiting` has no `VimControl`, so `g …` chords do not fire there.

### 2.5 Terminal

Node `Terminal` (`crates/terminal_view/src/terminal_view.rs`).

| Entry | Present when |
|---|---|
| `Terminal` | always |
| `vi_mode` | terminal vi (copy) mode is on |
| `screen = normal \| alt` | always; `alt` while a full-screen program (vim, less, htop) owns the screen |
| `selection` | text is selected |
| `DECCKM` | application cursor keys |
| `DECPAM` / `DECPNM` | application / numeric keypad (exactly one is present) |
| `DECTCEM` | cursor visible |
| `DECAWM` | auto-wrap |
| `DECOM` | origin mode |
| `IRM` | insert mode |
| `LNM` | line-feed / new-line mode |
| `report_focus` | the program asked for focus in/out events |
| `alternate_scroll` | alternate scroll mode |
| `bracketed_paste` | bracketed paste enabled |
| `any_mouse_reporting` | any mouse mode active |
| `mouse_reporting = off \| click \| drag \| motion` | always |
| `mouse_format = normal \| utf8 \| sgr` | always |

Panel wrapper: the terminal panel's dock node is `Dock`, and the agent panel's terminals appear as
`AgentPanel > Terminal`.

### 2.6 Panels in docks

All of these are built from defaults, so they also carry `os`.

| Node | Extra entries |
|---|---|
| `ProjectPanel` | `menu`; `editing` (filename box focused) or `not_editing` |
| `OutlinePanel` | `menu`; `editing` (filter box focused) or `not_editing` |
| `CollabPanel` | `menu`; `editing` (channel name or filter focused) or `not_editing` |
| `GitPanel` | `CommitEditor` when the commit box is focused; otherwise `menu` plus `ChangesList` or `HistoryList` for the active tab. The branch and repository popups are separate nodes `GitBranchSelector` and `GitRepositorySelector`; the defaults exclude them tree-wide with `!GitBranchSelector && !GitRepositorySelector` |
| `AgentPanel` | none; children: `AcpThread`, `MessageEditor`, `ModeSelector`, `Terminal` |
| `DebugPanel` | none; children `VariableList`, `BreakpointList`, `DebugConsole`, `DebugSessionItem` |
| `ThreadsSidebar` | `menu`; then `searching`, `editing` (renaming) or `not_searching` |
| `Panel` | generic wrapper element name used by some panel bodies |

### 2.7 Search bars and pickers

| Node | Extra entries |
|---|---|
| `BufferSearchBar` | `in_replace` when the replace field is focused. Query and replace fields are child `Editor` nodes (`BufferSearchBar > Editor`). Also used by the keymap editor, extensions page and agent registry for their search fields |
| `ProjectSearchBar` | `in_replace` likewise |
| `ProjectSearchView` | results area of a project search (only when it has matches) |
| `GitGraphSearchBar` | search field in the git graph |
| `AcpThreadSearchBar` | find-in-thread bar in the agent panel |
| `TextFinder` | find inside search results |
| `Picker` | `with_preview` when the picker shows a preview pane. The query box is `Picker > Editor` |

Named pickers and modals that wrap a `Picker` (bind with `Name || (Name > Picker > Editor)` to cover
both list and input): `FileFinder`, `CommandPalette`, `RecentProjects`, `WorktreePicker`, `GitBranchSelector`,
`StashList`, `TextFinder`, `ChannelModal`, `LspCommandSelector`, `CallHierarchyPicker`, `ToolchainSelector`,
`ThemeSelector`, `IconThemeSelector`, `LanguageSelector`, `EncodingSelector`, `TasksModal`, `GoToLine`,
`TabSwitcher`, `ThreadSwitcher`, `ProjectPickerModal`, `RefPickerModal`, `SidebarRecentProjects`.

### 2.8 Other named views

These nodes are plain names added with `.key_context("Name")`; they carry no attributes unless listed.

| Area | Nodes |
|---|---|
| Git | `GitCommit` (+ child Editor, `auto_height`), `GitDiff`, `StashDiff`, `FileHistoryView`, `GitGraph`, `GitRepositorySelector`, `RenameBranchModal`, `StashMessageModal`, `AskPass` (+ child Editor), `PasswordPrompt` |
| Agent | `AcpThread` (+ `AcpThread > Editor` with `start_of_input`, `end_of_input`, `use_modifier_to_send`), `MessageEditor`, `ModeSelector`, `AgentDiff`, `AgentFeedbackMessageEditor`, `ThreadHistory`, `ThreadsArchiveView`, `ThreadImportModal`, `ManageProfilesModal`, `ConfigureContextServerModal`, `AddContextMenu`, `SkillCreator`, `InlineAssistant` (+ child Editor) |
| Debugger | `VariableList`, `BreakpointList`, `DebugConsole` (+ child Editor), `DebugSessionItem`, `AttachModal`, `RunModal` |
| Notebook | `NotebookEditor` with `notebook_mode = command \| edit`; cells are `NotebookEditor > Editor` |
| Markdown / preview | `Markdown` (rendered markdown element), `MarkdownPreview`, `SvgPreview`, `ImageViewer`, `HighlightsTreeView`, `ComponentPreview`, `ThemePreview` |
| Settings and keymap | `SettingsWindow`, `SettingsWindow > NavigationMenu` (`menu`, plus `search` when the filter is focused), `KeymapEditor` (`menu`), `KeybindEditorModal` (+ `showing_completions`), `KeystrokeInput`, `KeyContextView` |
| Onboarding / misc | `Welcome`, `Onboarding` (`menu`), `OnboardingAiConfigurationModal`, `ZedPredictModal`, `RatePredictionModal`, `RatePredictionsModal`, `EditPredictionContext`, `Diagnostics` (+ child Editor), `InvalidBuffer`, `InvalidItem`, `Prompt`, `OpenUrlModal`, `SecurityModal`, `SshConnectionModal`, `RemoteServerModal`, `ContainerModal`, `CallStatsModal`, `SharedScreen`, `ApplicationMenu`, `NestedMenu`, `Popover`, `StatusBar`, `TitleEditor`, `TextInput`, `Root` |

### 2.9 The `menu` flag

`menu` is the one flag shared across many nodes. It marks "a list that consumes up/down/enter/escape".
It is set by: Editor (completion or code-action list visible), ProjectPanel, OutlinePanel, CollabPanel,
GitPanel (Changes/History list), ThreadsSidebar, KeymapEditor, Onboarding, the settings NavigationMenu,
and generic pickers (`Picker || menu` in the defaults). `!menu` therefore means "no list is open anywhere",
which is why vim's normal-mode keys are bound with `VimControl && !menu`.

## 3. Debugging a context

1. Run `dev: open key context view`. The top shows the live tree; press keys to see which binding
   matched and at which depth.
2. `zed: open default keymap` shows the contexts the defaults use; copy the exact expression when
   overriding.
3. A red "Errors in user keymap file" banner means a context failed to parse (unbalanced parentheses,
   a quoted value, or `==` against something that is not an identifier) or an action name is unknown.
   Zed keeps the last good keymap until the file is fixed.
4. If a binding "works in the wrong places", the usual cause is a bare `Editor` context that also
   matches search fields, pickers and the commit box. Narrow with `mode == full` or a `>` parent.
5. If a chord is slow, a shorter binding with the same prefix is live at the same time. Find it in the
   default keymap and bind the prefix to `null` in your context.
