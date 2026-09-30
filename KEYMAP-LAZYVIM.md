# Zed keymap guide (LazyVim layout, Zed 1.21)

Leader is `space`. Press it and wait: the which-key popup lists what is available *where you are*.
The menu is context aware: a buffer, the file explorer and the git panel each show their own set.

Files: `keymap.json` (bindings), `settings.json` (vim + which-key + editor feel), this guide.

## Mental model

- **Vim first.** Everything Zed's vim mode ships still works (`ciw`, `ys`, `gcc`, `zc`, `ctrl-w …`, `:` commands).
- **Leader groups are letters you already know from LazyVim:** `b`uffers, `c`ode, `d`ebug, `f`iles, `g`it,
  `s`earch, `t`asks, `u`i, `w`indows, `x` diagnostics, `a`gent, `q`uit.
- **Same key, different place, sensible meaning.** `space f` is "find" in a buffer, but "file operations" in
  the explorer. `space g` is repo actions everywhere, hunk actions in a buffer, staging in the git panel.
- **macOS keys stay.** `cmd-p`, `cmd-shift-p`, `cmd-b`, `cmd-shift-f`, `cmd-s`, `cmd-w`, `cmd-/` all work.

## No-leader keys (buffer)

| Key | Action |
|---|---|
| `ctrl-h/j/k/l` | move between panes (also from the explorer and git panel) |
| `ctrl-arrows` | resize pane |
| `ctrl-s` | save (normal, visual, insert) |
| `ctrl-/` | toggle terminal (also from inside the terminal) |
| `H` / `L`, `[b` / `]b` | previous / next tab, `[B` / `]B` move the tab |
| `alt-j` / `alt-k` | move line or selection (insert mode too, unless a Copilot prediction is showing) |
| `s` / `S` | sneak: jump forward / backward to two characters (flash.nvim). Use `cl` / `cc` for the old `s` / `S` |
| `gw` | labelled jump: every word on screen gets a label, type it to jump there (flash / easymotion style; `gq` still rewraps) |
| `ctrl-space` / `backspace` | grow / shrink selection by syntax node |
| `>` / `<` (visual) | indent and keep the selection |
| `gd gD gr gI gy` | definition, declaration, references, implementation, type definition |
| `K` / `gK` | hover / signature help (`ctrl-k` in insert mode) |
| `gai` / `gao` | incoming / outgoing calls |
| `gsa gsd gsr` | add / delete / replace surrounding (vim's `ys ds cs` still work) |
| `gx` / `gf` | open URL / open file under cursor |
| `]d [d` `]e [e` `]w [w` | next / prev diagnostic, error, warning |
| `]h [h` | next / prev git hunk (`]c [c` too) |
| `]] [[` | next / prev reference of the symbol under the cursor |
| `]f [f` `]F [F` | next / prev function start / end |
| `]q [q` | next / prev excerpt in search results or diagnostics |

## Leader: buffer

| Key | Action |
|---|---|
| `space space` | find files |
| `space ,` | buffer list: `j` / `k` move, `gg` / `G` first / last, `q` closes the selected buffer, `enter` opens |
| `space /` | grep the project |
| `space :` | command palette |
| `` space ` `` | last buffer |
| `space -` / `space \|` | split below / right |
| `space e` / `space E` | focus explorer (press again to hide) / show or hide the right dock, where the explorer lives |
| **b** `b d o p P l r` | other, close, close others, pin, close non-pinned, close left, close right |
| **f** `f b e n p c k t T` | files, buffers, explorer at current file, new file, recent projects, settings, keymap, terminal, new terminal |
| **g** `g d l c p P f b o S` | git panel, project diff, log graph, commit, push, pull, fetch, branches, open modified, stash |
| **s** `g r s S d h R` | grep, search & replace, symbols, workspace symbols, diagnostics, docs, reopen last picker |
| **t** `t l o` | pick task, rerun last, output (terminal) |
| **d** `a c i o O P t u` | start, continue, step into, out, over, pause, stop, debug panel |
| **u** `b C z Z` | dark/light, theme picker, zen (centered), zoom pane |
| **w** `h j k l H J K L s v d o m =` | focus, swap, split, close, close others, zoom, equalize |
| `space x x` | project diagnostics |
| **a** `a n` | agent panel, new thread |
| **q** `q w` | quit Zed, close window |

## Leader: buffer (continued)

| Key | Action |
|---|---|
| `space K` | hover |
| **c** `a r f o d s l p` | code action, rename, format, organize imports, line diagnostics, symbols sidebar, LSP info, markdown preview |
| `space g B` / `space g Y` | open / copy permalink to the line |
| **gh** `s u r S R p d b B` | stage hunk, unstage hunk, revert hunk, stage file, revert file, preview hunk, expand all hunks, blame line, blame buffer |
| `space s b` / `space s w` | search in buffer / grep word (visual: grep selection) |
| `space t r` | run nearest task |
| `space d b` / `space d B` | toggle breakpoint / log breakpoint |
| **u** `w l L h d g G m e` | wrap, line numbers, relative numbers, inlay hints, diagnostics, indent guides, inline blame, minimap, edit predictions |
| `space x X` | diagnostics (same view; buffer filter is a Zed limitation) |
| `space c f` (visual) | format selection |
| `space a e` (visual) | edit selection with the agent |

## Leader: panels (explorer, git, outline, debug, agent)

Panels get navigation only, so the popup stays short:

| Key | Action |
|---|---|
| `space space` `space ,` `space /` `space :` | files, buffers, grep, commands |
| `space e` / `space E` | focus explorer / show or hide it |
| `space g g` / `space f t` / `space a a` | git panel / terminal / agent panel |
| `space w h j k l` / `space w d` | move to a pane / close this panel |
| `space q q` | quit |

## Leader: explorer only (`space e`, then keys)

| Key | Action |
|---|---|
| **f** `n d r x` | new file, new directory, rename, move to trash |
| **f** `y Y` | copy path, copy relative path |
| **f** `o R /` | open with system app, reveal in Finder, search in this directory |
| **f** `v s` | open in vertical / horizontal split |
| **f** `h c` | toggle gitignored files, collapse all |

Zed's own explorer keys still apply: `a` / `A` new file / dir, `D` delete, `R` rename, `x` reveal, `h` / `l`
collapse / expand, `enter` open, `escape` back.

## Leader and vim keys: git panel only (`space g g`, then keys)

| Key | Action |
|---|---|
| `j k h l gg G` | move, collapse, expand, first, last |
| `s` / `S` | toggle staged / stage all |
| `u` | unstage file |
| `x` | discard changes (asks first) |
| `c` / `i` | commit / jump to the commit message |
| **g** `s u x A U` | stage file, unstage file, discard, stage all, unstage all |
| **g** `c a m G` | commit, amend, commit message, generate commit message |
| **g** `p P f b d l h o S` | push, pull, fetch, branches, diff, log, history tab, open modified, stash |

`cmd-enter` commits.

## How it works (read this before editing)

**Timeout.** Zed waits one second for the rest of a chord *only when the first key is also a complete
binding*. That constant is not configurable in 1.21. So every context with a leader binds bare `space` to
`null`; then a lone `space` waits forever and the popup stays open. If you add a leader block for a new
panel and that panel already uses `space` (Zed's changes list, outline and explorer do), add `"space": null`
to it, or the popup will vanish after a second.

**Contexts.** Bindings whose context matches deeper in the UI tree win, then later definitions win. The
tree is `Workspace > Dock > ProjectPanel` for panels and `Workspace > Pane > Editor` for buffers. The shared
leader block uses `!Editor && !Terminal` for panels, exactly like Zed's stock vim keymap, and
`vim_mode == normal || vim_mode == visual` for buffers. List panels carry a `menu` flag, so never add `!menu`
to a panel context.

**Prefix collisions.** A key that is both a leaf and a prefix becomes a one-second chord. That is why `g r n`,
`g r r`, `g r i`, `g r a` and `g s` are nulled (they made `gr` and `gs…` slow), and why `space f` is nulled
in the shared block (Zed binds it to the file finder in panels). Avoid adding `space c` as a leaf while
`space c a` exists, and so on.

**Which-key.** `which_key.delay_ms` is 300 in settings.json to match LazyVim's `timeoutlen`. Group labels
cannot be customised in 1.21; the popup shows "+N keybinds" per prefix.

**Verifying names.** `zed: open default keymap` lists every default binding, and the command palette
shows every action name. A bad name shows a red "Errors in user keymap file" banner and Zed keeps the
last good keymap.

## Settings that shape the feel

| Setting | Why |
|---|---|
| `base_keymap: "Zed"` | native Zed bindings underneath |
| `vim.use_system_clipboard: "always"` | LazyVim `clipboard=unnamedplus` |
| `vim.use_smartcase_find`, `use_smartcase_search` | `f`/`t` and `/` ignore case unless you type a capital |
| `vim.toggle_relative_line_numbers` + `relative_line_numbers` | relative in normal, absolute in insert |
| `vertical_scroll_margin: 4`, `horizontal_scroll_margin: 8` | `scrolloff` / `sidescrolloff` |
| `soft_wrap: "none"`, `current_line_highlight: "all"`, `show_whitespaces: "trailing"` | `wrap`, `cursorline`, `list` |
| `format_on_save: "on"` | LazyVim autoformat; `workspace: save without format` when you need to skip it |
| `which_key.enabled` + `delay_ms: 300` | the popup |

## Not portable from LazyVim

- Terminal has no leader (space must type a space there) and `ctrl-h/j/k/l` are left alone so `ctrl-l` still clears.
- No quickfix list, sessions, todo-comment picker, or lazygit. `]q` / `[q` walk multibuffer excerpts instead.
- `gco` / `gcO` are omitted on purpose: a `g c o` chord would make the `gc` operator wait a second.
