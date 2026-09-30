# Learn the keymap (Zed 1.21, Helix first)

`base_keymap` is `None`, so keymap.json is the entire keymap. It is compiled from `keymap.kc`, one
tree in the keycook DSL: the Zed defaults worth keeping, the Helix and vim modal grammar, and our
semantic layer (lines marked `// ours`). Keys marked **(ours)** below come from that layer; the rest are
Zed's own bindings carried over. To change something, edit `keymap.kc` (Zed gives you completion and
diagnostics through the keycook extension), run `keycook build keymap.kc`, and Zed reloads.

## 0. The model in four lines

1. **Select, then act.** Helix style. Motions extend a selection; `d`, `c`, `y` act on it. Nothing to memorise like `diw`: press `w` (select word), then `d`.
2. **Three modes in a buffer.** Normal (block cursor), select (`v`, extends instead of replacing the selection), insert (`i` `a` `o`). `escape` collapses to normal.
3. **Prefix menus.** `g` goes somewhere, `m` matches pairs, `[` `]` jump to the previous or next thing, `z` moves the view, `space` runs a command. Press the prefix and wait: the which-key popup lists what follows.
4. **Same letters everywhere.** `j` `k` move in lists too. `d` deletes the thing under the cursor in the explorer and discards in the git panel. `.` opens the context menu in a panel, `space .` in a buffer. `ctrl-h/j/k/l` move between panes from anywhere.

## 1. Buffer: moving and selecting

| Key | Does |
|---|---|
| `h j k l`, arrows | move (`j` `k` follow wrapped lines) |
| `w b e`, `W B E` | select to next word start, previous word start, word end (`W` on whitespace only) |
| `f t F T` + char | select up to a character forward or backward; `alt-.` repeats |
| `x` | select whole line, again extends |
| `%` | select the whole file |
| `s` | select every regex match inside the selection (multi cursor) |
| `;` / `alt-;` | collapse selection to a cursor / flip cursor to the other end |
| `,` | keep only the newest selection |
| `C` / `alt-C` | add a cursor on the next / previous line |
| `alt-o` / `alt-i` | expand / shrink selection by syntax node |
| `alt-n` / `alt-p` | select next / previous sibling syntax node |
| `n` / `N`, `/` `?` `*` | next / previous match, search, search backward, search selection |
| `v` | select mode: movements extend instead of replacing; `v` or `escape` back |
| `G` with count, `gg` | go to line, file start |
| `ctrl-d` `ctrl-u` | half page down / up |

## 2. Buffer: changing

| Key | Does |
|---|---|
| `d` / `alt-d` | delete selection (yank / no yank) |
| `c` / `alt-c` | change selection (yank / no yank) |
| `y` `p` `P` `R` | yank, paste after, paste before, replace selection with clipboard |
| `r` + char | replace every selected character |
| `i` `a` `I` `A` `o` `O` | insert before, after, line start, line end, line below, line above |
| `u` `U` | undo, redo |
| `.` | repeat last change |
| `>` `<` `=` | indent, outdent, auto indent |
| `` ` `` / `` alt-` `` / `~` | lower case, upper case, swap case |
| `J` | join lines |
| `ctrl-c` | toggle comment |
| `ctrl-a` `ctrl-x` | increment / decrement number |
| `_` | trim selections |
| `q` `Q` | replay macro, record macro |
| `ctrl-s` **(ours)** | save (normal, select and insert) |

## 3. Goto `g`: every key here only moves

| Key | Goes to |
|---|---|
| `g g` / `g e` | file start / end |
| `g h` / `g l` / `g s` | line start / end / first non blank |
| `g t` / `g c` / `g b` | top / centre / bottom of the screen |
| `g d` / `g D` | definition / declaration |
| `g y` **(ours)** | type definition |
| `g i` / `g r` | implementation / references |
| `g a` | the file you had open before (alternate) |
| `g n` / `g p` | next / previous tab |
| `g w` | a word by label (type the two letters) |
| `g .` | last modification |
| `g m` / `g M` **(ours)** | next / previous git hunk |
| `g o` / `g O` **(ours)** | a symbol in this file / in the project (pickers) |
| `g f` / `g x` | file / URL under cursor |
| `g q` | rewrap (operator, takes a selection) |

`g r n`, `g r r`, `g r i`, `g r a` are disabled so `g r` fires immediately.

## 4. Match `m` and brackets `[` `]`

| Key | Does |
|---|---|
| `m m` | jump to the matching bracket |
| `m i` + obj / `m a` + obj | select inside / around an object: `w` word, `(` `[` `{` `<` `"` `'` `` ` `` pairs, `m` closest pair, `f` function, `t` type, `c` comment, `a` argument, `p` paragraph, `x` tag, `i` indent, `e` file |
| `m s` + char | surround the selection |
| `m r` + old + new | replace surround |
| `m d` + char | delete surround |
| `] d` / `[ d` | next / previous diagnostic |
| `] g` / `[ g` | next / previous git hunk |
| `] c` / `[ c` | next / previous comment |
| `] b` / `[ b` | next / previous tab |
| `] x` / `[ x` | shrink / grow selection by syntax node |
| `] space` / `[ space` | empty line below / above |
| `] -` `] +` `] =` | next lesser / greater / same indent |

## 5. View `z`

`z z` centre, `z t` top, `z b` bottom, `z c` centre (same as `z z`). These also work in the explorer and outline panel.

## 6. Space: run something

Helix's own space menu, plus the overlay. Bare `space` waits forever, so the popup stays open.

| Key | Does |
|---|---|
| `space f` | find file |
| `space b` | switch buffer (all panes) |
| `space s` / `space S` | symbol in file / in project |
| `space /` | search project |
| `space k` | hover docs |
| `space r` | rename symbol |
| `space a` | code actions |
| `space d` | next diagnostic |
| `space c` | toggle comment |
| `space y` / `space p` | copy / paste with the system clipboard |
| `space h` | select all matches of the selection |
| `space w` + `h j k l v s q` | window: focus, split right, split down, close |
| `space G` + key | debugger: `l` start, `c` continue, `b` breakpoint, `n` `i` `o` step, `h` pause, `t` stop |
| `space .` **(ours)** | context menu at the cursor (rare commands, navigate with `j` `k` `enter`) |
| `space :` **(ours)** | command palette |
| `space e` **(ours)** | explorer (again to hide) |
| `space g` **(ours)** | git panel |
| `space t` **(ours)** | terminal panel |
| `space x` **(ours)** | project diagnostics |
| `space m` **(ours)** | markdown preview beside |
| `space u` + `w z i d b` **(ours)** | toggles: wrap, zen, inlay hints, diagnostics, blame |
| `space q` / `space Q` **(ours)** | close window / quit |

## 7. Explorer (`space e`)

vim.json gives the navigation; the overlay gives the same letters as the buffer.

| Key | Does |
|---|---|
| `j k`, `g g`, `G` | move, first, last |
| `h l` | collapse / expand, `-` parent |
| `enter` | open |
| `o` / `O` **(ours)** | open in vertical / horizontal split |
| `a` / `A` **(ours)** | new file / new folder |
| `r` **(ours)** | rename |
| `d` / `D` **(ours)** | move to trash / delete permanently |
| `y` / `Y` **(ours)** | copy entry / copy relative path |
| `x` / `p` **(ours)** | cut / paste |
| `/` | search inside this folder |
| `.` **(ours)** | context menu |
| `g .` **(ours)** | go to the file that is open in the editor |
| `H` **(ours)** | show or hide gitignored files |
| `z c` **(ours)** | collapse all |
| `] c` `[ c`, `] d` `[ d` | next / previous changed file, diagnostic |
| `escape` | back to the editor |

## 8. Git panel (`space g`)

| Key | Does |
|---|---|
| `j k`, `g g`, `G`, `h l` | move, first, last, collapse, expand |
| `enter` / `o` **(ours)** | open the diff for this file |
| `O` **(ours)** | open all modified files |
| `s` **(ours)** / `x` | toggle staged (`x` is Zed's) |
| `S` **(ours)** / `X` | stage all |
| `u` **(ours)** / `U` | unstage file / unstage all |
| `d` **(ours)** | discard this file's changes (asks first) |
| `c` / `C` **(ours)** | commit / amend |
| `I` **(ours)** | generate a commit message |
| `i` | jump into the commit message; there `ctrl-s` commits, `ctrl-j` returns **(ours)** |
| `f` `p` `P` **(ours)** | fetch, pull, push |
| `b` **(ours)** | branches |
| `T` **(ours)** | stash all |
| `z z` **(ours)** | project diff |
| `g h` / `g c` **(ours)** | history tab / changes tab |
| `y` **(ours)** | copy relative path |
| `.` **(ours)** | panel menu |

## 9. Pickers, terminal, panes

- **Pickers** (`space f`, `space :`, `space b`): type to filter; `ctrl-j` `ctrl-k` move **(ours)**, `ctrl-l` toggles the preview **(ours)**, `ctrl-v` secondary open (split where supported) **(ours)**, `enter` opens, `escape` closes. `ctrl-n` `ctrl-p` also work.
- **Terminal** (`space t`): keys go to the shell. `ctrl-/` toggles the terminal panel from anywhere **(ours)**. `ctrl-shift-space` is Zed's terminal vi mode.
- **Panes**: `ctrl-h/j/k/l` from a buffer or a panel **(ours)**; `space w v` / `space w s` split; `space w q` closes.

## 10. What was left out of Zed's defaults

- Every `cmd-` shortcut except quit, close, save, open, new, palette, file finder, settings, clipboard, undo, select all, find, zoom, and the arrow/backspace line-editing chords for insert mode. Every function key. Every `cmd-k` chord. Pane and tab numbers. `ctrl-` Emacs aliases of arrows and backspace in the editor. Three-modifier chords.
- Vim's `ctrl-w` window chords (use `ctrl-h/j/k/l` and `space w`), tab cycling with `ctrl-pageup/down` (use `g n` / `g p`), the insert-mode `ctrl-x` menu, and the `g r n/r/i/a` chords.
- Whole features: notebooks, collaboration, onboarding, the keymap editor, skill creator, thread history and switcher, image viewer, worktree and toolchain pickers.

What was kept is exactly what `keymap.kc` lists. `tools/keymap-atlas.html` shows the result per focus.

## 11. If something waits or misfires

- A key that pauses one second is a prefix of another binding. `keycook check keymap.kc` warns about
  the ones it can see; the atlas (section 1, pick the focus, look for the "chord" pill) shows the rest.
  Bind the leaf or the prefix to `null`.
- `dev: open key context view` shows the live context if a binding does not fire where you expect.
- Zed keeps the last good keymap on a JSON or action-name error and shows a red banner.
