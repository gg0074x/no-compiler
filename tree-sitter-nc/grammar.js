/// <reference types="tree-sitter-cli/dsl" />
// @ts-check

module.exports = grammar({
  name: 'nc',

  rules: {
    source_file: $ => repeat(
      choice(
        $.electrical_signal_represented_as_a_binary_number,
        $.non_electrical_signal_represented_as_a_binary_number
      )
    ),

    electrical_signal_represented_as_a_binary_number: $ => choice(
      $.high_voltage_electrical_signal_represented_as_a_binary_number,
      $.low_voltage_electrical_signal_represented_as_a_binary_number
    ),

    high_voltage_electrical_signal_represented_as_a_binary_number: _ => "1",
    low_voltage_electrical_signal_represented_as_a_binary_number: _ => "0",

    non_electrical_signal_represented_as_a_binary_number: _ => token(/[^01]+/)

  }
});
