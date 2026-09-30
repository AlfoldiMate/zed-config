//! Zed extension: wires the `keycook` binary in as the language server for `.kc` files.

use zed_extension_api::{self as zed, LanguageServerId, Result};

struct KeycookExtension;

impl zed::Extension for KeycookExtension {
    fn new() -> Self {
        KeycookExtension
    }

    fn language_server_command(&mut self, _id: &LanguageServerId, worktree: &zed::Worktree) -> Result<zed::Command> {
        // Prefer a `keycook` on PATH; fall back to the cargo install location.
        let path = worktree.which("keycook").unwrap_or_else(|| {
            let home = std::env::var("HOME").unwrap_or_default();
            format!("{home}/.cargo/bin/keycook")
        });
        Ok(zed::Command { command: path, args: vec!["lsp".to_string()], env: vec![] })
    }
}

zed::register_extension!(KeycookExtension);
