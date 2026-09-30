# Redundancy in Zed 1.21.0 default keymaps

Generated from `assets/keymaps/default-macos.json` and `assets/keymaps/vim.json` at tag `v1.21.0`. A *group* is one action (with the same argument) reachable by several keystrokes; a *pair* is one keystroke bound to the same action in more than one section.

## default-macos.json

### A. Same action, same context, several keystrokes: 91 groups, 104 extra bindings

| Context | Action | Keys |
|---|---|---|
| `(none)` | `menu::Cancel` | `cmd-escape`, `ctrl-c`, `ctrl-escape`, `escape` |
| `(none)` | `menu::SelectFirst` | `cmd-up`, `home`, `pageup`, `shift-pageup` |
| `(none)` | `menu::SelectLast` | `cmd-down`, `end`, `pagedown`, `shift-pagedown` |
| `(none)` | `menu::SelectNext` | `ctrl-n`, `down`, `tab` |
| `(none)` | `menu::SelectPrevious` | `ctrl-p`, `shift-tab`, `up` |
| `CommitEditor > Editor` | `git_panel::FocusChanges` | `alt-up`, `shift-tab`, `tab` |
| `Editor` | `editor::Backspace` | `backspace`, `ctrl-h`, `shift-backspace` |
| `Editor` | `editor::SelectToBeginningOfLine` {"stop_at_indent": true, "stop_at_soft_wraps": true} | `cmd-shift-left`, `ctrl-shift-a`, `shift-home` |
| `Editor` | `editor::SelectToEndOfLine` {"stop_at_soft_wraps": true} | `cmd-shift-right`, `ctrl-shift-e`, `shift-end` |
| `GitPanel && ChangesList && !GitBranchSelector && !GitRepositorySelector` | `git_panel::FocusEditor` | `alt-down`, `shift-tab`, `tab` |
| `(none)` | `menu::SecondaryConfirm` | `cmd-enter`, `ctrl-enter` |
| `(none)` | `zed::IncreaseBufferFontSize` {"persist": false} | `cmd-+`, `cmd-=` |
| `AcpThread` | `agent::ScrollOutputLineDown` | `ctrl-alt-down`, `down` |
| `AcpThread` | `agent::ScrollOutputLineUp` | `ctrl-alt-up`, `up` |
| `AcpThread` | `agent::ScrollOutputPageDown` | `ctrl-pagedown`, `pagedown` |
| `AcpThread` | `agent::ScrollOutputPageUp` | `ctrl-pageup`, `pageup` |
| `AcpThread` | `agent::ScrollOutputToBottom` | `ctrl-end`, `end` |
| `AcpThread` | `agent::ScrollOutputToNextMessage` | `ctrl-alt-pagedown`, `shift-pagedown` |
| `AcpThread` | `agent::ScrollOutputToPreviousMessage` | `ctrl-alt-pageup`, `shift-pageup` |
| `AcpThread` | `agent::ScrollOutputToTop` | `ctrl-home`, `home` |
| `AcpThread > Editor && end_of_input` | `agent::ScrollOutputPageDown` | `ctrl-pagedown`, `pagedown` |
| `AcpThread > Editor && start_of_input` | `agent::ScrollOutputPageUp` | `ctrl-pageup`, `pageup` |
| `AgentDiff` | `agent::Keep` | `cmd-alt-y`, `cmd-y` |
| `Editor` | `editor::Delete` | `ctrl-d`, `delete` |
| `Editor` | `editor::DeleteToNextSubwordEnd` | `ctrl-alt-d`, `ctrl-alt-delete` |
| `Editor` | `editor::DeleteToPreviousSubwordStart` | `ctrl-alt-backspace`, `ctrl-alt-h` |
| `Editor` | `editor::DeleteToPreviousWordStart` {"ignore_brackets": false, "ignore_newlines": false} | `alt-backspace`, `ctrl-w` |
| `Editor` | `editor::MoveDown` | `ctrl-n`, `down` |
| `Editor` | `editor::MoveLeft` | `ctrl-b`, `left` |
| `Editor` | `editor::MoveRight` | `ctrl-f`, `right` |
| `Editor` | `editor::MoveToBeginning` | `cmd-home`, `cmd-up` |
| `Editor` | `editor::MoveToBeginningOfLine` {"stop_at_indent": true, "stop_at_soft_wraps": true} | `cmd-left`, `home` |
| `Editor` | `editor::MoveToEnclosingBracket` | `cmd-\|`, `ctrl-m` |
| `Editor` | `editor::MoveToEnd` | `cmd-down`, `cmd-end` |
| `Editor` | `editor::MoveToEndOfLine` {"stop_at_soft_wraps": true} | `cmd-right`, `end` |
| `Editor` | `editor::MoveToNextSubwordEnd` | `ctrl-alt-f`, `ctrl-alt-right` |
| `Editor` | `editor::MoveToPreviousSubwordStart` | `ctrl-alt-b`, `ctrl-alt-left` |
| `Editor` | `editor::MoveUp` | `ctrl-p`, `up` |
| `Editor` | `editor::Rewrap` | `cmd-k cmd-q`, `cmd-k q` |
| `Editor` | `editor::SelectAllMatches` | `cmd-f2`, `cmd-shift-l` |
| `Editor` | `editor::SelectDown` | `ctrl-shift-n`, `shift-down` |
| `Editor` | `editor::SelectLargerSyntaxNode` | `cmd-ctrl-right`, `ctrl-shift-right` |
| `Editor` | `editor::SelectLeft` | `ctrl-shift-b`, `shift-left` |
| `Editor` | `editor::SelectRight` | `ctrl-shift-f`, `shift-right` |
| `Editor` | `editor::SelectSmallerSyntaxNode` | `cmd-ctrl-left`, `ctrl-shift-left` |
| `Editor` | `editor::SelectToNextSubwordEnd` | `ctrl-alt-shift-f`, `ctrl-alt-shift-right` |
| `Editor` | `editor::SelectToPreviousSubwordStart` | `ctrl-alt-shift-b`, `ctrl-alt-shift-left` |
| `Editor` | `editor::SelectUp` | `ctrl-shift-p`, `shift-up` |
| `Editor` | `editor::ToggleBlockComments` | `cmd-k cmd-/`, `shift-alt-a` |
| `Editor && (showing_code_actions \|\| showing_completions)` | `editor::ContextMenuNext` | `ctrl-n`, `down` |
| `Editor && (showing_code_actions \|\| showing_completions)` | `editor::ContextMenuPrevious` | `ctrl-p`, `up` |
| `Editor && editor_agent_diff` | `agent::Keep` | `cmd-alt-y`, `cmd-y` |
| `Editor && mode == auto_height` | `editor::Newline` | `ctrl-enter`, `shift-enter` |
| `Editor && mode == full` | `editor::Newline` | `enter`, `shift-enter` |
| `GitPanel && (ChangesList \|\| HistoryList) && !GitBranchSelector && !GitRepositorySelector` | `menu::SelectNext` | `down`, `shift-down` |
| `GitPanel && (ChangesList \|\| HistoryList) && !GitBranchSelector && !GitRepositorySelector` | `menu::SelectPrevious` | `shift-up`, `up` |
| `GitPanel && ChangesList && !GitBranchSelector && !GitRepositorySelector` | `git::RestoreFile` {"skip_prompt": false} | `backspace`, `delete` |
| `GitPanel && ChangesList && !GitBranchSelector && !GitRepositorySelector` | `git::RestoreFile` {"skip_prompt": true} | `cmd-backspace`, `cmd-delete` |
| `GitPanel && ChangesList && !GitBranchSelector && !GitRepositorySelector` | `git::ToggleStaged` | `cmd-alt-y`, `space` |
| `ImageViewer` | `image_viewer::ZoomIn` | `cmd-+`, `cmd-=` |
| `NotebookEditor && notebook_mode == command` | `notebook::DeleteCell` | `backspace`, `d d` |
| `Onboarding` | `zed::IncreaseUiFontSize` {"persist": false} | `cmd-+`, `cmd-=` |
| `Pane` | `pane::ActivateNextItem` | `alt-cmd-right`, `cmd-}` |
| `Pane` | `pane::ActivatePreviousItem` | `alt-cmd-left`, `cmd-{` |
| `ProjectPanel` | `project_panel::Delete` {"skip_prompt": false} | `cmd-alt-backspace`, `cmd-delete` |
| `ProjectPanel` | `project_panel::Rename` | `enter`, `f2` |
| `ProjectPanel` | `project_panel::Trash` {"skip_prompt": false} | `backspace`, `delete` |
| `ProjectSearchBar` | `search::ToggleRegex` | `alt-cmd-g`, `alt-cmd-x` |
| `ProjectSearchView` | `search::ToggleRegex` | `alt-cmd-g`, `alt-cmd-x` |
| `Prompt` | `menu::SelectNext` | `l`, `right` |
| `Prompt` | `menu::SelectPrevious` | `h`, `left` |
| `SettingsWindow` | `settings_editor::ToggleFocusNav` | `cmd-shift-e`, `left` |
| `SettingsWindow` | `workspace::CloseWindow` | `cmd-w`, `escape` |
| `SettingsWindow > NavigationMenu` | `settings_editor::FocusNextNavEntry` | `down`, `tab` |
| `SettingsWindow > NavigationMenu` | `settings_editor::FocusPreviousNavEntry` | `shift-tab`, `up` |
| `TabSwitcher` | `menu::SelectPrevious` | `ctrl-shift-tab`, `ctrl-up` |
| `Terminal` | `pane::SplitRight` | `cmd-d`, `ctrl-alt-right` |
| `Terminal` | `terminal::ScrollPageDown` | `cmd-down`, `shift-pagedown` |
| `Terminal` | `terminal::ScrollPageUp` | `cmd-up`, `shift-pageup` |
| `Terminal` | `terminal::ScrollToBottom` | `cmd-end`, `shift-end` |
| `Terminal` | `terminal::ScrollToTop` | `cmd-home`, `shift-home` |
| `Terminal` | `terminal::SendText` "\u001bb" | `alt-b`, `alt-left` |
| `Terminal` | `terminal::SendText` "\u001bf" | `alt-f`, `alt-right` |
| `VariableList` | `variable_list::RemoveWatch` | `backspace`, `delete` |
| `Welcome` | `zed::IncreaseUiFontSize` {"persist": false} | `cmd-+`, `cmd-=` |
| `Workspace` | `projects::OpenRecent` | `alt-cmd-o`, `ctrl-r` |
| `Workspace` | `workspace::FocusNextPart` | `cmd-f6`, `f6` |
| `Workspace` | `workspace::ToggleRightDock` | `cmd-alt-b`, `cmd-r` |
| `Workspace` | `zed::ToggleFullScreen` | `ctrl-cmd-f`, `fn-f` |
| `Workspace && debugger_stopped` | `debugger::StepInto` | `ctrl-f11`, `f11` |
| `Workspace && debugger_stopped` | `debugger::StepOver` | `f10`, `f7` |

### B. Same keystroke and action in overlapping contexts: 51 pairs

8 have no other binding of that key anywhere, so the repeat cannot be there to beat a deeper default (likely redundant). 43 re-bind a key that is bound to something else in another context (probably deliberate re-asserting).

**Likely redundant**

| Key | Action | Contexts |
|---|---|---|
| `cmd-alt-f` | `keymap_editor::ToggleKeystrokeSearch` | `KeymapEditor`; `KeymapEditor > BufferSearchBar` |
| `ctrl-alt-pagedown` | `agent::ScrollOutputToNextMessage` | `AcpThread`; `AcpThread > Editor` |
| `ctrl-alt-pageup` | `agent::ScrollOutputToPreviousMessage` | `AcpThread`; `AcpThread > Editor` |
| `ctrl-end` | `agent::ScrollOutputToBottom` | `AcpThread`; `AcpThread > Editor`; `AcpThread > Editor && end_of_input` |
| `ctrl-home` | `agent::ScrollOutputToTop` | `AcpThread`; `AcpThread > Editor`; `AcpThread > Editor && start_of_input` |
| `shift-alt-y` | `agent::KeepAll` | `AgentDiff`; `Editor && editor_agent_diff`; `AcpThread > Editor` |
| `shift-alt-z` | `agent::RejectAll` | `AgentDiff`; `Editor && editor_agent_diff`; `AcpThread > Editor` |
| `shift-ctrl-r` | `agent::OpenAgentDiff` | `Editor && editor_agent_diff`; `AcpThread > Editor` |

**Re-asserted against another binding of the same key**

| Key | Action | Contexts | Competing bindings of the key |
|---|---|---|---|
| `alt-down` | `notebook::MoveCellDown` | `NotebookEditor`; `NotebookEditor > Editor` | `collab_panel::MoveChannelDown` in `CollabPanel`; `editor::MoveLineDown` in `Editor`; `git_panel::FocusEditor` in `GitPanel && ChangesList && !GitBranchSelector && !GitRepositorySelector`; `markdown::ScrollDownByItem` in `MarkdownPreview` |
| `alt-enter` | `editor::OpenExcerpts` | `AcpThread > Editor && mode == full`; `!AcpThread > Editor && mode == full`; `OutlinePanel && not_editing` | `collab_panel::OpenSelectedChannelNotes` in `CollabPanel`; `console::WatchExpression` in `DebugConsole > Editor`; `editor::Newline` in `AgentFeedbackMessageEditor > Editor`; `editor::OpenSelectionsInMultibuffer` in `Editor && mode == full` … |
| `alt-tab` | `agent::CycleFavoriteModels` | `AcpThread`; `AcpThread > Editor`; `InlineAssistant > Editor` | `editor::AcceptEditPrediction` in `Editor && edit_prediction`; `editor::NextEditPrediction` in `Editor && mode == full && edit_prediction`; `editor::ShowEditPrediction` in `Editor && !edit_prediction`; `git::GenerateCommitMessage` in `CommitEditor > Editor` … |
| `alt-tab` | `git::GenerateCommitMessage` | `CommitEditor > Editor`; `GitCommit > Editor && mode == auto_height` | `agent::CycleFavoriteModels` in `AcpThread`; `agent::CycleFavoriteModels` in `AcpThread > Editor`; `agent::CycleFavoriteModels` in `InlineAssistant > Editor`; `editor::AcceptEditPrediction` in `Editor && edit_prediction` … |
| `alt-up` | `notebook::MoveCellUp` | `NotebookEditor`; `NotebookEditor > Editor` | `collab_panel::MoveChannelUp` in `CollabPanel`; `editor::MoveLineUp` in `Editor`; `git_panel::FocusChanges` in `CommitEditor > Editor`; `markdown::ScrollUpByItem` in `MarkdownPreview` |
| `cmd-enter` | `git::Commit` | `GitDiff > Editor`; `CommitEditor > Editor`; `GitPanel`; `GitCommit > Editor && mode == auto_height` | `agent::Chat` in `AcpThread > Editor && use_modifier_to_send`; `agent::ChatWithFollow` in `AcpThread > Editor`; `editor::NewlineBelow` in `Editor && mode == full`; `menu::Confirm` in `AcpThread > ModeSelector` … |
| `cmd-enter` | `notebook::Run` | `NotebookEditor`; `NotebookEditor > Editor` | `agent::Chat` in `AcpThread > Editor && use_modifier_to_send`; `agent::ChatWithFollow` in `AcpThread > Editor`; `editor::NewlineBelow` in `Editor && mode == full`; `git::Commit` in `CommitEditor > Editor` … |
| `cmd-enter` | `search::ReplaceAll` | `BufferSearchBar && in_replace > Editor`; `ProjectSearchBar && in_replace > Editor` | `agent::Chat` in `AcpThread > Editor && use_modifier_to_send`; `agent::ChatWithFollow` in `AcpThread > Editor`; `editor::NewlineBelow` in `Editor && mode == full`; `git::Commit` in `CommitEditor > Editor` … |
| `cmd-enter` | `skill_creator::SaveSkill` | `SkillCreator`; `SkillCreator > Editor` | `agent::Chat` in `AcpThread > Editor && use_modifier_to_send`; `agent::ChatWithFollow` in `AcpThread > Editor`; `editor::NewlineBelow` in `Editor && mode == full`; `git::Commit` in `CommitEditor > Editor` … |
| `cmd-f` | `agent::ToggleSearch` | `AcpThread`; `AcpThread > Editor`; `AgentPanel > Terminal` | `agents_sidebar::FocusSidebarFilter` in `ThreadsSidebar`; `buffer_search::Deploy` in `Editor && mode == full`; `buffer_search::Deploy` in `MarkdownPreview`; `buffer_search::Deploy` in `Terminal` … |
| `cmd-i` | `agent::ToggleProfileSelector` | `AcpThread`; `AcpThread > Editor` | `debugger::ToggleSessionPicker` in `DebugPanel`; `editor::ShowSignatureHelp` in `Editor` |
| `cmd-m` | `notebook::AddCodeBlock` | `NotebookEditor`; `NotebookEditor > Editor` | `settings_editor::Minimize` in `SettingsWindow`; `zed::Minimize` in `(none)` |
| `cmd-n` | `agent::NewThread` | `AgentPanel`; `AcpThread`; `AgentPanel > Terminal` | `agents_sidebar::NewThreadInGroup` in `ThreadsSidebar`; `project_panel::NewFile` in `ProjectPanel`; `workspace::NewFile` in `Welcome`; `workspace::NewFile` in `Workspace && !Terminal` … |
| `cmd-shift-backspace` | `zeta::ThumbsDownActivePrediction` | `RatePredictionsModal`; `RatePredictionsModal > Editor` | `agent::RemoveFirstQueuedMessage` in `AcpThread > Editor`; `agent::RemoveSelectedThread` in `ThreadsSidebar`; `branch_picker::DeleteBranch` in `GitBranchSelector \|\| (GitBranchSelector > Picker > Editor)`; `editor::GoToPreviousChange` in `Editor && mode == full` … |
| `cmd-shift-enter` | `git::Amend` | `GitDiff > Editor`; `CommitEditor > Editor`; `GitPanel`; `GitCommit > Editor && mode == auto_height` | `agent::SendImmediately` in `AcpThread > Editor`; `editor::NewlineAbove` in `Editor && mode == full`; `editor::ToggleFoldAll` in `BufferSearchBar`; `inline_assistant::ThumbsUpResult` in `InlineAssistant > Editor` … |
| `cmd-shift-enter` | `notebook::RunAll` | `NotebookEditor`; `NotebookEditor > Editor` | `agent::SendImmediately` in `AcpThread > Editor`; `editor::NewlineAbove` in `Editor && mode == full`; `editor::ToggleFoldAll` in `BufferSearchBar`; `git::Amend` in `CommitEditor > Editor` … |
| `cmd-shift-enter` | `zeta::ThumbsUpActivePrediction` | `RatePredictionsModal`; `RatePredictionsModal > Editor` | `agent::SendImmediately` in `AcpThread > Editor`; `editor::NewlineAbove` in `Editor && mode == full`; `editor::ToggleFoldAll` in `BufferSearchBar`; `git::Amend` in `CommitEditor > Editor` … |
| `cmd-shift-m` | `notebook::AddMarkdownBlock` | `NotebookEditor`; `NotebookEditor > Editor` | `diagnostics::Deploy` in `Workspace` |
| `cmd-shift-r` | `notebook::RestartKernel` | `NotebookEditor`; `NotebookEditor > Editor` | `task::Spawn` in `Workspace && !Terminal` |
| `cmd-w` | `workspace::CloseWindow` | `SettingsWindow`; `SkillCreator`; `SkillCreator > Editor` | `pane::CloseActiveItem` in `Pane`; `workspace::CloseActiveDock` in `Workspace` |
| `ctrl-alt-down` | `agent::ScrollOutputLineDown` | `AcpThread`; `AcpThread > Editor` | `pane::SplitDown` in `Terminal` |
| `ctrl-alt-up` | `agent::ScrollOutputLineUp` | `AcpThread`; `AcpThread > Editor` | `pane::SplitUp` in `Terminal` |
| `ctrl-enter` | `editor::Newline` | `Editor && mode == auto_height`; `BufferSearchBar && !in_replace > Editor`; `BufferSearchBar \|\| ProjectSearchBar`; `ProjectSearchBar && !in_replace > Editor` | `assistant::InlineAssist` in `!AcpThread > Editor && mode == full`; `assistant::InlineAssist` in `Terminal`; `menu::SecondaryConfirm` in `(none)` |
| `ctrl-pagedown` | `agent::ScrollOutputPageDown` | `AcpThread`; `AcpThread > Editor`; `AcpThread > Editor && end_of_input` | `editor::LineDown` in `Editor` |
| `ctrl-pageup` | `agent::ScrollOutputPageUp` | `AcpThread`; `AcpThread > Editor`; `AcpThread > Editor && start_of_input` | `editor::LineUp` in `Editor` |
| `down` | `menu::SelectNext` | `(none)`; `GitPanel && (ChangesList \|\| HistoryList) && !GitBranchSelector && !GitRepositorySelector`; `Picker > Editor`; `KeybindEditorModal > Editor`; `NotebookEditor && notebook_mode == command` | `agent::ScrollOutputLineDown` in `AcpThread`; `editor::ContextMenuNext` in `Editor && (showing_code_actions \|\| showing_completions)`; `editor::MoveDown` in `Editor`; `editor::SignatureHelpNext` in `Editor && showing_signature_help && !showing_completions` … |
| `down` | `search::NextHistoryQuery` | `BufferSearchBar && !in_replace > Editor`; `ProjectSearchBar > Editor` | `agent::ScrollOutputLineDown` in `AcpThread`; `editor::ContextMenuNext` in `Editor && (showing_code_actions \|\| showing_completions)`; `editor::MoveDown` in `Editor`; `editor::SignatureHelpNext` in `Editor && showing_signature_help && !showing_completions` … |
| `enter` | `editor::Newline` | `Editor && mode == full`; `AcpThread > Editor && use_modifier_to_send`; `CommitEditor > Editor`; `GitCommit > Editor && mode == auto_height`; `ConfigureContextServerModal > Editor`; `NotebookEditor > Editor` | `agent::Chat` in `AcpThread > Editor && !use_modifier_to_send`; `agent::SelectNextThreadMatch` in `AcpThreadSearchBar`; `editor::ConfirmCodeAction` in `Editor && showing_code_actions`; `editor::ConfirmCompletion` in `Editor && showing_completions` … |
| `enter` | `menu::Confirm` | `(none)`; `AgentFeedbackMessageEditor > Editor`; `ThreadsSidebar`; `Editor && inline_input`; `GitPanel && (ChangesList \|\| HistoryList) && !GitBranchSelector && !GitRepositorySelector`; `DebugConsole > Editor` | `agent::Chat` in `AcpThread > Editor && !use_modifier_to_send`; `agent::SelectNextThreadMatch` in `AcpThreadSearchBar`; `editor::ConfirmCodeAction` in `Editor && showing_code_actions`; `editor::ConfirmCompletion` in `Editor && showing_completions` … |
| `enter` | `search::ReplaceNext` | `BufferSearchBar && in_replace > Editor`; `ProjectSearchBar && in_replace > Editor` | `agent::Chat` in `AcpThread > Editor && !use_modifier_to_send`; `agent::SelectNextThreadMatch` in `AcpThreadSearchBar`; `editor::ConfirmCodeAction` in `Editor && showing_code_actions`; `editor::ConfirmCompletion` in `Editor && showing_completions` … |
| `escape` | `menu::Cancel` | `(none)`; `AgentFeedbackMessageEditor > Editor`; `OutlinePanel && not_editing`; `ProjectPanel`; `GitPanel && (ChangesList \|\| HistoryList) && !GitBranchSelector && !GitRepositorySelector`; `GitCommit > Editor && mode == auto_height`; `Picker > Editor`; `ZedPredictModal`; `ConfigureContextServerModal > Editor`; `OnboardingAiConfigurationModal`; `KeybindEditorModal` | `agent::DismissThreadSearch` in `AcpThreadSearchBar`; `buffer_search::Dismiss` in `BufferSearchBar`; `editor::Cancel` in `Editor`; `git::Cancel` in `GitPanel && CommitEditor` … |
| `pagedown` | `agent::ScrollOutputPageDown` | `AcpThread`; `AcpThread > Editor && end_of_input` | `editor::ContextMenuLast` in `Editor && (showing_code_actions \|\| showing_completions)`; `editor::MovePageDown` in `Editor`; `markdown::ScrollPageDown` in `MarkdownPreview`; `menu::SelectLast` in `(none)` … |
| `pageup` | `agent::ScrollOutputPageUp` | `AcpThread`; `AcpThread > Editor && start_of_input` | `editor::ContextMenuFirst` in `Editor && (showing_code_actions \|\| showing_completions)`; `editor::MovePageUp` in `Editor`; `markdown::ScrollPageUp` in `MarkdownPreview`; `menu::SelectFirst` in `(none)` … |
| `shift-enter` | `agent::SelectPreviousThreadMatch` | `AcpThreadSearchBar`; `AcpThreadSearchBar > Editor` | `collab_panel::ToggleSelectedChannelFavorite` in `CollabPanel`; `editor::ConfirmCompletionReplace` in `Editor && showing_completions`; `editor::ExpandExcerpts` in `!AcpThread > Editor && mode == full`; `editor::Newline` in `Editor && mode == auto_height` … |
| `shift-enter` | `editor::Newline` | `Editor && mode == full`; `Editor && mode == auto_height` | `agent::SelectPreviousThreadMatch` in `AcpThreadSearchBar`; `agent::SelectPreviousThreadMatch` in `AcpThreadSearchBar > Editor`; `collab_panel::ToggleSelectedChannelFavorite` in `CollabPanel`; `editor::ConfirmCompletionReplace` in `Editor && showing_completions` … |
| `shift-enter` | `notebook::RunAndAdvance` | `NotebookEditor`; `NotebookEditor > Editor` | `agent::SelectPreviousThreadMatch` in `AcpThreadSearchBar`; `agent::SelectPreviousThreadMatch` in `AcpThreadSearchBar > Editor`; `collab_panel::ToggleSelectedChannelFavorite` in `CollabPanel`; `editor::ConfirmCompletionReplace` in `Editor && showing_completions` … |
| `shift-enter` | `search::SelectPreviousMatch` | `BufferSearchBar`; `BufferSearchBar && !in_replace > Editor` | `agent::SelectPreviousThreadMatch` in `AcpThreadSearchBar`; `agent::SelectPreviousThreadMatch` in `AcpThreadSearchBar > Editor`; `collab_panel::ToggleSelectedChannelFavorite` in `CollabPanel`; `editor::ConfirmCompletionReplace` in `Editor && showing_completions` … |
| `shift-tab` | `agent::CycleModeSelector` | `AcpThread`; `AcpThread > Editor` | `editor::Backtab` in `Editor`; `editor::PreviousSnippetTabstop` in `Editor && in_snippet && has_previous_tabstop && !showing_completions`; `git_graph::FocusPreviousTabStop` in `GitGraph`; `git_graph::FocusPreviousTabStop` in `GitGraphSearchBar > Editor` … |
| `shift-tab` | `skill_creator::FocusPreviousField` | `SkillCreator`; `SkillCreator > Editor` | `agent::CycleModeSelector` in `AcpThread`; `agent::CycleModeSelector` in `AcpThread > Editor`; `editor::Backtab` in `Editor`; `editor::PreviousSnippetTabstop` in `Editor && in_snippet && has_previous_tabstop && !showing_completions` … |
| `tab` | `channel_modal::ToggleMode` | `ChannelModal`; `ChannelModal > Picker > Editor` | `buffer_search::FocusEditor` in `BufferSearchBar`; `editor::AcceptEditPrediction` in `Editor && edit_prediction && edit_prediction_mode == eager && !showing_completions`; `editor::ComposeCompletion` in `Editor && showing_completions`; `editor::NextSnippetTabstop` in `Editor && in_snippet && has_next_tabstop && !showing_completions` … |
| `tab` | `skill_creator::FocusNextField` | `SkillCreator`; `SkillCreator > Editor` | `buffer_search::FocusEditor` in `BufferSearchBar`; `channel_modal::ToggleMode` in `ChannelModal`; `channel_modal::ToggleMode` in `ChannelModal > Picker > Editor`; `editor::AcceptEditPrediction` in `Editor && edit_prediction && edit_prediction_mode == eager && !showing_completions` … |
| `up` | `menu::SelectPrevious` | `(none)`; `GitPanel && (ChangesList \|\| HistoryList) && !GitBranchSelector && !GitRepositorySelector`; `Picker > Editor`; `KeybindEditorModal > Editor`; `NotebookEditor && notebook_mode == command` | `agent::ScrollOutputLineUp` in `AcpThread`; `editor::ContextMenuPrevious` in `Editor && (showing_code_actions \|\| showing_completions)`; `editor::MoveUp` in `Editor`; `editor::SignatureHelpPrevious` in `Editor && showing_signature_help && !showing_completions` … |
| `up` | `search::PreviousHistoryQuery` | `BufferSearchBar && !in_replace > Editor`; `ProjectSearchBar > Editor` | `agent::ScrollOutputLineUp` in `AcpThread`; `editor::ContextMenuPrevious` in `Editor && (showing_code_actions \|\| showing_completions)`; `editor::MoveUp` in `Editor`; `editor::SignatureHelpPrevious` in `Editor && showing_signature_help && !showing_completions` … |

## vim.json

### A. Same action, same context, several keystrokes: 98 groups, 124 extra bindings

| Context | Action | Keys |
|---|---|---|
| `VimControl && !menu` | `editor::GoToDefinitionSplit` | `ctrl-w ]`, `ctrl-w ctrl-]`, `ctrl-w d`, `ctrl-w g d` |
| `VimControl && !menu \|\| !Editor && !Terminal` | `pane::CloseActiveItem` | `ctrl-w c`, `ctrl-w ctrl-c`, `ctrl-w ctrl-q`, `ctrl-w q` |
| `VimControl && !menu \|\| !Editor && !Terminal` | `workspace::ActivatePreviousPane` | `ctrl-w ctrl-p`, `ctrl-w ctrl-shift-w`, `ctrl-w p`, `ctrl-w shift-w` |
| `vim_operator == gq` | `vim::CurrentLine` | `g q`, `g w`, `q`, `w` |
| `NotebookEditor && notebook_mode == command` | `notebook::EnterEditMode` | `a`, `enter`, `i` |
| `VimControl && !menu` | `vim::Down` | `ctrl-j`, `down`, `j` |
| `VimControl && !menu` | `vim::NextLineStart` | `+`, `ctrl-m`, `enter` |
| `VimControl && !menu \|\| !Editor && !Terminal` | `pane::SplitHorizontal` | `ctrl-w ctrl-s`, `ctrl-w s`, `ctrl-w shift-s` |
| `VimControl && !menu \|\| !Editor && !Terminal` | `workspace::ActivatePaneDown` | `ctrl-w ctrl-j`, `ctrl-w down`, `ctrl-w j` |
| `VimControl && !menu \|\| !Editor && !Terminal` | `workspace::ActivatePaneLeft` | `ctrl-w ctrl-h`, `ctrl-w h`, `ctrl-w left` |
| `VimControl && !menu \|\| !Editor && !Terminal` | `workspace::ActivatePaneRight` | `ctrl-w ctrl-l`, `ctrl-w l`, `ctrl-w right` |
| `VimControl && !menu \|\| !Editor && !Terminal` | `workspace::ActivatePaneUp` | `ctrl-w ctrl-k`, `ctrl-w k`, `ctrl-w up` |
| `vim_mode == insert` | `vim::NormalBefore` | `ctrl-[`, `ctrl-c`, `escape` |
| `vim_mode == insert` | `vim::PushLiteral` {} | `ctrl-q`, `ctrl-shift-q`, `ctrl-v` |
| `vim_mode == operator` | `vim::ClearOperators` | `ctrl-[`, `ctrl-c`, `escape` |
| `vim_mode == replace` | `vim::NormalBefore` | `ctrl-[`, `ctrl-c`, `escape` |
| `vim_mode == replace` | `vim::PushLiteral` {} | `ctrl-q`, `ctrl-shift-q`, `ctrl-v` |
| `vim_mode == visual` | `vim::SwitchToNormalMode` | `ctrl-[`, `ctrl-c`, `escape` |
| `vim_mode == visual` | `vim::VisualDelete` | `d`, `delete`, `x` |
| `vim_mode == waiting` | `vim::ClearOperators` | `ctrl-[`, `ctrl-c`, `escape` |
| `vim_operator == helix_next` | `vim::NextComment` | `*`, `/`, `c` |
| `vim_operator == helix_previous` | `vim::PreviousComment` | `*`, `/`, `c` |
| `!Editor && !Terminal` | `pane::DeploySearch` | `g /`, `space /` |
| `(vim_mode == helix_normal \|\| vim_mode == helix_select) && !menu` | `editor::Paste` | `shift-r`, `space p` |
| `(vim_mode == helix_normal \|\| vim_mode == helix_select) && !menu` | `editor::ToggleComments` {"advance_downwards": false, "comment_empty_lines": false} | `ctrl-c`, `space c` |
| `(vim_mode == helix_normal \|\| vim_mode == helix_select) && !menu` | `pane::ActivateNextItem` | `g n`, `shift-l` |
| `(vim_mode == helix_normal \|\| vim_mode == helix_select) && !menu` | `pane::ActivatePreviousItem` | `g p`, `shift-h` |
| `(vim_mode == helix_normal \|\| vim_mode == helix_select) && !menu` | `pane::SplitDown` | `space w d`, `space w s` |
| `(vim_mode == helix_normal \|\| vim_mode == helix_select) && !menu` | `pane::SplitRight` | `space w r`, `space w v` |
| `(vim_mode == helix_normal \|\| vim_mode == helix_select) && !menu` | `vim::WrappingLeft` | `h`, `left` |
| `(vim_mode == helix_normal \|\| vim_mode == helix_select) && !menu` | `vim::WrappingRight` | `l`, `right` |
| `CommitEditor > Editor && VimControl && !menu` | `git_panel::FocusChanges` | `shift-tab`, `tab` |
| `GitCommit > Editor && VimControl && vim_mode == normal` | `menu::Cancel` | `ctrl-c`, `escape` |
| `OutlinePanel && not_editing` | `outline_panel::ScrollCursorCenter` | `z c`, `z z` |
| `OutlinePanel && not_editing` | `vim::MenuSelectNext` | `down`, `j` |
| `OutlinePanel && not_editing` | `vim::MenuSelectPrevious` | `k`, `up` |
| `ProjectPanel && not_editing` | `project_panel::OpenPermanent` | `enter`, `t` |
| `ProjectPanel && not_editing` | `project_panel::ScrollCursorCenter` | `z c`, `z z` |
| `ProjectPanel && not_editing` | `vim::MenuSelectNext` | `down`, `j` |
| `ProjectPanel && not_editing` | `vim::MenuSelectPrevious` | `k`, `up` |
| `ThreadsSidebar && !Editor` | `agents_sidebar::NewThreadInGroup` | `o`, `shift-o` |
| `ThreadsSidebar && !Editor` | `menu::SelectChild` | `l`, `z o` |
| `ThreadsSidebar && !Editor` | `menu::SelectParent` | `h`, `z c` |
| `VimControl && !menu` | `editor::FindAllReferences` | `g r r`, `g shift-a` |
| `VimControl && !menu` | `editor::GoToDefinition` | `ctrl-]`, `g d` |
| `VimControl && !menu` | `editor::GoToImplementation` | `g r i`, `g shift-i` |
| `VimControl && !menu` | `editor::GoToTypeDefinitionSplit` | `ctrl-w g shift-d`, `ctrl-w shift-d` |
| `VimControl && !menu` | `editor::Hover` | `g h`, `shift-k` |
| `VimControl && !menu` | `editor::OpenExcerptsSplit` | `ctrl-w g space`, `ctrl-w space` |
| `VimControl && !menu` | `editor::ToggleCodeActions` | `g .`, `g r a` |
| `VimControl && !menu` | `outline::Toggle` | `g s`, `g shift-o` |
| `VimControl && !menu` | `vim::Down` {"display_lines": true} | `g down`, `g j` |
| `VimControl && !menu` | `vim::EndOfLine` | `$`, `end` |
| `VimControl && !menu` | `vim::EndOfLine` {"display_lines": true} | `g $`, `g end` |
| `VimControl && !menu` | `vim::Left` | `h`, `left` |
| `VimControl && !menu` | `vim::PageDown` | `ctrl-f`, `pagedown` |
| `VimControl && !menu` | `vim::PageUp` | `ctrl-b`, `pageup` |
| `VimControl && !menu` | `vim::Right` | `l`, `right` |
| `VimControl && !menu` | `vim::StartOfLine` | `0`, `home` |
| `VimControl && !menu` | `vim::StartOfLine` {"display_lines": true} | `g 0`, `g home` |
| `VimControl && !menu` | `vim::SwitchToNormalMode` | `ctrl-[`, `escape` |
| `VimControl && !menu` | `vim::Tab` | `shift-tab`, `tab` |
| `VimControl && !menu` | `vim::ToggleVisualBlock` | `ctrl-q`, `ctrl-v` |
| `VimControl && !menu` | `vim::Up` | `k`, `up` |
| `VimControl && !menu` | `vim::Up` {"display_lines": true} | `g k`, `g up` |
| `VimControl && !menu \|\| !Editor && !Terminal` | `pane::ActivateNextItem` | `ctrl-w ctrl-g t`, `ctrl-w g t` |
| `VimControl && !menu \|\| !Editor && !Terminal` | `pane::ActivatePreviousItem` | `ctrl-w ctrl-g shift-t`, `ctrl-w g shift-t` |
| `VimControl && !menu \|\| !Editor && !Terminal` | `pane::CloseAllItems` | `ctrl-w a`, `ctrl-w ctrl-a` |
| `VimControl && !menu \|\| !Editor && !Terminal` | `pane::SplitVertical` | `ctrl-w ctrl-v`, `ctrl-w v` |
| `VimControl && !menu \|\| !Editor && !Terminal` | `workspace::ActivateNextPane` | `ctrl-w ctrl-w`, `ctrl-w w` |
| `VimControl && !menu \|\| !Editor && !Terminal` | `workspace::CloseInactiveTabsAndPanes` | `ctrl-w ctrl-o`, `ctrl-w o` |
| `VimControl && !menu \|\| !Editor && !Terminal` | `workspace::NewFileSplitHorizontal` | `ctrl-w ctrl-n`, `ctrl-w n` |
| `VimControl && !menu \|\| !Editor && !Terminal` | `workspace::SwapPaneAdjacent` | `ctrl-w ctrl-x`, `ctrl-w x` |
| `VimControl && vim_mode == helix_normal && !menu && !BufferSearchBar` | `vim::Down` | `g down`, `g j` |
| `VimControl && vim_mode == helix_normal && !menu && !BufferSearchBar` | `vim::Down` {"display_lines": true} | `down`, `j` |
| `VimControl && vim_mode == helix_normal && !menu && !BufferSearchBar` | `vim::Up` | `g k`, `g up` |
| `VimControl && vim_mode == helix_normal && !menu && !BufferSearchBar` | `vim::Up` {"display_lines": true} | `k`, `up` |
| `vim_mode == insert && !(showing_code_actions \|\| showing_completions)` | `editor::ShowWordCompletions` | `ctrl-n`, `ctrl-p` |
| `vim_mode == normal` | `vim::DeleteRight` | `delete`, `x` |
| `vim_mode == normal` | `vim::InsertBefore` | `i`, `insert` |
| `vim_mode == normal` | `vim::PushRewrap` | `g q`, `g w` |
| `vim_mode == normal \|\| vim_mode == visual \|\| vim_mode == operator` | `vim::NextComment` | `] *`, `] /` |
| `vim_mode == normal \|\| vim_mode == visual \|\| vim_mode == operator` | `vim::PreviousComment` | `[ *`, `[ /` |
| `vim_mode == normal \|\| vim_mode == visual \|\| vim_mode == operator` | `workspace::FollowNextCollaborator` | `[ f`, `] f` |
| `vim_mode == visual` | `vim::Paste` {"preserve_clipboard": true} | `g shift-r`, `shift-p` |
| `vim_mode == visual` | `vim::Rewrap` | `g q`, `g w` |
| `vim_mode == visual` | `vim::Substitute` | `c`, `s` |
| `vim_mode == visual` | `vim::SubstituteLine` | `shift-r`, `shift-s` |
| `vim_mode == visual` | `vim::VisualDeleteLine` | `shift-d`, `shift-x` |
| `vim_mode == waiting` | `vim::PushLiteral` {} | `ctrl-q`, `ctrl-v` |
| `vim_operator == a \|\| vim_operator == i \|\| vim_operator == cs \|\| vim_operator == helix_next \|\| vim_operator == helix_previous` | `vim::CurlyBrackets` | `shift-b`, `}` |
| `vim_operator == a \|\| vim_operator == i \|\| vim_operator == cs \|\| vim_operator == helix_next \|\| vim_operator == helix_previous` | `vim::Parentheses` | `)`, `b` |
| `vim_operator == a \|\| vim_operator == i \|\| vim_operator == cs \|\| vim_operator == helix_next \|\| vim_operator == helix_previous` | `vim::SquareBrackets` | `]`, `r` |
| `vim_operator == g?` | `vim::CurrentLine` | `?`, `g ?` |
| `vim_operator == gR` | `vim::CurrentLine` | `r`, `shift-r` |
| `vim_operator == gU` | `vim::CurrentLine` | `g shift-u`, `shift-u` |
| `vim_operator == gu` | `vim::CurrentLine` | `g u`, `u` |
| `vim_operator == g~` | `vim::CurrentLine` | `g ~`, `~` |

### B. Same keystroke and action in overlapping contexts: 25 pairs

10 have no other binding of that key anywhere, so the repeat cannot be there to beat a deeper default (likely redundant). 15 re-bind a key that is bound to something else in another context (probably deliberate re-asserting).

**Likely redundant**

| Key | Action | Contexts |
|---|---|---|
| `1` | `vim::Number` 1 | `VimControl && !menu`; `ProjectPanel && not_editing`; `OutlinePanel && not_editing` |
| `2` | `vim::Number` 2 | `VimControl && !menu`; `ProjectPanel && not_editing`; `OutlinePanel && not_editing` |
| `3` | `vim::Number` 3 | `VimControl && !menu`; `ProjectPanel && not_editing`; `OutlinePanel && not_editing` |
| `4` | `vim::Number` 4 | `VimControl && !menu`; `ProjectPanel && not_editing`; `OutlinePanel && not_editing` |
| `5` | `vim::Number` 5 | `VimControl && !menu`; `ProjectPanel && not_editing`; `OutlinePanel && not_editing` |
| `6` | `vim::Number` 6 | `VimControl && !menu`; `ProjectPanel && not_editing`; `OutlinePanel && not_editing` |
| `7` | `vim::Number` 7 | `VimControl && !menu`; `ProjectPanel && not_editing`; `OutlinePanel && not_editing` |
| `8` | `vim::Number` 8 | `VimControl && !menu`; `ProjectPanel && not_editing`; `OutlinePanel && not_editing` |
| `9` | `vim::Number` 9 | `VimControl && !menu`; `ProjectPanel && not_editing`; `OutlinePanel && not_editing` |
| `alt-.` | `vim::RepeatFind` | `vim_mode == helix_select`; `(vim_mode == helix_normal \|\| vim_mode == helix_select) && !menu` |

**Re-asserted against another binding of the same key**

| Key | Action | Contexts | Competing bindings of the key |
|---|---|---|---|
| `0` | `vim::Number` 0 | `VimControl && VimCount`; `ProjectPanel && not_editing`; `OutlinePanel && not_editing` | `vim::StartOfLine` in `VimControl && !menu` |
| `;` | `vim::HelixCollapseSelection` | `vim_mode == helix_select`; `(vim_mode == helix_normal \|\| vim_mode == helix_select) && !menu` | `vim::RepeatFind` in `VimControl && !menu` |
| `ctrl-a` | `vim::Increment` | `vim_mode == normal`; `vim_mode == visual`; `vim_mode == helix_select`; `(vim_mode == normal \|\| vim_mode == helix_normal) && !menu` | `vim::Literal` in `vim_mode == literal` |
| `ctrl-x` | `vim::Decrement` | `vim_mode == normal`; `vim_mode == visual`; `vim_mode == helix_select`; `(vim_mode == normal \|\| vim_mode == helix_normal) && !menu` | `vim::Literal` in `vim_mode == literal` |
| `down` | `vim::MenuSelectNext` | `ProjectPanel && not_editing`; `OutlinePanel && not_editing` | `vim::Down` in `VimControl && !menu`; `vim::Down` in `VimControl && vim_mode == helix_normal && !menu && !BufferSearchBar` |
| `escape` | `menu::Cancel` | `Editor && mode == full && VimControl && vim_mode == normal && !menu && os == windows`; `Editor && mode == full && VimControl && vim_mode == helix_normal && !menu && !BufferSearchBar && os == windows`; `GitCommit > Editor && VimControl && vim_mode == normal`; `ThreadsSidebar > Editor && VimControl && vim_mode == normal` | `buffer_search::Dismiss` in `BufferSearchBar && !in_replace`; `editor::Cancel` in `(vim_mode == normal \|\| vim_mode == helix_normal) && !menu`; `notebook::EnterCommandMode` in `NotebookEditor > Editor && VimControl && vim_mode == normal`; `vim::ClearOperators` in `vim_mode == operator` … |
| `escape` | `vim::SwitchToHelixNormalMode` | `VimControl && vim_mode == helix_normal && !menu && !BufferSearchBar`; `vim_mode == helix_select && !menu && !BufferSearchBar` | `buffer_search::Dismiss` in `BufferSearchBar && !in_replace`; `editor::Cancel` in `(vim_mode == normal \|\| vim_mode == helix_normal) && !menu`; `menu::Cancel` in `Editor && mode == full && VimControl && vim_mode == helix_normal && !menu && !BufferSearchBar && os == windows`; `menu::Cancel` in `Editor && mode == full && VimControl && vim_mode == normal && !menu && os == windows` … |
| `g g` | `menu::SelectFirst` | `ProjectPanel && not_editing`; `OutlinePanel && not_editing`; `GitGraph && !GitGraphSearchBar`; `GitPanel && ChangesList && !GitBranchSelector && !GitRepositorySelector`; `FileHistoryView`; `NotebookEditor && notebook_mode == command`; `ThreadsSidebar && !Editor` | `markdown::ScrollToTop` in `MarkdownPreview`; `settings_editor::FocusFirstNavEntry` in `SettingsWindow > NavigationMenu && !search`; `vim::StartOfDocument` in `VimControl && !menu` |
| `j` | `vim::MenuSelectNext` | `ProjectPanel && not_editing`; `OutlinePanel && not_editing`; `GitGraph && !GitGraphSearchBar` | `editor::MoveDown` in `ThreadsSidebar > Editor && VimControl && vim_mode == normal`; `menu::SelectNext` in `FileHistoryView`; `menu::SelectNext` in `GitPanel && (ChangesList \|\| HistoryList) && !GitBranchSelector && !GitRepositorySelector`; `menu::SelectNext` in `NotebookEditor && notebook_mode == command` … |
| `k` | `vim::MenuSelectPrevious` | `ProjectPanel && not_editing`; `OutlinePanel && not_editing`; `GitGraph && !GitGraphSearchBar` | `editor::MoveUp` in `ThreadsSidebar > Editor && VimControl && vim_mode == normal`; `menu::SelectPrevious` in `FileHistoryView`; `menu::SelectPrevious` in `GitPanel && (ChangesList \|\| HistoryList) && !GitBranchSelector && !GitRepositorySelector`; `menu::SelectPrevious` in `NotebookEditor && notebook_mode == command` … |
| `shift-g` | `menu::SelectLast` | `ProjectPanel && not_editing`; `OutlinePanel && not_editing`; `GitGraph && !GitGraphSearchBar`; `GitPanel && ChangesList && !GitBranchSelector && !GitRepositorySelector`; `FileHistoryView`; `NotebookEditor && notebook_mode == command`; `ThreadsSidebar && !Editor` | `markdown::ScrollToBottom` in `MarkdownPreview`; `settings_editor::FocusLastNavEntry` in `SettingsWindow > NavigationMenu && !search`; `vim::EndOfDocument` in `VimControl && !menu`; `vim::HelixGotoLine` in `(vim_mode == helix_normal \|\| vim_mode == helix_select) && !menu` |
| `shift-tab` | `skill_creator::FocusPreviousField` | `SkillCreator`; `SkillCreator > Editor` | `editor::ContextMenuPrevious` in `Editor && (vim_mode == helix_normal \|\| vim_mode == helix_select) && showing_code_actions`; `git_graph::FocusPreviousTabStop` in `GitGraph`; `git_graph::FocusPreviousTabStop` in `GitGraphSearchBar > Editor`; `git_panel::FocusChanges` in `CommitEditor > Editor && VimControl && !menu` … |
| `shift-y` | `vim::YankLine` | `vim_mode == normal`; `(vim_mode == normal \|\| vim_mode == helix_normal) && !menu` | `vim::VisualYankLine` in `vim_mode == visual` |
| `tab` | `skill_creator::FocusNextField` | `SkillCreator`; `SkillCreator > Editor` | `editor::AcceptEditPrediction` in `Editor && edit_prediction && edit_prediction_mode == eager && !showing_completions`; `editor::ContextMenuNext` in `Editor && (vim_mode == helix_normal \|\| vim_mode == helix_select) && showing_code_actions`; `git_graph::FocusNextTabStop` in `GitGraph`; `git_graph::FocusNextTabStop` in `GitGraphSearchBar > Editor` … |
| `up` | `vim::MenuSelectPrevious` | `ProjectPanel && not_editing`; `OutlinePanel && not_editing` | `vim::Up` in `VimControl && !menu`; `vim::Up` in `VimControl && vim_mode == helix_normal && !menu && !BufferSearchBar` |

### C. vim.json bindings that repeat a default-macos.json key+action: 27

| Key | vim context | Action |
|---|---|---|
| `ctrl-c` | `GitCommit > Editor && VimControl && vim_mode == normal` | `menu::Cancel` |
| `ctrl-h` | `Picker > Editor` | `editor::Backspace` |
| `ctrl-n` | `Picker > Editor` | `menu::SelectNext` |
| `ctrl-p` | `Picker > Editor` | `menu::SelectPrevious` |
| `ctrl-w` | `vim_mode == insert` | `editor::DeleteToPreviousWordStart` |
| `enter` | `MessageEditor > Editor && VimControl` | `agent::Chat` |
| `enter` | `NotebookEditor && notebook_mode == command` | `notebook::EnterEditMode` |
| `enter` | `ThreadsSidebar > Editor && VimControl && vim_mode == normal` | `editor::Newline` |
| `escape` | `(vim_mode == normal \|\| vim_mode == helix_normal) && !menu` | `editor::Cancel` |
| `escape` | `BufferSearchBar && !in_replace` | `buffer_search::Dismiss` |
| `escape` | `Editor && mode == full && VimControl && vim_mode == helix_normal && !menu && !BufferSearchBar && os == windows` | `menu::Cancel` |
| `escape` | `Editor && mode == full && VimControl && vim_mode == normal && !menu && os == windows` | `menu::Cancel` |
| `escape` | `GitCommit > Editor && VimControl && vim_mode == normal` | `menu::Cancel` |
| `escape` | `NotebookEditor > Editor && VimControl && vim_mode == normal` | `notebook::EnterCommandMode` |
| `escape` | `ThreadsSidebar > Editor && VimControl && vim_mode == normal` | `menu::Cancel` |
| `shift-enter` | `NotebookEditor && notebook_mode == command` | `notebook::RunAndAdvance` |
| `shift-tab` | `CommitEditor > Editor && VimControl && !menu` | `git_panel::FocusChanges` |
| `shift-tab` | `GitGraph` | `git_graph::FocusPreviousTabStop` |
| `shift-tab` | `GitGraphSearchBar > Editor` | `git_graph::FocusPreviousTabStop` |
| `shift-tab` | `SkillCreator` | `skill_creator::FocusPreviousField` |
| `shift-tab` | `SkillCreator > Editor` | `skill_creator::FocusPreviousField` |
| `tab` | `CommitEditor > Editor && VimControl && !menu` | `git_panel::FocusChanges` |
| `tab` | `Editor && edit_prediction && edit_prediction_mode == eager && !showing_completions` | `editor::AcceptEditPrediction` |
| `tab` | `GitGraph` | `git_graph::FocusNextTabStop` |
| `tab` | `GitGraphSearchBar > Editor` | `git_graph::FocusNextTabStop` |
| `tab` | `SkillCreator` | `skill_creator::FocusNextField` |
| `tab` | `SkillCreator > Editor` | `skill_creator::FocusNextField` |
