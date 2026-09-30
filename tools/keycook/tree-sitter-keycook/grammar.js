// Tree-sitter grammar for keycook `.kc` files: enough structure for highlighting and outline.
module.exports = grammar({
  name: 'keycook',

  extras: $ => [/\s/, $.comment],

  word: $ => $.ident,

  conflicts: $ => [
    [$.context_header, $.key],
    [$.prefix_header, $.keys],
    [$.actions],
    [$.keys],
  ],

  rules: {
    source_file: $ => repeat($._item),

    _item: $ => choice($.alias, $.block, $.binding),

    comment: $ => token(seq('//', /.*/)),

    alias: $ => seq(field('name', $.alias_name), '=', field('value', $.alias_value)),
    alias_name: $ => /@[A-Za-z_][A-Za-z0-9_-]*/,
    alias_value: $ => /[^\n]+/,

    block: $ => seq(
      field('header', choice($.context_header, $.prefix_header)),
      '{',
      repeat($._item),
      '}',
    ),

    // a context expression: words, operators, parens, aliases, up to the `{`
    context_header: $ => repeat1(choice($.ident, $.string, $.alias_ref, '|')),
    alias_ref: $ => /@[A-Za-z_][A-Za-z0-9_-]*/,

    // `keys:` before a `{`
    prefix_header: $ => seq(repeat1($.key), ':'),

    binding: $ => seq(field('keys', $.keys), ':', field('actions', $.actions)),

    keys: $ => seq(repeat1($.key), repeat(seq('|', repeat1($.key)))),
    key: $ => choice($.ident, $.string),

    // shared word token for context words and keys
    ident: $ => /[^\s{}:"|@][^\s{}:"|]*/,

    actions: $ => seq(repeat1($._action), repeat(seq('|', repeat1($._action)))),
    _action: $ => choice($.null, $.action),
    null: $ => 'null',
    action: $ => seq(field('name', $.action_name), optional(field('args', $.args))),
    action_name: $ => token(prec(2, /[A-Za-z_][A-Za-z0-9_]*::[A-Za-z0-9_.]+/)),
    args: $ => token(prec(1, /\((?:[^()"\n]|"(?:[^"\\\n]|\\.)*"|\([^()\n]*\))*\)/)),

    string: $ => token(seq('"', repeat(choice(/[^"\\\n]/, /\\./)), '"')),
  },
});
