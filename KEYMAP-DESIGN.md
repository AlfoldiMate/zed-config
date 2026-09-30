# Keymap design: modal first, Helix base, semantic everywhere

Working notes for the next Zed keymap. This file records the idea, the constraints we found in Zed 1.21,
and the plan to build it. Reference material: `ZED-CONTEXTS.md` (every context and the matching rules),
`ZED-ACTIONS.md` (every action with arguments), `ZED-KEYMAP-REDUNDANCY.md` (what the defaults duplicate),
and `tools/keymap-atlas.html` (the explorer and rule editor).

## 1. The idea

- **Modal first, no plain-editor defaults.** The keymap is written for a modal user. The macOS and Emacs
  style bindings that Zed ships (`ctrl-n` for down, `cmd-shift-left` for select, four ways to cancel a
  menu) are not carried over. What remains of the default keymap is taken on purpose, by rule.
- **Strong like Neovim, fresh like Helix.** Helix's selection-first model and its minor-mode menus
  (`g` goto, `m` match, `z` view, `space`, `[` / `]`) are the base. Vim is the fallback for what Zed's
  Helix emulation does not implement yet, and for the operator grammar where it is simply better.
- **Semantic where it suits.** A key means the same thing in every view where that meaning exists.
  `j` is "next item" in a buffer, a list panel, a picker and a menu. `d` deletes the thing under the
  cursor whether it is a selection, a file in the explorer or a hunk in the git panel. Views that have no
  such action simply do not bind the key.
- **The same modal set everywhere possible.** Buffers get the full Helix set. List panels get the
  navigation, goto and action subset. Pickers get what their single-line input allows. The terminal
  keeps its own keys.
- **Goto is the navigation menu.** Helix's `g` prefix is extended meaningfully rather than adding
  another leader.
- **The right-click menu is the discoverable layer.** Rarely used commands are reached by opening
  the context menu with one key and then pressing the same semantic letter the item would have as a
  direct binding. This works in every view that has a context menu.

## 2. What Zed 1.21 gives us and where it pushes back

Facts from the source that shape the design (details in `ZED-CONTEXTS.md`).

- **Contexts are a focus chain plus state flags, not an inheritance tree.** The chain is
  `Workspace > Pane > Editor` for buffers and `Workspace > Dock > <Panel>` for panels. Flags such as
  `vim_mode`, `menu`, `in_replace` live on one node. Deeper wins, then later in file. `!X` scans the
  whole chain. This is what the atlas evaluates.
- **Helix mode is an emulation on top of vim mode.** It is marked work in progress in Zed's docs.
  The Helix contexts are `vim_mode == helix_normal`, `vim_mode == helix_select` and the `helix_mode`
  flag, which stays set while an operator is pending. Text objects after `m i` / `m a` also work with
  `]` / `[`. Whatever Helix key Zed lacks has to be expressed with a vim action or left out.
- **Every text input is an `Editor`.** The picker query, the search bar, the commit message and the
  agent box are all editors, in `single_line` or `auto_height` mode. A binding in a bare `Editor`
  context reaches all of them. `mode == full` isolates real buffers.
- **List panels carry `menu`.** ProjectPanel, OutlinePanel, CollabPanel, the GitPanel lists, the
  threads sidebar and pickers dispatch `menu::SelectNext`, `menu::Confirm` and friends. That is the
  hook for a shared navigation set: bind `j` / `k` / `enter` / `escape` to `menu::*` in a context that
  covers all list-like nodes, and every list follows.
- **Context menus are keyable.** The context menu component runs under a `menu` key context and
  handles `menu::*` navigation. It also dispatches a menu item when the item's own bound keystroke is
  pressed while the menu is open (`ContextMenu::on_action_dispatch` in `crates/ui`). Openers exist
  for the editor (`editor::OpenContextMenu`) and the explorer (`project_panel::OpenContextMenu`);
  other views need checking.
- **Chords wait one second when the prefix is also a binding.** Not configurable. A leader or a
  prefix key must be bound to `null` in every context where it is a prefix, or the popup times out.
  Prefixes that are also verbs (`g`, `m`, `z`) must never be leaves.
- **Vim keys in pickers are limited.** The picker input is an editor in insert mode, so plain letters
  type. Modal navigation in pickers is `ctrl-` keys or the `menu::*` set on the picker list. The atlas
  can show whether a `vim_mode == normal` chain in `Picker > Editor` matches anything.
- **The terminal has no modal layer of ours.** It has its own `vi_mode` flag. Leave it alone.

## 3. Architecture: layers, intents, rules

The keymap is generated, not written. Three inputs feed a generator that emits `keymap.json`.

```
Helix keymap (intent list, by mode and prefix)
        │
        ▼
intent matrix   intent × view family → Zed action (or none)
        │
        ▼
rules           take / drop / bind / unbind, in order
        │
        ▼
keymap.json     one section per context, no repeats, chord report clean
```

### Layers, from the bottom

| Layer | Content | Source |
|---|---|---|
| L0 chrome | `menu::*` navigation, picker plumbing, search-field keys, save, close, quit | taken from `default-macos.json` by rule |
| L1 Helix base | normal and select mode, `g` `m` `z` `[` `]` prefixes, insert-mode escapes | Helix contexts of `vim.json`, gaps filled with vim actions |
| L2 vim fallback | operators and motions Helix mode lacks in Zed, `:` commands | `vim.json` by rule, narrowed to what L1 does not cover |
| L3 semantic views | the same letters in explorer, git panel, outline, agent, pickers | generated from the intent matrix |
| L4 context menu | one opener key per view, item letters that match L3 | generated from the intent matrix, `menu` contexts |

### Intents

An intent is a name for a meaning, independent of view: `move.down`, `select.line`, `goto.definition`,
`item.delete`, `item.rename`, `item.open.split`, `panel.git`. Helix's keymap page is the intent list for
buffers. The view families are: buffer, list panel, picker, git panel, agent thread, terminal.

The matrix is a table intent × family with a Zed action in each cell, or empty. Empty cells are not
errors; they mean "this view does not have that meaning". A cell that should be filled but is not is a
gap to look for in `ZED-ACTIONS.md`.

### Rules

The atlas rule editor already runs `take`, `drop`, `bind`, `unbind`. The intent layer adds one rule:

```jsonc
{ "intent": "item.delete", "key": "d", "modes": ["helix_normal", "list"] }
```

The generator expands it to one binding per family that has the intent, with the context expression
built from the family (`vim_mode == helix_normal` for buffers, `ProjectPanel && not_editing` and the
other list nodes for lists). One rule, one key, no hand-written contexts, no repeats.

## 4. The base modal set

Taken from the Helix keymap page and mapped to what Zed implements. The first pass is "take every
Helix binding `vim.json` already has, then list the missing ones and decide vim fallback or drop".

| Helix group | Zed status | Plan |
|---|---|---|
| Movement (`h j k l w b e W B E f t F T`, `gg` `ge` `gh` `gl` `gs`, `ctrl-d/u`, `%`) | mostly present in helix contexts | take |
| Changes (`d c y p P r R J u U .`, `>` `<` `=`, `ctrl-a/x`, `~` `` ` `` `alt-`` `) | present via vim actions with Helix semantics | take, verify `alt-` variants |
| Selection (`s S ; alt-; , x X C alt-C ( ) alt-( alt-) &`) | partial (`x`, `s`, `;`, `,`, `C` exist) | take what exists, list the rest |
| Search (`/ ? n N *`) | present | take |
| Insert mode (`i a I A o O`, insert-mode `ctrl-` keys) | present | take letters, drop most `ctrl-` editing keys |
| Goto `g` | see section 5 | extend |
| Match `m` (`mm mi ma ms mr md`) | present (`helix_m`, `helix_ms`, `helix_mr`, `helix_md`) | take |
| View `z` (`zz zt zb zj zk`, `Z` sticky) | partial | take, add vim `z` equivalents |
| Bracket `[` `]` (`d D f t a c p g ...`) | partial (`helix_next`, `helix_previous`) | take, fill with `editor::GoTo*` |
| Space menu (`space f F b s S j k d r a h w /`) | not part of Zed's Helix layer | build as L3, see section 6 |
| Window `ctrl-w` | vim's `ctrl-w` set exists | take, expose also as `space w` |
| Popup `ctrl-p/n` in menus | menu:: set | L0 |

`ZED-KEYMAP-REDUNDANCY.md` shows where `vim.json` gives one action several keys (`ctrl-w h`,
`ctrl-w ctrl-h`, `ctrl-w left`, `space w h`). The rule set keeps one spelling per intent and mode.

## 5. Goto, extended meaningfully

Helix's `g` is "go to a place". Keep that meaning strict: every `g` key moves the cursor or the focus,
nothing else. Proposal for buffers, with Zed actions:

| Key | Meaning | Zed action |
|---|---|---|
| `g g` / `g e` | file start / end | `vim::StartOfDocument` / `vim::EndOfDocument` |
| `g h` / `g l` / `g s` | line start / end / first non-blank | `vim::StartOfLine` / `vim::EndOfLine` / `vim::FirstNonWhitespace` |
| `g d` / `g D` | definition / declaration | `editor::GoToDefinition` / `editor::GoToDeclaration` |
| `g y` / `g i` / `g r` | type definition / implementation / references | `editor::GoToTypeDefinition` / `editor::GoToImplementation` / `editor::FindAllReferences` |
| `g a` | last accessed file | `pane::AlternateFile` |
| `g m` | last modified file | `vim::HelixGotoLastModification` |
| `g n` / `g p` | next / previous buffer | `pane::ActivateNextItem` / `pane::ActivatePreviousItem` |
| `g .` | last modification position | `vim::HelixGotoLastModification` |
| `g w` | word with labels | `vim::HelixJumpToWord` |
| `g f` | file under cursor | `editor::OpenSelectedFilename` |
| `g x` | URL under cursor | `editor::OpenUrl` |
| `g c` / `g C` | next / previous change (hunk) | `editor::GoToHunk` / `editor::GoToPreviousHunk` |
| `g t` / `g b` | top / bottom of the screen | `vim::WindowTop` / `vim::WindowBottom` |
| `g o` | symbol in file (outline picker) | `outline::Toggle` |
| `g O` | symbol in project | `project_symbols::Toggle` |
| `g /` | grep the project | `pane::DeploySearch` |

Extension across views, same letters where the meaning holds:

- **Explorer and outline**: `g g` / `g e` first and last entry, `g h` collapse to parent
  (`project_panel::CollapseSelectedEntry`), `g .` reveal current file (`pane::RevealInProjectPanel`).
- **Git panel**: `g g` / `g e` first and last entry, `g c` next hunk in the diff, `g o` open the file
  (`git::OpenFileDiff` for the diff, `menu::Confirm` for the file).
- **Agent thread**: `g g` / `g e` start and end of output (`agent::ScrollOutputToTop`, `agent::ScrollOutputToBottom`).
- **Pickers**: `g g` / `g e` via `menu::SelectFirst` / `menu::SelectLast` if the input allows a `g` prefix;
  otherwise `ctrl-g` variants.

Rule: `g` is never a leaf anywhere, so it never waits. The atlas flags any leaf `g`.

## 6. Space and the context menu as the discoverable layer

Two menus, two jobs:

- **`space` is Helix's space menu**, extended with the LazyVim groups that proved useful in the current
  layer (`f` files, `b` buffers, `g` git, `s` search, `w` windows, `x` diagnostics, `u` toggles, `a`
  agent). Frequent commands live here. The which-key popup shows them. Bare `space` is `null` in every
  context that has the menu.
- **The context menu holds the rare commands.** One key opens the view's context menu at the cursor
  or selected item. Inside it, `j` / `k` / `enter` / `escape` navigate through `menu::*`, and each item
  can be triggered by the letter the same command would have as a direct binding, because the menu
  dispatches an item when its bound keystroke is pressed. So the letters are the same semantic
  alphabet, only reached through the menu.

Opener key proposal: `.` in normal mode where `.` is not repeat (Helix uses `.` for repeat, so in
buffers use `alt-.` or `space .`), and plain `.` in list panels where nothing repeats. Openers found so
far: `editor::OpenContextMenu`, `project_panel::OpenContextMenu`. Git panel, outline and agent views
need checking; where no opener exists the L4 layer for that view is empty.

Design rule for L4: a command gets a context-menu letter only if it is in the view's context menu. If
it is frequent, it moves to `space` or a direct key instead. That keeps the direct layer small.

## 7. The semantic alphabet

The letters that keep one meaning across views. Buffers use the Helix meaning; other views use the
closest item-level meaning.

| Key | Buffer (Helix) | List panels and pickers | Git panel |
|---|---|---|---|
| `j` `k` | line down / up | next / previous item | next / previous entry |
| `h` `l` | char left / right | collapse / expand | collapse / expand |
| `enter` | (insert newline in insert mode) | open / confirm | open diff |
| `d` | delete selection | delete entry (trash) | discard change |
| `y` | yank | copy path | copy commit hash |
| `p` | paste | paste entry | pull |
| `r` | replace char | rename | revert |
| `c` | change | (none) | commit |
| `s` | select regex | (none) | stage / unstage |
| `o` | open line below | open in split | open file |
| `a` | append | new file | amend |
| `/` | search | filter | filter |
| `.` | repeat | context menu | context menu |
| `g …` | goto | goto | goto |
| `space …` | space menu | space menu | space menu |
| `escape` | normal mode / collapse selection | cancel / close | cancel |

Conflicts are resolved by the intent matrix, not by hand: a letter that would mean two different things
in one view is a gap to decide, and the atlas report lists it.

## 8. Redundancy policy

- One key per intent per mode. No `ctrl-` Emacs aliases, no `cmd-` duplicates of modal keys, arrows only
  in insert mode and in inputs.
- A default binding enters the keymap only through a `take` rule with a reason in a `comment` rule.
- No key is both a leaf and a prefix in one context. Prefixes are `null` leaves where needed.
- Two contexts that only differ by flags share a binding only if the intent is identical; otherwise the
  binding is written once in the more general context.
- The atlas rule report must be clean: no overlapping repeats, no chord waits, no unknown actions,
  every context parses.

## 9. Tooling status and next steps

Done:

- `tools/keycook`: the DSL compiler, checks, language server and Zed extension; `keymap.kc` is the
  whole keymap in it.
- `ZED-CONTEXTS.md`, `ZED-ACTIONS.md`, `ZED-KEYMAP-REDUNDANCY.md` from the 1.21.0 source and binary.
- `tools/keymap-atlas.html`: focus tree with state flags, effective keymap per chain with shadowing and
  chord warnings, binding explorer, and the rule editor with report and keymap export.

Next:

1. **Intent list.** Transcribe the Helix keymap page into intents grouped by mode and prefix. Mark each
   as present in Zed's Helix layer, present via vim, or missing.
2. **Intent matrix.** Add the view families and fill buffer, list panel and git panel columns from
   `ZED-ACTIONS.md`. Add the matrix view to the atlas.
3. **Intent rule.** Add the `intent` rule to the atlas expander and generate contexts per family.
4. **Verify in Zed**, with the atlas next to it: `base_keymap` set to `None` plus `helix_mode`, and
   check that `vim.json` still loads; whether the picker input accepts a `g` prefix; which views have a
   context menu opener; whether menu items really trigger on their bound letter.
5. **Generate, load, iterate.** Copy the export into `keymap.json`, watch the which-key popup and the
   pending-keystroke indicator, fix by rule, regenerate.

Open questions to settle before step 5:

- Settled: `base_keymap: "None"` returns early in `load_default_keymap`, so it removes the chrome **and** vim.json. The keymap therefore uses `None` and carries what it needs itself: `tools/build-keymap.py` filters the two upstream files by take/drop rules and appends `tools/overlay.jsonc`. Upstream copies live in `tools/upstream/` so a Zed upgrade is a diff of those two files.
- Does Zed's Helix layer honour `x` extend-line and `;` collapse in `helix_select` the same as in
  `helix_normal`?
- Is there an opener action for the git panel and outline context menus, or only mouse?
- How much of the LazyVim `space` layer in `keymap.json.bak` survives as-is once it is expressed as
  intents.
