(comment) @comment

(alias_name) @variable
(alias_value) @type
(alias_ref) @variable
(body_use (alias_ref) @function.macro)

(context_header (ident) @type)
(context_header (string) @type)
(context_header (operator) @operator)
(context_header (paren) @punctuation.bracket)

(key (ident) @string.special)
(key (string) @string.special)
(key (operator) @string.special)
(key (paren) @string.special)
(prefix_header (key (ident) @keyword))
(prefix_header (key (string) @keyword))

(action_name) @function
(args) @property
(null) @constant.builtin

["{" "}"] @punctuation.bracket
":" @punctuation.delimiter

