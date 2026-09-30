# Zed config

My [Zed](https://zed.dev) setup: Helix-style modal editing (Zed's Helix mode over vim mode) with a
small overlay that makes the same letters mean the same thing in buffers, the explorer, the git panel
and pickers. Which-key popup on `space`.

- `settings.json` — vim, which-key, editor feel, theme.
- `keymap.kc` — the entire keymap, written in the keycook DSL (`base_keymap` is `None`). About 740 lines, one tree.
- `keymap.json` — generated from it with `keycook build keymap.kc`. Never edited by hand.
- `KEYMAP.md` — the learning guide: modes, the core Helix keys, goto, space, panels, git, pickers.
- `KEYMAP-LAZYVIM.md` and `keymap.json.bak` — the previous LazyVim-style layer, kept for reference.
- `ZED-CONTEXTS.md` — every context identifier Zed 1.21 exposes, the expression grammar, and recipes for combining them.
- `KEYMAP-DESIGN.md`, `KEYMAP-DSL.md` — the rationale and the DSL concept that led to keycook.
- `tools/keycook/` — the compiler, language server and Zed extension for the DSL (see its README). `tools/upstream/` holds the v1.21.0 keymaps its golden tests round-trip.
- `tools/keymap-atlas.html` — interactive explorer: focus tree, effective keymap per context, binding search, rule editor that exports a keymap.
- `ZED-ACTIONS.md` — every action in Zed 1.21 with its argument schema and default keys (generated from `zed --dump-all-actions`).

Built against Zed 1.21 on macOS. Zed 1.21 has a hard-coded one-second timeout for multi-key
chords that only starts when the prefix is also a full binding, so every leader context binds
bare `space` to `null`; that single trick is what makes the popup usable.

## Install

```sh
git clone https://github.com/AlfoldiMate/zed-config ~/.config/zed
```

Zed reloads both files on save. Verify any action name with `zed: open default keymap`.
