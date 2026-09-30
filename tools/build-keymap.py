#!/usr/bin/env python3
"""Build ~/.config/zed/keymap.json from upstream keymaps + rules + overlay.

Zed is run with `base_keymap: "None"`, which loads nothing of its own (not even vim.json),
so this file is the whole keymap. It is assembled from three parts, in this order:

  1. tools/upstream/default-macos.json  filtered by the DEFAULT rules below  (chrome: menus, pickers,
                                        search, panels, terminal, insert-mode editing, macOS basics)
  2. tools/upstream/vim.json            filtered by the VIM rules below      (Helix + vim modal grammar)
  3. tools/overlay.jsonc                as is                                (our semantic layer)

Later sections win at equal context depth, exactly as in Zed. Run:  python3 tools/build-keymap.py
"""
import json, re, sys
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
UP = ROOT / "tools" / "upstream"
OUT = ROOT / "keymap.json"


def strip_json_comments(text: str) -> str:
    out, i, n, in_str = [], 0, len(text), False
    while i < n:
        c = text[i]
        if in_str:
            out.append(c)
            if c == "\\":
                out.append(text[i + 1]); i += 2; continue
            if c == '"':
                in_str = False
            i += 1; continue
        if c == '"':
            in_str = True; out.append(c); i += 1; continue
        if text.startswith("//", i):
            j = text.find("\n", i); i = n if j < 0 else j; continue
        if text.startswith("/*", i):
            i = text.find("*/", i) + 2; continue
        out.append(c); i += 1
    return re.sub(r",(\s*[\]}])", r"\1", "".join(out))


def load(path):
    return json.loads(strip_json_comments(path.read_text()))


# ── DEFAULT rules: what survives from default-macos.json ─────────────────────────────────────────
# Sections whose context matches any of these are dropped whole (features not used, or non-modal UI).
DEFAULT_DROP_SECTIONS = [
    r"Notebook", r"Collab", r"ChannelModal", r"RatePredictions", r"ZedPredictModal", r"OnboardingAi",
    r"^Onboarding$", r"^Welcome$", r"SkillCreator", r"KeystrokeInput", r"KeybindEditorModal", r"KeymapEditor",
    r"EditPredictionContext", r"RunModal", r"^GitPicker$", r"ThreadHistory", r"ThreadsArchiveView",
    r"ThreadSwitcher", r"jupyter", r"multibuffer", r"^Editor && !agent_diff", r"^Markdown$",
    r"CallHierarchyPicker", r"LspCommandSelector", r"ToolchainSelector", r"ImageViewer", r"StashDiff",
    r"WorktreePicker", r"InvalidBuffer",
]
# cmd- keys that stay: the conventions every macOS app has, plus line editing for insert mode and inputs.
CMD_ALLOW = {
    "cmd-q", "cmd-w", "cmd-s", "cmd-shift-s", "cmd-shift-p", "cmd-p", "cmd-o", "cmd-n", "cmd-shift-n",
    "cmd-c", "cmd-v", "cmd-x", "cmd-z", "cmd-shift-z", "cmd-a", "cmd-f", "cmd-,",
    "cmd-=", "cmd-+", "cmd--", "cmd-0",
    "cmd-enter", "cmd-shift-enter", "cmd-backspace", "cmd-delete",
    "cmd-left", "cmd-right", "cmd-up", "cmd-down", "cmd-shift-left", "cmd-shift-right", "cmd-shift-up", "cmd-shift-down",
    "cmd-home", "cmd-end",
}
# Keys dropped everywhere in default-macos.json (aliases and function keys).
DEFAULT_DROP_KEYS = re.compile(
    r"^(?:"
    r"(?:[a-z-]+-)?f\d+$"                     # f1..f12 with any modifiers
    r"|fn-"                                   # fn- chords
    r"|cmd-k\b"                               # every cmd-k chord
    r"|cmd-\d$|ctrl-\d$"                      # pane / tab by number
    r"|cmd-escape$|ctrl-escape$"              # menu cancel aliases
    r"|shift-page(?:up|down)$|cmd-page(?:up|down)$|ctrl-page(?:up|down)$"
    r"|ctrl-cmd-|cmd-ctrl-|ctrl-alt-cmd-|cmd-alt-"  # three-modifier and cmd-alt chords
    r"|alt-cmd-|shift-alt-|alt-shift-"       # more alias chords (the modal keys cover these)
    r"|ctrl-tab$|ctrl-shift-tab$"             # tab cycling (g n / g p)
    r"|alt-shift-enter$|shift-escape$"
    r")"
)
# Inside plain Editor sections, ctrl-<letter> are Emacs aliases of arrows/backspace: gone.
EDITOR_CTRL_ALIAS = re.compile(r"^ctrl-(?:shift-)?[a-z]$")


def keep_default(ctx, key):
    if DEFAULT_DROP_KEYS.search(key):
        return False
    if key.startswith("cmd-") and key not in CMD_ALLOW:
        return False
    if re.match(r"^Editor(?: && mode == full)?$", ctx) and EDITOR_CTRL_ALIAS.match(key):
        return False
    return True


# ── VIM rules: what survives from vim.json ───────────────────────────────────────────────────────
VIM_DROP_SECTIONS = [r"os == windows", r"Notebook", r"^vim_mode == insert && !\("]
VIM_DROP_KEYS = re.compile(
    r"^(?:"
    r"ctrl-w(?: |$)"           # vim window chords; ctrl-h/j/k/l and space w replace them
    r"|g r [nria]$"            # LSP chords under g r; g r stays instant
    r"|ctrl-page(?:up|down)$"  # tab cycling aliases
    r"|ctrl-x(?: |$)"          # insert-mode ctrl-x menu
    r")"
)


def keep_vim(ctx, key):
    if key == "ctrl-w" and "insert" in ctx:
        return True  # insert-mode delete-word is not a window chord
    return not VIM_DROP_KEYS.search(key)


def take(path, drop_sections, keep, label):
    out, kept, total = [], 0, 0
    for i, sec in enumerate(load(path)):
        ctx = sec.get("context", "")
        b = sec.get("bindings") or {}
        total += len(b)
        if any(re.search(p, ctx) for p in drop_sections):
            continue
        nb = {k: v for k, v in b.items() if keep(ctx, k)}
        if not nb:
            continue
        kept += len(nb)
        out.append({"_comment": f"{label} #{i}", "context": ctx, "use_key_equivalents": sec.get("use_key_equivalents", False), "bindings": nb})
    return out, kept, total


def render(sections):
    lines = ["// ~/.config/zed/keymap.json — GENERATED by tools/build-keymap.py, do not edit by hand.",
             "// Edit tools/overlay.jsonc (our layer) or the rules in tools/build-keymap.py, then rebuild.",
             "// base_keymap is \"None\": this file is the entire keymap. Learn it from KEYMAP.md.", "["]
    for s in sections:
        lines.append(f"  // ── {s['_comment']}: {s['context'] or '(no context)'}")
        lines.append("  {")
        if s["context"]:
            lines.append(f"    \"context\": {json.dumps(s['context'])},")
        if s.get("use_key_equivalents"):
            lines.append("    \"use_key_equivalents\": true,")
        lines.append("    \"bindings\": {")
        for k, v in s["bindings"].items():
            lines.append(f"      {json.dumps(k)}: {json.dumps(v)},")
        lines.append("    },")
        lines.append("  },")
    lines.append("]")
    return "\n".join(lines) + "\n"


def main():
    d, dk, dt = take(UP / "default-macos.json", DEFAULT_DROP_SECTIONS, keep_default, "default")
    v, vk, vt = take(UP / "vim.json", VIM_DROP_SECTIONS, keep_vim, "vim")
    overlay = [{"_comment": f"overlay #{i}", "context": s.get("context", ""), "bindings": s["bindings"]} for i, s in enumerate(load(ROOT / "tools" / "overlay.jsonc"))]
    sections = d + v + overlay
    OUT.write_text(render(sections))
    ok = sum(len(s["bindings"]) for s in overlay)
    print(f"default {dk}/{dt}  vim {vk}/{vt}  overlay {ok}  -> {dk + vk + ok} bindings in {len(sections)} sections -> {OUT}")


if __name__ == "__main__":
    main()
