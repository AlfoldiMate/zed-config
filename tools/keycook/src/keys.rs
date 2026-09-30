//! Keystroke syntax: modifiers, named keys, ranges.

pub const MODIFIERS: &[&str] = &["ctrl", "alt", "shift", "cmd", "super", "win", "fn", "secondary"];

pub const NAMED_KEYS: &[&str] = &[
    "enter", "escape", "tab", "space", "backspace", "delete", "insert", "up", "down", "left", "right", "home", "end", "pageup", "pagedown",
    "f1", "f2", "f3", "f4", "f5", "f6", "f7", "f8", "f9", "f10", "f11", "f12", "f13", "f14", "f15", "f16", "f17", "f18", "f19",
    "ctrl", "alt", "shift", "cmd", "fn", "capslock", "menu",
];

/// Validate one keystroke (one token of a key sequence). Returns an error message if malformed.
pub fn check_keystroke(k: &str) -> Option<String> {
    if k.is_empty() {
        return Some("empty keystroke".into());
    }
    // split off modifiers: everything before the final '-' that is followed by a non-empty key
    let mut rest = k;
    let mut mods = Vec::new();
    loop {
        let Some(idx) = rest.find('-') else { break };
        if idx == 0 || idx + 1 >= rest.len() {
            break; // '-' itself or trailing '-' is the key
        }
        let m = &rest[..idx];
        if !MODIFIERS.contains(&m) {
            break;
        }
        mods.push(m);
        rest = &rest[idx + 1..];
    }
    let key = rest;
    if key.is_empty() {
        return Some(format!("`{k}` has no key after the modifiers"));
    }
    if key.chars().count() == 1 {
        return None;
    }
    if NAMED_KEYS.contains(&key) {
        return None;
    }
    let lower = key.to_ascii_lowercase();
    if NAMED_KEYS.contains(&lower.as_str()) {
        return Some(format!("`{k}`: named keys are lowercase (`{lower}`)"));
    }
    if let Some(hint) = match lower.as_str() {
        "esc" => Some("escape"),
        "return" | "cr" => Some("enter"),
        "bs" => Some("backspace"),
        "del" => Some("delete"),
        "pgup" => Some("pageup"),
        "pgdn" | "pgdown" => Some("pagedown"),
        "opt" | "option" => Some("alt-"),
        "command" | "meta" => Some("cmd-"),
        "control" => Some("ctrl-"),
        _ => None,
    } {
        return Some(format!("`{k}` is not a Zed key; use `{hint}`"));
    }
    if key.contains('-') && !mods.is_empty() {
        return Some(format!("`{k}`: `{}` is not a modifier", key.split('-').next().unwrap_or(key)));
    }
    Some(format!("`{k}` is not a single character or a named key (enter, escape, tab, space, up, f1…)"))
}

/// Expand `a..b` inside a key token: `1..9` → 1 2 … 9, `ctrl-a..z` → ctrl-a … ctrl-z.
/// Returns (expanded keys, range values) or None when there is no range.
pub fn expand_range(tok: &str) -> Option<Result<Vec<(String, String)>, String>> {
    let idx = tok.find("..")?;
    let before = &tok[..idx];
    let after = &tok[idx + 2..];
    let (prefix, start) = match before.rfind('-') {
        Some(d) if d + 1 < before.len() => (&before[..=d], &before[d + 1..]),
        _ => ("", before),
    };
    let s = start.chars().collect::<Vec<_>>();
    let e = after.chars().collect::<Vec<_>>();
    if s.len() != 1 || e.len() != 1 {
        return Some(Err(format!("range `{tok}` must be `x..y` with single characters")));
    }
    let (s, e) = (s[0], e[0]);
    if s > e || !(s.is_ascii_alphanumeric() && e.is_ascii_alphanumeric()) {
        return Some(Err(format!("range `{tok}` must run upward over letters or digits")));
    }
    let mut out = Vec::new();
    let mut c = s;
    loop {
        out.push((format!("{prefix}{c}"), c.to_string()));
        if c == e {
            break;
        }
        c = (c as u8 + 1) as char;
    }
    Some(Ok(out))
}

/// Substitute `$`, `$-n`, `$+n` in argument text with the range value.
pub fn substitute_range(raw: &str, value: &str) -> String {
    let mut out = String::new();
    let chars: Vec<char> = raw.chars().collect();
    let mut i = 0;
    while i < chars.len() {
        if chars[i] == '$' {
            let mut j = i + 1;
            let mut delta: i64 = 0;
            if j < chars.len() && (chars[j] == '+' || chars[j] == '-') {
                let sign = if chars[j] == '-' { -1 } else { 1 };
                j += 1;
                let mut num = String::new();
                while j < chars.len() && chars[j].is_ascii_digit() {
                    num.push(chars[j]);
                    j += 1;
                }
                if let Ok(n) = num.parse::<i64>() {
                    delta = sign * n;
                }
            }
            if let Ok(n) = value.parse::<i64>() {
                out.push_str(&(n + delta).to_string());
            } else {
                out.push('"');
                out.push_str(value);
                out.push('"');
            }
            i = j;
            continue;
        }
        out.push(chars[i]);
        i += 1;
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn keystrokes() {
        assert!(check_keystroke("ctrl-shift-a").is_none());
        assert!(check_keystroke("cmd-|").is_none());
        assert!(check_keystroke("escape").is_none());
        assert!(check_keystroke("-").is_none());
        assert!(check_keystroke("ctrl--").is_none());
        assert!(check_keystroke("esc").is_some());
        assert!(check_keystroke("G").is_none());
        assert!(check_keystroke("opt-x").is_some());
    }

    #[test]
    fn ranges() {
        let r = expand_range("1..3").unwrap().unwrap();
        assert_eq!(r, vec![("1".into(), "1".into()), ("2".into(), "2".into()), ("3".into(), "3".into())]);
        let r = expand_range("ctrl-a..c").unwrap().unwrap();
        assert_eq!(r[2].0, "ctrl-c");
        assert!(expand_range("plain").is_none());
        assert_eq!(substitute_range("$-1", "3"), "2");
        assert_eq!(substitute_range("x=$", "a"), "x=\"a\"");
    }
}
