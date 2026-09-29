# Zed config

My [Zed](https://zed.dev) setup: vim mode with a LazyVim-style `space` leader, a which-key
popup that stays open, and a leader that changes with context (buffer, file explorer, git panel).

- `settings.json` — vim, which-key, editor feel, theme.
- `keymap.json` — the bindings, organised in layers with comments.
- `KEYMAP.md` — cheat sheet and the rules to know before editing (chord timeout, contexts).

Built against Zed 1.21 on macOS. Zed 1.21 has a hard-coded one-second timeout for multi-key
chords that only starts when the prefix is also a full binding, so every leader context binds
bare `space` to `null`; that single trick is what makes the popup usable.

## Install

```sh
git clone https://github.com/AlfoldiMate/zed-config ~/.config/zed
```

Zed reloads both files on save. Verify any action name with `zed: open default keymap`.
