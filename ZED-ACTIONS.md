# Zed actions reference (Zed 1.21.0)

Every action registered in the installed Zed 1.21.0 binary, dumped with `zed --dump-all-actions` (a hidden CLI flag), merged with the default keystrokes from `assets/keymaps/default-macos.json` and `assets/keymaps/vim.json` at tag `v1.21.0`.

Total: 1406 actions in 90 namespaces. 185 actions take an argument.

## How to bind an action

```jsonc
"cmd-s": "workspace::Save",                         // no argument: a plain string
"cmd-1": ["workspace::ActivatePane", 0],            // scalar argument: [name, value]
"cmd-shift-f": ["pane::DeploySearch", { "replace_enabled": true }], // object argument: [name, { … }]
"cmd-r": null,                                      // disable every binding of cmd-r at this depth and above
"alt-down": ["workspace::SendKeystrokes", "down down"], // replay keystrokes
"ctrl-a": ["action::Sequence", ["editor::SelectAll", "editor::Copy"]], // run several actions
```

Rules that come from the source (`crates/settings/src/keymap_file.rs`, `crates/gpui/src/action.rs`):

- Names are `namespace::PascalCase`. The command palette shows the humanised form (`workspace: save`), and the reference below lists both.
- An action whose **Argument** column reads `—` must be bound as a bare string. Binding it as `[name, value]` is a keymap error.
- For object arguments every field marked `?` is optional and takes the shown default. Unknown fields are rejected when the schema has `additionalProperties: false`, which is the case for almost all of them.
- A section can also carry `"unbind": { "cmd-s": "workspace::Save" }` to remove one specific default binding of that key (the one dispatching the named action, whatever its context) without shadowing the key for other actions. The target must be a real action name, not `null` and not `zed::Unbind`. The same thing written as a binding is `"cmd-s": ["zed::Unbind", "workspace::Save"]`.
- Deprecated aliases still load, but the keymap editor rewrites them to the new name. They are listed under **Aliases** so old snippets can be recognised.
- Actions can be *conditional*: a handler may decline (propagate) and the next binding up the tree runs. That is why a `null` binding is sometimes needed to stop the fallback (documented in `docs/src/key-bindings.md`).

Argument types written in `PascalCase` in the tables (`SaveIntent`, `Object`, `RevealTarget`, …) are expanded in the [Argument types](#argument-types) appendix at the end.

## Namespaces

| Namespace | Actions | What it covers |
|---|---|---|
| [action](#action) | 1 | meta-actions (sequence) |
| [activity_indicator](#activity_indicator) | 1 |  |
| [agent](#agent) | 88 | agent panel, threads, model and profile switching |
| [agents_sidebar](#agents_sidebar) | 4 | threads sidebar |
| [app_menu](#app_menu) | 3 | application menu bar |
| [assistant](#assistant) | 1 |  |
| [auto_update](#auto_update) | 4 | updater |
| [bedrock](#bedrock) | 2 |  |
| [branch_picker](#branch_picker) | 7 | git branch picker |
| [buffer_search](#buffer_search) | 5 | in-buffer find and replace |
| [call_hierarchy](#call_hierarchy) | 3 | LSP call hierarchy picker |
| [channel_modal](#channel_modal) | 4 |  |
| [cli](#cli) | 2 |  |
| [client](#client) | 3 |  |
| [collab](#collab) | 14 | collaboration calls and screen sharing |
| [collab_panel](#collab_panel) | 13 | collab panel list navigation |
| [command_palette](#command_palette) | 2 | command palette |
| [console](#console) | 1 |  |
| [context_server](#context_server) | 1 |  |
| [copilot_edit_predictions](#copilot_edit_predictions) | 1 |  |
| [debug_panel](#debug_panel) | 2 |  |
| [debugger](#debugger) | 35 | debug adapter control |
| [dev](#dev) | 26 | developer tools (key context view, inspector, etc.) |
| [diagnostics](#diagnostics) | 4 | project diagnostics view |
| [edit_prediction](#edit_prediction) | 4 | edit prediction provider UI |
| [editor](#editor) | 303 | text editing, movement, selection, LSP, folding, git hunks |
| [encoding_selector](#encoding_selector) | 1 |  |
| [feedback](#feedback) | 3 |  |
| [file_finder](#file_finder) | 3 | file finder picker |
| [git](#git) | 77 | git operations (stage, commit, push, blame, permalinks) |
| [git_graph](#git_graph) | 11 | commit graph view |
| [git_onboarding](#git_onboarding) | 1 |  |
| [git_panel](#git_panel) | 23 | git panel list |
| [git_picker](#git_picker) | 2 | branch/stash picker tabs |
| [go_to_line](#go_to_line) | 1 | go to line modal |
| [highlights_tree_view](#highlights_tree_view) | 3 |  |
| [icon_theme_selector](#icon_theme_selector) | 1 |  |
| [image_viewer](#image_viewer) | 5 |  |
| [inline_assistant](#inline_assistant) | 2 |  |
| [journal](#journal) | 1 |  |
| [keymap_editor](#keymap_editor) | 11 | keymap editor UI |
| [keystroke_input](#keystroke_input) | 3 |  |
| [language_selector](#language_selector) | 1 |  |
| [line_ending_selector](#line_ending_selector) | 1 |  |
| [lsp_command_selector](#lsp_command_selector) | 2 |  |
| [lsp_tool](#lsp_tool) | 1 |  |
| [markdown](#markdown) | 15 | rendered markdown views |
| [menu](#menu) | 11 | generic list/menu navigation (pickers, completions, context menus) |
| [multi_workspace](#multi_workspace) | 11 | multiple workspaces in one window |
| [new_process_modal](#new_process_modal) | 4 |  |
| [notebook](#notebook) | 16 | Jupyter notebook editor |
| [onboarding](#onboarding) | 4 |  |
| [outline](#outline) | 1 |  |
| [outline_panel](#outline_panel) | 18 | outline panel |
| [pane](#pane) | 36 | tabs, splits, item navigation |
| [picker](#picker) | 9 | picker preview and toggles |
| [project_panel](#project_panel) | 46 | file explorer |
| [project_search](#project_search) | 6 | project-wide search view |
| [project_symbols](#project_symbols) | 1 |  |
| [projects](#projects) | 4 |  |
| [recent_projects](#recent_projects) | 3 |  |
| [remote_debug](#remote_debug) | 3 |  |
| [repl](#repl) | 9 | REPL kernels |
| [search](#search) | 15 | shared search bar options |
| [settings_editor](#settings_editor) | 14 | settings UI |
| [settings_profile_selector](#settings_profile_selector) | 1 |  |
| [skill_creator](#skill_creator) | 4 |  |
| [snippets](#snippets) | 2 |  |
| [stash_picker](#stash_picker) | 2 |  |
| [svg](#svg) | 3 |  |
| [syntax_tree_view](#syntax_tree_view) | 1 |  |
| [tab_switcher](#tab_switcher) | 4 | tab switcher modal |
| [tabular_data](#tabular_data) | 2 |  |
| [task](#task) | 2 | tasks |
| [terminal](#terminal) | 20 | integrated terminal |
| [terminal_panel](#terminal_panel) | 2 | terminal panel |
| [text_finder](#text_finder) | 5 | find in project search results |
| [theme](#theme) | 1 |  |
| [theme_selector](#theme_selector) | 2 | theme picker |
| [toast](#toast) | 1 |  |
| [toolchain](#toolchain) | 2 |  |
| [variable_list](#variable_list) | 8 |  |
| [vim](#vim) | 263 | vim / helix mode |
| [welcome](#welcome) | 1 |  |
| [window](#window) | 4 |  |
| [workspace](#workspace) | 92 | docks, panels, saving, windows, tasks |
| [worktree_picker](#worktree_picker) | 2 |  |
| [zed](#zed) | 62 | app-level (settings, keymap, quit, about, extensions) |
| [zed_predict_onboarding](#zed_predict_onboarding) | 1 |  |
| [zeta](#zeta) | 6 | Zed edit prediction (zeta) |

## action

| Action | Palette name | Argument | Description | Default keys |
|---|---|---|---|---|
| `action::Sequence` | action: sequence | `[KeymapAction, …]` | Runs a sequence of actions. NOTE: This does **not** wait for asynchronous actions to complete before running the next action. |  |

## activity_indicator

| Action | Palette name | Argument | Description | Default keys |
|---|---|---|---|---|
| `activity_indicator::ShowErrorMessage` | activity indicator: show error message | — | Displays error messages from language servers in the status bar. |  |

## agent

| Action | Palette name | Argument | Description | Default keys |
|---|---|---|---|---|
| `agent::AddSelectionToThread` | agent: add selection to thread | — | Add the current selection as context for threads in the agent panel. Aliases: `assistant::QuoteSelection`, `agent::QuoteSelection` | `cmd->` (Editor && mode == full); `cmd->` (AcpThread); `cmd->` (Terminal) |
| `agent::AllowAlways` | agent: allow always | — | Allow this operation and remember the choice. | `cmd-alt-y` (AcpThread) |
| `agent::AllowOnce` | agent: allow once | — | Allow this operation only this time. | `cmd-y` (AcpThread) |
| `agent::ArchiveSelectedThread` | agent: archive selected thread | — | Archives the currently selected thread. | `backspace` (ThreadsArchiveView); `shift-backspace` (ThreadsSidebar) |
| `agent::AuthorizeToolCall` | agent: authorize tool call | `{ tool_call_id: string, option_id: string, option_kind: string }` | Action to authorize a tool call with a specific permission option. This is used by the permission granularity dropdown to authorize tool calls. Argument: `tool_call_id`: The tool call ID to authorize. `option_id`: The permission option ID to use. `option_kind`: The kind of permission option (serialized as string). |  |
| `agent::Chat` | agent: chat | — | Starts a chat conversation with the agent. | `enter` (AcpThread > Editor && !use_modifier_to_send); `cmd-enter` (AcpThread > Editor && use_modifier_to_send); `enter` (MessageEditor > Editor && VimControl) [vim] |
| `agent::ChatWithFollow` | agent: chat with follow | — | Starts a chat conversation with follow-up enabled. | `cmd-enter` (AcpThread > Editor) |
| `agent::ClearMessageQueue` | agent: clear message queue | — | Clears all messages from the queue. | `cmd-alt-backspace` (AcpThread > Editor) |
| `agent::CopyThreadToClipboard` | agent: copy thread to clipboard | — | Copies the current thread to the clipboard as JSON for debugging. |  |
| `agent::CreateSkillFromUrl` | agent: create skill from url | — | Opens the skill creator window to import a skill from a GitHub URL. |  |
| `agent::CycleFavoriteModels` | agent: cycle favorite models | — | Cycles through favorited models in the ACP model selector. | `alt-tab` (AcpThread); `alt-tab` (AcpThread > Editor); `alt-tab` (InlineAssistant > Editor) |
| `agent::CycleModeSelector` | agent: cycle mode selector | — | Cycles through available session modes. | `shift-tab` (AcpThread); `shift-tab` (AcpThread > Editor) |
| `agent::CycleNextInlineAssist` | agent: cycle next inline assist | — | Cycles to the next inline assist suggestion. | `ctrl-]` (InlineAssistant > Editor) |
| `agent::CyclePreviousInlineAssist` | agent: cycle previous inline assist | — | Cycles to the previous inline assist suggestion. | `ctrl-[` (InlineAssistant > Editor) |
| `agent::CycleThinkingEffort` | agent: cycle thinking effort | — | Cycles through available thinking effort levels for the current model. | `ctrl-'` (AcpThread > Editor) |
| `agent::DismissThreadSearch` | agent: dismiss thread search | — | Closes the thread search bar. | `escape` (AcpThreadSearchBar) |
| `agent::EditFirstQueuedMessage` | agent: edit first queued message | — | Edits the first message in the queue (the next one to be sent). | `cmd-ctrl-e` (AcpThread > Editor) |
| `agent::ExpandMessageEditor` | agent: expand message editor | — | Expands the message editor to full size. | `shift-alt-escape` (AcpThread) |
| `agent::FocusAgent` | agent: focus agent | — |  |  |
| `agent::FocusDown` | agent: focus down | — | Moves focus down in the interface. |  |
| `agent::FocusLeft` | agent: focus left | — | Moves focus left in the interface. |  |
| `agent::FocusRight` | agent: focus right | — | Moves focus right in the interface. |  |
| `agent::FocusUp` | agent: focus up | — | Moves focus up in the interface. |  |
| `agent::Follow` | agent: follow | — | Follows the agent's suggestions. |  |
| `agent::ImportThreadsFromOtherChannels` | agent: import threads from other channels | — | Import agent threads from other Zed release channels (e.g. Preview, Nightly). |  |
| `agent::Keep` | agent: keep | — | Keeps the current suggestion or change. | `cmd-y` (AgentDiff); `cmd-alt-y` (AgentDiff); `cmd-y` (Editor && editor_agent_diff); `cmd-alt-y` (Editor && editor_agent_diff) |
| `agent::KeepAll` | agent: keep all | — | Keeps all suggestions or changes. | `shift-alt-y` (AgentDiff); `shift-alt-y` (Editor && editor_agent_diff); `shift-alt-y` (AcpThread > Editor) |
| `agent::LoadThreadFromClipboard` | agent: load thread from clipboard | — | Loads a thread from the clipboard JSON for debugging. |  |
| `agent::LogoutAgent` | agent: logout agent | — | Logs out of the current external agent |  |
| `agent::ManageProfiles` | agent: manage profiles | `{ customize_tools?: AgentProfileId \| null }` | Opens the profile management interface for configuring agent tools and settings. | `cmd-alt-p` (AcpThread) |
| `agent::ManageSkills` | agent: manage skills | — | Opens the skills manager in the settings window. Aliases: `agent::OpenRulesLibrary`, `assistant::OpenRulesLibrary`, `assistant::DeployPromptLibrary` | `cmd-alt-l` (AcpThread) |
| `agent::NewExternalAgentThread` | agent: new external agent thread | `{ agent: string }` | Creates a new external agent conversation thread. Argument: `agent`: The agent id to use for the conversation. |  |
| `agent::NewNativeAgentThreadFromSummary` | agent: new native agent thread from summary | `{ from_session_id: SessionId }` |  |  |
| `agent::NewTerminalThread` | agent: new terminal thread | — | Starts a new terminal thread. |  |
| `agent::NewThread` | agent: new thread | — | Creates a new conversation thread, optionally based on an existing thread. | `cmd-n` (AgentPanel); `cmd-n` (AcpThread); `cmd-n` (AgentPanel > Terminal) |
| `agent::OpenActiveThreadAsMarkdown` | agent: open active thread as markdown | — | Opens the active thread as a markdown file. |  |
| `agent::OpenAddContextMenu` | agent: open add context menu | — | Opens the "Add Context" menu in the message editor. | `ctrl-;` (AcpThread > Editor) |
| `agent::OpenAgentDiff` | agent: open agent diff | — | Opens the agent diff view to review changes. | `shift-ctrl-r` (Editor && editor_agent_diff); `shift-ctrl-r` (AcpThread > Editor) |
| `agent::OpenGlobalAGENTS.mdRules` | agent: open global AGENTS.md rules | — | Opens the user-global AGENTS.md rules file. |  |
| `agent::OpenOnboardingModal` | agent: open onboarding modal | — | Opens the agent onboarding modal. |  |
| `agent::OpenPermissionDropdown` | agent: open permission dropdown | — | Opens the permission granularity dropdown for the current tool call. | `cmd-alt-a` (AcpThread) |
| `agent::OpenProjectAGENTS.mdRules` | agent: open project AGENTS.md rules | — | Opens the project AGENTS.md rules file. |  |
| `agent::OpenSettings` | agent: open settings | — | Opens the agent settings UI. Aliases: `agent::OpenConfiguration` | `cmd-alt-c` (AgentPanel) |
| `agent::OpenSkillCreator` | agent: open skill creator | — | Opens the skill creator window for creating a new skill. |  |
| `agent::PasteRaw` | agent: paste raw | — | Pastes clipboard content without any formatting. | `cmd-shift-v` (AcpThread > Editor) |
| `agent::ReauthenticateAgent` | agent: reauthenticate agent | — | Triggers re-authentication on Gemini |  |
| `agent::Reject` | agent: reject | — | Rejects the current suggestion or change. | `cmd-alt-z` (AgentDiff); `cmd-alt-z` (Editor && editor_agent_diff) |
| `agent::RejectAll` | agent: reject all | — | Rejects all suggestions or changes. | `shift-alt-z` (AgentDiff); `shift-alt-z` (Editor && editor_agent_diff); `shift-alt-z` (AcpThread > Editor) |
| `agent::RejectOnce` | agent: reject once | — | Reject this operation only this time. | `cmd-alt-z` (AcpThread) |
| `agent::RemoveFirstQueuedMessage` | agent: remove first queued message | — | Removes the first message from the queue (the next one to be sent). | `cmd-shift-backspace` (AcpThread > Editor) |
| `agent::RemoveSelectedThread` | agent: remove selected thread | — | Removes the currently selected thread. | `shift-backspace` (ThreadHistory > Editor); `cmd-shift-backspace` (ThreadsSidebar); `d d` (ThreadsSidebar && !Editor) [vim] |
| `agent::RenameSelectedThread` | agent: rename selected thread | — | Renames the currently selected thread. | `shift-r` (ThreadsSidebar && not_searching) |
| `agent::RerunRulesToSkillsMigration` | agent: rerun rules to skills migration | — | Reruns the rules-to-skills migration. |  |
| `agent::ResetAgentZoom` | agent: reset agent zoom | — | Resets the agent panel zoom levels (agent UI and buffer font sizes). |  |
| `agent::ResetFastModeWarnings` | agent: reset fast mode warnings | — | Re-enables the fast mode warning for every provider and model. |  |
| `agent::ResetOnboarding` | agent: reset onboarding | — | Resets the agent onboarding state. |  |
| `agent::ResetTrialEndUpsell` | agent: reset trial end upsell | — | Resets the trial end upsell notification. |  |
| `agent::ResetTrialUpsell` | agent: reset trial upsell | — | Resets the trial upsell notification. |  |
| `agent::ResolveConflictedFilesWithAgent` | agent: resolve conflicted files with agent | `{ conflicted_file_paths: [string, …] }` | Opens a new agent thread to resolve merge conflicts in the given file paths. Argument: `conflicted_file_paths`: File paths with unresolved conflicts (for project-wide resolution). |  |
| `agent::ResolveConflictsWithAgent` | agent: resolve conflicts with agent | `{ conflicts: [ConflictContent, …] }` | Opens a new agent thread to resolve specific merge conflicts. Argument: `conflicts`: Individual conflicts with their full text. |  |
| `agent::ReviewBranchDiff` | agent: review branch diff | `{ diff_text: string, base_ref: string }` | Opens a new agent thread with the provided branch diff for review. Argument: `diff_text`: The full text of the diff to review. `base_ref`: The base ref that the diff was computed against (e.g. "main"). |  |
| `agent::ScrollOutputLineDown` | agent: scroll output line down | — | Scroll the output down by three lines. | `down` (AcpThread); `ctrl-alt-down` (AcpThread); `ctrl-alt-down` (AcpThread > Editor) |
| `agent::ScrollOutputLineUp` | agent: scroll output line up | — | Scroll the output up by three lines. | `up` (AcpThread); `ctrl-alt-up` (AcpThread); `ctrl-alt-up` (AcpThread > Editor) |
| `agent::ScrollOutputPageDown` | agent: scroll output page down | — | Scroll the output by one page down. | `pagedown` (AcpThread); `ctrl-pagedown` (AcpThread); `ctrl-pagedown` (AcpThread > Editor); `pagedown` (AcpThread > Editor && end_of_input); `ctrl-pagedown` (AcpThread > Editor && end_of_input) |
| `agent::ScrollOutputPageUp` | agent: scroll output page up | — | Scroll the output by one page up. | `pageup` (AcpThread); `ctrl-pageup` (AcpThread); `ctrl-pageup` (AcpThread > Editor); `pageup` (AcpThread > Editor && start_of_input); `ctrl-pageup` (AcpThread > Editor && start_of_input) |
| `agent::ScrollOutputToBottom` | agent: scroll output to bottom | — | Scroll the output to the bottom. | `end` (AcpThread); `ctrl-end` (AcpThread); `ctrl-end` (AcpThread > Editor); `ctrl-end` (AcpThread > Editor && end_of_input) |
| `agent::ScrollOutputToNextMessage` | agent: scroll output to next message | — | Scroll the output to the next user message. | `shift-pagedown` (AcpThread); `ctrl-alt-pagedown` (AcpThread); `ctrl-alt-pagedown` (AcpThread > Editor) |
| `agent::ScrollOutputToPreviousMessage` | agent: scroll output to previous message | — | Scroll the output to the previous user message. | `shift-pageup` (AcpThread); `ctrl-alt-pageup` (AcpThread); `ctrl-alt-pageup` (AcpThread > Editor) |
| `agent::ScrollOutputToTop` | agent: scroll output to top | — | Scroll the output to the top. | `home` (AcpThread); `ctrl-home` (AcpThread); `ctrl-home` (AcpThread > Editor); `ctrl-home` (AcpThread > Editor && start_of_input) |
| `agent::SelectAgent` | agent: select agent | `{ agent: string }` | Selects the agent used for new threads in the agent panel, without opening the panel. The selected agent is launched the next time the panel is opened. Argument: `agent`: The id of the agent to select. |  |
| `agent::SelectNextThreadMatch` | agent: select next thread match | — | Selects the next thread search match. | `cmd-g` (AcpThread); `enter` (AcpThreadSearchBar) |
| `agent::SelectPermissionGranularity` | agent: select permission granularity | `{ tool_call_id: string, index: integer ≥ 0 }` | Action to select a permission granularity option from the dropdown. This updates the selected granularity without triggering authorization. Argument: `tool_call_id`: The tool call ID for which to select the granularity. `index`: The index of the selected granularity option. |  |
| `agent::SelectPreviousThreadMatch` | agent: select previous thread match | — | Selects the previous thread search match. | `cmd-shift-g` (AcpThread); `shift-enter` (AcpThreadSearchBar); `shift-enter` (AcpThreadSearchBar > Editor) |
| `agent::SendImmediately` | agent: send immediately | — | Interrupts the current generation and sends the message immediately. | `cmd-shift-enter` (AcpThread > Editor) |
| `agent::SendNextQueuedMessage` | agent: send next queued message | — | Sends the next queued message immediately. | `cmd-shift-alt-enter` (AcpThread > Editor) |
| `agent::Toggle` | agent: toggle | — | Toggles the agent panel. |  |
| `agent::ToggleCommandPattern` | agent: toggle command pattern | `{ tool_call_id: string, pattern_index: integer ≥ 0 }` | Action to toggle a command pattern checkbox in the permission dropdown. Argument: `tool_call_id`: The tool call ID for which to toggle the pattern. `pattern_index`: The index of the command pattern to toggle. |  |
| `agent::ToggleFastMode` | agent: toggle fast mode | — | Toggles fast mode for models that support it. | `cmd-alt-.` (AcpThread > Editor) |
| `agent::ToggleFocus` | agent: toggle focus | — | Aliases: `assistant::ToggleFocus` | `cmd-?` (Workspace) |
| `agent::ToggleModelSelector` | agent: toggle model selector | — | Toggles the language model selector dropdown. Aliases: `assistant::ToggleModelSelector`, `assistant2::ToggleModelSelector` | `cmd-alt-/` (AcpThread); `cmd-alt-/` (InlineAssistant > Editor) |
| `agent::ToggleNewThreadMenu` | agent: toggle new thread menu | — | Toggles the menu to create new agent threads. | `cmd-alt-shift-n` (AgentPanel) |
| `agent::ToggleOptionsMenu` | agent: toggle options menu | — | Toggles the options menu for agent settings and preferences. | `cmd-alt-m` (AgentPanel) |
| `agent::ToggleProfileSelector` | agent: toggle profile selector | — | Toggles the profile or mode selector for switching between agent profiles. | `cmd-i` (AcpThread); `cmd-i` (AcpThread > Editor) |
| `agent::ToggleSearch` | agent: toggle search | — | Toggles in-thread search over the current agent thread's contents. | `cmd-f` (AcpThread); `cmd-f` (AcpThread > Editor); `cmd-f` (AgentPanel > Terminal) |
| `agent::ToggleSteerFirstQueuedMessage` | agent: toggle steer first queued message | — | Toggles steering for the first queued message: when on, it interrupts the agent at its next step instead of waiting for it to finish. | `cmd-ctrl-s` (AcpThread > Editor) |
| `agent::ToggleThinkingEffortMenu` | agent: toggle thinking effort menu | — | Toggles the thinking effort selector menu open or closed. | `cmd-alt-'` (AcpThread > Editor) |
| `agent::ToggleThinkingMode` | agent: toggle thinking mode | — | Toggles thinking mode for models that support extended thinking. | `cmd-alt-k` (AcpThread > Editor) |
| `agent::UndoLastReject` | agent: undo last reject | — | Undoes the most recent reject operation, restoring the rejected changes. | `shift-alt-u` (AcpThread > Editor) |

## agents_sidebar

| Action | Palette name | Argument | Description | Default keys |
|---|---|---|---|---|
| `agents_sidebar::FocusSidebarFilter` | agents sidebar: focus sidebar filter | — | Moves focus to the sidebar's search/filter editor. | `cmd-f` (ThreadsSidebar); `/` (ThreadsSidebar && !Editor) [vim] |
| `agents_sidebar::NewThreadInGroup` | agents sidebar: new thread in group | — | Creates a new thread in the currently selected or active project group. | `cmd-n` (ThreadsSidebar); `o` (ThreadsSidebar && !Editor) [vim]; `shift-o` (ThreadsSidebar && !Editor) [vim] |
| `agents_sidebar::ToggleThreadHistory` | agents sidebar: toggle thread history | — | Toggles between the thread list and the thread history. | `cmd-g` (ThreadsSidebar) |
| `agents_sidebar::ToggleThreadSwitcher` | agents sidebar: toggle thread switcher | `{ select_last?: boolean = false }` | Toggles the thread switcher popup when the sidebar is focused. | `ctrl-tab` (AgentPanel); `ctrl-shift-tab` {"select_last": true} (AgentPanel); `ctrl-tab` (ThreadsSidebar); `ctrl-shift-tab` {"select_last": true} (ThreadsSidebar); `ctrl-tab` (ThreadSwitcher); +1 more |

## app_menu

| Action | Palette name | Argument | Description | Default keys |
|---|---|---|---|---|
| `app_menu::ActivateMenuLeft` | app menu: activate menu left | — | Activates the menu on the left in the client-side application menu. Does not apply to platform menu bars (e.g. on macOS). |  |
| `app_menu::ActivateMenuRight` | app menu: activate menu right | — | Activates the menu on the right in the client-side application menu. Does not apply to platform menu bars (e.g. on macOS). |  |
| `app_menu::OpenApplicationMenu` | app menu: open application menu | `string` | Opens the named menu in the client-side application menu. Does not apply to platform menu bars (e.g. on macOS). |  |

## assistant

| Action | Palette name | Argument | Description | Default keys |
|---|---|---|---|---|
| `assistant::InlineAssist` | assistant: inline assist | `{ prompt?: string \| null }` | Deploys the assistant interface with the specified configuration. | `ctrl-enter` (!AcpThread > Editor && mode == full); `ctrl-enter` (Terminal); `ctrl-x ctrl-a` (vim_mode == insert) [vim] |

## auto_update

| Action | Palette name | Argument | Description | Default keys |
|---|---|---|---|---|
| `auto_update::Check` | auto update: check | — | Checks for available updates. |  |
| `auto_update::DismissMessage` | auto update: dismiss message | — | Dismisses the update error message. |  |
| `auto_update::ViewReleaseNotes` | auto update: view release notes | — | Opens the release notes for the current version in a browser. |  |
| `auto_update::ViewReleaseNotesLocally` | auto update: view release notes locally | — | Opens the release notes for the current version in a new tab. |  |

## bedrock

| Action | Palette name | Argument | Description | Default keys |
|---|---|---|---|---|
| `bedrock::Tab` | bedrock: tab | — |  |  |
| `bedrock::TabPrev` | bedrock: tab prev | — |  |  |

## branch_picker

| Action | Palette name | Argument | Description | Default keys |
|---|---|---|---|---|
| `branch_picker::CycleBranchFilter` | branch picker: cycle branch filter | — | Cycle through branch filters. | `cmd-shift-i` (GitBranchSelector \|\| (GitBranchSelector > Picker > Editor)) |
| `branch_picker::DeleteBranch` | branch picker: delete branch | — | Deletes the selected git branch or remote. | `cmd-shift-backspace` (GitBranchSelector \|\| (GitBranchSelector > Picker > Editor)) |
| `branch_picker::ForceDeleteBranch` | branch picker: force delete branch | — | Force deletes the selected git branch or remote. | `cmd-alt-shift-backspace` (GitBranchSelector \|\| (GitBranchSelector > Picker > Editor)) |
| `branch_picker::ShowAllBranches` | branch picker: show all branches | — | Show all branches. |  |
| `branch_picker::ShowLocalBranches` | branch picker: show local branches | — | Show only local branches. |  |
| `branch_picker::ShowRemoteBranches` | branch picker: show remote branches | — | Show only remote branches. |  |
| `branch_picker::ToggleFilterMenu` | branch picker: toggle filter menu | — | Toggles the branch filter menu. | `cmd-k` (GitBranchSelector \|\| (GitBranchSelector > Picker > Editor)) |

## buffer_search

| Action | Palette name | Argument | Description | Default keys |
|---|---|---|---|---|
| `buffer_search::Deploy` | buffer search: deploy | `{ focus?: boolean = true, replace_enabled?: boolean = false, selection_search_enabled?: boolean = false }` | Opens the buffer search interface with the specified configuration. | `cmd-f` (Editor && mode == full); `cmd-alt-l` {"selection_search_enabled": true} (Editor && mode == full); `cmd-f` (Terminal); `cmd-f` (MarkdownPreview); `/` (MarkdownPreview) [vim] |
| `buffer_search::DeployReplace` | buffer search: deploy replace | — | Deploys the search and replace interface. |  |
| `buffer_search::Dismiss` | buffer search: dismiss | — | Dismisses the search bar. | `escape` (BufferSearchBar); `escape` (BufferSearchBar && !in_replace) [vim] |
| `buffer_search::FocusEditor` | buffer search: focus editor | — | Focuses back on the editor. | `tab` (BufferSearchBar) |
| `buffer_search::UseSelectionForFind` | buffer search: use selection for find | — | Sets the search query from the selection or word under cursor. | `cmd-e` (Editor && mode == full); `*` ((vim_mode == helix_normal \|\| vim_mode == helix_select) && !menu) [vim] |

## call_hierarchy

| Action | Palette name | Argument | Description | Default keys |
|---|---|---|---|---|
| `call_hierarchy::ShowIncomingCalls` | call hierarchy: show incoming calls | — |  | `cmd-k cmd-h` (Editor) |
| `call_hierarchy::ShowOutgoingCalls` | call hierarchy: show outgoing calls | — |  |  |
| `call_hierarchy::ToggleDirection` | call hierarchy: toggle direction | — |  | `cmd-k cmd-h` (CallHierarchyPicker > Picker > Editor) |

## channel_modal

| Action | Palette name | Argument | Description | Default keys |
|---|---|---|---|---|
| `channel_modal::RemoveMember` | channel modal: remove member | — | Removes the selected member from the channel. |  |
| `channel_modal::SelectNextControl` | channel modal: select next control | — | Selects the next control in the channel modal. |  |
| `channel_modal::ToggleMemberAdmin` | channel modal: toggle member admin | — | Toggles admin status for the selected member. |  |
| `channel_modal::ToggleMode` | channel modal: toggle mode | — | Toggles between invite members and manage members mode. | `tab` (ChannelModal); `tab` (ChannelModal > Picker > Editor) |

## cli

| Action | Palette name | Argument | Description | Default keys |
|---|---|---|---|---|
| `cli::InstallCliBinary` | cli: install cli binary | — | Installs the Zed CLI tool to the system PATH. |  |
| `cli::RegisterZedScheme` | cli: register zed scheme | — | Registers the zed:// URL scheme handler. |  |

## client

| Action | Palette name | Argument | Description | Default keys |
|---|---|---|---|---|
| `client::Reconnect` | client: reconnect | — | Reconnects to the collaboration server. |  |
| `client::SignIn` | client: sign in | — | Signs in to Zed account. |  |
| `client::SignOut` | client: sign out | — | Signs out of Zed account. |  |

## collab

| Action | Palette name | Argument | Description | Default keys |
|---|---|---|---|---|
| `collab::CopyLink` | collab: copy link | — | Copies a link to the current position in the channel buffer. |  |
| `collab::CopyRoomId` | collab: copy room id | — | Copies the current room name and session id for debugging purposes. |  |
| `collab::Deafen` | collab: deafen | — | Deafens yourself (mute both microphone and speakers). |  |
| `collab::LeaveCall` | collab: leave call | — | Leaves the current call. |  |
| `collab::Mute` | collab: mute | — | Mutes your microphone. |  |
| `collab::OpenChannelNotes` | collab: open channel notes | — | Opens the channel notes for the current call. Use `collab_panel::OpenSelectedChannelNotes` to open the channel notes for the selected channel in the collab panel. If you want to open a specific channel, use `zed::OpenZedUrl` with a channel notes URL - can be copied via "Copy link to section" in the context menu of the channel notes buffer. These URLs look like `https://zed.dev/channel/channel-name-CHANNEL_ID/notes`. |  |
| `collab::OpenChannelNotesById` | collab: open channel notes by id | `{ channel_id: integer ≥ 0 }` | Opens the channel notes for a specific channel by its ID. |  |
| `collab::ScreenShare` | collab: screen share | — | Shares your screen with collaborators. |  |
| `collab::ShareProject` | collab: share project | — | Shares the current project with collaborators. |  |
| `collab::ShowCallStats` | collab: show call stats | — | Show call diagnostics and connection quality statistics. |  |
| `collab::SimulateUpdateAvailable` | collab: simulate update available | — | A debug action to simulate an update being available to test the update banner UI. |  |
| `collab::SwitchBranch` | collab: switch branch | — | Switches to a different git branch. |  |
| `collab::ToggleProjectMenu` | collab: toggle project menu | — | Toggles the project menu dropdown. |  |
| `collab::ToggleUserMenu` | collab: toggle user menu | — | Toggles the user menu dropdown. |  |

## collab_panel

| Action | Palette name | Argument | Description | Default keys |
|---|---|---|---|---|
| `collab_panel::CollapseSelectedChannel` | collab panel: collapse selected channel | — | Collapses the selected channel in the tree view. |  |
| `collab_panel::ExpandSelectedChannel` | collab panel: expand selected channel | — | Expands the selected channel in the tree view. |  |
| `collab_panel::InsertSpace` | collab panel: insert space | — | Inserts a space character in the filter input. | `space` ((CollabPanel && editing) > Editor) |
| `collab_panel::MoveChannelDown` | collab panel: move channel down | — | Moves the selected channel down in the list. | `alt-down` (CollabPanel) |
| `collab_panel::MoveChannelUp` | collab panel: move channel up | — | Moves the selected channel up in the list. | `alt-up` (CollabPanel) |
| `collab_panel::MoveSelected` | collab panel: move selected | — | Moves the selected item to the current location. |  |
| `collab_panel::OpenSelectedChannelNotes` | collab panel: open selected channel notes | — | Opens the meeting notes for the selected channel in the panel. Use `collab::OpenChannelNotes` to open the channel notes for the current call. | `alt-enter` (CollabPanel) |
| `collab_panel::Remove` | collab panel: remove | — | Removes the selected channel or contact. | `ctrl-backspace` (CollabPanel && not_editing) |
| `collab_panel::Secondary` | collab panel: secondary | — | Opens the context menu for the selected item. |  |
| `collab_panel::StartMoveChannel` | collab panel: start move channel | — | Starts moving a channel to a new location. |  |
| `collab_panel::Toggle` | collab panel: toggle | — | Toggles the collab panel. |  |
| `collab_panel::ToggleFocus` | collab panel: toggle focus | — | Toggles focus on the collaboration panel. | `cmd-shift-c` |
| `collab_panel::ToggleSelectedChannelFavorite` | collab panel: toggle selected channel favorite | — | Toggles whether the selected channel is in the Favorites section. | `shift-enter` (CollabPanel) |

## command_palette

| Action | Palette name | Argument | Description | Default keys |
|---|---|---|---|---|
| `command_palette::RemoveSelected` | command palette: remove selected | — |  | `shift-backspace` (CommandPalette \|\| (CommandPalette > Picker > Editor)) |
| `command_palette::Toggle` | command palette: toggle | — | Toggles the command palette. | `cmd-shift-p` (Workspace); `:` (vim_mode == normal) [vim]; `:` ((vim_mode == helix_normal \|\| vim_mode == helix_select) && !menu) [vim]; `:` (!Editor && !Terminal) [vim]; `:` (ProjectPanel && not_editing) [vim] |

## console

| Action | Palette name | Argument | Description | Default keys |
|---|---|---|---|---|
| `console::WatchExpression` | console: watch expression | — | Adds an expression to the watch list. | `alt-enter` (DebugConsole > Editor) |

## context_server

| Action | Palette name | Argument | Description | Default keys |
|---|---|---|---|---|
| `context_server::Restart` | context server: restart | — | Restarts the context server. |  |

## copilot_edit_predictions

| Action | Palette name | Argument | Description | Default keys |
|---|---|---|---|---|
| `copilot_edit_predictions::Reinstall` | copilot edit predictions: reinstall | — | Reinstalls the Copilot Edit Predictions language server. Aliases: `copilot::Reinstall` |  |

## debug_panel

| Action | Palette name | Argument | Description | Default keys |
|---|---|---|---|---|
| `debug_panel::Toggle` | debug panel: toggle | — | Toggles the debug panel. |  |
| `debug_panel::ToggleFocus` | debug panel: toggle focus | — | Toggles focus on the debug panel. | `cmd-shift-d` (Workspace) |

## debugger

| Action | Palette name | Argument | Description | Default keys |
|---|---|---|---|---|
| `debugger::ClearAllBreakpoints` | debugger: clear all breakpoints | — | Clears all breakpoints in the project. |  |
| `debugger::Continue` | debugger: continue | — | Continues all threads until the next breakpoint. | `f5` (Workspace && debugger_stopped); `space shift-g c` ((vim_mode == helix_normal \|\| vim_mode == helix_select) && !menu) [vim] |
| `debugger::ContinueThread` | debugger: continue thread | — | Continues the selected thread until the next breakpoint. |  |
| `debugger::Detach` | debugger: detach | — | Detaches the debugger from the running process. |  |
| `debugger::EvaluateSelectedText` | debugger: evaluate selected text | — | Evaluates the selected text in the debugger context. |  |
| `debugger::FocusBreakpointList` | debugger: focus breakpoint list | — | Focuses on the breakpoint list panel. |  |
| `debugger::FocusConsole` | debugger: focus console | — | Focuses on the debugger console panel. |  |
| `debugger::FocusFrames` | debugger: focus frames | — | Focuses on the call stack frames panel. |  |
| `debugger::FocusLoadedSources` | debugger: focus loaded sources | — | Focuses on the loaded sources panel. |  |
| `debugger::FocusModules` | debugger: focus modules | — | Focuses on the loaded modules panel. |  |
| `debugger::FocusTerminal` | debugger: focus terminal | — | Focuses on the terminal panel. |  |
| `debugger::FocusVariables` | debugger: focus variables | — | Focuses on the variables panel. |  |
| `debugger::GoToSelectedAddress` | debugger: go to selected address | — |  |  |
| `debugger::NextBreakpointProperty` | debugger: next breakpoint property | — | Navigates to the next breakpoint property in the list. | `right` (BreakpointList) |
| `debugger::OpenProjectDebugTasks` | debugger: open project debug tasks | — | Opens the project debug tasks configuration. |  |
| `debugger::Pause` | debugger: pause | — | Pauses the currently running program. | `f6` (Workspace && debugger_session); `space shift-g h` ((vim_mode == helix_normal \|\| vim_mode == helix_select) && !menu) [vim] |
| `debugger::PreviousBreakpointProperty` | debugger: previous breakpoint property | — | Navigates to the previous breakpoint property in the list. | `left` (BreakpointList) |
| `debugger::Rerun` | debugger: rerun | — | Reruns the last debugging session. Aliases: `debugger::RerunLastSession` | `f5` (Workspace) |
| `debugger::RerunSession` | debugger: rerun session | — | Reruns the current debugging session with the same configuration. | `shift-cmd-f5` (Workspace && debugger_session) |
| `debugger::Restart` | debugger: restart | — | Restarts the current debugging session. | `space shift-g r` ((vim_mode == helix_normal \|\| vim_mode == helix_select) && !menu) [vim] |
| `debugger::RunToCursor` | debugger: run to cursor | — | Runs program execution to the current cursor position. |  |
| `debugger::Start` | debugger: start | — | Starts a new debugging session. | `f4`; `space shift-g l` ((vim_mode == helix_normal \|\| vim_mode == helix_select) && !menu) [vim] |
| `debugger::StepBack` | debugger: step back | — | Steps back to the previous statement. |  |
| `debugger::StepInto` | debugger: step into | — | Steps into the next function call. | `f11` (Workspace && debugger_stopped); `ctrl-f11` (Workspace && debugger_stopped); `space shift-g i` ((vim_mode == helix_normal \|\| vim_mode == helix_select) && !menu) [vim] |
| `debugger::StepOut` | debugger: step out | — | Steps out of the current function. | `shift-f11` (Workspace && debugger_stopped); `space shift-g o` ((vim_mode == helix_normal \|\| vim_mode == helix_select) && !menu) [vim] |
| `debugger::StepOver` | debugger: step over | — | Steps over the current line. | `f7` (Workspace && debugger_stopped); `f10` (Workspace && debugger_stopped); `space shift-g n` ((vim_mode == helix_normal \|\| vim_mode == helix_select) && !menu) [vim] |
| `debugger::Stop` | debugger: stop | — | Stops the debugging session. | `shift-f5` (Workspace && debugger_session); `space shift-g t` ((vim_mode == helix_normal \|\| vim_mode == helix_select) && !menu) [vim] |
| `debugger::ToggleDataBreakpoint` | debugger: toggle data breakpoint | `{ access_type?: DataBreakpointAccessType \| null }` | Set a data breakpoint on the selected variable or memory region. Argument: `access_type`: The type of data breakpoint Read & Write Read Write |  |
| `debugger::ToggleEnableBreakpoint` | debugger: toggle enable breakpoint | — | Toggles the enabled state of a breakpoint. | `space` (BreakpointList) |
| `debugger::ToggleExpandItem` | debugger: toggle expand item | — | Toggles expansion of the selected item in the debugger UI. | `shift-alt-escape` (DebugPanel) |
| `debugger::ToggleIgnoreBreakpoints` | debugger: toggle ignore breakpoints | — | Toggles whether to ignore all breakpoints. |  |
| `debugger::ToggleSessionPicker` | debugger: toggle session picker | — | Toggles the session picker dropdown. | `cmd-i` (DebugPanel) |
| `debugger::ToggleThreadPicker` | debugger: toggle thread picker | — | Toggles the thread picker dropdown. | `cmd-t` (DebugPanel) |
| `debugger::ToggleUserFrames` | debugger: toggle user frames | — | Toggle the user frame filter in the stack frame list When toggled on, only frames from the user's code are shown When toggled off, all frames are shown |  |
| `debugger::UnsetBreakpoint` | debugger: unset breakpoint | — | Removes a breakpoint. | `backspace` (BreakpointList) |

## dev

| Action | Palette name | Argument | Description | Default keys |
|---|---|---|---|---|
| `dev::CopyAccessibilityTree` | dev: copy accessibility tree | — | Copies the current accessibility tree to the clipboard as JSON, without opening a buffer. See [`DumpAccessibilityTree`]. |  |
| `dev::CopyDebugAdapterArguments` | dev: copy debug adapter arguments | — | Copies debug adapter launch arguments to clipboard. |  |
| `dev::DebugFilesystemWatching` | dev: debug filesystem watching | — | Open an app-wide recording of raw local filesystem watcher events. |  |
| `dev::DumpAccessibilityTree` | dev: dump accessibility tree | — | Dumps the current accessibility tree (the last update sent to the platform adapter) to a new buffer as JSON, for debugging what is exposed to assistive technology. |  |
| `dev::DumpInputLatencyHistogram` | dev: dump input latency histogram | — | Opens a buffer showing the input-to-frame latency histogram for the current window. |  |
| `dev::DumpWorkspaceInfo` | dev: dump workspace info | — | Dumps multi-workspace state (projects, worktrees, active threads) into a new buffer. |  |
| `dev::EditPredictionContextGoBack` | dev: edit prediction context go back | — | Go to the previous context retrieval run | `alt-left` (EditPredictionContext > Editor) |
| `dev::EditPredictionContextGoForward` | dev: edit prediction context go forward | — | Go to the next context retrieval run | `alt-right` (EditPredictionContext > Editor) |
| `dev::HangAction` | dev: hang action | — | Causes a performance hang to test performance monitoring |  |
| `dev::HangBackground` | dev: hang background | — | Causes a performance hang to test performance monitoring |  |
| `dev::HangForeground` | dev: hang foreground | — | Causes a performance hang to test performance monitoring |  |
| `dev::OpenAcpLogs` | dev: open acp logs | — |  |  |
| `dev::OpenDebugAdapterLogs` | dev: open debug adapter logs | — | Opens the debug adapter protocol logs viewer. |  |
| `dev::OpenEditPredictionContextView` | dev: open edit prediction context view | — | Opens the edit prediction context view. |  |
| `dev::OpenHighlightsTreeView` | dev: open highlights tree view | — | Opens the highlights tree view for the current file. |  |
| `dev::OpenKeyContextView` | dev: open key context view | — | Opens the key context view for debugging keybindings. |  |
| `dev::OpenLanguageServerLogs` | dev: open language server logs | — | Opens the language server protocol logs viewer. |  |
| `dev::OpenSyntaxTreeView` | dev: open syntax tree view | — | Opens the syntax tree view for the current file. |  |
| `dev::OpenThemePreview` | dev: open theme preview | — | Opens the theme preview window. |  |
| `dev::OpenUrlPrompt` | dev: open url prompt | — | Opens a prompt to enter a URL to open. |  |
| `dev::ResetFrameOverlayStats` | dev: reset frame overlay stats | — | Resets the debug frame-time overlay's statistics, except for the total frame count. | `ctrl-alt-shift-o` |
| `dev::ShowAllSidebarThreadMetadata` | dev: show all sidebar thread metadata | — | Shows metadata for all threads in the sidebar. |  |
| `dev::ShowGitJobQueue` | dev: show git job queue | — | Shows the current git job queue debug state for the active repository. |  |
| `dev::ShowThreadMetadata` | dev: show thread metadata | — | Shows metadata for the currently active thread. |  |
| `dev::ToggleFpsOverlay` | dev: toggle fps overlay | — | Cycles the debug frame-time overlay between hidden, current frame-time, and detailed frame-time statistics. | `ctrl-alt-shift-p` |
| `dev::ToggleInspector` | dev: toggle inspector | — | Toggles the developer inspector for debugging UI elements. | `cmd-alt-i` |

## diagnostics

| Action | Palette name | Argument | Description | Default keys |
|---|---|---|---|---|
| `diagnostics::Deploy` | diagnostics: deploy | — | Opens the project diagnostics view. | `cmd-shift-m` (Workspace) |
| `diagnostics::DeployCurrentFile` | diagnostics: deploy current file | — | Opens the project diagnostics view for the currently focused file. |  |
| `diagnostics::ToggleDiagnosticsRefresh` | diagnostics: toggle diagnostics refresh | — | Toggles automatic refresh of diagnostics. | `ctrl-r` (Diagnostics) |
| `diagnostics::ToggleWarnings` | diagnostics: toggle warnings | — | Toggles the display of warning-level diagnostics. |  |

## edit_prediction

| Action | Palette name | Argument | Description | Default keys |
|---|---|---|---|---|
| `edit_prediction::ClearHistory` | edit prediction: clear history | — | Clears the edit prediction history. |  |
| `edit_prediction::RatePredictions` | edit prediction: rate predictions | — | Opens the rate completions modal. | `ctrl-cmd-z` |
| `edit_prediction::ResetOnboarding` | edit prediction: reset onboarding | — | Resets the edit prediction onboarding state. |  |
| `edit_prediction::ToggleMenu` | edit prediction: toggle menu | — | Toggles the edit prediction menu. | `ctrl-cmd-i` |

## editor

| Action | Palette name | Argument | Description | Default keys |
|---|---|---|---|---|
| `editor::AcceptEditPrediction` | editor: accept edit prediction | — | Accepts the full edit prediction. | `alt-tab` (Editor && edit_prediction); `tab` (Editor && edit_prediction && edit_prediction_mode == eager && !showing_completions); `tab` (Editor && edit_prediction && edit_prediction_mode == eager && !showing_completions) [vim] |
| `editor::AcceptNextLineEditPrediction` | editor: accept next line edit prediction | — |  | `ctrl-cmd-down` (Editor && edit_prediction) |
| `editor::AcceptNextWordEditPrediction` | editor: accept next word edit prediction | — | Accepts a partial edit prediction. Aliases: `editor::AcceptPartialCopilotSuggestion` | `ctrl-cmd-right` (Editor && edit_prediction) |
| `editor::AddSelectionAbove` | editor: add selection above | `{ skip_soft_wrap?: boolean = true }` | Adds a cursor above the current selection. | `cmd-ctrl-p` {"skip_soft_wrap": false} (Editor); `cmd-alt-up` {"skip_soft_wrap": true} (Editor) |
| `editor::AddSelectionBelow` | editor: add selection below | `{ skip_soft_wrap?: boolean = true }` | Adds a cursor below the current selection. | `cmd-ctrl-n` {"skip_soft_wrap": false} (Editor); `cmd-alt-down` {"skip_soft_wrap": true} (Editor) |
| `editor::AlignSelections` | editor: align selections | — | Aligns selections from different rows into the same column |  |
| `editor::ApplyAllDiffHunks` | editor: apply all diff hunks | — | Applies all diff hunks in the editor. |  |
| `editor::ApplyDiffHunk` | editor: apply diff hunk | — | Applies the diff hunk at the current position. |  |
| `editor::AutoIndent` | editor: auto indent | — | Automatically adjusts indentation based on context. |  |
| `editor::Backspace` | editor: backspace | — | Deletes the character before the cursor. | `shift-backspace` (Editor); `ctrl-h` (Editor); `backspace` (Editor); `ctrl-h` (Picker > Editor) [vim] |
| `editor::Backtab` | editor: backtab | — | Removes a tab character or outdents. | `shift-tab` (Editor) |
| `editor::BlameHover` | editor: blame hover | — | Shows git blame information for the current line. | `cmd-k cmd-b` (Editor); `g B` (VimControl && !menu) [vim] |
| `editor::BlamePreviousRevision` | editor: blame previous revision | — | Opens a blame of the file at the revision preceding the blame entry at cursor. |  |
| `editor::BlameRevision` | editor: blame revision | — | Opens a blame of the file at the revision of the blame entry at cursor. |  |
| `editor::Cancel` | editor: cancel | — | Cancels the current operation. | `escape` (Editor); `ctrl-[` (vim_mode == normal) [vim]; `ctrl-x ctrl-z` (vim_mode == insert) [vim]; `escape` ((vim_mode == normal \|\| vim_mode == helix_normal) && !menu) [vim]; `ctrl-[` (VimControl && vim_mode == helix_normal && !menu && !BufferSearchBar) [vim] |
| `editor::CancelEditReviewComment` | editor: cancel edit review comment | `{ id: integer ≥ 0 }` | Cancels an inline edit of a review comment. |  |
| `editor::CancelFlycheck` | editor: cancel flycheck | — | Cancels the running flycheck operation. |  |
| `editor::CancelLanguageServerWork` | editor: cancel language server work | — | Cancels pending language server work. |  |
| `editor::ClearFlycheck` | editor: clear flycheck | — | Clears flycheck results. |  |
| `editor::CollapseAllDiffHunks` | editor: collapse all diff hunks | — | Collapses all diff hunks in the editor. |  |
| `editor::ComposeCompletion` | editor: compose completion | `{ item_ix?: integer ≥ 0 \| null }` | Composes multiple completion suggestions into a single completion. | `tab` (Editor && showing_completions) |
| `editor::ConfirmCodeAction` | editor: confirm code action | `{ item_ix?: integer ≥ 0 \| null }` | Confirms and applies the currently selected code action. | `enter` (Editor && showing_code_actions) |
| `editor::ConfirmCompletion` | editor: confirm completion | `{ item_ix?: integer ≥ 0 \| null }` | Confirms and accepts the currently selected completion suggestion. | `enter` (Editor && showing_completions) |
| `editor::ConfirmCompletionInsert` | editor: confirm completion insert | — | Confirms completion by inserting at cursor. |  |
| `editor::ConfirmCompletionReplace` | editor: confirm completion replace | — | Confirms completion by replacing existing text. | `shift-enter` (Editor && showing_completions) |
| `editor::ConfirmEditReviewComment` | editor: confirm edit review comment | `{ id: integer ≥ 0 }` | Confirms an inline edit of a review comment. |  |
| `editor::ConfirmRename` | editor: confirm rename | — | Confirms the rename operation. | `enter` (Editor && renaming) |
| `editor::ContextMenuFirst` | editor: context menu first | — | Navigates to the first item in the context menu. | `pageup` (Editor && (showing_code_actions \|\| showing_completions)) |
| `editor::ContextMenuLast` | editor: context menu last | — | Navigates to the last item in the context menu. | `pagedown` (Editor && (showing_code_actions \|\| showing_completions)) |
| `editor::ContextMenuNext` | editor: context menu next | — | Navigates to the next item in the context menu. | `down` (Editor && (showing_code_actions \|\| showing_completions)); `ctrl-n` (Editor && (showing_code_actions \|\| showing_completions)); `tab` (Editor && (vim_mode == helix_normal \|\| vim_mode == helix_select) && showing_code_actions) [vim] |
| `editor::ContextMenuPrevious` | editor: context menu previous | — | Navigates to the previous item in the context menu. | `up` (Editor && (showing_code_actions \|\| showing_completions)); `ctrl-p` (Editor && (showing_code_actions \|\| showing_completions)); `shift-tab` (Editor && (vim_mode == helix_normal \|\| vim_mode == helix_select) && showing_code_actions) [vim] |
| `editor::ConvertFromBase64` | editor: convert from base64 | — | Base64-decodes the selected text or word under cursor. |  |
| `editor::ConvertIndentationToSpaces` | editor: convert indentation to spaces | — | Converts indentation from tabs to spaces. |  |
| `editor::ConvertIndentationToTabs` | editor: convert indentation to tabs | — | Converts indentation from spaces to tabs. |  |
| `editor::ConvertToBase64` | editor: convert to base64 | — | Base64-encodes the selected text or word under cursor. |  |
| `editor::ConvertToKebabCase` | editor: convert to kebab case | — | Converts selected text to kebab-case. |  |
| `editor::ConvertToLowerCamelCase` | editor: convert to lower camel case | — | Converts selected text to lowerCamelCase. |  |
| `editor::ConvertToLowerCase` | editor: convert to lower case | — | Converts selected text to lowercase. |  |
| `editor::ConvertToOppositeCase` | editor: convert to opposite case | — | Toggles the case of selected text. |  |
| `editor::ConvertToRot13` | editor: convert to rot13 | — | Applies ROT13 cipher to selected text. |  |
| `editor::ConvertToRot47` | editor: convert to rot47 | — | Applies ROT47 cipher to selected text. |  |
| `editor::ConvertToSentenceCase` | editor: convert to sentence case | — | Converts selected text to sentence case. |  |
| `editor::ConvertToSnakeCase` | editor: convert to snake case | — | Converts selected text to snake_case. |  |
| `editor::ConvertToTitleCase` | editor: convert to title case | — | Converts selected text to Title Case. |  |
| `editor::ConvertToUpperCamelCase` | editor: convert to upper camel case | — | Converts selected text to UpperCamelCase. |  |
| `editor::ConvertToUpperCase` | editor: convert to upper case | — | Converts selected text to UPPERCASE. |  |
| `editor::Copy` | editor: copy | — | Copies selected text to the clipboard. | `cmd-c` (Editor); `space y` ((vim_mode == helix_normal \|\| vim_mode == helix_select) && !menu) [vim] |
| `editor::CopyAndTrim` | editor: copy and trim | — | Copies selected text to the clipboard with leading/trailing whitespace trimmed. |  |
| `editor::CopyFileLocation` | editor: copy file location | — | Copies the current file location to the clipboard. |  |
| `editor::CopyFileName` | editor: copy file name | — | Copies the current file name to the clipboard. |  |
| `editor::CopyFileNameWithoutExtension` | editor: copy file name without extension | — | Copies the file name without extension to the clipboard. |  |
| `editor::CopyHighlightJson` | editor: copy highlight json | — | Copies the highlighted text as JSON. |  |
| `editor::CopyPermalinkToLine` | editor: copy permalink to line | — | Copies a permalink to the current line or selection. |  |
| `editor::Cut` | editor: cut | — | Cuts selected text to the clipboard. | `cmd-x` (Editor) |
| `editor::CutToEndOfLine` | editor: cut to end of line | `{ stop_at_newlines?: boolean = false }` | Cuts from cursor to end of line. |  |
| `editor::Delete` | editor: delete | — | Deletes the character after the cursor. | `ctrl-d` (Editor); `delete` (Editor); `alt-d` ((vim_mode == helix_normal \|\| vim_mode == helix_select) && !menu) [vim] |
| `editor::DeleteLine` | editor: delete line | — | Deletes the current line. | `cmd-shift-k` (Editor) |
| `editor::DeleteReviewComment` | editor: delete review comment | `{ id: integer ≥ 0 }` | Deletes a stored review comment. |  |
| `editor::DeleteToBeginningOfLine` | editor: delete to beginning of line | `{ stop_at_indent?: boolean = false }` | Deletes from the cursor to the beginning of the current line. | `cmd-backspace` (Editor); `ctrl-u` (vim_mode == insert) [vim]; `ctrl-u` (Picker > Editor) [vim] |
| `editor::DeleteToEndOfLine` | editor: delete to end of line | — | Deletes from cursor to end of line. | `cmd-delete` (Editor) |
| `editor::DeleteToNextSubwordEnd` | editor: delete to next subword end | `{ ignore_newlines?: boolean = false, ignore_brackets?: boolean = false }` | Deletes from the cursor to the end of the next subword. Stops before the end of the next subword, if whitespace sequences of length >= 2 are encountered. | `ctrl-alt-delete` (Editor); `ctrl-alt-d` (Editor) |
| `editor::DeleteToNextWordEnd` | editor: delete to next word end | `{ ignore_newlines?: boolean = false, ignore_brackets?: boolean = false }` | Deletes from the cursor to the end of the next word. Stops before the end of the next word, if whitespace sequences of length >= 2 are encountered. | `alt-delete` {…} (Editor) |
| `editor::DeleteToPreviousSubwordStart` | editor: delete to previous subword start | `{ ignore_newlines?: boolean = false, ignore_brackets?: boolean = false }` | Deletes from the cursor to the start of the previous subword. Stops before the start of the previous subword, if whitespace sequences of length >= 2 are encountered. | `ctrl-alt-backspace` (Editor); `ctrl-alt-h` (Editor) |
| `editor::DeleteToPreviousWordStart` | editor: delete to previous word start | `{ ignore_newlines?: boolean = false, ignore_brackets?: boolean = false }` | Deletes from the cursor to the start of the previous word. Stops before the start of the previous word, if whitespace sequences of length >= 2 are encountered. | `alt-backspace` {…} (Editor); `ctrl-w` {…} (Editor); `ctrl-w` {…} (vim_mode == insert) [vim]; `ctrl-w` (Picker > Editor) [vim] |
| `editor::DiffClipboardWithSelection` | editor: diff clipboard with selection | — | Diffs the text stored in the clipboard against the current selection. |  |
| `editor::DisableBreakpoint` | editor: disable breakpoint | — | Disables the breakpoint at the current line. |  |
| `editor::DisplayCursorNames` | editor: display cursor names | — | Displays names of all active cursors. | `ctrl-cmd-c` |
| `editor::DuplicateLineDown` | editor: duplicate line down | — | Duplicates the current line below. | `alt-shift-down` (Editor) |
| `editor::DuplicateLineUp` | editor: duplicate line up | — | Duplicates the current line above. | `alt-shift-up` (Editor) |
| `editor::DuplicateSelection` | editor: duplicate selection | — | Duplicates the current selection. |  |
| `editor::EditBookmark` | editor: edit bookmark | — | Edits the bookmark's label at the current line. |  |
| `editor::EditLogBreakpoint` | editor: edit log breakpoint | — | Edits the log message for a breakpoint. | `shift-f9` (Editor); `space shift-g ctrl-l` ((vim_mode == helix_normal \|\| vim_mode == helix_select) && !menu) [vim] |
| `editor::EditReviewComment` | editor: edit review comment | `{ id: integer ≥ 0 }` | Edits a stored review comment inline. |  |
| `editor::EnableBreakpoint` | editor: enable breakpoint | — | Enables the breakpoint at the current line. |  |
| `editor::ExpandAllDiffHunks` | editor: expand all diff hunks | — | Expands all diff hunks in the editor. Aliases: `editor::ExpandAllHunkDiffs` | `cmd-"` (Editor) |
| `editor::ExpandExcerpts` | editor: expand excerpts | `{ lines?: integer ≥ 0 = 0 }` | Expands all excerpts with selections. | `shift-enter` (!AcpThread > Editor && mode == full) |
| `editor::ExpandExcerptsDown` | editor: expand excerpts down | `{ lines?: integer ≥ 0 = 0 }` | Expands excerpts below the current position. |  |
| `editor::ExpandExcerptsUp` | editor: expand excerpts up | `{ lines?: integer ≥ 0 = 0 }` | Expands excerpts above the current position. |  |
| `editor::ExpandMacroRecursively` | editor: expand macro recursively | — | Expands macros recursively at cursor position. |  |
| `editor::FindAllReferences` | editor: find all references | `{ always_open_multibuffer?: boolean = true, open_results_in?: OpenResultsIn \| null }` | Finds all references to the symbol at cursor. Argument: `open_results_in`: Where to show the references. Falls back to the `lsp_results_location` setting when omitted. A single result is always opened directly. | `alt-shift-f12` (Editor); `g r r` (VimControl && !menu) [vim]; `g shift-a` (VimControl && !menu) [vim]; `g r` ((vim_mode == helix_normal \|\| vim_mode == helix_select) && !menu) [vim] |
| `editor::FindNextMatch` | editor: find next match | — | Finds the next match in the search. |  |
| `editor::FindPreviousMatch` | editor: find previous match | — | Finds the previous match in the search. |  |
| `editor::Fold` | editor: fold | — | Folds the current code block. | `alt-cmd-[` (Editor); `z c` (VimControl && !menu) [vim] |
| `editor::FoldAll` | editor: fold all | — | Folds all foldable regions in the editor. | `cmd-k cmd-0` (Editor); `z shift-m` (VimControl && !menu) [vim]; `z shift-m` (ThreadsSidebar && !Editor) [vim] |
| `editor::FoldAtLevel` | editor: fold at level | `integer ≥ 0` | Folds all code blocks at the specified indentation level. |  |
| `editor::FoldAtLevel_1` | editor: fold at level 1 | — | Folds all code blocks at indentation level 1. | `cmd-k cmd-1` (Editor) |
| `editor::FoldAtLevel_2` | editor: fold at level 2 | — | Folds all code blocks at indentation level 2. | `cmd-k cmd-2` (Editor) |
| `editor::FoldAtLevel_3` | editor: fold at level 3 | — | Folds all code blocks at indentation level 3. | `cmd-k cmd-3` (Editor) |
| `editor::FoldAtLevel_4` | editor: fold at level 4 | — | Folds all code blocks at indentation level 4. | `cmd-k cmd-4` (Editor) |
| `editor::FoldAtLevel_5` | editor: fold at level 5 | — | Folds all code blocks at indentation level 5. | `cmd-k cmd-5` (Editor) |
| `editor::FoldAtLevel_6` | editor: fold at level 6 | — | Folds all code blocks at indentation level 6. | `cmd-k cmd-6` (Editor) |
| `editor::FoldAtLevel_7` | editor: fold at level 7 | — | Folds all code blocks at indentation level 7. | `cmd-k cmd-7` (Editor) |
| `editor::FoldAtLevel_8` | editor: fold at level 8 | — | Folds all code blocks at indentation level 8. | `cmd-k cmd-8` (Editor) |
| `editor::FoldAtLevel_9` | editor: fold at level 9 | — | Folds all code blocks at indentation level 9. | `cmd-k cmd-9` (Editor) |
| `editor::FoldFunctionBodies` | editor: fold function bodies | — | Folds all function bodies in the editor. |  |
| `editor::FoldRecursive` | editor: fold recursive | — | Folds the current code block and all its children. | `cmd-k cmd-[` (Editor); `z shift-c` (VimControl && !menu) [vim] |
| `editor::FoldSelectedRanges` | editor: fold selected ranges | — | Folds the selected ranges. | `z f` (VimControl && !menu) [vim] |
| `editor::Format` | editor: format | — | Formats the entire document. | `cmd-shift-i` (Editor) |
| `editor::FormatSelections` | editor: format selections | — | Formats only the selected text. This action is only available when the active formatter can format ranges. When using a language server, this sends an LSP range formatting request for each selection, and is hidden when the selected buffer's configured language server does not advertise range-formatting support. When using Prettier, Prettier's own range formatting is used to format the encompassing range of all selections, and resulting edits outside the selected ranges are discarded. External command formatters do not support range formatting and are skipped. |  |
| `editor::GoToDeclaration` | editor: go to declaration | `{ open_results_in?: OpenResultsIn \| null }` | Goes to the declaration of the symbol at cursor. Argument: `open_results_in`: Where to show the declarations. Falls back to the `lsp_results_location` setting when omitted. A single result is always opened directly. | `ctrl-f12` (Editor); `g shift-d` (VimControl && !menu) [vim] |
| `editor::GoToDeclarationSplit` | editor: go to declaration split | — | Goes to declaration in a split pane. | `alt-ctrl-f12` (Editor) |
| `editor::GoToDefinition` | editor: go to definition | `{ open_results_in?: OpenResultsIn \| null }` | Goes to the definition of the symbol at cursor. Argument: `open_results_in`: Where to show the definitions. Falls back to the `lsp_results_location` setting when omitted. A single result is always opened directly. | `f12` (Editor); `ctrl-]` (VimControl && !menu) [vim]; `g d` (VimControl && !menu) [vim] |
| `editor::GoToDefinitionSplit` | editor: go to definition split | — | Goes to definition in a split pane. | `alt-f12` (Editor); `ctrl-w d` (VimControl && !menu) [vim]; `ctrl-w g d` (VimControl && !menu) [vim]; `ctrl-w ]` (VimControl && !menu) [vim]; `ctrl-w ctrl-]` (VimControl && !menu) [vim] |
| `editor::GoToDiagnostic` | editor: go to diagnostic | `{ severity?: GoToDiagnosticSeverityFilter }` | Expands the diagnostic under the cursor, if any, in case diagnostics are not yet active. Otherwise, goes to the next diagnostic in the file. | `f8` {…} (Editor); `g ]` (VimControl && !menu) [vim]; `] d` (vim_mode == normal) [vim]; `space d` ((vim_mode == helix_normal \|\| vim_mode == helix_select) && !menu) [vim]; `d` (vim_operator == helix_next) [vim] |
| `editor::GoToHunk` | editor: go to hunk | — | Goes to the next diff hunk. | `cmd-f8` (!AcpThread > Editor && mode == full); `] c` (vim_mode == normal) [vim]; `g` (vim_operator == helix_next) [vim] |
| `editor::GoToImplementation` | editor: go to implementation | `{ open_results_in?: OpenResultsIn \| null }` | Goes to the implementation of the symbol at cursor. Argument: `open_results_in`: Where to show the implementations. Falls back to the `lsp_results_location` setting when omitted. A single result is always opened directly. | `shift-f12` (Editor); `g r i` (VimControl && !menu) [vim]; `g shift-i` (VimControl && !menu) [vim]; `g i` ((vim_mode == helix_normal \|\| vim_mode == helix_select) && !menu) [vim] |
| `editor::GoToImplementationSplit` | editor: go to implementation split | — | Goes to implementation in a split pane. |  |
| `editor::GoToNextBookmark` | editor: go to next bookmark | — | Goes to the next bookmark in the file. |  |
| `editor::GoToNextChange` | editor: go to next change | — | Goes to the next change in the file. | `cmd-shift-alt-backspace` (Editor && mode == full) |
| `editor::GoToNextDocumentHighlight` | editor: go to next document highlight | — | Goes to the next document highlight. |  |
| `editor::GoToNextReference` | editor: go to next reference | — | Goes to the next reference to the symbol under the cursor. |  |
| `editor::GoToNextSymbol` | editor: go to next symbol | — | Goes to the next symbol. |  |
| `editor::GoToParentModule` | editor: go to parent module | — | Goes to the parent module of the current file. |  |
| `editor::GoToPreviousBookmark` | editor: go to previous bookmark | — | Goes to the previous bookmark in the file. |  |
| `editor::GoToPreviousChange` | editor: go to previous change | — | Goes to the previous change in the file. | `cmd-shift-backspace` (Editor && mode == full) |
| `editor::GoToPreviousDiagnostic` | editor: go to previous diagnostic | `{ severity?: GoToDiagnosticSeverityFilter }` | Expands the diagnostic under the cursor, if any, in case diagnostics are not yet active. Otherwise, goes to the previous diagnostic in the file. | `shift-f8` {…} (Editor); `g [` (VimControl && !menu) [vim]; `[ d` (vim_mode == normal) [vim]; `d` (vim_operator == helix_previous) [vim] |
| `editor::GoToPreviousDocumentHighlight` | editor: go to previous document highlight | — | Goes to the previous document highlight. |  |
| `editor::GoToPreviousHunk` | editor: go to previous hunk | — | Goes to the previous diff hunk. | `cmd-shift-f8` (!AcpThread > Editor && mode == full); `[ c` (vim_mode == normal) [vim]; `g` (vim_operator == helix_previous) [vim] |
| `editor::GoToPreviousReference` | editor: go to previous reference | — | Goes to the previous reference to the symbol under the cursor. |  |
| `editor::GoToPreviousSymbol` | editor: go to previous symbol | — | Goes to the previous symbol. |  |
| `editor::GoToTypeDefinition` | editor: go to type definition | `{ open_results_in?: OpenResultsIn \| null }` | Goes to the type definition of the symbol at cursor. Argument: `open_results_in`: Where to show the definitions. Falls back to the `lsp_results_location` setting when omitted. A single result is always opened directly. | `cmd-f12` (Editor); `g y` (VimControl && !menu) [vim] |
| `editor::GoToTypeDefinitionSplit` | editor: go to type definition split | — | Goes to type definition in a split pane. | `alt-cmd-f12` (Editor); `ctrl-w shift-d` (VimControl && !menu) [vim]; `ctrl-w g shift-d` (VimControl && !menu) [vim] |
| `editor::HalfPageDown` | editor: half page down | — | Scrolls down by half a page. |  |
| `editor::HalfPageUp` | editor: half page up | — | Scrolls up by half a page. |  |
| `editor::HandleInput` | editor: handle input | `string` | Handles text input in the editor. |  |
| `editor::Hover` | editor: hover | — | Shows hover information for the symbol at cursor. | `cmd-k cmd-i` (Editor); `shift-k` (VimControl && !menu) [vim]; `g h` (VimControl && !menu) [vim]; `space k` ((vim_mode == helix_normal \|\| vim_mode == helix_select) && !menu) [vim] |
| `editor::Indent` | editor: indent | — | Increases indentation of selected lines. | `cmd-]` (Editor) |
| `editor::InsertSnippet` | editor: insert snippet | `{ language?: string \| null, name?: string \| null, snippet?: string \| null }` | Inserts a snippet at the cursor. Argument: `language`: Language name if using a named snippet, or `None` for a global snippet This is typically lowercase and matches the filename containing the snippet, without the `.json` extension. `name`: Name if using a named snippet `snippet`: Snippet body, if not using a named snippet |  |
| `editor::InsertUuidV4` | editor: insert uuid v4 | — | Inserts a UUID v4 at cursor position. |  |
| `editor::InsertUuidV7` | editor: insert uuid v7 | — | Inserts a UUID v7 at cursor position. |  |
| `editor::JoinLines` | editor: join lines | — | Joins the current line with the next line. | `ctrl-j` (Editor) |
| `editor::KillRingCut` | editor: kill ring cut | — | Cuts to kill ring (Emacs-style). | `ctrl-k` (Editor) |
| `editor::KillRingYank` | editor: kill ring yank | — | Yanks from kill ring (Emacs-style). | `ctrl-y` (Editor) |
| `editor::LineDown` | editor: line down | — | Moves cursor down one line. | `ctrl-pagedown` (Editor) |
| `editor::LineUp` | editor: line up | — | Moves cursor up one line. | `ctrl-pageup` (Editor) |
| `editor::MoveDown` | editor: move down | — | Moves cursor down. | `down` (Editor); `ctrl-n` (Editor); `j` (ThreadsSidebar > Editor && VimControl && vim_mode == normal) [vim] |
| `editor::MoveDownByLines` | editor: move down by lines | `{ lines?: integer ≥ 0 = 0 }` | Moves the cursor down by a specified number of lines. |  |
| `editor::MoveLeft` | editor: move left | — | Moves cursor left. | `ctrl-b` (Editor); `left` (Editor) |
| `editor::MoveLineDown` | editor: move line down | — | Moves the current line down. | `alt-down` (Editor); `] e` (vim_mode == normal \|\| vim_mode == visual \|\| vim_mode == operator) [vim] |
| `editor::MoveLineUp` | editor: move line up | — | Moves the current line up. | `alt-up` (Editor); `[ e` (vim_mode == normal \|\| vim_mode == visual \|\| vim_mode == operator) [vim] |
| `editor::MovePageDown` | editor: move page down | `{ center_cursor?: boolean = false }` | Moves the cursor down by one page. | `pagedown` (Editor); `ctrl-v` {"center_cursor": true} (Editor) |
| `editor::MovePageUp` | editor: move page up | `{ center_cursor?: boolean = false }` | Moves the cursor up by one page. | `pageup` (Editor); `ctrl-shift-v` {"center_cursor": true} (Editor) |
| `editor::MoveRight` | editor: move right | — | Moves cursor right. | `ctrl-f` (Editor); `right` (Editor) |
| `editor::MoveToBeginning` | editor: move to beginning | — | Moves cursor to the beginning of the document. | `cmd-up` (Editor); `cmd-home` (Editor) |
| `editor::MoveToBeginningOfLine` | editor: move to beginning of line | `{ stop_at_soft_wraps?: boolean = true, stop_at_indent?: boolean = false }` | Moves the cursor to the beginning of the current line. | `cmd-left` {…} (Editor); `ctrl-a` {…} (Editor); `home` {…} (Editor) |
| `editor::MoveToEnclosingBracket` | editor: move to enclosing bracket | — | Moves cursor to the enclosing bracket. | `cmd-\|` (Editor); `ctrl-m` (Editor) |
| `editor::MoveToEnd` | editor: move to end | — | Moves cursor to the end of the document. | `cmd-down` (Editor); `cmd-end` (Editor) |
| `editor::MoveToEndOfExcerpt` | editor: move to end of excerpt | — | Moves cursor to the end of the current excerpt. |  |
| `editor::MoveToEndOfLargerSyntaxNode` | editor: move to end of larger syntax node | — | Moves cursor to the end of the next larger syntax node. | `alt-e` ((vim_mode == helix_normal \|\| vim_mode == helix_select) && !menu) [vim] |
| `editor::MoveToEndOfLine` | editor: move to end of line | `{ stop_at_soft_wraps?: boolean = true }` | Moves the cursor to the end of the current line. | `cmd-right` {"stop_at_soft_wraps": true} (Editor); `ctrl-e` {"stop_at_soft_wraps": false} (Editor); `end` {"stop_at_soft_wraps": true} (Editor) |
| `editor::MoveToEndOfParagraph` | editor: move to end of paragraph | — | Moves cursor to the end of the paragraph. | `ctrl-down` (Editor) |
| `editor::MoveToEndOfPreviousExcerpt` | editor: move to end of previous excerpt | — | Moves cursor to the end of the previous excerpt. |  |
| `editor::MoveToNextCommentParagraph` | editor: move to next comment paragraph | — | Moves cursor to the start of the next comment paragraph. |  |
| `editor::MoveToNextSubwordEnd` | editor: move to next subword end | — | Moves cursor to the end of the next subword. | `ctrl-alt-right` (Editor); `ctrl-alt-f` (Editor) |
| `editor::MoveToNextWordEnd` | editor: move to next word end | — | Moves cursor to the end of the next word. | `alt-right` (Editor) |
| `editor::MoveToPreviousCommentParagraph` | editor: move to previous comment paragraph | — | Moves cursor to the start of the previous comment paragraph. |  |
| `editor::MoveToPreviousSubwordStart` | editor: move to previous subword start | — | Moves cursor to the start of the previous subword. | `ctrl-alt-left` (Editor); `ctrl-alt-b` (Editor) |
| `editor::MoveToPreviousWordStart` | editor: move to previous word start | — | Moves cursor to the start of the previous word. | `alt-left` (Editor) |
| `editor::MoveToStartOfExcerpt` | editor: move to start of excerpt | — | Moves cursor to the start of the current excerpt. | `cmd-up` (Editor && multibuffer) |
| `editor::MoveToStartOfLargerSyntaxNode` | editor: move to start of larger syntax node | — | Moves cursor to the start of the next larger syntax node. | `alt-b` ((vim_mode == helix_normal \|\| vim_mode == helix_select) && !menu) [vim] |
| `editor::MoveToStartOfNextExcerpt` | editor: move to start of next excerpt | — | Moves cursor to the start of the next excerpt. | `cmd-down` (Editor && multibuffer) |
| `editor::MoveToStartOfParagraph` | editor: move to start of paragraph | — | Moves cursor to the start of the paragraph. | `ctrl-up` (Editor) |
| `editor::MoveUp` | editor: move up | — | Moves cursor up. | `up` (Editor); `ctrl-p` (Editor); `k` (ThreadsSidebar > Editor && VimControl && vim_mode == normal) [vim] |
| `editor::MoveUpByLines` | editor: move up by lines | `{ lines?: integer ≥ 0 = 0 }` | Moves the cursor up by a specified number of lines. |  |
| `editor::Newline` | editor: newline | — | Inserts a new line and moves cursor to it. | `shift-enter` (Editor && mode == full); `enter` (Editor && mode == full); `ctrl-enter` (Editor && mode == auto_height); `shift-enter` (Editor && mode == auto_height); `alt-enter` (AgentFeedbackMessageEditor > Editor); +9 more |
| `editor::NewlineAbove` | editor: newline above | — | Inserts a new line above the current line. | `cmd-shift-enter` (Editor && mode == full) |
| `editor::NewlineBelow` | editor: newline below | — | Inserts a new line below the current line. | `cmd-enter` (Editor && mode == full); `ctrl-shift-enter` (Editor && mode == auto_height) |
| `editor::NextEditPrediction` | editor: next edit prediction | — | Navigates to the next edit prediction. | `alt-tab` (Editor && mode == full && edit_prediction) |
| `editor::NextScreen` | editor: next screen | — | Scrolls to the next screen. |  |
| `editor::NextSnippetTabstop` | editor: next snippet tabstop | — | Goes to the next snippet tabstop if one exists. | `tab` (Editor && in_snippet && has_next_tabstop && !showing_completions) |
| `editor::OpenContextMenu` | editor: open context menu | — | Opens the context menu at cursor position. |  |
| `editor::OpenDocs` | editor: open docs | — | Opens documentation for the symbol at cursor. |  |
| `editor::OpenExcerpts` | editor: open excerpts | — | Opens excerpts from the current file. | `alt-enter` (AcpThread > Editor && mode == full); `alt-enter` (!AcpThread > Editor && mode == full); `alt-enter` (OutlinePanel && not_editing); `g space` (VimControl && !menu) [vim] |
| `editor::OpenExcerptsSplit` | editor: open excerpts split | — | Opens excerpts in a split pane. | `cmd-alt-enter` (!AcpThread > Editor && mode == full); `cmd-alt-enter` (OutlinePanel && not_editing); `ctrl-w space` (VimControl && !menu) [vim]; `ctrl-w g space` (VimControl && !menu) [vim] |
| `editor::OpenGitBlameCommit` | editor: open git blame commit | — | Opens the git commit for the blame at cursor. |  |
| `editor::OpenPermalinkToLine` | editor: open permalink to line | — | Opens a permalink to the current line or selection. |  |
| `editor::OpenProposedChangesEditor` | editor: open proposed changes editor | — | Opens the proposed changes editor. |  |
| `editor::OpenSelectedFilename` | editor: open selected filename | — | Opens the file whose name is selected in the editor. Aliases: `editor::OpenFile` | `g f` (VimControl && !menu) [vim] |
| `editor::OpenSelectionsInMultibuffer` | editor: open selections in multibuffer | — | Opens all selections in a multibuffer. | `alt-enter` (Editor && mode == full) |
| `editor::OpenUrl` | editor: open url | — | Opens the URL at cursor position. | `g x` (VimControl && !menu) [vim] |
| `editor::OrganizeImports` | editor: organize imports | — | Organizes import statements. | `alt-shift-o` (Editor) |
| `editor::Outdent` | editor: outdent | — | Decreases indentation of selected lines. | `cmd-[` (Editor) |
| `editor::PageDown` | editor: page down | — | Scrolls down by one page. | `cmd-pagedown` (Editor) |
| `editor::PageUp` | editor: page up | — | Scrolls up by one page. | `cmd-pageup` (Editor) |
| `editor::Paste` | editor: paste | — | Pastes from clipboard. | `cmd-v` (Editor); `ctrl-shift-v` (vim_mode == insert) [vim]; `shift-r` ((vim_mode == helix_normal \|\| vim_mode == helix_select) && !menu) [vim]; `space p` ((vim_mode == helix_normal \|\| vim_mode == helix_select) && !menu) [vim]; `ctrl-shift-v` (vim_mode == replace) [vim] |
| `editor::PreviousEditPrediction` | editor: previous edit prediction | — | Navigates to the previous edit prediction. | `alt-shift-tab` (Editor && mode == full && edit_prediction) |
| `editor::PreviousSnippetTabstop` | editor: previous snippet tabstop | — | Goes to the previous snippet tabstop if one exists. | `shift-tab` (Editor && in_snippet && has_previous_tabstop && !showing_completions) |
| `editor::Redo` | editor: redo | — | Redoes the last undone edit. | `cmd-shift-z` (Editor); `shift-u` ((vim_mode == helix_normal \|\| vim_mode == helix_select) && !menu) [vim] |
| `editor::RedoSelection` | editor: redo selection | — | Redoes the last selection change. | `cmd-shift-u` (Editor) |
| `editor::ReloadFile` | editor: reload file | — | Reloads the file from disk. |  |
| `editor::Rename` | editor: rename | — | Renames the symbol at cursor. | `f2` (Editor); `g r n` (VimControl && !menu) [vim]; `space r` ((vim_mode == helix_normal \|\| vim_mode == helix_select) && !menu) [vim]; `d` (vim_operator == c) [vim] |
| `editor::RestartLanguageServer` | editor: restart language server | — | Restarts the language server for the current file. |  |
| `editor::RevealInFileManager` | editor: reveal in file manager | — | Reveals the current file in the system file manager. | `cmd-k r` (Editor); `cmd-k r` (ImageViewer) |
| `editor::ReverseLines` | editor: reverse lines | — | Reverses the order of selected lines. |  |
| `editor::Rewrap` | editor: rewrap | — | Rewraps text to fit within the preferred line length. | `cmd-k cmd-q` (Editor); `cmd-k q` (Editor) |
| `editor::RotateSelectionsBackward` | editor: rotate selections backward | — | Rotates selections or lines backward. |  |
| `editor::RotateSelectionsForward` | editor: rotate selections forward | — | Rotates selections or lines forward. |  |
| `editor::RunFlycheck` | editor: run flycheck | — | Runs flycheck diagnostics. |  |
| `editor::SaveLocation` | editor: save location | — | Saves the current location to navigation history. | `ctrl-s` ((vim_mode == helix_normal \|\| vim_mode == helix_select) && !menu) [vim] |
| `editor::ScrollCursorBottom` | editor: scroll cursor bottom | — | Scrolls the cursor to the bottom of the viewport. | `z b` (VimControl && !menu) [vim] |
| `editor::ScrollCursorCenter` | editor: scroll cursor center | — | Scrolls the cursor to the center of the viewport. | `ctrl-l` (Editor); `z z` (VimControl && !menu) [vim]; `z c` ((vim_mode == helix_normal \|\| vim_mode == helix_select) && !menu) [vim] |
| `editor::ScrollCursorCenterTopBottom` | editor: scroll cursor center top bottom | — | Cycles cursor position between center, top, and bottom. |  |
| `editor::ScrollCursorTop` | editor: scroll cursor top | — | Scrolls the cursor to the top of the viewport. | `z t` (VimControl && !menu) [vim] |
| `editor::SelectAll` | editor: select all | — | Selects all text in the editor. | `cmd-a` (Editor); `cmd-a` (Terminal); `%` ((vim_mode == helix_normal \|\| vim_mode == helix_select) && !menu) [vim] |
| `editor::SelectAllMatches` | editor: select all matches | — | Selects all matches of the current selection. | `cmd-shift-l` (Editor); `cmd-f2` (Editor); `g a` (VimControl && !menu) [vim]; `space h` ((vim_mode == helix_normal \|\| vim_mode == helix_select) && !menu) [vim] |
| `editor::SelectAroundDelimiters` | editor: select around delimiters | — | Selects the nearest enclosing delimiters (brackets, braces, parentheses, or quotes) together with the content between them. Repeating the action expands the selection to the next enclosing pair. |  |
| `editor::SelectDown` | editor: select down | — | Extends selection down. | `shift-down` (Editor); `ctrl-shift-n` (Editor) |
| `editor::SelectDownByLines` | editor: select down by lines | `{ lines?: integer ≥ 0 = 0 }` | Extends selection down by a specified number of lines. |  |
| `editor::SelectEnclosingSymbol` | editor: select enclosing symbol | — | Selects the enclosing symbol. | `cmd-alt-e` (Editor && mode == full) |
| `editor::SelectInsideDelimiters` | editor: select inside delimiters | — | Selects the content within the nearest enclosing delimiters (brackets, braces, parentheses, or quotes), excluding the delimiters. Repeating the action expands the selection to the next enclosing pair. |  |
| `editor::SelectLargerSyntaxNode` | editor: select larger syntax node | — | Selects the next larger syntax node. | `cmd-ctrl-right` (Editor); `ctrl-shift-right` (Editor); `alt-o` ((vim_mode == helix_normal \|\| vim_mode == helix_select) && !menu) [vim]; `x` (vim_operator == helix_previous) [vim] |
| `editor::SelectLeft` | editor: select left | — | Extends selection left. | `shift-left` (Editor); `ctrl-shift-b` (Editor) |
| `editor::SelectLine` | editor: select line | — | Selects the current line. | `cmd-l` (Editor); `shift-x` ((vim_mode == helix_normal \|\| vim_mode == helix_select) && !menu) [vim] |
| `editor::SelectNext` | editor: select next | `{ replace_newest?: boolean = false }` | Selects the next occurrence of the current selection. | `cmd-d` {"replace_newest": false} (Editor); `cmd-k cmd-d` {"replace_newest": true} (Editor); `g >` {"replace_newest": true} (VimControl && !menu) [vim] |
| `editor::SelectNextSyntaxNode` | editor: select next syntax node | — | Selects the next syntax node sibling. | `cmd-ctrl-down` (Editor); `alt-n` ((vim_mode == helix_normal \|\| vim_mode == helix_select) && !menu) [vim] |
| `editor::SelectPageDown` | editor: select page down | — | Extends selection down by one page. | `shift-pagedown` (Editor) |
| `editor::SelectPageUp` | editor: select page up | — | Extends selection up by one page. | `shift-pageup` (Editor) |
| `editor::SelectPrevious` | editor: select previous | `{ replace_newest?: boolean = false }` | Selects the previous occurrence of the current selection. | `ctrl-cmd-d` {"replace_newest": false} (Editor); `cmd-k ctrl-cmd-d` {"replace_newest": true} (Editor); `g <` {"replace_newest": true} (VimControl && !menu) [vim] |
| `editor::SelectPreviousSyntaxNode` | editor: select previous syntax node | — | Selects the previous syntax node sibling. | `cmd-ctrl-up` (Editor); `alt-p` ((vim_mode == helix_normal \|\| vim_mode == helix_select) && !menu) [vim] |
| `editor::SelectRight` | editor: select right | — | Extends selection right. | `shift-right` (Editor); `ctrl-shift-f` (Editor) |
| `editor::SelectSmallerSyntaxNode` | editor: select smaller syntax node | — | Selects the next smaller syntax node. | `cmd-ctrl-left` (Editor); `ctrl-shift-left` (Editor); `alt-i` ((vim_mode == helix_normal \|\| vim_mode == helix_select) && !menu) [vim]; `x` (vim_operator == helix_next) [vim] |
| `editor::SelectToBeginning` | editor: select to beginning | — | Selects to the beginning of the document. | `cmd-shift-up` (Editor) |
| `editor::SelectToBeginningOfLine` | editor: select to beginning of line | `{ stop_at_soft_wraps?: boolean = false, stop_at_indent?: boolean = false }` | Selects from the cursor to the beginning of the current line. | `cmd-shift-left` {…} (Editor); `shift-home` {…} (Editor); `ctrl-shift-a` {…} (Editor) |
| `editor::SelectToEnd` | editor: select to end | — | Selects to the end of the document. | `cmd-shift-down` (Editor) |
| `editor::SelectToEndOfExcerpt` | editor: select to end of excerpt | — | Selects to the end of the current excerpt. |  |
| `editor::SelectToEndOfLargerSyntaxNode` | editor: select to end of larger syntax node | — | Selects to the end of the next larger syntax node. |  |
| `editor::SelectToEndOfLine` | editor: select to end of line | `{ stop_at_soft_wraps?: boolean = false }` | Selects from the cursor to the end of the current line. | `cmd-shift-right` {"stop_at_soft_wraps": true} (Editor); `shift-end` {"stop_at_soft_wraps": true} (Editor); `ctrl-shift-e` {"stop_at_soft_wraps": true} (Editor) |
| `editor::SelectToEndOfParagraph` | editor: select to end of paragraph | — | Selects to the end of the paragraph. | `ctrl-shift-down` (Editor) |
| `editor::SelectToEndOfPreviousExcerpt` | editor: select to end of previous excerpt | — | Selects to the end of the previous excerpt. |  |
| `editor::SelectToNextSubwordEnd` | editor: select to next subword end | — | Selects to the end of the next subword. | `ctrl-alt-shift-right` (Editor); `ctrl-alt-shift-f` (Editor) |
| `editor::SelectToNextWordEnd` | editor: select to next word end | — | Selects to the end of the next word. | `alt-shift-right` (Editor) |
| `editor::SelectToPreviousSubwordStart` | editor: select to previous subword start | — | Selects to the start of the previous subword. | `ctrl-alt-shift-left` (Editor); `ctrl-alt-shift-b` (Editor) |
| `editor::SelectToPreviousWordStart` | editor: select to previous word start | — | Selects to the start of the previous word. | `alt-shift-left` (Editor) |
| `editor::SelectToStartOfExcerpt` | editor: select to start of excerpt | — | Selects to the start of the current excerpt. | `cmd-shift-up` (Editor && multibuffer) |
| `editor::SelectToStartOfLargerSyntaxNode` | editor: select to start of larger syntax node | — | Selects to the start of the next larger syntax node. |  |
| `editor::SelectToStartOfNextExcerpt` | editor: select to start of next excerpt | — | Selects to the start of the next excerpt. | `cmd-shift-down` (Editor && multibuffer) |
| `editor::SelectToStartOfParagraph` | editor: select to start of paragraph | — | Selects to the start of the paragraph. | `ctrl-shift-up` (Editor) |
| `editor::SelectUp` | editor: select up | — | Extends selection up. | `shift-up` (Editor); `ctrl-shift-p` (Editor) |
| `editor::SelectUpByLines` | editor: select up by lines | `{ lines?: integer ≥ 0 = 0 }` | Extends selection up by a specified number of lines. |  |
| `editor::SendReviewToAgent` | editor: send review to agent | — | Sends all stored review comments to the Agent panel. |  |
| `editor::SetMark` | editor: set mark | — | Sets a mark at the current position. |  |
| `editor::ShowCharacterPalette` | editor: show character palette | — | Shows the system character palette. | `ctrl-cmd-space` (Editor) |
| `editor::ShowCompletions` | editor: show completions | — | Shows code completion suggestions at the cursor position. | `ctrl-space` (Editor); `ctrl-x ctrl-o` (vim_mode == insert) [vim] |
| `editor::ShowEditPrediction` | editor: show edit prediction | — | Shows edit prediction at cursor. | `alt-tab` (Editor && !edit_prediction); `ctrl-x ctrl-c` (vim_mode == insert) [vim] |
| `editor::ShowSignatureHelp` | editor: show signature help | — | Shows signature help for the current function. | `cmd-i` (Editor); `ctrl-s` (vim_mode == insert) [vim] |
| `editor::ShowWordCompletions` | editor: show word completions | — | Shows word completions. | `ctrl-shift-space` (Editor); `ctrl-p` (vim_mode == insert && !(showing_code_actions \|\| showing_completions)) [vim]; `ctrl-n` (vim_mode == insert && !(showing_code_actions \|\| showing_completions)) [vim] |
| `editor::ShuffleLines` | editor: shuffle lines | — | Randomly shuffles selected lines. |  |
| `editor::SignatureHelpNext` | editor: signature help next | — | Navigates to the next signature in the signature help popup. | `down` (Editor && showing_signature_help && !showing_completions); `ctrl-n` ((vim_mode == insert \|\| vim_mode == normal) && showing_signature_help && !showing_completions) [vim] |
| `editor::SignatureHelpPrevious` | editor: signature help previous | — | Navigates to the previous signature in the signature help popup. | `up` (Editor && showing_signature_help && !showing_completions); `ctrl-p` ((vim_mode == insert \|\| vim_mode == normal) && showing_signature_help && !showing_completions) [vim] |
| `editor::SortLinesByLength` | editor: sort lines by length | — | Sorts selected lines by length. |  |
| `editor::SortLinesCaseInsensitive` | editor: sort lines case insensitive | — | Sorts selected lines case-insensitively. |  |
| `editor::SortLinesCaseSensitive` | editor: sort lines case sensitive | — | Sorts selected lines case-sensitively. |  |
| `editor::SpawnNearestTask` | editor: spawn nearest task | `{ reveal?: RevealStrategy = "always" }` | Spawns the nearest available task from the current cursor position. |  |
| `editor::SplitSelectionIntoLines` | editor: split selection into lines | `{ keep_selections?: boolean = false }` | Splits selection into individual lines. Argument: `keep_selections`: Keep the text selected after splitting instead of collapsing to cursors. | `alt-s` {"keep_selections": true} ((vim_mode == helix_normal \|\| vim_mode == helix_select) && !menu) [vim] |
| `editor::StopLanguageServer` | editor: stop language server | — | Stops the language server for the current file. |  |
| `editor::SubmitDiffReviewComment` | editor: submit diff review comment | — | Stores the diff review comment locally (for later batch submission). |  |
| `editor::SwapSelectionEnds` | editor: swap selection ends | — | Swaps the start and end of the current selection. |  |
| `editor::SwitchSourceHeader` | editor: switch source header | — | Switches between source and header files. |  |
| `editor::Tab` | editor: tab | — | Inserts a tab character or indents. | `tab` (Editor) |
| `editor::ToggleAllDiffHunks` | editor: toggle all diff hunks | — | Toggles all diff hunks in the editor. Collapses all hunks if any are currently expanded, otherwise expands all hunks. |  |
| `editor::ToggleAutoSignatureHelp` | editor: toggle auto signature help | — | Toggles automatic signature help. |  |
| `editor::ToggleBlockComments` | editor: toggle block comments | — | Toggles block comment markers for the selected text. | `cmd-k cmd-/` (Editor); `shift-alt-a` (Editor) |
| `editor::ToggleBookmark` | editor: toggle bookmark | — | Toggles a bookmark at the current line. |  |
| `editor::ToggleBookmarkWithLabel` | editor: toggle bookmark with label | — | Toggles a bookmark at the current line, prompting for a label when adding one. |  |
| `editor::ToggleBreadcrumb` | editor: toggle breadcrumb | — | Toggles breadcrumbs display. |  |
| `editor::ToggleBreakpoint` | editor: toggle breakpoint | — | Toggles a breakpoint at the current line. | `f9` (Editor); `space shift-g b` ((vim_mode == helix_normal \|\| vim_mode == helix_select) && !menu) [vim] |
| `editor::ToggleCase` | editor: toggle case | — | Toggles the case of selected text. |  |
| `editor::ToggleCodeActions` | editor: toggle code actions | `object` | Toggles the display of available code actions at the cursor position. | `cmd-.` (Editor); `g r a` (VimControl && !menu) [vim]; `g .` (VimControl && !menu) [vim]; `ctrl-x ctrl-l` (vim_mode == insert) [vim]; `space a` ((vim_mode == helix_normal \|\| vim_mode == helix_select) && !menu) [vim] |
| `editor::ToggleCodeLens` | editor: toggle code lens | — | Toggles code lens display. |  |
| `editor::ToggleComments` | editor: toggle comments | `{ advance_downwards?: boolean = false, ignore_indent?: boolean = false, comment_empty_lines?: boolean = true }` | Toggles comment markers for the selected lines. Argument: `comment_empty_lines`: Whether to add comment markers to blank lines inside a multi-line selection. A line of only whitespace counts as blank. Defaults to true. | `cmd-/` {…} (Editor); `ctrl-c` {…} ((vim_mode == helix_normal \|\| vim_mode == helix_select) && !menu) [vim]; `space c` {…} ((vim_mode == helix_normal \|\| vim_mode == helix_select) && !menu) [vim] |
| `editor::ToggleDiagnostics` | editor: toggle diagnostics | — | Toggles the diagnostics panel. |  |
| `editor::ToggleEditPrediction` | editor: toggle edit prediction | — | Toggles edit prediction feature. | `ctrl-cmd-e` (Editor) |
| `editor::ToggleFocus` | editor: toggle focus | — | Toggles focus back to the last active buffer. | `enter` (OutlinePanel && not_editing) [vim] |
| `editor::ToggleFold` | editor: toggle fold | — | Toggles folding at the current position. | `cmd-k cmd-l` (Editor); `z a` (VimControl && !menu) [vim]; `z a` (ThreadsSidebar && !Editor) [vim] |
| `editor::ToggleFoldAll` | editor: toggle fold all | — | Toggles all folds in a buffer or all excerpts in multibuffer. | `cmd-shift-enter` (BufferSearchBar) |
| `editor::ToggleFoldRecursive` | editor: toggle fold recursive | — | Toggles recursive folding at the current position. | `z shift-a` (VimControl && !menu) [vim] |
| `editor::ToggleGitBlameInline` | editor: toggle git blame inline | — | Toggles inline git blame display. |  |
| `editor::ToggleIndentGuides` | editor: toggle indent guides | — | Toggles indent guides display. |  |
| `editor::ToggleInlayHints` | editor: toggle inlay hints | — | Toggles inlay hints display. | `ctrl-:` (!AcpThread > Editor && mode == full) |
| `editor::ToggleInlineDiagnostics` | editor: toggle inline diagnostics | — | Toggles inline diagnostics display. |  |
| `editor::ToggleInlineValues` | editor: toggle inline values | — | Toggles inline values display. |  |
| `editor::ToggleLineNumbers` | editor: toggle line numbers | — | Toggles line numbers display. | `cmd-;` (Editor) |
| `editor::ToggleMinimap` | editor: toggle minimap | — | Toggles the minimap display. |  |
| `editor::ToggleRelativeLineNumbers` | editor: toggle relative line numbers | — | Toggles relative line numbers display. |  |
| `editor::ToggleReviewCommentsExpanded` | editor: toggle review comments expanded | — | Toggles the expanded state of the comments section in the overlay. |  |
| `editor::ToggleSelectedDiffHunks` | editor: toggle selected diff hunks | — | Toggles diff display for selected hunks. Aliases: `editor::ToggleHunkDiff` | `cmd-'` (Editor); `g o` ((vim_mode == helix_normal \|\| vim_mode == helix_select) && !menu) [vim]; `o` (vim_operator == d) [vim] |
| `editor::ToggleSelectionMenu` | editor: toggle selection menu | — | Toggles the selection menu. |  |
| `editor::ToggleSemanticHighlights` | editor: toggle semantic highlights | — | Toggles semantic highlights display. |  |
| `editor::ToggleSoftWrap` | editor: toggle soft wrap | — | Toggles soft wrap mode. | `cmd-k z` (Editor && mode == full) |
| `editor::ToggleSplitDiff` | editor: toggle split diff | — |  |  |
| `editor::ToggleTabBar` | editor: toggle tab bar | — | Toggles the tab bar display. |  |
| `editor::Transpose` | editor: transpose | — | Transposes characters around cursor. | `ctrl-t` (Editor) |
| `editor::Undo` | editor: undo | — | Undoes the last edit. | `cmd-z` (Editor) |
| `editor::UndoSelection` | editor: undo selection | — | Undoes the last selection change. | `cmd-u` (Editor) |
| `editor::UnfoldAll` | editor: unfold all | — | Unfolds all folded regions. | `cmd-k cmd-j` (Editor); `z shift-r` (VimControl && !menu) [vim]; `z shift-r` (ThreadsSidebar && !Editor) [vim] |
| `editor::UnfoldLines` | editor: unfold lines | — | Unfolds lines at cursor. | `alt-cmd-]` (Editor); `z o` (VimControl && !menu) [vim] |
| `editor::UnfoldRecursive` | editor: unfold recursive | — | Unfolds recursively at cursor. | `cmd-k cmd-]` (Editor); `z shift-o` (VimControl && !menu) [vim] |
| `editor::UniqueLinesCaseInsensitive` | editor: unique lines case insensitive | — | Removes duplicate lines (case-insensitive). |  |
| `editor::UniqueLinesCaseSensitive` | editor: unique lines case sensitive | — | Removes duplicate lines (case-sensitive). |  |
| `editor::UnwrapSyntaxNode` | editor: unwrap syntax node | — | Removes the surrounding syntax node (for example brackets, or closures) from the current selections. |  |
| `editor::ViewBookmarks` | editor: view bookmarks | — | Opens a view of all bookmarks in the project. |  |
| `editor::WrapSelectionsInTag` | editor: wrap selections in tag | — | Wraps selections in tag specified by language. |  |
| `editor::WrapWithAbbreviation` | editor: wrap with abbreviation | — | Wraps selections in an expanded Emmet abbreviation. |  |

## encoding_selector

| Action | Palette name | Argument | Description | Default keys |
|---|---|---|---|---|
| `encoding_selector::Toggle` | encoding selector: toggle | — | Toggles the encoding selector modal. | `cmd-k n` (Workspace) |

## feedback

| Action | Palette name | Argument | Description | Default keys |
|---|---|---|---|---|
| `feedback::EmailZed` | feedback: email zed | — | Opens email client to send feedback to Zed support. |  |
| `feedback::FileBugReport` | feedback: file bug report | — | Opens the bug report form. |  |
| `feedback::RequestFeature` | feedback: request feature | — | Opens the feature request form. |  |

## file_finder

| Action | Palette name | Argument | Description | Default keys |
|---|---|---|---|---|
| `file_finder::OpenWithoutDismiss` | file finder: open without dismiss | — | Opens the selected file in the editor without dismissing the file finder, so additional files can be opened in sequence. |  |
| `file_finder::SelectPrevious` | file finder: select previous | — | Selects the previous item in the file finder. |  |
| `file_finder::Toggle` | file finder: toggle | `{ separate_history?: boolean = false, include_ignored?: boolean \| null }` | Toggles the file finder interface. | `cmd-p` (Workspace); `space f` ((vim_mode == helix_normal \|\| vim_mode == helix_select) && !menu) [vim]; `space f` (!Editor && !Terminal) [vim] |

## git

| Action | Palette name | Argument | Description | Default keys |
|---|---|---|---|---|
| `git::Add` | git: add | — | Adds files to the git staging area. |  |
| `git::AddToGitInfoExclude` | git: add to git info exclude | — | Adds a file to the repository's .git/info/exclude. |  |
| `git::AddToGitignore` | git: add to gitignore | — | Adds a file to .gitignore. |  |
| `git::Amend` | git: amend | — | Amends the last commit with staged changes. | `cmd-shift-enter` (GitDiff > Editor); `cmd-shift-enter` (CommitEditor > Editor); `cmd-shift-enter` (GitPanel); `cmd-shift-enter` (GitCommit > Editor && mode == auto_height) |
| `git::ApplyCurrentStash` | git: apply current stash | — |  | `ctrl-space` (StashDiff > Editor) |
| `git::Blame` | git: blame | — | Shows git blame information for the current file. Aliases: `editor::ToggleGitBlame` | `cmd-alt-g b` (Editor) |
| `git::Branch` | git: branch | — | Opens the git branch selector. Aliases: `branches::OpenRecent` |  |
| `git::Cancel` | git: cancel | — | Cancels the current git operation. | `escape` (GitPanel && CommitEditor) |
| `git::CheckoutBranch` | git: checkout branch | — | Checks out a different git branch. |  |
| `git::Clone` | git: clone | — | Clones a repository. |  |
| `git::Commit` | git: commit | — | Creates a new commit with staged changes. | `cmd-enter` (GitDiff > Editor); `cmd-enter` (CommitEditor > Editor); `cmd-enter` (GitPanel); `cmd-enter` (GitCommit > Editor && mode == auto_height) |
| `git::CompareWithBranch` | git: compare with branch | — | Compare with a specific branch |  |
| `git::CopyBranchName` | git: copy branch name | — | Copies the current branch name to the clipboard. |  |
| `git::CopyFilePermalink` | git: copy file permalink | — | Copies a permalink for the selected file on its Git hosting provider. |  |
| `git::CreatePullRequest` | git: create pull request | — | Creates a pull request for the current branch. |  |
| `git::CreateRemote` | git: create remote | — | Create a git remote. |  |
| `git::CreateWorktree` | git: create worktree | `{ worktree_name?: string \| null, branch_target: NewWorktreeBranchTarget }` | Creates a new git worktree and switches the workspace to it. Dispatched by the unified worktree picker when the user selects a "Create new worktree" entry. Argument: `worktree_name`: When this is None, Zed will randomly generate a worktree name. |  |
| `git::Diff` | git: diff | — | Shows the diff against the configured diff base: working changes relative to HEAD, or all branch changes relative to the default branch, following the `git.diff_base` setting. | `shift-ctrl-d` (AcpThread > Editor); `ctrl-g d` (GitPanel) |
| `git::DiffBranch` | git: diff branch | — | Shows the diff between the working directory and your default branch (typically main or master). Aliases: `git::BranchDiff` |  |
| `git::DiffHead` | git: diff head | — | Shows working changes relative to HEAD. |  |
| `git::DropCurrentStash` | git: drop current stash | — |  | `ctrl-shift-backspace` (StashDiff > Editor) |
| `git::ExpandCommitEditor` | git: expand commit editor | — | Expands the commit message editor. | `shift-escape` (CommitEditor > Editor) |
| `git::Fetch` | git: fetch | — | Fetches changes from the remote repository. | `ctrl-g ctrl-g` (GitPanel) |
| `git::FetchFrom` | git: fetch from | — | Fetches changes from a specific remote. |  |
| `git::FileHistory` | git: file history | — | Shows the git history for the selected file, folder, or project. |  |
| `git::FilterRemotes` | git: filter remotes | — | Filter remotes. |  |
| `git::ForcePush` | git: force push | — | Force pushes commits to the remote repository. | `ctrl-g shift-up` (GitPanel) |
| `git::GenerateCommitMessage` | git: generate commit message | — | Generates a commit message using AI. | `alt-tab` (CommitEditor > Editor); `alt-tab` (GitCommit > Editor && mode == auto_height) |
| `git::Init` | git: init | — | Initializes a new git repository. |  |
| `git::LeaderAndFollower` | git: leader and follower | — |  |  |
| `git::OpenFileAtHead` | git: open file at head | — |  |  |
| `git::OpenFileDiff` | git: open file diff | — | Opens the current file in a solo diff view. |  |
| `git::OpenFilePermalink` | git: open file permalink | — | Opens a permalink for the selected file on its Git hosting provider. |  |
| `git::OpenModifiedFiles` | git: open modified files | — | Opens all modified files in the editor. | `cmd-alt-g m` (Editor) |
| `git::OpenWorktreeInNewWindow` | git: open worktree in new window | `{ path: string }` | Opens an existing worktree in a new window. Dispatched by the worktree picker's "Open in New Window" button. |  |
| `git::PopCurrentStash` | git: pop current stash | — |  | `ctrl-shift-space` (StashDiff > Editor) |
| `git::Pull` | git: pull | — | Pulls changes from the remote repository. | `ctrl-g down` (GitPanel) |
| `git::PullRebase` | git: pull rebase | — | Pulls changes from the remote repository with rebase. | `ctrl-g shift-down` (GitPanel) |
| `git::Push` | git: push | — | Pushes commits to the remote repository. | `ctrl-g up` (GitPanel) |
| `git::PushTo` | git: push to | — | Pushes commits to a specific remote branch. |  |
| `git::RenameBranch` | git: rename branch | `{ branch?: string \| null }` | Renames a git branch. Argument: `branch`: The branch to rename. Default: the current branch. |  |
| `git::Restore` | git: restore | — | Restores the selected hunks to their original state. Aliases: `editor::RevertSelectedHunks` | `cmd-alt-z` (Editor && !agent_diff && !AgentPanel); `g shift-r` ((vim_mode == helix_normal \|\| vim_mode == helix_select) && !menu) [vim]; `p` (vim_operator == d) [vim] |
| `git::RestoreAndNext` | git: restore and next | — | Restores the selected hunks to their original state and moves to the next one. | `cmd-alt-z` (GitDiff > Editor) |
| `git::RestoreFile` | git: restore file | `{ skip_prompt?: boolean = false }` | Restores a file to its last committed state, discarding local changes. Aliases: `editor::RevertFile` | `backspace` {"skip_prompt": false} (GitPanel && ChangesList && !GitBranchSelector && !GitRepositorySelector); `delete` {"skip_prompt": false} (GitPanel && ChangesList && !GitBranchSelector && !GitRepositorySelector); `cmd-backspace` {"skip_prompt": true} (GitPanel && ChangesList && !GitBranchSelector && !GitRepositorySelector); `cmd-delete` {"skip_prompt": true} (GitPanel && ChangesList && !GitBranchSelector && !GitRepositorySelector) |
| `git::RestoreTrackedFiles` | git: restore tracked files | — | Restores all tracked files to their last committed state. | `ctrl-g backspace` (GitPanel) |
| `git::ReviewDiff` | git: review diff | — | Opens a new agent thread with the branch diff for review. | `cmd-alt-g r` (Editor) |
| `git::SelectRepo` | git: select repo | — | Selects a different repository. |  |
| `git::Signoff` | git: signoff | — | Enable the --signoff option. |  |
| `git::SkipHooks` | git: skip hooks | — | Runs the next commit with `git commit --no-verify`. |  |
| `git::StageAll` | git: stage all | — | Stages all changes in the repository. | `cmd-ctrl-y` (GitDiff > Editor); `cmd-ctrl-y` (GitPanel); `shift-x` (GitPanel && ChangesList && !GitBranchSelector && !GitRepositorySelector) [vim] |
| `git::StageAndNext` | git: stage and next | — | Stages the current hunk and moves to the next one. | `cmd-y` (Editor && !agent_diff && !AgentPanel); `g u` ((vim_mode == helix_normal \|\| vim_mode == helix_select) && !menu) [vim]; `u` (vim_operator == d) [vim] |
| `git::StageFile` | git: stage file | — | Stages the current file. | `cmd-y` (GitPanel && ChangesList && !GitBranchSelector && !GitRepositorySelector) |
| `git::StageRange` | git: stage range | — | Stage status entries between an anchor entry and the cursor. | `shift-space` (GitPanel && ChangesList && !GitBranchSelector && !GitRepositorySelector); `g x` (GitPanel && ChangesList && !GitBranchSelector && !GitRepositorySelector) [vim] |
| `git::StageSection` | git: stage section | — | Stages every entry in the section containing the selected entry. |  |
| `git::StashAll` | git: stash all | — | Stashes all changes in the repository, including untracked files. |  |
| `git::StashApply` | git: stash apply | — | Apply the most recent stash. |  |
| `git::StashPop` | git: stash pop | — | Pops the most recent stash. |  |
| `git::StashStaged` | git: stash staged | — | Stashes staged changes in the repository, leaving unstaged changes in place. |  |
| `git::StashTracked` | git: stash tracked | — | Stashes tracked changes in the repository, leaving untracked files in place. |  |
| `git::Switch` | git: switch | — | Switches to a different git branch. |  |
| `git::SwitchWorktree` | git: switch worktree | `{ path: string, display_name: string }` | Switches the workspace to an existing linked worktree. Dispatched by the unified worktree picker when the user selects an existing worktree. |  |
| `git::ToggleDiffBase` | git: toggle diff base | — | Toggles the git diff base between HEAD and the default branch. |  |
| `git::ToggleFillCommitEditor` | git: toggle fill commit editor | — | Toggles whether the commit message editor fills all the available vertical space within the git panel. | `alt-shift-escape` (CommitEditor > Editor) |
| `git::ToggleStaged` | git: toggle staged | — | Toggles the staged state of the hunk or status entry at cursor. | `cmd-alt-y` (Editor && !agent_diff && !AgentPanel); `cmd-alt-y` (GitPanel && ChangesList && !GitBranchSelector && !GitRepositorySelector); `space` (GitPanel && ChangesList && !GitBranchSelector && !GitRepositorySelector); `g shift-o` ((vim_mode == helix_normal \|\| vim_mode == helix_select) && !menu) [vim]; `shift-o` (vim_operator == d) [vim]; +1 more |
| `git::TrashUntrackedFiles` | git: trash untracked files | — | Moves all untracked files to trash. | `ctrl-g shift-backspace` (GitPanel) |
| `git::Uncommit` | git: uncommit | — | Undoes the last commit, keeping changes in the working directory. |  |
| `git::UnstageAll` | git: unstage all | — | Unstages all changes in the repository. | `cmd-ctrl-shift-y` (GitDiff > Editor); `cmd-ctrl-shift-y` (GitPanel); `shift-u` (GitPanel && ChangesList && !GitBranchSelector && !GitRepositorySelector) [vim] |
| `git::UnstageAndNext` | git: unstage and next | — | Unstages the current hunk and moves to the next one. | `cmd-shift-y` (Editor && !agent_diff && !AgentPanel); `g shift-u` ((vim_mode == helix_normal \|\| vim_mode == helix_select) && !menu) [vim]; `shift-u` (vim_operator == d) [vim] |
| `git::UnstageFile` | git: unstage file | — | Unstages the current file. | `cmd-shift-y` (GitPanel && ChangesList && !GitBranchSelector && !GitRepositorySelector) |
| `git::UnstageSection` | git: unstage section | — | Unstages every entry in the section containing the selected entry. |  |
| `git::ViewCommit` | git: view commit | — |  |  |
| `git::ViewFile` | git: view file | — | Opens the selected file in the editor without a diff view. |  |
| `git::ViewStagedChanges` | git: view staged changes | — | Shows staged changes across the project. |  |
| `git::ViewStash` | git: view stash | — | Opens the git stash selector. |  |
| `git::ViewUncommittedChanges` | git: view uncommitted changes | — | Shows uncommitted changes across the project. |  |
| `git::ViewUnstagedChanges` | git: view unstaged changes | — | Shows unstaged changes across the project. |  |
| `git::Worktree` | git: worktree | — | Opens the git worktree selector. | `cmd-ctrl-w` (Workspace) |

## git_graph

| Action | Palette name | Argument | Description | Default keys |
|---|---|---|---|---|
| `git_graph::CopyCommitSha` | git graph: copy commit sha | — | Copies the SHA of the selected commit to the clipboard. |  |
| `git_graph::CopyCommitTag` | git graph: copy commit tag | — | Copies a tag from the selected commit to the clipboard. |  |
| `git_graph::FocusNextTabStop` | git graph: focus next tab stop | — | Focuses the next git graph tab stop. | `tab` (GitGraph); `tab` (GitGraphSearchBar > Editor); `tab` (GitGraph) [vim]; `tab` (GitGraphSearchBar > Editor) [vim] |
| `git_graph::FocusPreviousTabStop` | git graph: focus previous tab stop | — | Focuses the previous git graph tab stop. | `shift-tab` (GitGraph); `shift-tab` (GitGraphSearchBar > Editor); `shift-tab` (GitGraph) [vim]; `shift-tab` (GitGraphSearchBar > Editor) [vim] |
| `git_graph::FocusSearch` | git graph: focus search | — | Focuses the search field. |  |
| `git_graph::Open` | git graph: open | — | Opens the Git Graph Tab. |  |
| `git_graph::OpenAtCommit` | git graph: open at commit | `{ sha: string }` | Opens the Git Graph Tab at a specific commit. |  |
| `git_graph::OpenCommitView` | git graph: open commit view | — | Opens the commit view for the selected commit. |  |
| `git_graph::ScrollDown` | git graph: scroll down | — | Selects a commit half a page below the current selection. | `ctrl-d` (GitGraph && !GitGraphSearchBar) [vim] |
| `git_graph::ScrollUp` | git graph: scroll up | — | Selects a commit half a page above the current selection. | `ctrl-u` (GitGraph && !GitGraphSearchBar) [vim] |
| `git_graph::ToggleChangedFilesView` | git graph: toggle changed files view | — | Toggles the selected commit's changed files between flat and tree views. |  |

## git_onboarding

| Action | Palette name | Argument | Description | Default keys |
|---|---|---|---|---|
| `git_onboarding::OpenGitIntegrationOnboarding` | git onboarding: open git integration onboarding | — | Opens the git integration onboarding modal. |  |

## git_panel

| Action | Palette name | Argument | Description | Default keys |
|---|---|---|---|---|
| `git_panel::ActivateChangesTab` | git panel: activate changes tab | — | Activates the Changes tab. | `cmd-1` (GitPanel) |
| `git_panel::ActivateHistoryTab` | git panel: activate history tab | — | Activates the History tab. | `cmd-2` (GitPanel) |
| `git_panel::Close` | git panel: close | — | Closes the git panel. |  |
| `git_panel::CollapseSelectedEntry` | git panel: collapse selected entry | — | Collapses the selected entry to hide its children. | `left` (GitPanel && ChangesList && !GitBranchSelector && !GitRepositorySelector); `h` (GitPanel && ChangesList && !GitBranchSelector && !GitRepositorySelector) [vim] |
| `git_panel::ExpandSelectedEntry` | git panel: expand selected entry | — | Expands the selected entry to show its children. | `right` (GitPanel && ChangesList && !GitBranchSelector && !GitRepositorySelector); `l` (GitPanel && ChangesList && !GitBranchSelector && !GitRepositorySelector) [vim] |
| `git_panel::FirstEntry` | git panel: first entry | — | Select first git panel menu item, and show it in the diff view | `cmd-up` (GitPanel && ChangesList && !GitBranchSelector && !GitRepositorySelector) |
| `git_panel::FocusChanges` | git panel: focus changes | — | Focuses on the changes list. | `tab` (CommitEditor > Editor); `shift-tab` (CommitEditor > Editor); `alt-up` (CommitEditor > Editor); `tab` (CommitEditor > Editor && VimControl && !menu) [vim]; `shift-tab` (CommitEditor > Editor && VimControl && !menu) [vim] |
| `git_panel::FocusEditor` | git panel: focus editor | — | Focuses on the commit message editor. | `alt-down` (GitPanel && ChangesList && !GitBranchSelector && !GitRepositorySelector); `tab` (GitPanel && ChangesList && !GitBranchSelector && !GitRepositorySelector); `shift-tab` (GitPanel && ChangesList && !GitBranchSelector && !GitRepositorySelector); `i` (GitPanel && ChangesList && !GitBranchSelector && !GitRepositorySelector) [vim] |
| `git_panel::LastEntry` | git panel: last entry | — | Select last git panel menu item, and show it in the diff view | `cmd-down` (GitPanel && ChangesList && !GitBranchSelector && !GitRepositorySelector) |
| `git_panel::NextEntry` | git panel: next entry | — | Select next git panel menu item, and show it in the diff view | `down` (GitPanel && ChangesList && !GitBranchSelector && !GitRepositorySelector) |
| `git_panel::OpenMenu` | git panel: open menu | — | Opens the git panel menu. |  |
| `git_panel::PreviousEntry` | git panel: previous entry | — | Select previous git panel menu item, and show it in the diff view | `up` (GitPanel && ChangesList && !GitBranchSelector && !GitRepositorySelector) |
| `git_panel::SetGroupByNone` | git panel: set group by none | — | Disables grouping entries by status. |  |
| `git_panel::SetGroupByStaging` | git panel: set group by staging | — | Groups entries by staging state. |  |
| `git_panel::SetGroupByStatus` | git panel: set group by status | — | Groups entries by status. |  |
| `git_panel::SetSortByName` | git panel: set sort by name | — | Sorts entries by name. |  |
| `git_panel::SetSortByPath` | git panel: set sort by path | — | Sorts entries by path. |  |
| `git_panel::Toggle` | git panel: toggle | — | Toggles the git panel. |  |
| `git_panel::ToggleFillCoAuthors` | git panel: toggle fill co authors | — | Toggles automatic co-author suggestions. |  |
| `git_panel::ToggleFocus` | git panel: toggle focus | — | Toggles focus on the git panel. | `ctrl-shift-g` (Workspace) |
| `git_panel::ToggleTreeView` | git panel: toggle tree view | — | Toggles showing entries in tree vs flat view. |  |
| `git_panel::ViewStagedChanges` | git panel: view staged changes | — | View staged changes |  |
| `git_panel::ViewUnstagedChanges` | git panel: view unstaged changes | — | View unstaged changes |  |

## git_picker

| Action | Palette name | Argument | Description | Default keys |
|---|---|---|---|---|
| `git_picker::ActivateBranchesTab` | git picker: activate branches tab | — |  | `cmd-1` (GitPicker) |
| `git_picker::ActivateStashTab` | git picker: activate stash tab | — |  | `cmd-2` (GitPicker) |

## go_to_line

| Action | Palette name | Argument | Description | Default keys |
|---|---|---|---|---|
| `go_to_line::Toggle` | go to line: toggle | — | Toggles the go to line dialog. | `ctrl-g` (Editor && mode == full) |

## highlights_tree_view

| Action | Palette name | Argument | Description | Default keys |
|---|---|---|---|---|
| `highlights_tree_view::ToggleSemanticTokens` | highlights tree view: toggle semantic tokens | — | Toggles showing semantic token highlights. |  |
| `highlights_tree_view::ToggleSyntaxTokens` | highlights tree view: toggle syntax tokens | — | Toggles showing syntax token highlights. |  |
| `highlights_tree_view::ToggleTextHighlights` | highlights tree view: toggle text highlights | — | Toggles showing text highlights. |  |

## icon_theme_selector

| Action | Palette name | Argument | Description | Default keys |
|---|---|---|---|---|
| `icon_theme_selector::Toggle` | icon theme selector: toggle | `{ themes_filter?: [string, …] \| null }` | Toggles the icon theme selector interface. Argument: `themes_filter`: A list of icon theme names to filter the theme selector down to. |  |

## image_viewer

| Action | Palette name | Argument | Description | Default keys |
|---|---|---|---|---|
| `image_viewer::FitToView` | image viewer: fit to view | — | Fit the image to view. | `cmd-shift-0` (ImageViewer) |
| `image_viewer::ResetZoom` | image viewer: reset zoom | — | Reset zoom to 100%. | `cmd-0` (ImageViewer) |
| `image_viewer::ZoomIn` | image viewer: zoom in | — | Zoom in the image. | `cmd-=` (ImageViewer); `cmd-+` (ImageViewer) |
| `image_viewer::ZoomOut` | image viewer: zoom out | — | Zoom out the image. | `cmd--` (ImageViewer) |
| `image_viewer::ZoomToActualSize` | image viewer: zoom to actual size | — | Zoom to actual size (100%). | `cmd-1` (ImageViewer) |

## inline_assistant

| Action | Palette name | Argument | Description | Default keys |
|---|---|---|---|---|
| `inline_assistant::ThumbsDownResult` | inline assistant: thumbs down result | — |  | `cmd-shift-backspace` (InlineAssistant > Editor) |
| `inline_assistant::ThumbsUpResult` | inline assistant: thumbs up result | — |  | `cmd-shift-enter` (InlineAssistant > Editor) |

## journal

| Action | Palette name | Argument | Description | Default keys |
|---|---|---|---|---|
| `journal::NewJournalEntry` | journal: new journal entry | — | Creates a new journal entry for today. |  |

## keymap_editor

| Action | Palette name | Argument | Description | Default keys |
|---|---|---|---|---|
| `keymap_editor::CopyAction` | keymap editor: copy action | — | Copies the action name to clipboard. | `cmd-c` (KeymapEditor) |
| `keymap_editor::CopyContext` | keymap editor: copy context | — | Copies the context predicate to clipboard. | `cmd-shift-c` (KeymapEditor) |
| `keymap_editor::CreateBinding` | keymap editor: create binding | — | Creates a new key binding for the selected action. | `alt-enter` (KeymapEditor) |
| `keymap_editor::DeleteBinding` | keymap editor: delete binding | — | Deletes the selected key binding. |  |
| `keymap_editor::EditBinding` | keymap editor: edit binding | — | Edits the selected key binding. | `enter` (KeymapEditor) |
| `keymap_editor::OpenCreateKeybindingModal` | keymap editor: open create keybinding modal | — | Creates a new key binding from scratch, prompting for the action. | `cmd-k` (KeymapEditor) |
| `keymap_editor::ShowMatchingKeybinds` | keymap editor: show matching keybinds | — | Shows matching keystrokes for the currently selected binding | `cmd-t` (KeymapEditor) |
| `keymap_editor::ToggleConflictFilter` | keymap editor: toggle conflict filter | — | Toggles Conflict Filtering | `cmd-alt-c` (KeymapEditor) |
| `keymap_editor::ToggleExactKeystrokeMatching` | keymap editor: toggle exact keystroke matching | — | Toggles exact matching for keystroke search |  |
| `keymap_editor::ToggleKeystrokeSearch` | keymap editor: toggle keystroke search | — | Toggle Keystroke search | `cmd-alt-f` (KeymapEditor); `cmd-alt-f` (KeymapEditor > BufferSearchBar) |
| `keymap_editor::ToggleNoActionBindings` | keymap editor: toggle no action bindings | — | Toggles whether NoAction bindings are shown |  |

## keystroke_input

| Action | Palette name | Argument | Description | Default keys |
|---|---|---|---|---|
| `keystroke_input::ClearKeystrokes` | keystroke input: clear keystrokes | — | Clears the recorded keystrokes | `delete` (KeystrokeInput) |
| `keystroke_input::StartRecording` | keystroke input: start recording | — | Starts recording keystrokes | `enter` (KeystrokeInput) |
| `keystroke_input::StopRecording` | keystroke input: stop recording | — | Stops recording keystrokes | `escape escape escape` (KeystrokeInput) |

## language_selector

| Action | Palette name | Argument | Description | Default keys |
|---|---|---|---|---|
| `language_selector::Toggle` | language selector: toggle | — | Toggles the language selector modal. | `cmd-k m` (Workspace) |

## line_ending_selector

| Action | Palette name | Argument | Description | Default keys |
|---|---|---|---|---|
| `line_ending_selector::Toggle` | line ending selector: toggle | — | Toggles the line ending selector modal. |  |

## lsp_command_selector

| Action | Palette name | Argument | Description | Default keys |
|---|---|---|---|---|
| `lsp_command_selector::Toggle` | lsp command selector: toggle | — |  |  |
| `lsp_command_selector::ToggleArgumentsFocus` | lsp command selector: toggle arguments focus | — |  | `tab` (LspCommandSelector > Picker > Editor) |

## lsp_tool

| Action | Palette name | Argument | Description | Default keys |
|---|---|---|---|---|
| `lsp_tool::ToggleMenu` | lsp tool: toggle menu | — | Toggles the language server tool menu. | `ctrl-cmd-l` |

## markdown

| Action | Palette name | Argument | Description | Default keys |
|---|---|---|---|---|
| `markdown::CloseAndReturnToEditor` | markdown: close and return to editor | — | Closes the markdown preview and returns focus to the source editor. | `cmd-shift-v` (MarkdownPreview) |
| `markdown::Copy` | markdown: copy | — | Copies the selected text to the clipboard. | `cmd-c` (Markdown) |
| `markdown::CopyAsMarkdown` | markdown: copy as markdown | — | Copies the selected text as markdown to the clipboard. |  |
| `markdown::OpenFollowingPreview` | markdown: open following preview | — | Opens a following markdown preview that syncs with the editor. |  |
| `markdown::OpenPreview` | markdown: open preview | — | Opens a markdown preview for the current file. | `cmd-shift-v` (Editor && extension == md) |
| `markdown::OpenPreviewToTheSide` | markdown: open preview to the side | — | Opens a markdown preview in a split pane. | `cmd-k v` (Editor && extension == md) |
| `markdown::ScrollDown` | markdown: scroll down | — | Scrolls down by approximately one visual line. | `down` (MarkdownPreview); `ctrl-e` (MarkdownPreview) [vim] |
| `markdown::ScrollDownByItem` | markdown: scroll down by item | — | Scrolls down by one markdown element in the markdown preview | `alt-down` (MarkdownPreview) |
| `markdown::ScrollPageDown` | markdown: scroll page down | — | Scrolls down by one page in the markdown preview. Aliases: `markdown::MovePageDown` | `pagedown` (MarkdownPreview); `ctrl-d` (MarkdownPreview) [vim] |
| `markdown::ScrollPageUp` | markdown: scroll page up | — | Scrolls up by one page in the markdown preview. Aliases: `markdown::MovePageUp` | `pageup` (MarkdownPreview); `ctrl-u` (MarkdownPreview) [vim] |
| `markdown::ScrollToBottom` | markdown: scroll to bottom | — | Scrolls to the bottom of the markdown preview. | `cmd-down` (MarkdownPreview); `shift-g` (MarkdownPreview) [vim] |
| `markdown::ScrollToTop` | markdown: scroll to top | — | Scrolls to the top of the markdown preview. | `cmd-up` (MarkdownPreview); `g g` (MarkdownPreview) [vim] |
| `markdown::ScrollUp` | markdown: scroll up | — | Scrolls up by approximately one visual line. | `up` (MarkdownPreview); `ctrl-y` (MarkdownPreview) [vim] |
| `markdown::ScrollUpByItem` | markdown: scroll up by item | — | Scrolls up by one markdown element in the markdown preview | `alt-up` (MarkdownPreview) |
| `markdown::ToggleBlockQuote` | markdown: toggle block quote | — | Toggles a block quote (`> `) prefix on the selected lines (or the current line) while in Markdown files. |  |

## menu

| Action | Palette name | Argument | Description | Default keys |
|---|---|---|---|---|
| `menu::Cancel` | menu: cancel | — | Cancels the current menu operation. | `cmd-escape`; `ctrl-escape`; `ctrl-c`; `escape`; `escape` (AgentFeedbackMessageEditor > Editor); +16 more |
| `menu::Confirm` | menu: confirm | — | Confirms the selected menu item. | `enter`; `enter` (AgentFeedbackMessageEditor > Editor); `cmd-enter` (AcpThread > ModeSelector); `enter` (ThreadsSidebar); `space` (ThreadsSidebar && not_searching); +7 more |
| `menu::EndSlot` | menu: end slot | — |  |  |
| `menu::Restart` | menu: restart | — | Restarts the menu from the beginning. | `alt-shift-enter` |
| `menu::SecondaryConfirm` | menu: secondary confirm | — | Performs secondary confirmation action. | `ctrl-enter`; `cmd-enter` |
| `menu::SelectChild` | menu: select child | — | Enters a submenu (navigates to child menu). | `right` (menu); `right` (ThreadsSidebar); `cmd-k right` (CallHierarchyPicker > Picker > Editor); `l` (ThreadsSidebar && !Editor) [vim]; `z o` (ThreadsSidebar && !Editor) [vim] |
| `menu::SelectFirst` | menu: select first | — | Selects the first item in the menu. | `home`; `shift-pageup`; `pageup`; `cmd-up`; `g g` (ProjectPanel && not_editing) [vim]; +6 more |
| `menu::SelectLast` | menu: select last | — | Selects the last item in the menu. | `end`; `shift-pagedown`; `pagedown`; `cmd-down`; `shift-g` (ProjectPanel && not_editing) [vim]; +6 more |
| `menu::SelectNext` | menu: select next | — | Selects the next item in the menu. | `tab`; `ctrl-n`; `down`; `right` (Prompt); `l` (Prompt); +13 more |
| `menu::SelectParent` | menu: select parent | — | Exits a submenu (navigates to parent menu). | `left` (menu); `left` (ThreadsSidebar); `cmd-k left` (CallHierarchyPicker > Picker > Editor); `h` (ThreadsSidebar && !Editor) [vim]; `z c` (ThreadsSidebar && !Editor) [vim] |
| `menu::SelectPrevious` | menu: select previous | — | Selects the previous item in the menu. | `shift-tab`; `ctrl-p`; `up`; `left` (Prompt); `h` (Prompt); +14 more |

## multi_workspace

| Action | Palette name | Argument | Description | Default keys |
|---|---|---|---|---|
| `multi_workspace::CloseWorkspaceSidebar` | multi workspace: close workspace sidebar | — | Closes the workspace sidebar. |  |
| `multi_workspace::FocusWorkspaceSidebar` | multi workspace: focus workspace sidebar | — | Moves focus to or from the workspace sidebar without closing it. | `cmd-alt-;` (Workspace) |
| `multi_workspace::MoveProjectDown` | multi workspace: move project down | — | Moves the active project down in the sidebar. |  |
| `multi_workspace::MoveProjectToNewWindow` | multi workspace: move project to new window | — | Moves the active project to a new window. |  |
| `multi_workspace::MoveProjectUp` | multi workspace: move project up | — | Moves the active project up in the sidebar. |  |
| `multi_workspace::NewThread` | multi workspace: new thread | — | Creates a new thread in the current workspace. |  |
| `multi_workspace::NextProject` | multi workspace: next project | — | Activates the next project in the sidebar. | `] p` (ThreadsSidebar && !Editor) [vim] |
| `multi_workspace::NextThread` | multi workspace: next thread | — | Activates the next thread in sidebar order. |  |
| `multi_workspace::PreviousProject` | multi workspace: previous project | — | Activates the previous project in the sidebar. | `[ p` (ThreadsSidebar && !Editor) [vim] |
| `multi_workspace::PreviousThread` | multi workspace: previous thread | — | Activates the previous thread in sidebar order. |  |
| `multi_workspace::ToggleWorkspaceSidebar` | multi workspace: toggle workspace sidebar | — | Toggles the workspace switcher sidebar. | `cmd-alt-j` (Workspace) |

## new_process_modal

| Action | Palette name | Argument | Description | Default keys |
|---|---|---|---|---|
| `new_process_modal::ActivateAttachTab` | new process modal: activate attach tab | — |  | `cmd-3` (RunModal) |
| `new_process_modal::ActivateDebugTab` | new process modal: activate debug tab | — |  | `cmd-2` (RunModal) |
| `new_process_modal::ActivateLaunchTab` | new process modal: activate launch tab | — |  | `cmd-4` (RunModal) |
| `new_process_modal::ActivateTaskTab` | new process modal: activate task tab | — |  | `cmd-1` (RunModal) |

## notebook

| Action | Palette name | Argument | Description | Default keys |
|---|---|---|---|---|
| `notebook::AddCodeBlock` | notebook: add code block | — | Adds a new code cell. | `cmd-m` (NotebookEditor); `cmd-m` (NotebookEditor > Editor) |
| `notebook::AddMarkdownBlock` | notebook: add markdown block | — | Adds a new markdown cell. | `cmd-shift-m` (NotebookEditor); `cmd-shift-m` (NotebookEditor > Editor) |
| `notebook::ClearOutputs` | notebook: clear outputs | — | Clears all cell outputs. |  |
| `notebook::DeleteCell` | notebook: delete cell | — | Deletes the current cell. | `d d` (NotebookEditor && notebook_mode == command); `backspace` (NotebookEditor && notebook_mode == command) |
| `notebook::EnterCommandMode` | notebook: enter command mode | — | Exits the cell editor and returns to cell command mode. | `escape` (NotebookEditor > Editor); `escape` (NotebookEditor > Editor && VimControl && vim_mode == normal) [vim] |
| `notebook::EnterEditMode` | notebook: enter edit mode | — | Enters the current cell's editor (edit mode). | `enter` (NotebookEditor && notebook_mode == command); `i` (NotebookEditor && notebook_mode == command) [vim]; `a` (NotebookEditor && notebook_mode == command) [vim]; `enter` (NotebookEditor && notebook_mode == command) [vim] |
| `notebook::InterruptKernel` | notebook: interrupt kernel | — | Interrupts the current execution. | `cmd-c` (NotebookEditor) |
| `notebook::MoveCellDown` | notebook: move cell down | — | Moves the current cell down. | `alt-down` (NotebookEditor); `alt-down` (NotebookEditor > Editor) |
| `notebook::MoveCellUp` | notebook: move cell up | — | Moves the current cell up. | `alt-up` (NotebookEditor); `alt-up` (NotebookEditor > Editor) |
| `notebook::NotebookMoveDown` | notebook: notebook move down | — | Move down in cells. | `j` (NotebookEditor > Editor && VimControl && vim_mode == normal) [vim] |
| `notebook::NotebookMoveUp` | notebook: notebook move up | — | Move up in cells. | `k` (NotebookEditor > Editor && VimControl && vim_mode == normal) [vim] |
| `notebook::OpenNotebook` | notebook: open notebook | — | Opens a Jupyter notebook file. |  |
| `notebook::RestartKernel` | notebook: restart kernel | — | Restarts the kernel. | `cmd-shift-r` (NotebookEditor); `cmd-shift-r` (NotebookEditor > Editor) |
| `notebook::Run` | notebook: run | — | Runs the current cell and stays on it. | `cmd-enter` (NotebookEditor); `cmd-enter` (NotebookEditor > Editor); `ctrl-enter` (NotebookEditor && notebook_mode == command) [vim] |
| `notebook::RunAll` | notebook: run all | — | Runs all cells in the notebook. | `cmd-shift-enter` (NotebookEditor); `cmd-shift-enter` (NotebookEditor > Editor) |
| `notebook::RunAndAdvance` | notebook: run and advance | — | Runs the current cell and advances to the next cell. | `shift-enter` (NotebookEditor); `shift-enter` (NotebookEditor > Editor); `shift-enter` (NotebookEditor && notebook_mode == command) [vim] |

## onboarding

| Action | Palette name | Argument | Description | Default keys |
|---|---|---|---|---|
| `onboarding::Finish` | onboarding: finish | — | Finish the onboarding process. | `cmd-enter` (Onboarding) |
| `onboarding::OpenAccount` | onboarding: open account | — | Open the user account in zed.dev while in the onboarding flow. | `alt-shift-a` (Onboarding) |
| `onboarding::ResetHints` | onboarding: reset hints | — | Resets the welcome screen hints to their initial state. |  |
| `onboarding::SignIn` | onboarding: sign in | — | Sign in while in the onboarding flow. | `alt-tab` (Onboarding) |

## outline

| Action | Palette name | Argument | Description | Default keys |
|---|---|---|---|---|
| `outline::Toggle` | outline: toggle | — |  | `cmd-shift-o` (BufferSearchBar); `cmd-shift-o` (Editor && mode == full); `g s` (VimControl && !menu) [vim]; `g shift-o` (VimControl && !menu) [vim]; `space s` ((vim_mode == helix_normal \|\| vim_mode == helix_select) && !menu) [vim] |

## outline_panel

| Action | Palette name | Argument | Description | Default keys |
|---|---|---|---|---|
| `outline_panel::CollapseAllEntries` | outline panel: collapse all entries | — | Collapses all entries in the outline tree. |  |
| `outline_panel::CollapseSelectedEntry` | outline panel: collapse selected entry | — | Collapses the currently selected entry. | `left` (OutlinePanel && not_editing); `h` (OutlinePanel && not_editing) [vim] |
| `outline_panel::ExpandAllEntries` | outline panel: expand all entries | — | Expands all entries in the outline tree. |  |
| `outline_panel::ExpandSelectedEntry` | outline panel: expand selected entry | — | Expands the currently selected entry. | `right` (OutlinePanel && not_editing); `l` (OutlinePanel && not_editing) [vim] |
| `outline_panel::FoldDirectory` | outline panel: fold directory | — | Folds the selected directory. |  |
| `outline_panel::OpenSelectedEntry` | outline panel: open selected entry | — | Opens the selected entry in the editor. | `space` (OutlinePanel && not_editing) |
| `outline_panel::RevealInFileManager` | outline panel: reveal in file manager | — | Reveals the selected item in the system file manager. | `alt-cmd-r` (OutlinePanel && not_editing) |
| `outline_panel::ScrollCursorBottom` | outline panel: scroll cursor bottom | — | Scroll until the cursor displays at the bottom | `z b` (OutlinePanel && not_editing) [vim] |
| `outline_panel::ScrollCursorCenter` | outline panel: scroll cursor center | — | Scroll until the cursor displays at the center | `z z` (OutlinePanel && not_editing) [vim]; `z c` (OutlinePanel && not_editing) [vim] |
| `outline_panel::ScrollCursorTop` | outline panel: scroll cursor top | — | Scroll until the cursor displays at the top | `z t` (OutlinePanel && not_editing) [vim] |
| `outline_panel::ScrollDown` | outline panel: scroll down | — | Scroll half a page downwards | `ctrl-d` (OutlinePanel && not_editing) [vim] |
| `outline_panel::ScrollUp` | outline panel: scroll up | — | Scroll half a page upwards | `ctrl-u` (OutlinePanel && not_editing) [vim] |
| `outline_panel::SelectParent` | outline panel: select parent | — | Selects the parent of the current entry. | `-` (OutlinePanel && not_editing) [vim] |
| `outline_panel::Toggle` | outline panel: toggle | — | Toggles the outline panel. |  |
| `outline_panel::ToggleActiveEditorPin` | outline panel: toggle active editor pin | — | Toggles the pin status of the active editor. |  |
| `outline_panel::ToggleFocus` | outline panel: toggle focus | — | Toggles focus on the outline panel. | `cmd-shift-b` (Workspace) |
| `outline_panel::ToggleSymbols` | outline panel: toggle symbols | — | Toggles showing symbols, excerpts and search matches for multi-buffer views. |  |
| `outline_panel::UnfoldDirectory` | outline panel: unfold directory | — | Unfolds the selected directory. |  |

## pane

| Action | Palette name | Argument | Description | Default keys |
|---|---|---|---|---|
| `pane::ActivateItem` | pane: activate item | `integer ≥ 0` | Activates a specific item in the pane by its index. | `ctrl-1` 0 (Pane); `ctrl-2` 1 (Pane); `ctrl-3` 2 (Pane); `ctrl-4` 3 (Pane); `ctrl-5` 4 (Pane); +7 more |
| `pane::ActivateLastItem` | pane: activate last item | — | Activates the last item in the pane. | `ctrl-0` (Pane); `] shift-b` (vim_mode == normal \|\| vim_mode == visual \|\| vim_mode == operator) [vim]; `shift-b` (vim_operator == helix_next) [vim]; `] shift-b` (!Editor && !Terminal) [vim] |
| `pane::ActivateNextItem` | pane: activate next item | `{ wrap_around?: boolean = true }` | Activates the next item in the pane. Argument: `wrap_around`: Whether to wrap from the last item to the first item. | `alt-cmd-right` (Pane); `cmd-}` (Pane); `ctrl-tab` (RunModal); `] b` (vim_mode == normal \|\| vim_mode == visual \|\| vim_mode == operator) [vim]; `ctrl-pagedown` (vim_mode == helix_select) [vim]; +7 more |
| `pane::ActivatePreviousItem` | pane: activate previous item | `{ wrap_around?: boolean = true }` | Activates the previous item in the pane. Argument: `wrap_around`: Whether to wrap from the first item to the last item. | `alt-cmd-left` (Pane); `cmd-{` (Pane); `ctrl-shift-tab` (RunModal); `[ b` (vim_mode == normal \|\| vim_mode == visual \|\| vim_mode == operator) [vim]; `ctrl-pageup` (vim_mode == helix_select) [vim]; +7 more |
| `pane::AlternateFile` | pane: alternate file | — | Switches to the alternate file. | `ctrl-^` (VimControl && !menu) [vim]; `g a` ((vim_mode == helix_normal \|\| vim_mode == helix_select) && !menu) [vim] |
| `pane::CloseActiveItem` | pane: close active item | `{ save_intent?: SaveIntent \| null, close_pinned?: boolean = false }` | Closes the currently active item in the pane. | `cmd-w` {"close_pinned": false} (Pane); `shift-z shift-q` {"save_intent": "skip"} (VimControl && !menu) [vim]; `shift-z shift-z` {"save_intent": "save_all"} (VimControl && !menu) [vim]; `space w q` ((vim_mode == helix_normal \|\| vim_mode == helix_select) && !menu) [vim]; `ctrl-w ctrl-c` (VimControl && !menu \|\| !Editor && !Terminal) [vim]; +4 more |
| `pane::CloseAllItems` | pane: close all items | `{ save_intent?: SaveIntent \| null, close_pinned?: boolean = false }` | Closes all items in the pane. | `cmd-k w` {"close_pinned": false} (Pane); `ctrl-w ctrl-a` (VimControl && !menu \|\| !Editor && !Terminal) [vim]; `ctrl-w a` (VimControl && !menu \|\| !Editor && !Terminal) [vim] |
| `pane::CloseCleanItems` | pane: close clean items | `{ close_pinned?: boolean = false }` | Closes all items that have no unsaved changes. | `cmd-k u` {"close_pinned": false} (Pane) |
| `pane::CloseItemsToTheLeft` | pane: close items to the left | `{ close_pinned?: boolean = false }` | Closes all items to the left of the current item. | `cmd-k e` {"close_pinned": false} (Pane) |
| `pane::CloseItemsToTheRight` | pane: close items to the right | `{ close_pinned?: boolean = false }` | Closes all items to the right of the current item. | `cmd-k t` {"close_pinned": false} (Pane) |
| `pane::CloseMultibufferItems` | pane: close multibuffer items | `{ save_intent?: SaveIntent \| null, close_pinned?: boolean = false }` | Closes all multibuffers in the pane. |  |
| `pane::CloseOtherItems` | pane: close other items | `{ save_intent?: SaveIntent \| null, close_pinned?: boolean = false }` | Closes all inactive items in the pane. Aliases: `pane::CloseInactiveItems` | `alt-cmd-t` {"close_pinned": false} (Pane) |
| `pane::DeploySearch` | pane: deploy search | `{ replace_enabled?: boolean = false, included_files?: string \| null, excluded_files?: string \| null, query?: string \| null, regex?: boolean \| null, case_sensitive?: boolean \| null, whole_word?: boolean \| null, include_ignored?: boolean \| null }` | Opens the search interface with the specified configuration. | `cmd-shift-f` (Pane); `cmd-shift-f` (Workspace); `cmd-shift-h` {"replace_enabled": true} (Workspace); `g /` (VimControl && !menu) [vim]; `space /` ((vim_mode == helix_normal \|\| vim_mode == helix_select) && !menu) [vim]; +2 more |
| `pane::GoBack` | pane: go back | — | Navigates back in history. | `ctrl--` (AcpThread); `ctrl--` (ThreadHistory); `ctrl--` (Pane); `ctrl-o` (VimControl && !menu) [vim] |
| `pane::GoForward` | pane: go forward | — | Navigates forward in history. | `ctrl-_` (Pane); `ctrl-i` (VimControl && !menu) [vim] |
| `pane::GoToNewerTag` | pane: go to newer tag | — | Navigates forward in the tag stack. |  |
| `pane::GoToOlderTag` | pane: go to older tag | — | Navigates back in the tag stack. | `ctrl-t` (VimControl && !menu) [vim] |
| `pane::JoinAll` | pane: join all | — | Joins all panes into one. |  |
| `pane::JoinIntoNext` | pane: join into next | — | Joins this pane into the next pane. |  |
| `pane::ReopenClosedItem` | pane: reopen closed item | — | Reopens the most recently closed item. | `cmd-shift-t` (Workspace) |
| `pane::RevealInProjectPanel` | pane: reveal in project panel | `object` | Reveals the current item in the project panel. | `cmd-shift-e` (!AcpThread > Editor && mode == full) |
| `pane::SplitAndMoveDown` | pane: split and move down | — | Splits the pane downward, moving the current item. |  |
| `pane::SplitAndMoveLeft` | pane: split and move left | — | Splits the pane to the left, moving the current item. |  |
| `pane::SplitAndMoveRight` | pane: split and move right | — | Splits the pane to the right, moving the current item. |  |
| `pane::SplitAndMoveUp` | pane: split and move up | — | Splits the pane upward, moving the current item. |  |
| `pane::SplitDown` | pane: split down | `{ mode?: SplitMode }` | Splits the pane downward. | `cmd-k down` (Pane); `ctrl-alt-down` (Terminal); `space w s` ((vim_mode == helix_normal \|\| vim_mode == helix_select) && !menu) [vim]; `space w d` ((vim_mode == helix_normal \|\| vim_mode == helix_select) && !menu) [vim] |
| `pane::SplitHorizontal` | pane: split horizontal | `{ mode?: SplitMode }` | Splits the pane horizontally. | `ctrl-w shift-s` (VimControl && !menu \|\| !Editor && !Terminal) [vim]; `ctrl-w ctrl-s` (VimControl && !menu \|\| !Editor && !Terminal) [vim]; `ctrl-w s` (VimControl && !menu \|\| !Editor && !Terminal) [vim] |
| `pane::SplitLeft` | pane: split left | `{ mode?: SplitMode }` | Splits the pane to the left. | `cmd-k left` (Pane); `ctrl-alt-left` (Terminal) |
| `pane::SplitRight` | pane: split right | `{ mode?: SplitMode }` | Splits the pane to the right. | `cmd-\` (Editor); `cmd-k right` (Pane); `ctrl-alt-right` (Terminal); `cmd-d` (Terminal); `space w v` ((vim_mode == helix_normal \|\| vim_mode == helix_select) && !menu) [vim]; +1 more |
| `pane::SplitUp` | pane: split up | `{ mode?: SplitMode }` | Splits the pane upward. | `cmd-k up` (Pane); `ctrl-alt-up` (Terminal) |
| `pane::SplitVertical` | pane: split vertical | `{ mode?: SplitMode }` | Splits the pane vertically. | `ctrl-w ctrl-v` (VimControl && !menu \|\| !Editor && !Terminal) [vim]; `ctrl-w v` (VimControl && !menu \|\| !Editor && !Terminal) [vim] |
| `pane::SwapItemLeft` | pane: swap item left | — | Swaps the current item with the one to the left. | `ctrl-shift-pageup` (Pane) |
| `pane::SwapItemRight` | pane: swap item right | — | Swaps the current item with the one to the right. | `ctrl-shift-pagedown` (Pane) |
| `pane::TogglePinTab` | pane: toggle pin tab | — | Toggles pin status for the current tab. | `cmd-k shift-enter` (Pane) |
| `pane::TogglePreviewTab` | pane: toggle preview tab | — | Toggles preview mode for the current tab. |  |
| `pane::UnpinAllTabs` | pane: unpin all tabs | — | Unpins all tabs in the pane. |  |

## picker

| Action | Palette name | Argument | Description | Default keys |
|---|---|---|---|---|
| `picker::ConfirmCompletion` | picker: confirm completion | — | Confirms the selected completion in the picker. | `tab` (Picker > Editor) |
| `picker::ConfirmInput` | picker: confirm input | `{ secondary: boolean }` | ConfirmInput is an alternative editor action which - instead of selecting active picker entry - treats pickers editor input literally, performing some kind of action on it. | `alt-enter` {"secondary": false} (Picker > Editor); `cmd-alt-enter` {"secondary": true} (Picker > Editor) |
| `picker::MultiSelectNext` | picker: multi select next | — | Toggles the current item in the multi-selection and advances to the next item, starting multi-select mode if it isn't already active |  |
| `picker::SetPreviewBelow` | picker: set preview below | — | Shows the preview below the results. |  |
| `picker::SetPreviewHidden` | picker: set preview hidden | — | Hides the preview. |  |
| `picker::SetPreviewRight` | picker: set preview right | — | Shows the preview to the right of the results. |  |
| `picker::ToggleActionsMenu` | picker: toggle actions menu | — | Opens the footer's actions menu. |  |
| `picker::ToggleMultiSelect` | picker: toggle multi select | — | Toggles multi-select mode, in which clicking items adds them to the selection instead of opening them |  |
| `picker::TogglePreview` | picker: toggle preview | — | Toggles the preview between hidden and visible. |  |

## project_panel

| Action | Palette name | Argument | Description | Default keys |
|---|---|---|---|---|
| `project_panel::CollapseAllEntries` | project panel: collapse all entries | — | Collapses all entries in the project tree. | `cmd-left` (ProjectPanel) |
| `project_panel::CollapseSelectedEntry` | project panel: collapse selected entry | — | Collapses the selected entry in the project tree. | `left` (ProjectPanel); `h` (ProjectPanel && not_editing) [vim] |
| `project_panel::CollapseSelectedEntryAndChildren` | project panel: collapse selected entry and children | — | Collapses the selected entry and its children in the project tree. |  |
| `project_panel::CompareMarkedFiles` | project panel: compare marked files | — | Opens a diff view to compare two marked files. | `alt-d` (ProjectPanel); `z d` (ProjectPanel && not_editing) [vim] |
| `project_panel::Copy` | project panel: copy | — | Copies the selected file or directory. | `cmd-c` (ProjectPanel) |
| `project_panel::Cut` | project panel: cut | — | Cuts the selected file or directory. | `cmd-x` (ProjectPanel) |
| `project_panel::Delete` | project panel: delete | `{ skip_prompt?: boolean = false }` | Permanently deletes the selected file or directory. | `cmd-delete` {"skip_prompt": false} (ProjectPanel); `cmd-alt-backspace` {"skip_prompt": false} (ProjectPanel); `shift-d` (ProjectPanel && not_editing) [vim] |
| `project_panel::DownloadFromRemote` | project panel: download from remote | — | Downloads the selected remote file |  |
| `project_panel::Duplicate` | project panel: duplicate | — | Duplicates the selected file or directory. | `cmd-d` (ProjectPanel) |
| `project_panel::ExpandAllEntries` | project panel: expand all entries | — | Expands all entries in the project tree. | `cmd-right` (ProjectPanel) |
| `project_panel::ExpandSelectedEntry` | project panel: expand selected entry | — | Expands the selected entry in the project tree. | `right` (ProjectPanel); `l` (ProjectPanel && not_editing) [vim] |
| `project_panel::ExpandSelectedEntryAndChildren` | project panel: expand selected entry and children | — | Expands the selected entry and its children in the project tree. |  |
| `project_panel::FoldDirectory` | project panel: fold directory | — | Folds the selected directory. |  |
| `project_panel::NewDirectory` | project panel: new directory | — | Creates a new directory. | `alt-cmd-n` (ProjectPanel); `d` (ProjectPanel && not_editing) [vim] |
| `project_panel::NewFile` | project panel: new file | — | Creates a new file. | `cmd-n` (ProjectPanel); `%` (ProjectPanel && not_editing) [vim] |
| `project_panel::NewSearchInDirectory` | project panel: new search in directory | — | Starts a new search in the selected directory. | `cmd-alt-shift-f` (ProjectPanel); `/` (ProjectPanel && not_editing) [vim] |
| `project_panel::Open` | project panel: open | — | Opens the selected file in the editor. | `space` (ProjectPanel && not_editing); `p` (ProjectPanel && not_editing) [vim] |
| `project_panel::OpenContextMenu` | project panel: open context menu | — | Opens the context menu for the selected entry. | `shift-f10` (ProjectPanel) |
| `project_panel::OpenMarkdownPreview` | project panel: open markdown preview | — | Opens a markdown preview for the selected file. |  |
| `project_panel::OpenPermanent` | project panel: open permanent | — | Opens the selected file in a permanent tab. | `enter` (ProjectPanel && not_editing) [vim]; `t` (ProjectPanel && not_editing) [vim] |
| `project_panel::OpenSplitHorizontal` | project panel: open split horizontal | — | Opens the selected file in a horizontal split. | `o` (ProjectPanel && not_editing) [vim] |
| `project_panel::OpenSplitVertical` | project panel: open split vertical | — | Opens the selected file in a vertical split. | `v` (ProjectPanel && not_editing) [vim] |
| `project_panel::Paste` | project panel: paste | — | Pastes the previously cut or copied item. | `cmd-v` (ProjectPanel) |
| `project_panel::Redo` | project panel: redo | — | Redoes the last undone file operation. | `cmd-shift-z` (ProjectPanel) |
| `project_panel::RemoveFromProject` | project panel: remove from project | — | Removes the selected folder from the project. |  |
| `project_panel::Rename` | project panel: rename | — | Renames the selected file or directory. | `enter` (ProjectPanel); `f2` (ProjectPanel); `shift-r` (ProjectPanel && not_editing) [vim] |
| `project_panel::RevealInFileManager` | project panel: reveal in file manager | — | Reveals the selected item in the system file manager. | `alt-cmd-r` (ProjectPanel); `x` (ProjectPanel && not_editing) [vim] |
| `project_panel::ScrollCursorBottom` | project panel: scroll cursor bottom | — | Scroll until the cursor displays at the bottom | `z b` (ProjectPanel && not_editing) [vim] |
| `project_panel::ScrollCursorCenter` | project panel: scroll cursor center | — | Scroll until the cursor displays at the center | `z z` (ProjectPanel && not_editing) [vim]; `z c` (ProjectPanel && not_editing) [vim] |
| `project_panel::ScrollCursorTop` | project panel: scroll cursor top | — | Scroll until the cursor displays at the top | `z t` (ProjectPanel && not_editing) [vim] |
| `project_panel::ScrollDown` | project panel: scroll down | — | Scroll half a page downwards | `ctrl-d` (ProjectPanel && not_editing) [vim] |
| `project_panel::ScrollUp` | project panel: scroll up | — | Scroll half a page upwards | `ctrl-u` (ProjectPanel && not_editing) [vim] |
| `project_panel::SelectNextDiagnostic` | project panel: select next diagnostic | `{ severity?: GoToDiagnosticSeverityFilter }` | Selects the next entry with diagnostics. | `] d` (ProjectPanel && not_editing) [vim] |
| `project_panel::SelectNextDirectory` | project panel: select next directory | — | Selects the next directory. | `}` (ProjectPanel && not_editing) [vim] |
| `project_panel::SelectNextGitEntry` | project panel: select next git entry | — | Selects the next entry with git changes. | `] c` (ProjectPanel && not_editing) [vim] |
| `project_panel::SelectParent` | project panel: select parent | — | Selects the parent directory. | `-` (ProjectPanel && not_editing) [vim] |
| `project_panel::SelectPrevDiagnostic` | project panel: select prev diagnostic | `{ severity?: GoToDiagnosticSeverityFilter }` | Selects the previous entry with diagnostics. | `[ d` (ProjectPanel && not_editing) [vim] |
| `project_panel::SelectPrevDirectory` | project panel: select prev directory | — | Selects the previous directory. | `{` (ProjectPanel && not_editing) [vim] |
| `project_panel::SelectPrevGitEntry` | project panel: select prev git entry | — | Selects the previous entry with git changes. | `[ c` (ProjectPanel && not_editing) [vim] |
| `project_panel::Toggle` | project panel: toggle | — | Toggles the project panel. |  |
| `project_panel::ToggleFocus` | project panel: toggle focus | — | Toggles focus on the project panel. | `cmd-shift-e` (AgentPanel); `cmd-shift-e` (Workspace) |
| `project_panel::ToggleHideGitIgnore` | project panel: toggle hide git ignore | — | Toggles visibility of git-ignored files. |  |
| `project_panel::ToggleHideHidden` | project panel: toggle hide hidden | — | Toggles visibility of hidden files. |  |
| `project_panel::Trash` | project panel: trash | `{ skip_prompt?: boolean = false }` | Moves the selected file or directory to the system trash. | `backspace` {"skip_prompt": false} (ProjectPanel); `delete` {"skip_prompt": false} (ProjectPanel); `cmd-backspace` {"skip_prompt": true} (ProjectPanel) |
| `project_panel::Undo` | project panel: undo | — | Undoes the last file operation. | `cmd-z` (ProjectPanel) |
| `project_panel::UnfoldDirectory` | project panel: unfold directory | — | Unfolds the selected directory. |  |

## project_search

| Action | Palette name | Argument | Description | Default keys |
|---|---|---|---|---|
| `project_search::NextField` | project search: next field | — | Moves to the next input field. |  |
| `project_search::OpenTextFinder` | project search: open text finder | — | Open a text picker showing the current result in a modal. | `alt-cmd-f` (ProjectSearchBar); `alt-cmd-f` (ProjectSearchView) |
| `project_search::SearchInNew` | project search: search in new | — | Searches in a new project search tab. | `cmd-enter` (ProjectSearchBar && !in_replace) |
| `project_search::ToggleAllSearchResults` | project search: toggle all search results | — | Toggles collapse/expand state of all search result excerpts. | `cmd-shift-enter` (ProjectSearchBar); `cmd-shift-enter` (ProjectSearchView) |
| `project_search::ToggleFilters` | project search: toggle filters | — | Toggles the search filters panel. | `cmd-shift-j` (ProjectSearchBar); `cmd-shift-j` (ProjectSearchView) |
| `project_search::ToggleFocus` | project search: toggle focus | — | Toggles focus between the search bar and the search results. | `escape` (ProjectSearchBar); `escape` (ProjectSearchView); `cmd-f` (Pane) |

## project_symbols

| Action | Palette name | Argument | Description | Default keys |
|---|---|---|---|---|
| `project_symbols::Toggle` | project symbols: toggle | — | Toggles the project symbols search. | `cmd-t` (Workspace); `g shift-s` (VimControl && !menu) [vim]; `space shift-s` ((vim_mode == helix_normal \|\| vim_mode == helix_select) && !menu) [vim]; `space shift-s` (!Editor && !Terminal) [vim] |

## projects

| Action | Palette name | Argument | Description | Default keys |
|---|---|---|---|---|
| `projects::InitializeDevContainer` | projects: initialize dev container | — |  |  |
| `projects::OpenDevContainer` | projects: open dev container | — | Opens the dev container connection modal. |  |
| `projects::OpenRecent` | projects: open recent | `{ create_new_window?: boolean \| null }` | Opens the recent projects interface. | `alt-cmd-o` (Workspace); `ctrl-r` (Workspace) |
| `projects::OpenRemote` | projects: open remote | `{ from_existing_connection?: boolean = false, create_new_window?: boolean \| null }` | Creates a project from a selected template. | `ctrl-cmd-o` {"from_existing_connection": false} (Workspace); `ctrl-cmd-shift-o` {"from_existing_connection": true} (Workspace) |

## recent_projects

| Action | Palette name | Argument | Description | Default keys |
|---|---|---|---|---|
| `recent_projects::AddToWorkspace` | recent projects: add to workspace | — |  | `cmd-shift-enter` (RecentProjects \|\| (RecentProjects > Picker > Editor)) |
| `recent_projects::RemoveSelected` | recent projects: remove selected | — |  | `shift-backspace` (RecentProjects \|\| (RecentProjects > Picker > Editor)) |
| `recent_projects::ToggleActionsMenu` | recent projects: toggle actions menu | — |  | `cmd-k` (RecentProjects \|\| (RecentProjects > Picker > Editor)) |

## remote_debug

| Action | Palette name | Argument | Description | Default keys |
|---|---|---|---|---|
| `remote_debug::SimulateDisconnect` | remote debug: simulate disconnect | — | Simulates a disconnection from the remote server for testing purposes. This will trigger the reconnection logic. |  |
| `remote_debug::SimulateTimeout` | remote debug: simulate timeout | — | Simulates a timeout/slow connection to the remote server for testing purposes. This will cause heartbeat failures and trigger reconnection. |  |
| `remote_debug::SimulateTimeoutExhausted` | remote debug: simulate timeout exhausted | — | Simulates a timeout/slow connection to the remote server for testing purposes. This will cause heartbeat failures and attempting a reconnection while having exhausted all attempts. |  |

## repl

| Action | Palette name | Argument | Description | Default keys |
|---|---|---|---|---|
| `repl::ClearCurrentOutput` | repl: clear current output | — | Clears the output of the cell at the current cursor position. |  |
| `repl::ClearOutputs` | repl: clear outputs | — | Clears all outputs in the REPL. |  |
| `repl::Interrupt` | repl: interrupt | — | Interrupts the currently running kernel. |  |
| `repl::RefreshKernelspecs` | repl: refresh kernelspecs | — | Refreshes the list of available kernelspecs. |  |
| `repl::Restart` | repl: restart | — | Restarts the current kernel. |  |
| `repl::Run` | repl: run | — | Runs the current cell and advances to the next one. | `ctrl-shift-enter` (Editor && jupyter) |
| `repl::RunInPlace` | repl: run in place | — | Runs the current cell without advancing. | `ctrl-alt-enter` (Editor && jupyter) |
| `repl::Sessions` | repl: sessions | — | Opens the REPL sessions panel. |  |
| `repl::Shutdown` | repl: shutdown | — | Shuts down the current kernel. |  |

## search

| Action | Palette name | Argument | Description | Default keys |
|---|---|---|---|---|
| `search::CycleMode` | search: cycle mode | — | Cycles through search modes. |  |
| `search::FocusSearch` | search: focus search | — | Focuses on the search input field. | `cmd-f` (AcpThreadSearchBar); `cmd-f` (BufferSearchBar); `cmd-shift-f` (ProjectSearchBar); `cmd-f` (KeymapEditor); `cmd-f` (SettingsWindow) |
| `search::NextHistoryQuery` | search: next history query | — | Navigates to the next query in search history. | `down` (BufferSearchBar && !in_replace > Editor); `down` (ProjectSearchBar > Editor) |
| `search::PreviousHistoryQuery` | search: previous history query | — | Navigates to the previous query in search history. | `up` (BufferSearchBar && !in_replace > Editor); `up` (ProjectSearchBar > Editor) |
| `search::ReplaceAll` | search: replace all | — | Replaces all matches. | `cmd-enter` (BufferSearchBar && in_replace > Editor); `cmd-enter` (ProjectSearchBar && in_replace > Editor) |
| `search::ReplaceNext` | search: replace next | — | Replaces the next match. | `enter` (BufferSearchBar && in_replace > Editor); `enter` (ProjectSearchBar && in_replace > Editor) |
| `search::SelectAllMatches` | search: select all matches | — | Selects all search matches. | `alt-enter` (BufferSearchBar); `alt-enter` (Pane) |
| `search::SelectNextMatch` | search: select next match | — | Selects the next search match. | `enter` (BufferSearchBar); `cmd-g` (Pane) |
| `search::SelectPreviousMatch` | search: select previous match | — | Selects the previous search match. | `shift-enter` (BufferSearchBar); `shift-enter` (BufferSearchBar && !in_replace > Editor); `cmd-shift-g` (Pane) |
| `search::ToggleCaseSensitive` | search: toggle case sensitive | — | Toggles case-sensitive search. | `alt-cmd-c` (AcpThread); `alt-cmd-c` (Pane) |
| `search::ToggleIncludeIgnored` | search: toggle include ignored | — | Toggles searching in ignored files. | `cmd-shift-i` (FileFinder \|\| (FileFinder > Picker > Editor)) |
| `search::ToggleRegex` | search: toggle regex | — | Toggles regular expression mode. | `alt-cmd-x` (AcpThread); `alt-cmd-g` (ProjectSearchBar); `alt-cmd-x` (ProjectSearchBar); `alt-cmd-g` (ProjectSearchView); `alt-cmd-x` (ProjectSearchView); +1 more |
| `search::ToggleReplace` | search: toggle replace | — | Toggles the replace interface. | `cmd-shift-h` (ProjectSearchBar); `cmd-shift-h` (ProjectSearchView); `cmd-shift-h` (Pane) |
| `search::ToggleSelection` | search: toggle selection | — | Toggles searching within selection only. | `cmd-alt-l` (BufferSearchBar); `cmd-alt-l` (Pane) |
| `search::ToggleWholeWord` | search: toggle whole word | — | Toggles whole word matching. | `alt-cmd-w` (AcpThread); `alt-cmd-w` (Pane) |

## settings_editor

| Action | Palette name | Argument | Description | Default keys |
|---|---|---|---|---|
| `settings_editor::CollapseNavEntry` | settings editor: collapse nav entry | — | Collapses the navigation entry. | `left` (SettingsWindow > NavigationMenu); `h` (SettingsWindow > NavigationMenu && !search) [vim] |
| `settings_editor::ExpandNavEntry` | settings editor: expand nav entry | — | Expands the navigation entry. | `right` (SettingsWindow > NavigationMenu); `l` (SettingsWindow > NavigationMenu && !search) [vim] |
| `settings_editor::FocusFile` | settings editor: focus file | `integer ≥ 0` |  | `ctrl-1` 0 (SettingsWindow); `ctrl-2` 1 (SettingsWindow); `ctrl-3` 2 (SettingsWindow); `ctrl-4` 3 (SettingsWindow); `ctrl-5` 4 (SettingsWindow); +5 more |
| `settings_editor::FocusFirstNavEntry` | settings editor: focus first nav entry | — | Focuses the first navigation entry. | `home` (SettingsWindow > NavigationMenu); `g g` (SettingsWindow > NavigationMenu && !search) [vim] |
| `settings_editor::FocusLastNavEntry` | settings editor: focus last nav entry | — | Focuses the last navigation entry. | `end` (SettingsWindow > NavigationMenu); `shift-g` (SettingsWindow > NavigationMenu && !search) [vim] |
| `settings_editor::FocusNextFile` | settings editor: focus next file | — | Focuses the next file in the file list. | `cmd-}` (SettingsWindow) |
| `settings_editor::FocusNextNavEntry` | settings editor: focus next nav entry | — | Focuses and opens the next navigation entry without moving focus to content. | `down` (SettingsWindow > NavigationMenu); `tab` (SettingsWindow > NavigationMenu); `j` (SettingsWindow > NavigationMenu && !search) [vim] |
| `settings_editor::FocusNextRootNavEntry` | settings editor: focus next root nav entry | — | Focuses the next root navigation entry. | `pagedown` (SettingsWindow > NavigationMenu) |
| `settings_editor::FocusPreviousFile` | settings editor: focus previous file | — | Focuses the previous file in the file list. | `cmd-{` (SettingsWindow) |
| `settings_editor::FocusPreviousNavEntry` | settings editor: focus previous nav entry | — | Focuses and opens the previous navigation entry without moving focus to content. | `up` (SettingsWindow > NavigationMenu); `shift-tab` (SettingsWindow > NavigationMenu); `k` (SettingsWindow > NavigationMenu && !search) [vim] |
| `settings_editor::FocusPreviousRootNavEntry` | settings editor: focus previous root nav entry | — | Focuses the previous root navigation entry. | `pageup` (SettingsWindow > NavigationMenu) |
| `settings_editor::Minimize` | settings editor: minimize | — | Minimizes the settings UI window. | `cmd-m` (SettingsWindow) |
| `settings_editor::OpenCurrentFile` | settings editor: open current file | — | Opens an editor for the current file | `cmd-,` (SettingsWindow) |
| `settings_editor::ToggleFocusNav` | settings editor: toggle focus nav | — | Toggles focus between the navbar and the main content. | `left` (SettingsWindow); `cmd-shift-e` (SettingsWindow) |

## settings_profile_selector

| Action | Palette name | Argument | Description | Default keys |
|---|---|---|---|---|
| `settings_profile_selector::Toggle` | settings profile selector: toggle | — |  | `ctrl-alt-cmd-p` (Workspace) |

## skill_creator

| Action | Palette name | Argument | Description | Default keys |
|---|---|---|---|---|
| `skill_creator::Cancel` | skill creator: cancel | — |  |  |
| `skill_creator::FocusNextField` | skill creator: focus next field | — |  | `tab` (SkillCreator); `tab` (SkillCreator > Editor); `tab` (SkillCreator) [vim]; `tab` (SkillCreator > Editor) [vim] |
| `skill_creator::FocusPreviousField` | skill creator: focus previous field | — |  | `shift-tab` (SkillCreator); `shift-tab` (SkillCreator > Editor); `shift-tab` (SkillCreator) [vim]; `shift-tab` (SkillCreator > Editor) [vim] |
| `skill_creator::SaveSkill` | skill creator: save skill | — |  | `cmd-enter` (SkillCreator); `cmd-enter` (SkillCreator > Editor) |

## snippets

| Action | Palette name | Argument | Description | Default keys |
|---|---|---|---|---|
| `snippets::ConfigureSnippets` | snippets: configure snippets | — | Opens the snippets configuration file. |  |
| `snippets::OpenFolder` | snippets: open folder | — | Opens the snippets folder in the file manager. |  |

## stash_picker

| Action | Palette name | Argument | Description | Default keys |
|---|---|---|---|---|
| `stash_picker::DropStashItem` | stash picker: drop stash item | — | Drop the selected stash entry. | `ctrl-shift-backspace` (StashList \|\| (StashList > Picker > Editor)) |
| `stash_picker::ShowStashItem` | stash picker: show stash item | — | Show the diff view of the selected stash entry. | `ctrl-shift-v` (StashList \|\| (StashList > Picker > Editor)) |

## svg

| Action | Palette name | Argument | Description | Default keys |
|---|---|---|---|---|
| `svg::OpenFollowingPreview` | svg: open following preview | — | Opens a following SVG preview that syncs with the editor. |  |
| `svg::OpenPreview` | svg: open preview | — | Opens an SVG preview for the current file. | `cmd-shift-v` (Editor && extension == svg) |
| `svg::OpenPreviewToTheSide` | svg: open preview to the side | — | Opens an SVG preview in a split pane. | `cmd-k v` (Editor && extension == svg) |

## syntax_tree_view

| Action | Palette name | Argument | Description | Default keys |
|---|---|---|---|---|
| `syntax_tree_view::UseActiveEditor` | syntax tree view: use active editor | — | Update the syntax tree view to show the last focused file. |  |

## tab_switcher

| Action | Palette name | Argument | Description | Default keys |
|---|---|---|---|---|
| `tab_switcher::CloseSelectedItem` | tab switcher: close selected item | — | Closes the selected item in the tab switcher. | `ctrl-backspace` (TabSwitcher) |
| `tab_switcher::OpenInActivePane` | tab switcher: open in active pane | — | Toggles the tab switcher showing all tabs across all panes, deduplicated by path. Opens selected items in the active pane. |  |
| `tab_switcher::Toggle` | tab switcher: toggle | `{ select_last?: boolean = false }` | Toggles the tab switcher interface. | `ctrl-shift-tab` {"select_last": true} (Workspace); `ctrl-tab` (Workspace) |
| `tab_switcher::ToggleAll` | tab switcher: toggle all | — | Toggles between showing all tabs or just the current pane's tabs. | `space b` ((vim_mode == helix_normal \|\| vim_mode == helix_select) && !menu) [vim] |

## tabular_data

| Action | Palette name | Argument | Description | Default keys |
|---|---|---|---|---|
| `tabular_data::OpenPreview` | tabular data: open preview | — |  | `cmd-shift-v` (Editor && (extension == csv \|\| extension == tsv \|\| extension == ssv \|\| extension == psv)) |
| `tabular_data::OpenPreviewToTheSide` | tabular data: open preview to the side | — |  | `cmd-k v` (Editor && (extension == csv \|\| extension == tsv \|\| extension == ssv \|\| extension == psv)) |

## task

| Action | Palette name | Argument | Description | Default keys |
|---|---|---|---|---|
| `task::Rerun` | task: rerun | `{ reevaluate_context?: boolean = false, allow_concurrent_runs?: boolean \| null, use_new_terminal?: boolean \| null }` | Reruns the last task. Argument: `reevaluate_context`: Controls whether the task context is reevaluated prior to execution of a task. If it is not, environment variables such as ZED_COLUMN, ZED_FILE are gonna be the same as in the last execution of a task If it is, these variables will be updated to reflect current state of editor at the time task::Rerun is executed. default: false `allow_concurrent_runs`: Overrides `allow_concurrent_runs` property of the task being reran. Default: null `use_new_terminal`: Overrides `use_new_terminal` property of the task being reran. Default: null | `cmd-alt-r` {"reevaluate_context": false} (Workspace && !Terminal) |
| `task::Spawn` | task: spawn | `{ task_name: string, reveal_target?: RevealTarget \| null } \| { task_tag: string, reveal_target?: RevealTarget \| null } \| { reveal_target?: RevealTarget \| null }` | Spawns a task with name or opens tasks modal. Argument: variant 1: Spawns a task by the name given. variant 2: Spawns a task by the tag given. variant 3: Spawns a task via modal's selection. `reveal_target`: Selected task's `reveal_target` property override. | `cmd-shift-r` (Workspace && !Terminal); `ctrl-alt-shift-r` {"reveal_target": "center"} (Workspace && !Terminal) |

## terminal

| Action | Palette name | Argument | Description | Default keys |
|---|---|---|---|---|
| `terminal::Clear` | terminal: clear | — | Clears the terminal screen. | `cmd-k` (Terminal) |
| `terminal::Copy` | terminal: copy | — | Copies selected text to the clipboard. | `cmd-c` (Terminal) |
| `terminal::Paste` | terminal: paste | — | Pastes from the clipboard. | `cmd-v` (Terminal) |
| `terminal::PasteText` | terminal: paste text | — | Pastes the text from the clipboard. | `ctrl-cmd-v` (Terminal) |
| `terminal::RenameTerminal` | terminal: rename terminal | — | Renames the terminal tab. |  |
| `terminal::RerunTask` | terminal: rerun task | — | Reruns the last executed task in the terminal. | `cmd-alt-r` (Terminal) |
| `terminal::ScrollHalfPageDown` | terminal: scroll half page down | — | Scrolls down by half a page. |  |
| `terminal::ScrollHalfPageUp` | terminal: scroll half page up | — | Scrolls up by half a page. |  |
| `terminal::ScrollLineDown` | terminal: scroll line down | — | Scrolls down by one line. | `shift-down` (Terminal) |
| `terminal::ScrollLineUp` | terminal: scroll line up | — | Scrolls up by one line. | `shift-up` (Terminal) |
| `terminal::ScrollPageDown` | terminal: scroll page down | — | Scrolls down by one page. | `shift-pagedown` (Terminal); `cmd-down` (Terminal) |
| `terminal::ScrollPageUp` | terminal: scroll page up | — | Scrolls up by one page. | `shift-pageup` (Terminal); `cmd-up` (Terminal) |
| `terminal::ScrollToBottom` | terminal: scroll to bottom | — | Scrolls to the bottom of the terminal buffer. | `shift-end` (Terminal); `cmd-end` (Terminal) |
| `terminal::ScrollToTop` | terminal: scroll to top | — | Scrolls to the top of the terminal buffer. | `shift-home` (Terminal); `cmd-home` (Terminal) |
| `terminal::SearchTest` | terminal: search test | — | Searches for text in the terminal. |  |
| `terminal::SelectAll` | terminal: select all | — | Selects all text in the terminal. |  |
| `terminal::SendKeystroke` | terminal: send keystroke | `string` | Sends a keystroke sequence to the terminal. | `cmd-backspace` "ctrl-u" (Terminal); `cmd-delete` "ctrl-k" (Terminal); `cmd-right` "ctrl-e" (Terminal); `cmd-left` "ctrl-a" (Terminal); `up` "up" (Terminal); +8 more |
| `terminal::SendText` | terminal: send text | `string` | Sends the specified text directly to the terminal. | `alt-delete` "\u001bd" (Terminal); `alt-left` "\u001bb" (Terminal); `alt-right` "\u001bf" (Terminal); `alt-b` "\u001bb" (Terminal); `alt-f` "\u001bf" (Terminal); +1 more |
| `terminal::ShowCharacterPalette` | terminal: show character palette | — | Shows the character palette for special characters. | `ctrl-cmd-space` (Terminal) |
| `terminal::ToggleViMode` | terminal: toggle vi mode | — | Toggles vi mode in the terminal. | `ctrl-shift-space` (Terminal) |

## terminal_panel

| Action | Palette name | Argument | Description | Default keys |
|---|---|---|---|---|
| `terminal_panel::Toggle` | terminal panel: toggle | — | Toggles the terminal panel. | `ctrl-`` (Workspace) |
| `terminal_panel::ToggleFocus` | terminal panel: toggle focus | — | Toggles focus on the terminal panel. |  |

## text_finder

| Action | Palette name | Argument | Description | Default keys |
|---|---|---|---|---|
| `text_finder::Fold` | text finder: fold | — |  |  |
| `text_finder::ToProjectSearch` | text finder: to project search | — |  |  |
| `text_finder::Toggle` | text finder: toggle | — | Opens the Project Search Picker. | `alt-cmd-f` (Editor && mode == full); `alt-cmd-f` (BufferSearchBar) |
| `text_finder::ToggleFoldAll` | text finder: toggle fold all | — |  |  |
| `text_finder::Unfold` | text finder: unfold | — |  |  |

## theme

| Action | Palette name | Argument | Description | Default keys |
|---|---|---|---|---|
| `theme::ToggleMode` | theme: toggle mode | — |  | `cmd-k cmd-shift-t` (Workspace) |

## theme_selector

| Action | Palette name | Argument | Description | Default keys |
|---|---|---|---|---|
| `theme_selector::Reload` | theme selector: reload | — | Reloads all themes from disk. |  |
| `theme_selector::Toggle` | theme selector: toggle | `{ themes_filter?: [string, …] \| null }` | Toggles the theme selector interface. Argument: `themes_filter`: A list of theme names to filter the theme selector down to. | `cmd-k cmd-t` (Workspace) |

## toast

| Action | Palette name | Argument | Description | Default keys |
|---|---|---|---|---|
| `toast::RunAction` | toast: run action | — | Runs the action associated with a toast notification. | `alt-shift-enter` (Workspace) |

## toolchain

| Action | Palette name | Argument | Description | Default keys |
|---|---|---|---|---|
| `toolchain::AddToolchain` | toolchain: add toolchain | — | Adds a new toolchain for the current project. | `cmd-k cmd-m` (Workspace); `cmd-shift-a` (ToolchainSelector) |
| `toolchain::Select` | toolchain: select | — | Selects a toolchain for the current project. |  |

## variable_list

| Action | Palette name | Argument | Description | Default keys |
|---|---|---|---|---|
| `variable_list::AddWatch` | variable list: add watch | — | Adds the selected variable to the watch list. | `alt-enter` (VariableList) |
| `variable_list::CollapseSelectedEntry` | variable list: collapse selected entry | — | Collapses the selected variable entry to hide its children. | `left` (VariableList) |
| `variable_list::CopyVariableName` | variable list: copy variable name | — | Copies the variable name to the clipboard. | `cmd-alt-c` (VariableList) |
| `variable_list::CopyVariableValue` | variable list: copy variable value | — | Copies the variable value to the clipboard. | `cmd-c` (VariableList) |
| `variable_list::EditVariable` | variable list: edit variable | — | Edits the value of the selected variable. | `enter` (VariableList) |
| `variable_list::ExpandSelectedEntry` | variable list: expand selected entry | — | Expands the selected variable entry to show its children. | `right` (VariableList) |
| `variable_list::GoToMemory` | variable list: go to memory | — | Jump to variable's memory location. |  |
| `variable_list::RemoveWatch` | variable list: remove watch | — | Removes the selected variable from the watch list. | `delete` (VariableList); `backspace` (VariableList) |

## vim

| Action | Palette name | Argument | Description | Default keys |
|---|---|---|---|---|
| `vim::AngleBrackets` | vim: angle brackets | `{ opening?: boolean = false }` | Operates on text within or around angle brackets `<>`. | `<` {"opening": true} (vim_operator == a \|\| vim_operator == i \|\| vim_operator == cs \|\| vim_operator == helix_next \|\| vim_operator == helix_previous) [vim]; `>` (vim_operator == a \|\| vim_operator == i \|\| vim_operator == cs \|\| vim_operator == helix_next \|\| vim_operator == helix_previous) [vim] |
| `vim::AnyBrackets` | vim: any brackets | — | Selects text within any type of brackets. |  |
| `vim::AnyPair` | vim: any pair | — | Selects text within the closest surrounding pair of any type (brackets, quotes, backticks, or vertical bars), based on the language's bracket queries, like Helix's `m` text object. | `m` ((vim_operator == a \|\| vim_operator == i \|\| vim_operator == helix_next \|\| vim_operator == helix_previous) && helix_mode) [vim] |
| `vim::AnyQuotes` | vim: any quotes | — | Selects text within any type of quotes. |  |
| `vim::Argument` | vim: argument | — | Selects a function argument. | `a` (vim_operator == a \|\| vim_operator == i \|\| vim_operator == cs \|\| vim_operator == helix_next \|\| vim_operator == helix_previous) [vim] |
| `vim::ArgumentRequired` | vim: argument required | — | Indicates that an argument is required for the command. |  |
| `vim::AutoIndent` | vim: auto indent | — | Automatically adjusts indentation based on syntax. | `=` (vim_mode == visual) [vim]; `=` ((vim_mode == helix_normal \|\| vim_mode == helix_select) && !menu) [vim] |
| `vim::BackQuotes` | vim: back quotes | — | Selects text within backticks. | ``` (vim_operator == a \|\| vim_operator == i \|\| vim_operator == cs \|\| vim_operator == helix_next \|\| vim_operator == helix_previous) [vim] |
| `vim::ChangeCase` | vim: change case | — | Toggles the case of selected text. | `~` (vim_mode == visual) [vim]; `~` (vim_mode == helix_select) [vim]; `~` ((vim_mode == normal \|\| vim_mode == helix_normal) && !menu) [vim] |
| `vim::ChangeListNewer` | vim: change list newer | — | Navigates to a newer position in the change list. | `g ,` (VimControl && !menu) [vim] |
| `vim::ChangeListOlder` | vim: change list older | — | Navigates to an older position in the change list. | `g ;` (VimControl && !menu) [vim] |
| `vim::ChangeToEndOfLine` | vim: change to end of line | — | Changes from cursor to end of line. | `shift-c` (vim_mode == normal) [vim] |
| `vim::Class` | vim: class | — | Selects a class definition. | `c` (vim_operator == a \|\| vim_operator == i \|\| vim_operator == cs \|\| vim_operator == helix_next \|\| vim_operator == helix_previous) [vim]; `t` ((vim_operator == a \|\| vim_operator == i \|\| vim_operator == helix_next \|\| vim_operator == helix_previous) && helix_mode) [vim] |
| `vim::ClearExchange` | vim: clear exchange | — | Clears the exchange register. | `c` (vim_operator == cx) [vim] |
| `vim::ClearOperators` | vim: clear operators | — | Clears any pending operators. | `ctrl-c` (vim_mode == waiting) [vim]; `ctrl-[` (vim_mode == waiting) [vim]; `escape` (vim_mode == waiting) [vim]; `ctrl-c` (vim_mode == operator) [vim]; `ctrl-[` (vim_mode == operator) [vim]; +1 more |
| `vim::ColumnLeft` | vim: column left | — | Scrolls left by one column. | `z h` (VimControl && !menu) [vim] |
| `vim::ColumnRight` | vim: column right | — | Scrolls right by one column. | `z l` (VimControl && !menu) [vim] |
| `vim::Comment` | vim: comment | — | Selects a comment block. | `g c` (vim_mode == operator) [vim]; `c` ((vim_operator == a \|\| vim_operator == i \|\| vim_operator == helix_next \|\| vim_operator == helix_previous) && helix_mode) [vim] |
| `vim::ConvertToLowerCase` | vim: convert to lower case | — | Converts selected text to lowercase. | `u` (vim_mode == visual) [vim]; ``` ((vim_mode == helix_normal \|\| vim_mode == helix_select) && !menu) [vim] |
| `vim::ConvertToRot13` | vim: convert to rot13 | — | Applies ROT13 cipher to selected text. | `g ?` (vim_mode == visual) [vim] |
| `vim::ConvertToRot47` | vim: convert to rot47 | — | Applies ROT47 cipher to selected text. |  |
| `vim::ConvertToUpperCase` | vim: convert to upper case | — | Converts selected text to uppercase. | `shift-u` (vim_mode == visual) [vim]; `alt-`` ((vim_mode == helix_normal \|\| vim_mode == helix_select) && !menu) [vim] |
| `vim::CountCommand` | vim: count command | — | Executes a command with a count prefix. | `:` (VimControl && VimCount) [vim] |
| `vim::CurlyBrackets` | vim: curly brackets | `{ opening?: boolean = false }` | Operates on text within or around curly brackets `{}`. | `{` {"opening": true} (vim_operator == a \|\| vim_operator == i \|\| vim_operator == cs \|\| vim_operator == helix_next \|\| vim_operator == helix_previous) [vim]; `}` (vim_operator == a \|\| vim_operator == i \|\| vim_operator == cs \|\| vim_operator == helix_next \|\| vim_operator == helix_previous) [vim]; `shift-b` (vim_operator == a \|\| vim_operator == i \|\| vim_operator == cs \|\| vim_operator == helix_next \|\| vim_operator == helix_previous) [vim] |
| `vim::CurrentLine` | vim: current line | — | Selects the current line. | `c` (vim_operator == c) [vim]; `d` (vim_operator == d) [vim]; `g u` (vim_operator == gu) [vim]; `u` (vim_operator == gu) [vim]; `g shift-u` (vim_operator == gU) [vim]; +20 more |
| `vim::Decrement` | vim: decrement | `{ step?: boolean = false }` | Decrements the number under the cursor or toggles boolean values. | `ctrl-x` (vim_mode == normal) [vim]; `ctrl-x` (vim_mode == visual) [vim]; `g ctrl-x` {"step": true} (vim_mode == visual) [vim]; `ctrl-x` (vim_mode == helix_select) [vim]; `ctrl-x` ((vim_mode == normal \|\| vim_mode == helix_normal) && !menu) [vim] |
| `vim::DeleteLeft` | vim: delete left | — | Deletes character to the left. | `shift-x` (vim_mode == normal) [vim] |
| `vim::DeleteRight` | vim: delete right | — | Deletes character to the right. | `delete` (vim_mode == normal) [vim]; `x` (vim_mode == normal) [vim] |
| `vim::DeleteToEndOfLine` | vim: delete to end of line | — | Deletes from cursor to end of line. | `shift-d` ((vim_mode == normal \|\| vim_mode == helix_normal) && !menu) [vim] |
| `vim::DoubleQuotes` | vim: double quotes | — | Selects text within double quotes. | `"` (vim_operator == a \|\| vim_operator == i \|\| vim_operator == cs \|\| vim_operator == helix_next \|\| vim_operator == helix_previous) [vim] |
| `vim::Down` | vim: down | `{ display_lines?: boolean = false }` | Moves cursor down by the specified number of lines. | `down` (VimControl && !menu) [vim]; `ctrl-j` (VimControl && !menu) [vim]; `j` (VimControl && !menu) [vim]; `g j` {"display_lines": true} (VimControl && !menu) [vim]; `g down` {"display_lines": true} (VimControl && !menu) [vim]; +4 more |
| `vim::EndOfDocument` | vim: end of document | — | Moves to the end of the document. | `shift-g` (VimControl && !menu) [vim]; `g e` ((vim_mode == helix_normal \|\| vim_mode == helix_select) && !menu) [vim] |
| `vim::EndOfLine` | vim: end of line | `{ display_lines?: boolean = false }` | Moves to the end of the current line. | `end` (VimControl && !menu) [vim]; `$` (VimControl && !menu) [vim]; `g $` {"display_lines": true} (VimControl && !menu) [vim]; `g end` {"display_lines": true} (VimControl && !menu) [vim]; `g l` ((vim_mode == helix_normal \|\| vim_mode == helix_select) && !menu) [vim] |
| `vim::EndOfLineDownward` | vim: end of line downward | — | Moves to the end of a line downward. | `g _` (VimControl && !menu) [vim] |
| `vim::EndOfParagraph` | vim: end of paragraph | — | Moves to the end of the paragraph. | `}` (VimControl && !menu) [vim] |
| `vim::EndRepeat` | vim: end repeat | — | Ends the repeat recording. |  |
| `vim::Enter` | vim: enter | — | Inserts a newline. | `enter` (vim_mode == replace) [vim]; `enter` (vim_mode == waiting) [vim] |
| `vim::EntireFile` | vim: entire file | — | Selects the entire file. | `e` (vim_operator == a \|\| vim_operator == i \|\| vim_operator == cs \|\| vim_operator == helix_next \|\| vim_operator == helix_previous) [vim] |
| `vim::Exchange` | vim: exchange | — | Exchanges text regions. | `x` (vim_operator == c) [vim] |
| `vim::FindCommand` | vim: find command | `{ query: string, backwards: boolean }` | Executes a find command to search for patterns in the buffer. |  |
| `vim::FirstNonWhitespace` | vim: first non whitespace | `{ display_lines?: boolean = false }` | Moves to the first non-whitespace character on the current line. | `^` (VimControl && !menu) [vim]; `g ^` {"display_lines": true} (VimControl && !menu) [vim]; `g s` ((vim_mode == helix_normal \|\| vim_mode == helix_select) && !menu) [vim] |
| `vim::GoToColumn` | vim: go to column | — | Goes to a specific column number. | `\|` (VimControl && !menu) [vim] |
| `vim::GoToNextReference` | vim: go to next reference | — | Goes to the next reference to the symbol under the cursor. | `] r` (vim_mode == normal \|\| vim_mode == visual \|\| vim_mode == operator) [vim] |
| `vim::GoToPercentage` | vim: go to percentage | — | Goes to a percentage position in the file. | `%` (VimControl && VimCount) [vim] |
| `vim::GoToPreviousReference` | vim: go to previous reference | — | Goes to the previous reference to the symbol under the cursor. | `[ r` (vim_mode == normal \|\| vim_mode == visual \|\| vim_mode == operator) [vim] |
| `vim::GoToPreviousTab` | vim: go to previous tab | — | Go to previous tab page (with count support). | `g shift-t` (VimControl && !menu \|\| !Editor && !Terminal) [vim] |
| `vim::GoToTab` | vim: go to tab | — | Go to tab page (with count support). | `g t` (VimControl && !menu \|\| !Editor && !Terminal) [vim] |
| `vim::HalfPageLeft` | vim: half page left | — | Scrolls left by half a page's width. | `z shift-h` (VimControl && !menu) [vim] |
| `vim::HalfPageRight` | vim: half page right | — | Scrolls right by half a page's width. | `z shift-l` (VimControl && !menu) [vim] |
| `vim::HelixAppend` | vim: helix append | — | Appends at the end of the selection. | `a` (vim_mode == helix_select) [vim]; `a` (VimControl && vim_mode == helix_normal && !menu && !BufferSearchBar) [vim] |
| `vim::HelixCollapseSelection` | vim: helix collapse selection | — | Collapse the current selection | `;` (vim_mode == helix_select) [vim]; `;` ((vim_mode == helix_normal \|\| vim_mode == helix_select) && !menu) [vim] |
| `vim::HelixDelete` | vim: helix delete | — | Deletes using Helix-style behavior. | `d` ((vim_mode == helix_normal \|\| vim_mode == helix_select) && !menu) [vim] |
| `vim::HelixDuplicateAbove` | vim: helix duplicate above | — | Copies all selections above. | `alt-shift-c` ((vim_mode == helix_normal \|\| vim_mode == helix_select) && !menu) [vim] |
| `vim::HelixDuplicateBelow` | vim: helix duplicate below | — | Copies all selections below. | `shift-c` ((vim_mode == helix_normal \|\| vim_mode == helix_select) && !menu) [vim] |
| `vim::HelixGotoLastModification` | vim: helix goto last modification | — | Goes to the location of the last modification. | `g .` ((vim_mode == helix_normal \|\| vim_mode == helix_select) && !menu) [vim] |
| `vim::HelixGotoLine` | vim: helix goto line | — | Goes to the line specified by the count. | `shift-g` ((vim_mode == helix_normal \|\| vim_mode == helix_select) && !menu) [vim] |
| `vim::HelixInsert` | vim: helix insert | — | Inserts at the beginning of the selection. | `i` (VimControl && vim_mode == helix_normal && !menu && !BufferSearchBar) [vim] |
| `vim::HelixInsertEndOfLine` | vim: helix insert end of line | — | Inserts at the end of the current Helix cursor line. | `shift-a` (VimControl && vim_mode == helix_normal && !menu && !BufferSearchBar) [vim] |
| `vim::HelixJumpToWord` | vim: helix jump to word | — | Activate Helix-style word jump labels. | `g w` ((vim_mode == helix_normal \|\| vim_mode == helix_select) && !menu) [vim] |
| `vim::HelixKeepNewestSelection` | vim: helix keep newest selection | — | Removes all but the one selection that was created last. `Newest` can eventually be `Primary`. | `,` ((vim_mode == helix_normal \|\| vim_mode == helix_select) && !menu) [vim] |
| `vim::HelixPaste` | vim: helix paste | `{ before?: boolean = false }` | Pastes text from the specified register at the cursor position. | `p` ((vim_mode == helix_normal \|\| vim_mode == helix_select) && !menu) [vim]; `shift-p` {"before": true} ((vim_mode == helix_normal \|\| vim_mode == helix_select) && !menu) [vim] |
| `vim::HelixSelectLine` | vim: helix select line | — | Select entire line or multiple lines, extending downwards. | `x` ((vim_mode == helix_normal \|\| vim_mode == helix_select) && !menu) [vim] |
| `vim::HelixSelectNext` | vim: helix select next | — | Select the next match for the current search query. | `n` ((vim_mode == helix_normal \|\| vim_mode == helix_select) && !menu) [vim] |
| `vim::HelixSelectPrevious` | vim: helix select previous | — | Select the previous match for the current search query. | `shift-n` ((vim_mode == helix_normal \|\| vim_mode == helix_select) && !menu) [vim] |
| `vim::HelixSelectRegex` | vim: helix select regex | — | Select all matches of a given pattern within the current selection. | `s` ((vim_mode == helix_normal \|\| vim_mode == helix_select) && !menu) [vim] |
| `vim::HelixSubstitute` | vim: helix substitute | — | Delete the selection and enter edit mode. | `c` ((vim_mode == helix_normal \|\| vim_mode == helix_select) && !menu) [vim] |
| `vim::HelixSubstituteNoYank` | vim: helix substitute no yank | — | Delete the selection and enter edit mode, without yanking the selection. | `alt-c` ((vim_mode == helix_normal \|\| vim_mode == helix_select) && !menu) [vim] |
| `vim::HelixTrimSelections` | vim: helix trim selections | — | Trim leading and trailing whitespace from each selection. Originally-empty selections (cursors) are dropped before trimming. | `_` ((vim_mode == helix_normal \|\| vim_mode == helix_select) && !menu) [vim] |
| `vim::HelixYank` | vim: helix yank | — | Yanks the current selection or character if no selection. | `y` ((vim_mode == helix_normal \|\| vim_mode == helix_select) && !menu) [vim] |
| `vim::Increment` | vim: increment | `{ step?: boolean = false }` | Increments the number under the cursor or toggles boolean values. | `ctrl-a` (vim_mode == normal) [vim]; `ctrl-a` (vim_mode == visual) [vim]; `g ctrl-a` {"step": true} (vim_mode == visual) [vim]; `ctrl-a` (vim_mode == helix_select) [vim]; `ctrl-a` ((vim_mode == normal \|\| vim_mode == helix_normal) && !menu) [vim] |
| `vim::Indent` | vim: indent | — | Increases indentation of selected lines. | `>` (vim_mode == visual) [vim]; `ctrl-t` (vim_mode == insert) [vim]; `>` ((vim_mode == helix_normal \|\| vim_mode == helix_select) && !menu) [vim] |
| `vim::IndentObj` | vim: indent obj | `{ include_below?: boolean = false }` | Selects text at the same indentation level. | `i` (vim_operator == a \|\| vim_operator == i \|\| vim_operator == cs \|\| vim_operator == helix_next \|\| vim_operator == helix_previous) [vim]; `shift-i` {"include_below": true} (vim_operator == a \|\| vim_operator == i \|\| vim_operator == cs \|\| vim_operator == helix_next \|\| vim_operator == helix_previous) [vim] |
| `vim::InnerObject` | vim: inner object | — | Selects inner text object. |  |
| `vim::InsertAfter` | vim: insert after | — | Inserts text after the cursor. | `a` (vim_mode == normal) [vim]; `shift-a` (vim_mode == visual) [vim] |
| `vim::InsertAtPrevious` | vim: insert at previous | — | Inserts at the previous insert position. | `g i` (VimControl && !menu) [vim] |
| `vim::InsertBefore` | vim: insert before | — | Inserts text before the cursor. | `i` (vim_mode == normal) [vim]; `insert` (vim_mode == normal) [vim]; `shift-i` (vim_mode == visual) [vim]; `i` (vim_mode == helix_select) [vim]; `insert` ((vim_mode == helix_normal \|\| vim_mode == helix_select) && !menu) [vim]; +1 more |
| `vim::InsertEmptyLineAbove` | vim: insert empty line above | — | Inserts an empty line above without entering insert mode. | `[ space` (vim_mode == normal \|\| vim_mode == visual \|\| vim_mode == operator) [vim]; `space` (vim_operator == helix_previous) [vim] |
| `vim::InsertEmptyLineBelow` | vim: insert empty line below | — | Inserts an empty line below without entering insert mode. | `] space` (vim_mode == normal \|\| vim_mode == visual \|\| vim_mode == operator) [vim]; `space` (vim_operator == helix_next) [vim] |
| `vim::InsertEndOfLine` | vim: insert end of line | — | Inserts at the end of the line. | `shift-a` ((vim_mode == normal \|\| vim_mode == helix_normal) && !menu) [vim] |
| `vim::InsertFirstNonWhitespace` | vim: insert first non whitespace | — | Inserts at the first non-whitespace character. | `shift-i` ((vim_mode == normal \|\| vim_mode == helix_normal) && !menu) [vim] |
| `vim::InsertFromAbove` | vim: insert from above | — | Inserts the next character from the line above into the current line. | `ctrl-y` (vim_mode == insert) [vim] |
| `vim::InsertFromBelow` | vim: insert from below | — | Inserts the next character from the line below into the current line. | `ctrl-e` (vim_mode == insert) [vim] |
| `vim::InsertLineAbove` | vim: insert line above | — | Inserts a new line above the current line. | `shift-o` (vim_mode == helix_select) [vim]; `shift-o` ((vim_mode == normal \|\| vim_mode == helix_normal) && !menu) [vim] |
| `vim::InsertLineBelow` | vim: insert line below | — | Inserts a new line below the current line. | `o` (vim_mode == helix_select) [vim]; `o` ((vim_mode == normal \|\| vim_mode == helix_normal) && !menu) [vim] |
| `vim::JoinLines` | vim: join lines | — | Joins the current line with the next line. | `shift-j` (vim_mode == visual) [vim]; `shift-j` (vim_mode == helix_select) [vim]; `shift-j` ((vim_mode == normal \|\| vim_mode == helix_normal) && !menu) [vim] |
| `vim::JoinLinesNoWhitespace` | vim: join lines no whitespace | — | Joins lines without adding whitespace. | `g shift-j` (vim_mode == normal) [vim]; `g shift-j` (vim_mode == visual) [vim] |
| `vim::Left` | vim: left | — | Moves cursor left one character. | `left` (VimControl && !menu) [vim]; `h` (VimControl && !menu) [vim] |
| `vim::LineDown` | vim: line down | — | Scrolls down by one line. | `ctrl-e` (VimControl && !menu) [vim]; `ctrl-x ctrl-e` (vim_mode == insert) [vim]; `ctrl-e` (showing_completions) [vim] |
| `vim::LineUp` | vim: line up | — | Scrolls up by one line. | `ctrl-y` (VimControl && !menu) [vim]; `ctrl-x ctrl-y` (vim_mode == insert) [vim]; `ctrl-y` (showing_completions) [vim] |
| `vim::Literal` | vim: literal | `[any, …]` |  | `ctrl-@` ["ctrl-@", "\u0000"] (vim_mode == literal) [vim]; `ctrl-a` ["ctrl-a", "\u0001"] (vim_mode == literal) [vim]; `ctrl-b` ["ctrl-b", "\u0002"] (vim_mode == literal) [vim]; `ctrl-c` ["ctrl-c", "\u0003"] (vim_mode == literal) [vim]; `ctrl-d` ["ctrl-d", "\u0004"] (vim_mode == literal) [vim]; +32 more |
| `vim::Matching` | vim: matching | `{ match_quotes?: boolean = false }` | Moves to the matching bracket or delimiter. Argument: `match_quotes`: Whether to include quote characters (`'`, `"`, `` ` ``) when searching for matching pairs. When `false`, only brackets and parentheses are matched, which aligns with Neovim's default `%` behavior. | `%` {"match_quotes": true} (VimControl && !menu) [vim]; `m` {"match_quotes": true} (vim_operator == helix_m) [vim] |
| `vim::MaximizePane` | vim: maximize pane | — | Maximizes the current pane. | `ctrl-w _` (VimControl && !menu \|\| !Editor && !Terminal) [vim] |
| `vim::MenuSelectNext` | vim: menu select next | — | Selects (count) next menu item | `j` (ProjectPanel && not_editing) [vim]; `down` (ProjectPanel && not_editing) [vim]; `j` (OutlinePanel && not_editing) [vim]; `down` (OutlinePanel && not_editing) [vim]; `j` (GitGraph && !GitGraphSearchBar) [vim] |
| `vim::MenuSelectPrevious` | vim: menu select previous | — | Selects (count) previous menu item | `k` (ProjectPanel && not_editing) [vim]; `up` (ProjectPanel && not_editing) [vim]; `k` (OutlinePanel && not_editing) [vim]; `up` (OutlinePanel && not_editing) [vim]; `k` (GitGraph && !GitGraphSearchBar) [vim] |
| `vim::Method` | vim: method | — | Selects a method or function. | `f` (vim_operator == a \|\| vim_operator == i \|\| vim_operator == cs \|\| vim_operator == helix_next \|\| vim_operator == helix_previous) [vim] |
| `vim::MiddleOfLine` | vim: middle of line | `{ display_lines?: boolean = false }` | Moves to the middle of the current line. | `g shift-m` {"display_lines": true} (VimControl && !menu) [vim] |
| `vim::MiniBrackets` | vim: mini brackets | — | Selects text within the nearest brackets. |  |
| `vim::MiniQuotes` | vim: mini quotes | — | Selects text within the nearest quotes (single or double). | `q` (vim_operator == a \|\| vim_operator == i \|\| vim_operator == cs \|\| vim_operator == helix_next \|\| vim_operator == helix_previous) [vim] |
| `vim::MoveToNext` | vim: move to next | `{ case_sensitive?: boolean = true, partial_word?: boolean = false, regex?: boolean = true }` | Moves to the next search match. | `*` (VimControl && !menu) [vim]; `g *` {"partial_word": true} (VimControl && !menu) [vim]; `*` {"partial_word": true} (vim_mode == visual) [vim] |
| `vim::MoveToNextMatch` | vim: move to next match | — | Moves to the next search match. | `n` (VimControl && !menu) [vim] |
| `vim::MoveToPrevious` | vim: move to previous | `{ case_sensitive?: boolean = true, partial_word?: boolean = false, regex?: boolean = true }` | Moves to the previous search match. | `#` (VimControl && !menu) [vim]; `g #` {"partial_word": true} (VimControl && !menu) [vim]; `#` {"partial_word": true} (vim_mode == visual) [vim] |
| `vim::MoveToPreviousMatch` | vim: move to previous match | — | Moves to the previous search match. | `shift-n` (VimControl && !menu) [vim] |
| `vim::NextComment` | vim: next comment | — | Moves to the next comment. | `] *` (vim_mode == normal \|\| vim_mode == visual \|\| vim_mode == operator) [vim]; `] /` (vim_mode == normal \|\| vim_mode == visual \|\| vim_mode == operator) [vim]; `*` (vim_operator == helix_next) [vim]; `/` (vim_operator == helix_next) [vim]; `c` (vim_operator == helix_next) [vim] |
| `vim::NextGreaterIndent` | vim: next greater indent | — | Moves to the next line with greater indentation. | `] +` (vim_mode == normal \|\| vim_mode == visual \|\| vim_mode == operator) [vim]; `+` (vim_operator == helix_next) [vim] |
| `vim::NextLesserIndent` | vim: next lesser indent | — | Moves to the next line with lesser indentation. | `] -` (vim_mode == normal \|\| vim_mode == visual \|\| vim_mode == operator) [vim]; `-` (vim_operator == helix_next) [vim] |
| `vim::NextLineStart` | vim: next line start | — | Moves to the start of the next line. | `ctrl-m` (VimControl && !menu) [vim]; `+` (VimControl && !menu) [vim]; `enter` (VimControl && !menu) [vim] |
| `vim::NextMethodEnd` | vim: next method end | — | Moves to the end of the next method. | `] shift-m` (vim_mode == normal \|\| vim_mode == visual \|\| vim_mode == operator) [vim] |
| `vim::NextMethodStart` | vim: next method start | — | Moves to the start of the next method. | `] m` (vim_mode == normal \|\| vim_mode == visual \|\| vim_mode == operator) [vim] |
| `vim::NextSameIndent` | vim: next same indent | — | Moves to the next line with the same indentation. | `] =` (vim_mode == normal \|\| vim_mode == visual \|\| vim_mode == operator) [vim]; `=` (vim_operator == helix_next) [vim] |
| `vim::NextSectionEnd` | vim: next section end | — | Moves to the end of the next section. | `] [` (vim_mode == normal \|\| vim_mode == visual \|\| vim_mode == operator) [vim]; `shift-z` (vim_operator == helix_next) [vim] |
| `vim::NextSectionStart` | vim: next section start | — | Moves to the start of the next section. | `] ]` (vim_mode == normal \|\| vim_mode == visual \|\| vim_mode == operator) [vim]; `z` (vim_operator == helix_next) [vim] |
| `vim::NextSubwordEnd` | vim: next subword end | `{ ignore_punctuation?: boolean = false }` | Moves to the end of the next subword. |  |
| `vim::NextSubwordStart` | vim: next subword start | `{ ignore_punctuation?: boolean = false }` | Moves to the start of the next subword. |  |
| `vim::NextWordEnd` | vim: next word end | `{ ignore_punctuation?: boolean = false }` | Moves to the end of the next word. | `e` (VimControl && !menu) [vim]; `shift-e` {"ignore_punctuation": true} (VimControl && !menu) [vim] |
| `vim::NextWordStart` | vim: next word start | `{ ignore_punctuation?: boolean = false }` | Moves to the start of the next word. | `w` (VimControl && !menu) [vim]; `shift-w` {"ignore_punctuation": true} (VimControl && !menu) [vim] |
| `vim::NormalBefore` | vim: normal before | — | Switches to normal mode with cursor positioned before the current character. | `v` (vim_mode == helix_select) [vim]; `ctrl-c` (vim_mode == insert) [vim]; `ctrl-[` (vim_mode == insert) [vim]; `escape` (vim_mode == insert) [vim]; `ctrl-c` (vim_mode == replace) [vim]; +2 more |
| `vim::Number` | vim: number | `integer ≥ 0` | Number is used to manage vim's count. Pushing a digit multiplies the current value by 10 and adds the digit. | `1` 1 (VimControl && !menu) [vim]; `2` 2 (VimControl && !menu) [vim]; `3` 3 (VimControl && !menu) [vim]; `4` 4 (VimControl && !menu) [vim]; `5` 5 (VimControl && !menu) [vim]; +25 more |
| `vim::OpenDefaultKeymap` | vim: open default keymap | — | Opens the default keymap file. |  |
| `vim::OtherEnd` | vim: other end | — | Moves cursor to the other end of the selection. | `shift-o` (vim_mode == visual) [vim]; `alt-;` ((vim_mode == helix_normal \|\| vim_mode == helix_select) && !menu) [vim] |
| `vim::OtherEndRowAware` | vim: other end row aware | — | Moves cursor to the other end of the selection (row-aware). | `o` (vim_mode == visual) [vim] |
| `vim::Outdent` | vim: outdent | — | Decreases indentation of selected lines. | `<` (vim_mode == visual) [vim]; `ctrl-d` (vim_mode == insert) [vim]; `<` ((vim_mode == helix_normal \|\| vim_mode == helix_select) && !menu) [vim] |
| `vim::PageDown` | vim: page down | — | Scrolls down by one page. | `ctrl-f` (VimControl && !menu) [vim]; `pagedown` (VimControl && !menu) [vim] |
| `vim::PageUp` | vim: page up | — | Scrolls up by one page. | `ctrl-b` (VimControl && !menu) [vim]; `pageup` (VimControl && !menu) [vim] |
| `vim::Paragraph` | vim: paragraph | — | Selects a paragraph text object. | `p` (vim_operator == a \|\| vim_operator == i \|\| vim_operator == cs \|\| vim_operator == helix_next \|\| vim_operator == helix_previous) [vim] |
| `vim::Parentheses` | vim: parentheses | `{ opening?: boolean = false }` | Operates on text within or around parentheses `()`. | `(` {"opening": true} (vim_operator == a \|\| vim_operator == i \|\| vim_operator == cs \|\| vim_operator == helix_next \|\| vim_operator == helix_previous) [vim]; `)` (vim_operator == a \|\| vim_operator == i \|\| vim_operator == cs \|\| vim_operator == helix_next \|\| vim_operator == helix_previous) [vim]; `b` (vim_operator == a \|\| vim_operator == i \|\| vim_operator == cs \|\| vim_operator == helix_next \|\| vim_operator == helix_previous) [vim] |
| `vim::Paste` | vim: paste | `{ before?: boolean = false, preserve_clipboard?: boolean = false }` | Pastes text from the specified register at the cursor position. | `p` (vim_mode == visual) [vim]; `shift-p` {"preserve_clipboard": true} (vim_mode == visual) [vim]; `g shift-r` {"preserve_clipboard": true} (vim_mode == visual) [vim]; `p` (vim_mode == helix_select) [vim]; `p` ((vim_mode == normal \|\| vim_mode == helix_normal) && !menu) [vim]; +1 more |
| `vim::PreviousComment` | vim: previous comment | — | Moves to the previous comment. | `[ *` (vim_mode == normal \|\| vim_mode == visual \|\| vim_mode == operator) [vim]; `[ /` (vim_mode == normal \|\| vim_mode == visual \|\| vim_mode == operator) [vim]; `*` (vim_operator == helix_previous) [vim]; `/` (vim_operator == helix_previous) [vim]; `c` (vim_operator == helix_previous) [vim] |
| `vim::PreviousGreaterIndent` | vim: previous greater indent | — | Moves to the previous line with greater indentation. | `[ +` (vim_mode == normal \|\| vim_mode == visual \|\| vim_mode == operator) [vim]; `+` (vim_operator == helix_previous) [vim] |
| `vim::PreviousLesserIndent` | vim: previous lesser indent | — | Moves to the previous line with lesser indentation. | `[ -` (vim_mode == normal \|\| vim_mode == visual \|\| vim_mode == operator) [vim]; `-` (vim_operator == helix_previous) [vim] |
| `vim::PreviousLineStart` | vim: previous line start | — | Moves to the start of the previous line. | `-` (VimControl && !menu) [vim] |
| `vim::PreviousMethodEnd` | vim: previous method end | — | Moves to the end of the previous method. | `[ shift-m` (vim_mode == normal \|\| vim_mode == visual \|\| vim_mode == operator) [vim] |
| `vim::PreviousMethodStart` | vim: previous method start | — | Moves to the start of the previous method. | `[ m` (vim_mode == normal \|\| vim_mode == visual \|\| vim_mode == operator) [vim] |
| `vim::PreviousSameIndent` | vim: previous same indent | — | Moves to the previous line with the same indentation. | `[ =` (vim_mode == normal \|\| vim_mode == visual \|\| vim_mode == operator) [vim]; `=` (vim_operator == helix_previous) [vim] |
| `vim::PreviousSectionEnd` | vim: previous section end | — | Moves to the end of the previous section. | `[ ]` (vim_mode == normal \|\| vim_mode == visual \|\| vim_mode == operator) [vim]; `shift-z` (vim_operator == helix_previous) [vim] |
| `vim::PreviousSectionStart` | vim: previous section start | — | Moves to the start of the previous section. | `[ [` (vim_mode == normal \|\| vim_mode == visual \|\| vim_mode == operator) [vim]; `z` (vim_operator == helix_previous) [vim] |
| `vim::PreviousSubwordEnd` | vim: previous subword end | `{ ignore_punctuation?: boolean = false }` | Moves to the end of the previous subword. |  |
| `vim::PreviousSubwordStart` | vim: previous subword start | `{ ignore_punctuation?: boolean = false }` | Moves to the start of the previous subword. |  |
| `vim::PreviousWordEnd` | vim: previous word end | `{ ignore_punctuation?: boolean = false }` | Moves to the end of the previous word. | `g e` (VimControl && !menu) [vim]; `g shift-e` {"ignore_punctuation": true} (VimControl && !menu) [vim] |
| `vim::PreviousWordStart` | vim: previous word start | `{ ignore_punctuation?: boolean = false }` | Moves to the start of the previous word. | `b` (VimControl && !menu) [vim]; `shift-b` {"ignore_punctuation": true} (VimControl && !menu) [vim] |
| `vim::PushAddSurrounds` | vim: push add surrounds | — |  | `s` {} (vim_operator == y) [vim] |
| `vim::PushAutoIndent` | vim: push auto indent | — | Starts an auto-indent operation. | `=` (vim_mode == normal) [vim] |
| `vim::PushChange` | vim: push change | — | Starts a change operation. | `c` (vim_mode == normal) [vim] |
| `vim::PushChangeSurrounds` | vim: push change surrounds | `{ target?: Object \| null }` |  | `s` {} (vim_operator == c) [vim] |
| `vim::PushDelete` | vim: push delete | — | Starts a delete operation. | `d` (vim_mode == normal) [vim] |
| `vim::PushDeleteSurrounds` | vim: push delete surrounds | — | Deletes surrounding characters. | `s` (vim_operator == d) [vim] |
| `vim::PushDigraph` | vim: push digraph | `{ first_char?: string \| null }` |  | `ctrl-k` {} (vim_mode == insert) [vim]; `ctrl-k` {} (vim_mode == replace) [vim]; `ctrl-k` {} (vim_mode == waiting) [vim] |
| `vim::PushFindBackward` | vim: push find backward | `{ after: boolean, multiline: boolean }` |  | `shift-f` {"after": false, "multiline": false} (VimControl && !menu) [vim]; `shift-t` {"after": true, "multiline": false} (VimControl && !menu) [vim]; `shift-t` {"after": true, "multiline": true} ((vim_mode == helix_normal \|\| vim_mode == helix_select) && !menu) [vim]; `shift-f` {"after": false, "multiline": true} ((vim_mode == helix_normal \|\| vim_mode == helix_select) && !menu) [vim] |
| `vim::PushFindForward` | vim: push find forward | `{ before: boolean, multiline: boolean }` |  | `f` {"before": false, "multiline": false} (VimControl && !menu) [vim]; `t` {"before": true, "multiline": false} (VimControl && !menu) [vim]; `t` {"before": true, "multiline": true} ((vim_mode == helix_normal \|\| vim_mode == helix_select) && !menu) [vim]; `f` {"before": false, "multiline": true} ((vim_mode == helix_normal \|\| vim_mode == helix_select) && !menu) [vim] |
| `vim::PushForcedMotion` | vim: push forced motion | — | Starts a forced motion. | `v` (vim_operator == d) [vim]; `v` (vim_operator == y) [vim] |
| `vim::PushHelixMatch` | vim: push helix match | — | Starts a match operation. | `m` ((vim_mode == helix_normal \|\| vim_mode == helix_select) && !menu) [vim] |
| `vim::PushHelixNext` | vim: push helix next | `{ around: boolean }` | Selects the next object. | `]` {"around": true} ((vim_mode == helix_normal \|\| vim_mode == helix_select) && !menu) [vim] |
| `vim::PushHelixPrevious` | vim: push helix previous | `{ around: boolean }` | Selects the previous object. | `[` {"around": true} ((vim_mode == helix_normal \|\| vim_mode == helix_select) && !menu) [vim] |
| `vim::PushHelixSurroundAdd` | vim: push helix surround add | — | Adds surrounding characters in Helix mode. | `s` (vim_operator == helix_m) [vim] |
| `vim::PushHelixSurroundDelete` | vim: push helix surround delete | — | Deletes surrounding characters in Helix mode. | `d` (vim_operator == helix_m) [vim] |
| `vim::PushHelixSurroundReplace` | vim: push helix surround replace | — | Replaces surrounding characters in Helix mode. | `r` (vim_operator == helix_m) [vim] |
| `vim::PushIndent` | vim: push indent | — | Starts an indent operation. | `>` (vim_mode == normal) [vim] |
| `vim::PushJump` | vim: push jump | `{ line: boolean }` |  | `'` {"line": true} (VimControl && !menu) [vim]; ``` {"line": false} (VimControl && !menu) [vim] |
| `vim::PushLiteral` | vim: push literal | `{ prefix?: string \| null }` |  | `ctrl-v` {} (vim_mode == insert) [vim]; `ctrl-q` {} (vim_mode == insert) [vim]; `ctrl-shift-q` {} (vim_mode == insert) [vim]; `ctrl-v` {} (vim_mode == replace) [vim]; `ctrl-q` {} (vim_mode == replace) [vim]; +3 more |
| `vim::PushLowercase` | vim: push lowercase | — | Converts to lowercase. | `g u` (vim_mode == normal) [vim] |
| `vim::PushMark` | vim: push mark | — | Sets a mark at the current position. | `m` (VimControl && !menu) [vim] |
| `vim::PushObject` | vim: push object | `{ around: boolean }` |  | `i` {"around": false} (VimControl && !menu) [vim]; `a` {"around": true} (VimControl && !menu) [vim]; `i` {"around": false} (vim_mode == visual) [vim]; `a` {"around": true} (vim_mode == visual) [vim] |
| `vim::PushOppositeCase` | vim: push opposite case | — | Toggles case. | `g ~` (vim_mode == normal) [vim] |
| `vim::PushOutdent` | vim: push outdent | — | Starts an outdent operation. | `<` (vim_mode == normal) [vim] |
| `vim::PushRecordRegister` | vim: push record register | — | Starts recording to a register. |  |
| `vim::PushRegister` | vim: push register | — | Selects a register. | `"` (vim_mode == visual) [vim]; `ctrl-r` (vim_mode == insert) [vim]; `"` ((vim_mode == normal \|\| vim_mode == helix_normal) && !menu) [vim] |
| `vim::PushReplace` | vim: push replace | — | Starts a replace operation. | `r` (vim_mode == visual) [vim]; `r` (vim_mode == helix_select) [vim]; `r` ((vim_mode == normal \|\| vim_mode == helix_normal) && !menu) [vim] |
| `vim::PushReplaceWithRegister` | vim: push replace with register | — | Replaces with register contents. | `g shift-r` (VimControl && !menu) [vim] |
| `vim::PushReplayRegister` | vim: push replay register | — | Replays a register. | `@` (VimControl && !menu) [vim] |
| `vim::PushRewrap` | vim: push rewrap | — | Starts a rewrap operation. | `g w` (vim_mode == normal) [vim]; `g q` (vim_mode == normal) [vim]; `g q` ((vim_mode == helix_normal \|\| vim_mode == helix_select) && !menu) [vim] |
| `vim::PushRot13` | vim: push rot13 | — | Applies ROT13 encoding. | `g ?` (vim_mode == normal) [vim] |
| `vim::PushRot47` | vim: push rot47 | — | Applies ROT47 encoding. |  |
| `vim::PushShellCommand` | vim: push shell command | — | Starts a shell command operation. | `!` (vim_mode == normal) [vim] |
| `vim::PushSneak` | vim: push sneak | `{ first_char?: string \| null }` |  |  |
| `vim::PushSneakBackward` | vim: push sneak backward | `{ first_char?: string \| null }` |  |  |
| `vim::PushToggleBlockComments` | vim: push toggle block comments | — | Toggles block comments. | `g b` (vim_mode == normal) [vim] |
| `vim::PushToggleComments` | vim: push toggle comments | — | Toggles comments. | `g c` (vim_mode == normal) [vim] |
| `vim::PushUppercase` | vim: push uppercase | — | Converts to uppercase. | `g shift-u` (vim_mode == normal) [vim] |
| `vim::PushYank` | vim: push yank | — | Starts a yank operation. | `y` (vim_mode == normal) [vim] |
| `vim::Quotes` | vim: quotes | — | Selects text within single quotes. | `'` (vim_operator == a \|\| vim_operator == i \|\| vim_operator == cs \|\| vim_operator == helix_next \|\| vim_operator == helix_previous) [vim] |
| `vim::Redo` | vim: redo | — | Redoes the last undone change. | `ctrl-r` (vim_mode == normal) [vim]; `ctrl-r` ((vim_mode == helix_normal \|\| vim_mode == helix_select) && !menu) [vim] |
| `vim::Repeat` | vim: repeat | — | Repeats the last change. | `.` (VimControl && !menu) [vim]; `.` (vim_mode == helix_select) [vim] |
| `vim::RepeatFind` | vim: repeat find | — | Repeats the last character find. | `;` (VimControl && !menu) [vim]; `alt-.` (vim_mode == helix_select) [vim]; `alt-.` ((vim_mode == helix_normal \|\| vim_mode == helix_select) && !menu) [vim] |
| `vim::RepeatFindReversed` | vim: repeat find reversed | — | Repeats the last character find in reverse. | `,` (VimControl && !menu) [vim] |
| `vim::ReplayLastRecording` | vim: replay last recording | — | Replays the last recorded macro. | `shift-q` (VimControl && !menu) [vim]; `q` ((vim_mode == helix_normal \|\| vim_mode == helix_select) && !menu) [vim] |
| `vim::ResetPaneSizes` | vim: reset pane sizes | — | Resets all pane sizes to default. | `ctrl-w =` (VimControl && !menu \|\| !Editor && !Terminal) [vim] |
| `vim::ResizePaneDown` | vim: resize pane down | — | Resizes the pane downward. | `ctrl-w -` (VimControl && !menu \|\| !Editor && !Terminal) [vim] |
| `vim::ResizePaneLeft` | vim: resize pane left | — | Resizes the pane to the left. | `ctrl-w <` (VimControl && !menu \|\| !Editor && !Terminal) [vim] |
| `vim::ResizePaneRight` | vim: resize pane right | — | Resizes the pane to the right. | `ctrl-w >` (VimControl && !menu \|\| !Editor && !Terminal) [vim] |
| `vim::ResizePaneUp` | vim: resize pane up | — | Resizes the pane upward. | `ctrl-w +` (VimControl && !menu \|\| !Editor && !Terminal) [vim] |
| `vim::RestoreVisualSelection` | vim: restore visual selection | — | Restores the previous visual selection. | `g v` (VimControl && !menu) [vim] |
| `vim::Rewrap` | vim: rewrap | `{ line_length?: integer ≥ 0 \| null }` | Rewraps the selected text to fit within the line width. | `g q` (vim_mode == visual) [vim]; `g w` (vim_mode == visual) [vim] |
| `vim::Right` | vim: right | — | Moves cursor right one character. | `right` (VimControl && !menu) [vim]; `l` (VimControl && !menu) [vim] |
| `vim::ScrollDown` | vim: scroll down | — | Scrolls down by half a page. | `ctrl-d` (VimControl && !menu) [vim]; `ctrl-d` (showing_completions) [vim] |
| `vim::ScrollUp` | vim: scroll up | — | Scrolls up by half a page. | `ctrl-u` (VimControl && !menu) [vim]; `ctrl-u` (showing_completions) [vim] |
| `vim::Search` | vim: search | `{ backwards?: boolean = false, regex?: boolean = true }` | Initiates a search operation with the specified parameters. | `/` (VimControl && !menu) [vim]; `?` {"backwards": true} (VimControl && !menu) [vim] |
| `vim::SearchSubmit` | vim: search submit | — | Submits the current search query. | `enter` (BufferSearchBar && !in_replace) [vim] |
| `vim::SearchUnderCursor` | vim: search under cursor | `{ case_sensitive?: boolean = true, partial_word?: boolean = false, regex?: boolean = true }` | Searches for the word under the cursor without moving. |  |
| `vim::SearchUnderCursorPrevious` | vim: search under cursor previous | `{ case_sensitive?: boolean = true, partial_word?: boolean = false, regex?: boolean = true }` | Searches for the word under the cursor without moving (backwards). |  |
| `vim::SelectLargerSyntaxNode` | vim: select larger syntax node | — | Selects the next larger syntax node. | `[ x` (vim_mode == normal \|\| vim_mode == visual \|\| vim_mode == operator) [vim] |
| `vim::SelectNext` | vim: select next | — | Selects the next occurrence of the current selection. | `g l` (VimControl && !menu) [vim] |
| `vim::SelectNextMatch` | vim: select next match | — | Selects the next match of the current selection. | `g n` (VimControl && !menu) [vim] |
| `vim::SelectNextSyntaxNode` | vim: select next syntax node | — | Selects the next syntax node sibling. |  |
| `vim::SelectPrevious` | vim: select previous | — | Selects the previous occurrence of the current selection. | `g shift-l` (VimControl && !menu) [vim] |
| `vim::SelectPreviousMatch` | vim: select previous match | — | Selects the previous match of the current selection. | `g shift-n` (VimControl && !menu) [vim] |
| `vim::SelectPreviousSyntaxNode` | vim: select previous syntax node | — | Selects the previous syntax node sibling. |  |
| `vim::SelectRegister` | vim: select register | `string` |  |  |
| `vim::SelectSmallerSyntaxNode` | vim: select smaller syntax node | — | Selects the next smaller syntax node. | `] x` (vim_mode == normal \|\| vim_mode == visual \|\| vim_mode == operator) [vim] |
| `vim::Sentence` | vim: sentence | — | Selects a sentence text object. | `s` (vim_operator == a \|\| vim_operator == i \|\| vim_operator == cs \|\| vim_operator == helix_next \|\| vim_operator == helix_previous) [vim] |
| `vim::SentenceBackward` | vim: sentence backward | — | Moves to the start of the previous sentence. | `(` (VimControl && !menu) [vim] |
| `vim::SentenceForward` | vim: sentence forward | — | Moves to the start of the next sentence. | `)` (VimControl && !menu) [vim] |
| `vim::ShellCommand` | vim: shell command | — | Executes a shell command. | `!` (vim_mode == visual) [vim] |
| `vim::ShowLocation` | vim: show location | — | Shows the current location in the file. | `ctrl-g` (VimControl && !menu) [vim] |
| `vim::SquareBrackets` | vim: square brackets | `{ opening?: boolean = false }` | Operates on text within or around square brackets `[]`. | `[` {"opening": true} (vim_operator == a \|\| vim_operator == i \|\| vim_operator == cs \|\| vim_operator == helix_next \|\| vim_operator == helix_previous) [vim]; `]` (vim_operator == a \|\| vim_operator == i \|\| vim_operator == cs \|\| vim_operator == helix_next \|\| vim_operator == helix_previous) [vim]; `r` (vim_operator == a \|\| vim_operator == i \|\| vim_operator == cs \|\| vim_operator == helix_next \|\| vim_operator == helix_previous) [vim] |
| `vim::StartOfDocument` | vim: start of document | — | Moves to the start of the document. | `g g` (VimControl && !menu) [vim] |
| `vim::StartOfLine` | vim: start of line | `{ display_lines?: boolean = false }` | Moves to the start of the current line. | `0` (VimControl && !menu) [vim]; `home` (VimControl && !menu) [vim]; `g 0` {"display_lines": true} (VimControl && !menu) [vim]; `g home` {"display_lines": true} (VimControl && !menu) [vim]; `g h` ((vim_mode == helix_normal \|\| vim_mode == helix_select) && !menu) [vim] |
| `vim::StartOfLineDownward` | vim: start of line downward | — | Moves to the start of a line downward. | `_` (VimControl && !menu) [vim] |
| `vim::StartOfParagraph` | vim: start of paragraph | — | Moves to the start of the paragraph. | `{` (VimControl && !menu) [vim] |
| `vim::Substitute` | vim: substitute | — | Substitutes characters in the current selection. | `c` (vim_mode == visual) [vim]; `s` (vim_mode == visual) [vim]; `s` (vim_mode == helix_select) [vim]; `s` ((vim_mode == normal \|\| vim_mode == helix_normal) && !menu) [vim] |
| `vim::SubstituteLine` | vim: substitute line | — | Substitutes the entire line. | `shift-r` (vim_mode == visual) [vim]; `shift-s` (vim_mode == visual) [vim]; `shift-s` ((vim_mode == normal \|\| vim_mode == helix_normal) && !menu) [vim] |
| `vim::Subword` | vim: subword | `{ ignore_punctuation?: boolean = false }` | Selects a subword text object. |  |
| `vim::SwitchToHelixNormalMode` | vim: switch to helix normal mode | — | Switches to Helix-style normal mode. | `escape` (VimControl && vim_mode == helix_normal && !menu && !BufferSearchBar) [vim]; `escape` (vim_mode == helix_select && !menu && !BufferSearchBar) [vim] |
| `vim::SwitchToInsertMode` | vim: switch to insert mode | — | Switches to insert mode. | `/` (ThreadsSidebar > Editor && VimControl && vim_mode == normal) [vim] |
| `vim::SwitchToNormalMode` | vim: switch to normal mode | — | Switches to normal mode. | `escape` (VimControl && !menu) [vim]; `ctrl-[` (VimControl && !menu) [vim]; `ctrl-c` (vim_mode == visual) [vim]; `ctrl-[` (vim_mode == visual) [vim]; `escape` (vim_mode == visual) [vim]; +1 more |
| `vim::SwitchToReplaceMode` | vim: switch to replace mode | — | Switches to replace mode. |  |
| `vim::SwitchToVisualBlockMode` | vim: switch to visual block mode | — | Switches to visual block mode. |  |
| `vim::SwitchToVisualLineMode` | vim: switch to visual line mode | — | Switches to visual line mode. |  |
| `vim::SwitchToVisualMode` | vim: switch to visual mode | — | Switches to visual mode. |  |
| `vim::Tab` | vim: tab | — | Inserts a tab character. | `shift-tab` (VimControl && !menu) [vim]; `tab` (VimControl && !menu) [vim]; `tab` (vim_mode == replace) [vim]; `tab` (vim_mode == waiting) [vim] |
| `vim::Tag` | vim: tag | — | Selects an HTML/XML tag. | `t` (vim_operator == a \|\| vim_operator == i \|\| vim_operator == cs \|\| vim_operator == helix_next \|\| vim_operator == helix_previous) [vim]; `x` ((vim_operator == a \|\| vim_operator == i \|\| vim_operator == helix_next \|\| vim_operator == helix_previous) && helix_mode) [vim] |
| `vim::TemporaryNormal` | vim: temporary normal | — | Temporarily switches to normal mode for one command. | `ctrl-o` (vim_mode == insert) [vim] |
| `vim::ToggleBlockComments` | vim: toggle block comments | — | Toggles block comments for selected lines. | `g b` (vim_mode == visual) [vim] |
| `vim::ToggleComments` | vim: toggle comments | — | Toggles comments for selected lines. | `g c` (vim_mode == visual) [vim] |
| `vim::ToggleMarksView` | vim: toggle marks view | — | Toggles the marks view. |  |
| `vim::ToggleProjectPanelFocus` | vim: toggle project panel focus | — | Clears count or toggles project panel focus | `escape` (ProjectPanel && not_editing) [vim] |
| `vim::ToggleRecord` | vim: toggle record | — | Toggles macro recording. | `q` (VimControl && !menu) [vim]; `shift-q` ((vim_mode == helix_normal \|\| vim_mode == helix_select) && !menu) [vim] |
| `vim::ToggleRegistersView` | vim: toggle registers view | — | Toggles the registers view. |  |
| `vim::ToggleReplace` | vim: toggle replace | — | Toggles replace mode. | `shift-r` (VimControl && !menu) [vim]; `insert` (vim_mode == insert) [vim] |
| `vim::ToggleVisual` | vim: toggle visual | — | Toggles visual mode. | `v` (VimControl && !menu) [vim] |
| `vim::ToggleVisualBlock` | vim: toggle visual block | — | Toggles visual block mode. | `ctrl-v` (VimControl && !menu) [vim]; `ctrl-q` (VimControl && !menu) [vim] |
| `vim::ToggleVisualLine` | vim: toggle visual line | — | Toggles visual line mode. | `shift-v` (VimControl && !menu) [vim] |
| `vim::Undo` | vim: undo | — | Undoes the last change. | `u` (vim_mode == helix_select) [vim]; `u` ((vim_mode == normal \|\| vim_mode == helix_normal) && !menu) [vim] |
| `vim::UndoLastLine` | vim: undo last line | — | Undoes all changes to the most recently changed line. | `shift-u` ((vim_mode == normal \|\| vim_mode == helix_normal) && !menu) [vim] |
| `vim::UndoReplace` | vim: undo replace | — | Undoes the last replacement. | `backspace` (vim_mode == replace) [vim] |
| `vim::UnmatchedBackward` | vim: unmatched backward | `{ char?: string = "\u0000" }` | Finds the previous unmatched bracket or delimiter. | `[ {` {"char": "{"} (vim_mode == normal \|\| vim_mode == visual \|\| vim_mode == operator) [vim]; `[ (` {"char": "("} (vim_mode == normal \|\| vim_mode == visual \|\| vim_mode == operator) [vim] |
| `vim::UnmatchedForward` | vim: unmatched forward | `{ char?: string = "\u0000" }` | Finds the next unmatched bracket or delimiter. | `] }` {"char": "}"} (vim_mode == normal \|\| vim_mode == visual \|\| vim_mode == operator) [vim]; `] )` {"char": ")"} (vim_mode == normal \|\| vim_mode == visual \|\| vim_mode == operator) [vim] |
| `vim::Up` | vim: up | `{ display_lines?: boolean = false }` | Moves cursor up by the specified number of lines. | `up` (VimControl && !menu) [vim]; `k` (VimControl && !menu) [vim]; `g k` {"display_lines": true} (VimControl && !menu) [vim]; `g up` {"display_lines": true} (VimControl && !menu) [vim]; `k` {"display_lines": true} (VimControl && vim_mode == helix_normal && !menu && !BufferSearchBar) [vim]; +3 more |
| `vim::VerticalBars` | vim: vertical bars | — | Selects text within vertical bars (pipes). | `\|` (vim_operator == a \|\| vim_operator == i \|\| vim_operator == cs \|\| vim_operator == helix_next \|\| vim_operator == helix_previous) [vim] |
| `vim::VisualCommand` | vim: visual command | — | Executes a command in visual mode. | `:` (vim_mode == visual) [vim] |
| `vim::VisualDelete` | vim: visual delete | — | Deletes the visual selection. | `d` (vim_mode == visual) [vim]; `x` (vim_mode == visual) [vim]; `delete` (vim_mode == visual) [vim] |
| `vim::VisualDeleteLine` | vim: visual delete line | — | Deletes entire lines in visual selection. | `shift-d` (vim_mode == visual) [vim]; `shift-x` (vim_mode == visual) [vim] |
| `vim::VisualInsertEndOfLine` | vim: visual insert end of line | — | Inserts at the end of each line in visual selection. | `g shift-a` (vim_mode == visual) [vim] |
| `vim::VisualInsertFirstNonWhiteSpace` | vim: visual insert first non white space | — | Inserts at the first non-whitespace character of each line. | `g shift-i` (vim_mode == visual) [vim] |
| `vim::VisualYank` | vim: visual yank | — | Yanks (copies) the visual selection. | `y` (vim_mode == visual) [vim] |
| `vim::VisualYankLine` | vim: visual yank line | — | Yanks entire lines in visual selection. | `shift-y` (vim_mode == visual) [vim] |
| `vim::WindowBottom` | vim: window bottom | — | Moves to the bottom of the window. | `shift-l` (VimControl && !menu) [vim]; `g b` ((vim_mode == helix_normal \|\| vim_mode == helix_select) && !menu) [vim] |
| `vim::WindowMiddle` | vim: window middle | — | Moves to the middle of the window. | `shift-m` (VimControl && !menu) [vim]; `g c` ((vim_mode == helix_normal \|\| vim_mode == helix_select) && !menu) [vim] |
| `vim::WindowTop` | vim: window top | — | Moves to the top of the window. | `shift-h` (VimControl && !menu) [vim]; `g t` ((vim_mode == helix_normal \|\| vim_mode == helix_select) && !menu) [vim] |
| `vim::Word` | vim: word | `{ ignore_punctuation?: boolean = false }` | Selects a word text object. | `w` (vim_operator == a \|\| vim_operator == i \|\| vim_operator == cs \|\| vim_operator == helix_next \|\| vim_operator == helix_previous) [vim]; `shift-w` {"ignore_punctuation": true} (vim_operator == a \|\| vim_operator == i \|\| vim_operator == cs \|\| vim_operator == helix_next \|\| vim_operator == helix_previous) [vim] |
| `vim::WrappingLeft` | vim: wrapping left | — | Moves cursor left one character, wrapping to previous line. Aliases: `vim::Backspace` | `backspace` (VimControl && !menu) [vim]; `h` ((vim_mode == helix_normal \|\| vim_mode == helix_select) && !menu) [vim]; `left` ((vim_mode == helix_normal \|\| vim_mode == helix_select) && !menu) [vim] |
| `vim::WrappingRight` | vim: wrapping right | — | Moves cursor right one character, wrapping to next line. Aliases: `vim::Space` | `space` (VimControl && !menu) [vim]; `l` ((vim_mode == helix_normal \|\| vim_mode == helix_select) && !menu) [vim]; `right` ((vim_mode == helix_normal \|\| vim_mode == helix_select) && !menu) [vim] |
| `vim::Yank` | vim: yank | — | Yanks (copies) the selected text. |  |
| `vim::YankLine` | vim: yank line | — | Yanks the entire line. | `shift-y` (vim_mode == normal) [vim]; `shift-y` ((vim_mode == normal \|\| vim_mode == helix_normal) && !menu) [vim] |
| `vim::YankToEndOfLine` | vim: yank to end of line | — | Yanks from cursor to end of line. |  |

## welcome

| Action | Palette name | Argument | Description | Default keys |
|---|---|---|---|---|
| `welcome::OpenRecentProject` | welcome: open recent project | `integer ≥ 0` |  | `cmd-1` 0 (Welcome); `cmd-2` 1 (Welcome); `cmd-3` 2 (Welcome); `cmd-4` 3 (Welcome); `cmd-5` 4 (Welcome) |

## window

| Action | Palette name | Argument | Description | Default keys |
|---|---|---|---|---|
| `window::MergeAllWindows` | window: merge all windows | — |  |  |
| `window::MoveTabToNewWindow` | window: move tab to new window | — |  |  |
| `window::ShowNextWindowTab` | window: show next window tab | — |  |  |
| `window::ShowPreviousWindowTab` | window: show previous window tab | — |  |  |

## workspace

| Action | Palette name | Argument | Description | Default keys |
|---|---|---|---|---|
| `workspace::ActivateLastPane` | workspace: activate last pane | — | Activates the last pane in the workspace. |  |
| `workspace::ActivateNextPane` | workspace: activate next pane | — | Activates the next pane in the workspace. | `ctrl-w w` (VimControl && !menu \|\| !Editor && !Terminal) [vim]; `ctrl-w ctrl-w` (VimControl && !menu \|\| !Editor && !Terminal) [vim] |
| `workspace::ActivateNextWindow` | workspace: activate next window | — | Switches to the next window. |  |
| `workspace::ActivatePane` | workspace: activate pane | `integer ≥ 0` | Activates a specific pane by its index. | `cmd-1` 0 (Workspace); `cmd-2` 1 (Workspace); `cmd-3` 2 (Workspace); `cmd-4` 3 (Workspace); `cmd-5` 4 (Workspace); +4 more |
| `workspace::ActivatePaneDown` | workspace: activate pane down | — | Activates the pane below. | `cmd-k cmd-down` (Workspace); `space w j` ((vim_mode == helix_normal \|\| vim_mode == helix_select) && !menu) [vim]; `ctrl-w down` (VimControl && !menu \|\| !Editor && !Terminal) [vim]; `ctrl-w ctrl-j` (VimControl && !menu \|\| !Editor && !Terminal) [vim]; `ctrl-w j` (VimControl && !menu \|\| !Editor && !Terminal) [vim]; +1 more |
| `workspace::ActivatePaneLeft` | workspace: activate pane left | — | Activates the pane to the left. | `cmd-k cmd-left` (Workspace); `space w h` ((vim_mode == helix_normal \|\| vim_mode == helix_select) && !menu) [vim]; `ctrl-w left` (VimControl && !menu \|\| !Editor && !Terminal) [vim]; `ctrl-w ctrl-h` (VimControl && !menu \|\| !Editor && !Terminal) [vim]; `ctrl-w h` (VimControl && !menu \|\| !Editor && !Terminal) [vim]; +1 more |
| `workspace::ActivatePaneRight` | workspace: activate pane right | — | Activates the pane to the right. | `cmd-k cmd-right` (Workspace); `space w l` ((vim_mode == helix_normal \|\| vim_mode == helix_select) && !menu) [vim]; `ctrl-w right` (VimControl && !menu \|\| !Editor && !Terminal) [vim]; `ctrl-w ctrl-l` (VimControl && !menu \|\| !Editor && !Terminal) [vim]; `ctrl-w l` (VimControl && !menu \|\| !Editor && !Terminal) [vim]; +1 more |
| `workspace::ActivatePaneUp` | workspace: activate pane up | — | Activates the pane above. | `cmd-k cmd-up` (Workspace); `space w k` ((vim_mode == helix_normal \|\| vim_mode == helix_select) && !menu) [vim]; `ctrl-w up` (VimControl && !menu \|\| !Editor && !Terminal) [vim]; `ctrl-w ctrl-k` (VimControl && !menu \|\| !Editor && !Terminal) [vim]; `ctrl-w k` (VimControl && !menu \|\| !Editor && !Terminal) [vim]; +1 more |
| `workspace::ActivatePreviousPane` | workspace: activate previous pane | — | Activates the previous pane in the workspace. | `ctrl-w p` (VimControl && !menu \|\| !Editor && !Terminal) [vim]; `ctrl-w ctrl-p` (VimControl && !menu \|\| !Editor && !Terminal) [vim]; `ctrl-w shift-w` (VimControl && !menu \|\| !Editor && !Terminal) [vim]; `ctrl-w ctrl-shift-w` (VimControl && !menu \|\| !Editor && !Terminal) [vim] |
| `workspace::ActivatePreviousWindow` | workspace: activate previous window | — | Switches to the previous window. |  |
| `workspace::AddFolderToProject` | workspace: add folder to project | — | Adds a folder to the current project. | `cmd-shift-a` (RecentProjects \|\| (RecentProjects > Picker > Editor)) |
| `workspace::ClearAllNotifications` | workspace: clear all notifications | — | Clears all notifications. |  |
| `workspace::ClearBookmarks` | workspace: clear bookmarks | — | Clears all bookmarks in the project. |  |
| `workspace::ClearNavigationHistory` | workspace: clear navigation history | — | Clears all navigation history, including forward/backward navigation, recently opened files, and recently closed tabs. **This action is irreversible**. |  |
| `workspace::ClearTrustedWorktrees` | workspace: clear trusted worktrees | — | Clears all trusted worktrees, placing them in restricted mode on next open. Requires restart to take effect on already opened projects. |  |
| `workspace::CloseActiveDock` | workspace: close active dock | — | Closes the active dock. | `cmd-w` (Workspace) |
| `workspace::CloseAllDocks` | workspace: close all docks | — | Closes all docks. |  |
| `workspace::CloseAllItemsAndPanes` | workspace: close all items and panes | `{ save_intent?: SaveIntent \| null }` | Closes all items and panes in the workspace. | `cmd-k cmd-w` (Pane) |
| `workspace::CloseInactiveTabsAndPanes` | workspace: close inactive tabs and panes | `{ save_intent?: SaveIntent \| null }` | Closes all inactive tabs and panes in the workspace. | `ctrl-alt-cmd-w` (Pane); `ctrl-w ctrl-o` (VimControl && !menu \|\| !Editor && !Terminal) [vim]; `ctrl-w o` (VimControl && !menu \|\| !Editor && !Terminal) [vim] |
| `workspace::CloseItemInAllPanes` | workspace: close item in all panes | `{ save_intent?: SaveIntent \| null, close_pinned?: boolean = false }` | Closes the active item across all panes. |  |
| `workspace::CloseProject` | workspace: close project | — | Closes the current project. |  |
| `workspace::CloseWindow` | workspace: close window | — | Closes the current window. | `cmd-shift-w`; `cmd-w` (SettingsWindow); `escape` (SettingsWindow); `cmd-w` (SkillCreator); `cmd-w` (SkillCreator > Editor) |
| `workspace::CopyPath` | workspace: copy path | — | Aliases: `editor::CopyPath`, `outline_panel::CopyPath`, `project_panel::CopyPath` | `cmd-alt-c` (OutlinePanel && not_editing); `cmd-alt-c` (ProjectPanel); `cmd-alt-c` (GitPanel && ChangesList && !GitBranchSelector && !GitRepositorySelector) |
| `workspace::CopyRelativePath` | workspace: copy relative path | — | Aliases: `editor::CopyRelativePath`, `outline_panel::CopyRelativePath`, `project_panel::CopyRelativePath` | `alt-cmd-shift-c` (OutlinePanel && not_editing); `alt-cmd-shift-c` (ProjectPanel); `alt-cmd-shift-c` (GitPanel && ChangesList && !GitBranchSelector && !GitRepositorySelector) |
| `workspace::DecreaseActiveDockSize` | workspace: decrease active dock size | `{ px?: integer ≥ 0 = 0 }` | Decreases size of a currently focused dock by a given amount of pixels. Argument: `px`: For 0px parameter, uses UI font size value. | `ctrl-alt--` {"px": 0} (Workspace) |
| `workspace::DecreaseOpenDocksSize` | workspace: decrease open docks size | `{ px?: integer ≥ 0 = 0 }` | Decreases size of all currently visible docks uniformly, by a given amount of pixels. Argument: `px`: For 0px parameter, uses UI font size value. | `ctrl-alt-_` {"px": 0} (Workspace) |
| `workspace::Feedback` | workspace: feedback | — | Opens the feedback dialog. |  |
| `workspace::FocusCenterPane` | workspace: focus center pane | — | Moves Focus to the central panes in the workspace. |  |
| `workspace::FocusNextPart` | workspace: focus next part | — | Moves focus to the next major region of the window (editor, open panels, status bar), cycling and wrapping around. Intended as a discoverable, screen-reader-friendly way to navigate the window. | `f6` (Workspace); `cmd-f6` (Workspace) |
| `workspace::FocusPreviousPart` | workspace: focus previous part | — | Moves focus to the previous major region of the window. See [`FocusNextPart`]. | `shift-f6` (Workspace) |
| `workspace::FollowNextCollaborator` | workspace: follow next collaborator | — | Follows the next collaborator in the session. | `ctrl-alt-cmd-f`; `[ f` (vim_mode == normal \|\| vim_mode == visual \|\| vim_mode == operator) [vim]; `] f` (vim_mode == normal \|\| vim_mode == visual \|\| vim_mode == operator) [vim] |
| `workspace::FormatAndSave` | workspace: format and save | — | Formats and saves the current file, regardless of the format_on_save setting. |  |
| `workspace::IncreaseActiveDockSize` | workspace: increase active dock size | `{ px?: integer ≥ 0 = 0 }` | Increases size of a currently focused dock by a given amount of pixels. Argument: `px`: For 0px parameter, uses UI font size value. | `ctrl-alt-=` {"px": 0} (Workspace) |
| `workspace::IncreaseOpenDocksSize` | workspace: increase open docks size | `{ px?: integer ≥ 0 = 0 }` | Increases size of all currently visible docks uniformly, by a given amount of pixels. Argument: `px`: For 0px parameter, uses UI font size value. | `ctrl-alt-+` {"px": 0} (Workspace) |
| `workspace::MoveFocusedPanelToNextPosition` | workspace: move focused panel to next position | — | Moves the focused panel to the next position. |  |
| `workspace::MoveItemToPane` | workspace: move item to pane | `{ destination?: integer ≥ 0 = 1, focus?: boolean = true, clone?: boolean = false }` | Moves an item to a specific pane by index. |  |
| `workspace::MoveItemToPaneInDirection` | workspace: move item to pane in direction | `{ direction?: SplitDirection, focus?: boolean = true, clone?: boolean = false }` | Moves an item to a pane in the specified direction. |  |
| `workspace::MovePaneDown` | workspace: move pane down | — | Move the current pane to be at the very bottom. | `ctrl-w shift-j` (VimControl && !menu \|\| !Editor && !Terminal) [vim] |
| `workspace::MovePaneLeft` | workspace: move pane left | — | Move the current pane to be at the far left. | `ctrl-w shift-h` (VimControl && !menu \|\| !Editor && !Terminal) [vim] |
| `workspace::MovePaneRight` | workspace: move pane right | — | Move the current pane to be at the far right. | `ctrl-w shift-l` (VimControl && !menu \|\| !Editor && !Terminal) [vim] |
| `workspace::MovePaneUp` | workspace: move pane up | — | Move the current pane to be at the very top. | `ctrl-w shift-k` (VimControl && !menu \|\| !Editor && !Terminal) [vim] |
| `workspace::NewCenterTerminal` | workspace: new center terminal | `{ local?: boolean = false }` | Opens a new terminal in the center. Argument: `local`: If true, creates a local terminal even in remote projects. |  |
| `workspace::NewFile` | workspace: new file | — | Creates a new file. | `cmd-n` (Workspace && !Terminal); `cmd-n` (Welcome) |
| `workspace::NewFileSplit` | workspace: new file split | `SplitDirection` | Creates a new file in a split of the desired direction. |  |
| `workspace::NewFileSplitHorizontal` | workspace: new file split horizontal | — | Creates a new file in a horizontal split. | `ctrl-w ctrl-n` (VimControl && !menu \|\| !Editor && !Terminal) [vim]; `ctrl-w n` (VimControl && !menu \|\| !Editor && !Terminal) [vim] |
| `workspace::NewFileSplitVertical` | workspace: new file split vertical | — | Creates a new file in a vertical split. |  |
| `workspace::NewSearch` | workspace: new search | — | Opens a new search. |  |
| `workspace::NewTerminal` | workspace: new terminal | `{ local?: boolean = false }` | Opens a new terminal. Argument: `local`: If true, creates a local terminal even in remote projects. | `ctrl-~` (Workspace); `cmd-n` (Terminal) |
| `workspace::NewWindow` | workspace: new window | — | Opens a new window. | `cmd-shift-n` (Workspace) |
| `workspace::Open` | workspace: open | `{ create_new_window?: boolean \| null }` | Opens a file or directory. Argument: `create_new_window`: When true, opens in a new window. When false, adds to the current window as a new workspace (multi-workspace). When omitted, uses `default_open_behavior`. | `cmd-o` |
| `workspace::OpenComponentPreview` | workspace: open component preview | — | Opens the component preview. |  |
| `workspace::OpenFiles` | workspace: open files | — | Opens multiple files. |  |
| `workspace::OpenInTerminal` | workspace: open in terminal | — | Opens the current location in terminal. |  |
| `workspace::OpenTerminal` | workspace: open terminal | `{ working_directory: string, local?: boolean = false }` | Opens a new terminal with the specified working directory. Argument: `local`: If true, creates a local terminal even in remote projects. |  |
| `workspace::OpenWithSystem` | workspace: open with system | — | Opens the selected file with the system's default application. Aliases: `project_panel::OpenWithSystem` | `ctrl-shift-enter` (ProjectPanel); `ctrl-shift-enter` (InvalidBuffer); `s` (ProjectPanel && not_editing) [vim] |
| `workspace::Reload` | workspace: reload | — | Reloads the application |  |
| `workspace::ReloadActiveItem` | workspace: reload active item | — | Reloads the active item. |  |
| `workspace::ReopenLastPicker` | workspace: reopen last picker | — | Reopens the most recently dismissed picker in the current window. | `cmd-k cmd-p` (Workspace) |
| `workspace::ResetActiveDockSize` | workspace: reset active dock size | — | Resets the active dock to its default size. | `ctrl-alt-0` (Workspace) |
| `workspace::ResetOpenDocksSize` | workspace: reset open docks size | — | Resets all open docks to their default sizes. | `ctrl-alt-)` (Workspace) |
| `workspace::ResetPaneSizes` | workspace: reset pane sizes | — | Resets all panes in the center group to equal sizes, preserving the split layout. |  |
| `workspace::RestoreBanner` | workspace: restore banner | — | Restores the banner. |  |
| `workspace::Save` | workspace: save | `{ save_intent?: SaveIntent \| null }` | Saves the current file with the specified options. | `cmd-s` (Workspace) |
| `workspace::SaveAll` | workspace: save all | `{ save_intent?: SaveIntent \| null }` | Saves all open files in the workspace. | `cmd-alt-s` (Workspace) |
| `workspace::SaveAs` | workspace: save as | — | Saves the current file with a new name. | `cmd-shift-s` (Workspace) |
| `workspace::SaveWithoutFormat` | workspace: save without format | — | Saves without formatting. | `cmd-k s` (Workspace) |
| `workspace::SendKeystrokes` | workspace: send keystrokes | `string` | Sends a sequence of keystrokes to the active element. | `z enter` "z t ^" (VimControl && !menu) [vim]; `z -` "z b ^" (VimControl && !menu) [vim]; `z ^` "shift-h k z b ^" (VimControl && !menu) [vim]; `z +` "shift-l j z t ^" (VimControl && !menu) [vim]; `z .` "z z ^" (VimControl && !menu) [vim] |
| `workspace::ShutdownDebugAdapters` | workspace: shutdown debug adapters | — | Shuts down all debug adapters. |  |
| `workspace::SuppressNotification` | workspace: suppress notification | — | Suppresses the current notification. |  |
| `workspace::SwapPaneAdjacent` | workspace: swap pane adjacent | — |  | `ctrl-w x` (VimControl && !menu \|\| !Editor && !Terminal) [vim]; `ctrl-w ctrl-x` (VimControl && !menu \|\| !Editor && !Terminal) [vim] |
| `workspace::SwapPaneDown` | workspace: swap pane down | — | Swaps the current pane with the one below. | `cmd-k shift-down` (Workspace); `ctrl-w shift-down` (VimControl && !menu \|\| !Editor && !Terminal) [vim] |
| `workspace::SwapPaneLeft` | workspace: swap pane left | — | Swaps the current pane with the one to the left. | `cmd-k shift-left` (Workspace); `ctrl-w shift-left` (VimControl && !menu \|\| !Editor && !Terminal) [vim] |
| `workspace::SwapPaneRight` | workspace: swap pane right | — | Swaps the current pane with the one to the right. | `cmd-k shift-right` (Workspace); `ctrl-w shift-right` (VimControl && !menu \|\| !Editor && !Terminal) [vim] |
| `workspace::SwapPaneUp` | workspace: swap pane up | — | Swaps the current pane with the one above. | `cmd-k shift-up` (Workspace); `ctrl-w shift-up` (VimControl && !menu \|\| !Editor && !Terminal) [vim] |
| `workspace::ToggleAllDocks` | workspace: toggle all docks | — | Toggles all docks. | `alt-cmd-y` (Workspace) |
| `workspace::ToggleBottomDock` | workspace: toggle bottom dock | — | Toggles the bottom dock. | `cmd-j` (Workspace) |
| `workspace::ToggleCenteredLayout` | workspace: toggle centered layout | — | Toggles centered layout mode. |  |
| `workspace::ToggleEditPrediction` | workspace: toggle edit prediction | — | Toggles edit prediction feature globally for all files. |  |
| `workspace::ToggleEditorZoom` | workspace: toggle editor zoom | — | Toggles maximizing the active editor pane within the center area, hiding other split panes but leaving docks/panels unaffected. |  |
| `workspace::ToggleExpandItem` | workspace: toggle expand item | — | Toggles expansion of the selected item. |  |
| `workspace::ToggleHelixMode` | workspace: toggle helix mode | — | Toggles Helix mode on or off. |  |
| `workspace::ToggleLeftDock` | workspace: toggle left dock | — | Toggles the left dock. | `cmd-b` (Workspace) |
| `workspace::ToggleReadOnlyFile` | workspace: toggle read only file | — | Toggles read-only mode for the active item (if supported by that item). |  |
| `workspace::ToggleRightDock` | workspace: toggle right dock | — | Toggles the right dock. | `cmd-alt-b` (Workspace); `cmd-r` (Workspace) |
| `workspace::ToggleVimMode` | workspace: toggle vim mode | — | Toggles Vim mode on or off. |  |
| `workspace::ToggleWorktreeSecurity` | workspace: toggle worktree security | — | If any worktrees are in restricted mode, shows a modal with possible actions. If the modal is shown already, closes it without trusting any worktree. | `ctrl-cmd-s` |
| `workspace::ToggleZoom` | workspace: toggle zoom | — | Toggles zoom on the active pane. | `shift-escape` |
| `workspace::Unfollow` | workspace: unfollow | — | Stops following a collaborator. | `escape` (Workspace) |
| `workspace::UseAgenticLayout` | workspace: use agentic layout | — | Switches to the agentic panel layout. |  |
| `workspace::UseClassicLayout` | workspace: use classic layout | — | Switches to the classic, editor-focused panel layout. |  |
| `workspace::ZoomIn` | workspace: zoom in | — | Zooms in on the active pane. |  |
| `workspace::ZoomOut` | workspace: zoom out | — | Zooms out of the active pane. |  |

## worktree_picker

| Action | Palette name | Argument | Description | Default keys |
|---|---|---|---|---|
| `worktree_picker::DeleteWorktree` | worktree picker: delete worktree | — | Deletes the selected git worktree. | `cmd-shift-backspace` (WorktreePicker \|\| (WorktreePicker > Picker > Editor)) |
| `worktree_picker::ForceDeleteWorktree` | worktree picker: force delete worktree | — | Force deletes the selected git worktree. | `cmd-alt-shift-backspace` (WorktreePicker \|\| (WorktreePicker > Picker > Editor)) |

## zed

| Action | Palette name | Argument | Description | Default keys |
|---|---|---|---|---|
| `zed::About` | zed: about | — | Shows information about Zed. |  |
| `zed::AcpRegistry` | zed: acp registry | — | Opens the ACP registry. |  |
| `zed::CopyInstalledExtensionsIntoClipboard` | zed: copy installed extensions into clipboard | — | Copies installed extensions to the clipboard for bug reports. |  |
| `zed::CopySystemSpecsIntoClipboard` | zed: copy system specs into clipboard | — | Copies system specifications to the clipboard for bug reports. |  |
| `zed::DebugElements` | zed: debug elements | — | Opens the element inspector for debugging UI. |  |
| `zed::DecreaseBufferFontSize` | zed: decrease buffer font size | `{ persist?: boolean = false }` | Decreases the font size in the editor buffer. | `cmd--` {"persist": false} |
| `zed::DecreaseUiFontSize` | zed: decrease ui font size | `{ persist?: boolean = false }` | Decreases the font size of the user interface. | `cmd--` {"persist": false} (Onboarding); `cmd--` {"persist": false} (Welcome) |
| `zed::Extensions` | zed: extensions | `{ category_filter?: ExtensionCategoryFilter \| null, id?: string \| null }` | Opens the extensions management interface. Argument: `category_filter`: Filters the extensions page down to extensions that are in the specified category. `id`: Focuses just the extension with the specified ID. | `cmd-shift-x` (Workspace) |
| `zed::GetMerch` | zed: get merch | — | Opens the Zed merch store. |  |
| `zed::Hide` | zed: hide | — | Hides the application window. | `cmd-h` |
| `zed::HideOthers` | zed: hide others | — | Hides all other application windows. | `alt-cmd-h` |
| `zed::ImportCursorSettings` | zed: import cursor settings | `{ skip_prompt?: boolean = false }` | Imports settings from Cursor editor. |  |
| `zed::ImportVsCodeSettings` | zed: import vs code settings | `{ skip_prompt?: boolean = false }` | Imports settings from Visual Studio Code. |  |
| `zed::IncreaseBufferFontSize` | zed: increase buffer font size | `{ persist?: boolean = false }` | Increases the font size in the editor buffer. | `cmd-=` {"persist": false}; `cmd-+` {"persist": false} |
| `zed::IncreaseUiFontSize` | zed: increase ui font size | `{ persist?: boolean = false }` | Increases the font size of the user interface. | `cmd-=` {"persist": false} (Onboarding); `cmd-+` {"persist": false} (Onboarding); `cmd-=` {"persist": false} (Welcome); `cmd-+` {"persist": false} (Welcome) |
| `zed::InstallDevExtension` | zed: install dev extension | — | Installs an extension from a local directory for development. |  |
| `zed::Minimize` | zed: minimize | — | Minimizes the current window. | `cmd-m` |
| `zed::NoAction` | zed: no action | — | Action with special handling which unbinds the keybinding this is associated with, if it is the highest precedence match. |  |
| `zed::OpenAccountSettings` | zed: open account settings | — | Opens account settings. |  |
| `zed::OpenBrowser` | zed: open browser | `{ url: string }` | Opens a URL in the system's default web browser. |  |
| `zed::OpenDebugTasks` | zed: open debug tasks | — | Opens debug tasks configuration. |  |
| `zed::OpenDefaultKeymap` | zed: open default keymap | — | Opens the default keymap file. |  |
| `zed::OpenDefaultSettings` | zed: open default settings | — | Opens the default settings file. |  |
| `zed::OpenDocs` | zed: open docs | — | Opens the documentation website. |  |
| `zed::OpenKeymap` | zed: open keymap | — | Opens the keymap editor. Aliases: `zed_actions::OpenKeymapEditor` | `cmd-k cmd-s` (Workspace) |
| `zed::OpenKeymapFile` | zed: open keymap file | — | Opens the user keymap file. Aliases: `zed_actions::OpenKeymap` | `cmd-e` (KeymapEditor) |
| `zed::OpenLicenses` | zed: open licenses | — | Views open source licenses. |  |
| `zed::OpenLog` | zed: open log | — | Opens the Zed log file. |  |
| `zed::OpenOnboarding` | zed: open onboarding | — | Opens the onboarding view. |  |
| `zed::OpenPerformanceProfiler` | zed: open performance profiler | — | Opens the performance profiler. |  |
| `zed::OpenProjectSettings` | zed: open project settings | — | Opens project-specific settings. Aliases: `zed_actions::OpenProjectSettings` |  |
| `zed::OpenProjectSettingsFile` | zed: open project settings file | — | Opens project-specific settings file. |  |
| `zed::OpenProjectTasks` | zed: open project tasks | — | Opens the project tasks configuration. |  |
| `zed::OpenServerSettings` | zed: open server settings | — | Opens server settings. |  |
| `zed::OpenSettings` | zed: open settings | — | Opens the settings editor. Aliases: `zed_actions::OpenSettingsEditor` | `cmd-,` (!SettingsWindow) |
| `zed::OpenSettingsAt` | zed: open settings at | `{ path: string, target?: OpenSettingsAtTarget \| null }` | Opens the settings editor at a specific path. Argument: `path`: A path to a specific setting (e.g. `theme.mode`) `target`: The settings file to select before opening `path`. When omitted, the existing settings file selection is preserved. |  |
| `zed::OpenSettingsFile` | zed: open settings file | — | Opens the settings JSON file. Aliases: `zed_actions::OpenSettings` | `cmd-alt-,` |
| `zed::OpenSettingsPage` | zed: open settings page | `{ page: string, target?: OpenSettingsAtTarget \| null }` | Argument: `page`: A settings page title (e.g. `AI`). `target`: The settings file to select before opening `page`. When omitted, the existing settings file selection is preserved. |  |
| `zed::OpenStatusPage` | zed: open status page | — | Opens the Zed status page. |  |
| `zed::OpenTasks` | zed: open tasks | — | Opens the tasks panel. |  |
| `zed::OpenTelemetryLog` | zed: open telemetry log | — | Opens the telemetry log. |  |
| `zed::OpenWorktreeSetupTasks` | zed: open worktree setup tasks | — | Opens the project tasks configuration with worktree setup guidance. | `cmd-shift-c` (WorktreePicker \|\| (WorktreePicker > Picker > Editor)) |
| `zed::OpenZedRepo` | zed: open zed repo | — | Opens the Zed repository on GitHub. |  |
| `zed::OpenZedUrl` | zed: open zed url | `{ url: string }` | Opens a zed:// URL within the application. |  |
| `zed::Quit` | zed: quit | — | Quits the application. | `cmd-q` |
| `zed::RebuildDevExtension` | zed: rebuild dev extension | `{ extension_id?: string \| null }` | Rebuilds an installed dev extension. Argument: `extension_id`: The ID of the dev extension to rebuild. Default: opens a picker if multiple dev extensions are installed. |  |
| `zed::ReloadExtensions` | zed: reload extensions | — | Reloads all installed extensions. |  |
| `zed::ResetAllZoom` | zed: reset all zoom | `{ persist?: boolean = false }` | Resets all zoom levels (UI and buffer font sizes, including in the agent panel) to their default values. |  |
| `zed::ResetBufferFontSize` | zed: reset buffer font size | `{ persist?: boolean = false }` | Resets the buffer font size to the default value. | `cmd-0` {"persist": false} |
| `zed::ResetDatabase` | zed: reset database | — | Resets the application database. |  |
| `zed::ResetUiFontSize` | zed: reset ui font size | `{ persist?: boolean = false }` | Resets the UI font size to the default value. | `cmd-0` {"persist": false} (Onboarding); `cmd-0` {"persist": false} (Welcome) |
| `zed::RevealLogInFileManager` | zed: reveal log in file manager | — | Reveals the Zed log file in the system file manager. |  |
| `zed::ShowAll` | zed: show all | — | Shows all hidden windows. |  |
| `zed::ShowDefaultSemanticTokenRules` | zed: show default semantic token rules | — | Shows the default semantic token rules (read-only). |  |
| `zed::ShowUpdateNotification` | zed: show update notification | — | Shows the auto-update notification for testing. |  |
| `zed::ShowWelcome` | zed: show welcome | — | Show the Zed welcome screen |  |
| `zed::TestCrash` | zed: test crash | — | Triggers a hard crash for debugging. |  |
| `zed::TestPanic` | zed: test panic | — | Triggers a test panic for debugging. |  |
| `zed::ToggleBaseKeymapSelector` | zed: toggle base keymap selector | — | Toggles the base keymap selector modal. |  |
| `zed::ToggleFullScreen` | zed: toggle full screen | — | Toggles fullscreen mode. | `fn-f` (Workspace); `ctrl-cmd-f` (Workspace) |
| `zed::Unbind` | zed: unbind | `string` | Action with special handling which unbinds later bindings for the same keystrokes when they dispatch the named action, regardless of that action's context. In keymap JSON this is written as: `["zed::Unbind", "editor::NewLine"]` |  |
| `zed::Zoom` | zed: zoom | — | Zooms the window. |  |

## zed_predict_onboarding

| Action | Palette name | Argument | Description | Default keys |
|---|---|---|---|---|
| `zed_predict_onboarding::OpenZedPredictOnboarding` | zed predict onboarding: open zed predict onboarding | — | Opens the Zed Predict onboarding modal. |  |

## zeta

| Action | Palette name | Argument | Description | Default keys |
|---|---|---|---|---|
| `zeta::FocusPredictions` | zeta: focus predictions | — | Focuses on the completions list. | `escape` (RatePredictionsModal > Editor) |
| `zeta::NextEdit` | zeta: next edit | — | Navigates to the next edit in the completion history. | `shift-down` (RatePredictionsModal) |
| `zeta::PreviewPrediction` | zeta: preview prediction | — | Previews the selected completion. | `right` (RatePredictionsModal) |
| `zeta::PreviousEdit` | zeta: previous edit | — | Navigates to the previous edit in the completion history. | `shift-up` (RatePredictionsModal) |
| `zeta::ThumbsDownActivePrediction` | zeta: thumbs down active prediction | — | Rates the active completion with a thumbs down. | `cmd-shift-backspace` (RatePredictionsModal); `cmd-shift-backspace` (RatePredictionsModal > Editor) |
| `zeta::ThumbsUpActivePrediction` | zeta: thumbs up active prediction | — | Rates the active completion with a thumbs up. | `cmd-shift-enter` (RatePredictionsModal); `cmd-shift-enter` (RatePredictionsModal > Editor) |

## Argument types

Shared argument types referenced above. `oneOf` alternatives are listed one per line.

### RevealTarget

Where to spawn the task in the UI.

- `"center"` — In the central pane group, "main" editor area.
- `"dock"` — In the terminal dock, "regular" terminal items' place.

### NewWorktreeBranchTarget

Describes which ref to base a new git worktree on. The worktree is always created in a detached HEAD state; users can opt into creating a branch afterwards from the worktree itself.

- `{ kind: "current_branch" }` — Create a detached worktree from the current HEAD.
- `{ name: string, kind: "existing_branch" }` — Create a detached worktree at the tip of an existing branch.
- `{ remote_name: string, branch_name: string, kind: "remote_branch" }` — Create a detached worktree at the tip of a remote-tracking branch.

### OpenSettingsAtTarget

- `"user"`
- `{ project: { worktree_id: integer ≥ 0 } }`

### ExtensionCategoryFilter

- `"themes" | "icon_themes" | "languages" | "grammars" | "language_servers" | "context_servers" | "snippets" | "debug_adapters"`

### ConflictContent

A single merge conflict region extracted from a file.

- `{ file_path: string, conflict_text: string, ours_branch_name: string, theirs_branch_name: string }`

### SaveIntent

- `"save"` — write all files (even if unchanged) prompt before overwriting on-disk changes
- `"format_and_save"` — same as Save, but always formats regardless of the format_on_save setting
- `"save_without_format"` — same as Save, but without auto formatting
- `"save_all"` — write any files that have local changes prompt before overwriting on-disk changes
- `"save_as"` — always prompt for a new path
- `"close"` — prompt "you have unsaved changes" before writing
- `"overwrite"` — write all dirty files, don't prompt on conflict
- `"skip"` — skip all save-related behavior

### SplitDirection

- `"up" | "down" | "left" | "right"`

### SplitMode

- `"ClonePane"` — Clone the current pane.
- `"EmptyPane"` — Create an empty new pane.
- `"MovePane"` — Move the item into a new pane. This will map to nop if only one pane exists.

### Object

- `"sentence" | "paragraph" | "quotes" | "back_quotes" | "any_quotes" | "mini_quotes" | "double_quotes" | "vertical_bars" | "any_brackets" | "mini_brackets" | "any_pair" | "parentheses" | "square_brackets" | "curly_brackets" | "angle_brackets" | "argument" | "tag" | "method" | "class" | "comment" | "entire_file"`
- `{ word: { ignore_punctuation: boolean } }`
- `{ subword: { ignore_punctuation: boolean } }`
- `{ indent_obj: { include_below: boolean } }`

### KeymapAction

- `any` (any action name string or `[name, argument]` array)

### GoToDiagnosticSeverityFilter

Allows filtering diagnostics that should be moved to.

- `GoToDiagnosticSeverity` — Move to diagnostics of a specific severity.
- `{ min?: GoToDiagnosticSeverity, max?: GoToDiagnosticSeverity }` — Specify a range of severities to include.

### GoToDiagnosticSeverity

Determines the severity of the diagnostic that should be moved to.

- `"error"` — Errors
- `"warning"` — Warnings
- `"information"` — Information
- `"hint"` — Hints

### OpenResultsIn

Where to show LSP results that can contain multiple locations.

- `"multi_buffer"` — Open the results in a multibuffer.
- `"picker"` — Open the results in a filterable picker.

### RevealStrategy

What to do with the terminal pane and tab, after the command was started.

- `"always"` — Always show the task's pane, and focus the corresponding tab in it.
- `"no_focus"` — Always show the task's pane, add the task's tab in it, but don't focus it.
- `"never"` — Do not alter focus, but still add/reuse the task's tab in its pane.

### DataBreakpointAccessType

This enumeration defines all possible access types for data breakpoints.

- `"read" | "write" | "readWrite"`

### AgentProfileId

- `string`

### SessionId

A unique identifier for a conversation session between a client and agent. Sessions maintain their own context, conversation history, and state, allowing multiple independent interactions with the same agent. See protocol docs: [Session ID](https://agentclientprotocol.com/protocol/session-setup#session-id)

- `string`
