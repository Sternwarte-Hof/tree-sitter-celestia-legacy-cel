module.exports = grammar({
  name: 'celestia_legacy_cel',

  extras: $ => [
    /\s/,
    $.comment,
  ],

  rules: {
    source_file: $ => repeat(choice(
      $.script_block,
      $.command_statement,
    )),

    comment: $ => token(seq('#', /.*/)),

    script_block: $ => prec(2, seq(
      '{',
      repeat($.command_statement),
      '}',
    )),

    command_statement: $ => prec.right(1, seq(
      field('name', $.command_name),
      optional(field('body', $.property_group)),
    )),

    command_name: $ => $.identifier,

    property_group: $ => seq(
      '{',
      repeat($.property_entry),
      '}',
    ),

    property_entry: $ => prec(0, seq(
      field('key', choice($.identifier, $.string)),
      field('value', $._value),
    )),

    _value: $ => choice(
      $.string,
      $.number,
      $.boolean,
      $.array,
      $.property_group,
      $.identifier,
    ),

    array: $ => seq(
      '[',
      repeat($._value),
      ']',
    ),

    string: $ => token(seq(
      '"',
      repeat(choice(
        /[^"\\\r\n]+/,
        seq('\\', /./)
      )),
      '"'
    )),

    number: $ => token(seq(
      optional('-'),
      /\d+/,
      optional(seq('.', /\d+/)),
      optional(seq(/[eE]/, optional(/[+-]/), /\d+/))
    )),

    boolean: $ => choice('true', 'false'),

    identifier: $ => /[a-zA-Z_][a-zA-Z0-9_\-\.\/\|]*/,
  }
});
