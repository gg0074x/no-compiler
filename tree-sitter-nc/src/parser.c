#include "tree_sitter/parser.h"

#if defined(__GNUC__) || defined(__clang__)
#pragma GCC diagnostic ignored "-Wmissing-field-initializers"
#endif

#define LANGUAGE_VERSION 14
#define STATE_COUNT 6
#define LARGE_STATE_COUNT 5
#define SYMBOL_COUNT 7
#define ALIAS_COUNT 0
#define TOKEN_COUNT 4
#define EXTERNAL_TOKEN_COUNT 0
#define FIELD_COUNT 0
#define MAX_ALIAS_SEQUENCE_LENGTH 2
#define PRODUCTION_ID_COUNT 1

enum ts_symbol_identifiers {
  sym_high_voltage_electrical_signal_represented_as_a_binary_number = 1,
  sym_low_voltage_electrical_signal_represented_as_a_binary_number = 2,
  sym_non_electrical_signal_represented_as_a_binary_number = 3,
  sym_source_file = 4,
  sym_electrical_signal_represented_as_a_binary_number = 5,
  aux_sym_source_file_repeat1 = 6,
};

static const char * const ts_symbol_names[] = {
  [ts_builtin_sym_end] = "end",
  [sym_high_voltage_electrical_signal_represented_as_a_binary_number] = "high_voltage_electrical_signal_represented_as_a_binary_number",
  [sym_low_voltage_electrical_signal_represented_as_a_binary_number] = "low_voltage_electrical_signal_represented_as_a_binary_number",
  [sym_non_electrical_signal_represented_as_a_binary_number] = "non_electrical_signal_represented_as_a_binary_number",
  [sym_source_file] = "source_file",
  [sym_electrical_signal_represented_as_a_binary_number] = "electrical_signal_represented_as_a_binary_number",
  [aux_sym_source_file_repeat1] = "source_file_repeat1",
};

static const TSSymbol ts_symbol_map[] = {
  [ts_builtin_sym_end] = ts_builtin_sym_end,
  [sym_high_voltage_electrical_signal_represented_as_a_binary_number] = sym_high_voltage_electrical_signal_represented_as_a_binary_number,
  [sym_low_voltage_electrical_signal_represented_as_a_binary_number] = sym_low_voltage_electrical_signal_represented_as_a_binary_number,
  [sym_non_electrical_signal_represented_as_a_binary_number] = sym_non_electrical_signal_represented_as_a_binary_number,
  [sym_source_file] = sym_source_file,
  [sym_electrical_signal_represented_as_a_binary_number] = sym_electrical_signal_represented_as_a_binary_number,
  [aux_sym_source_file_repeat1] = aux_sym_source_file_repeat1,
};

static const TSSymbolMetadata ts_symbol_metadata[] = {
  [ts_builtin_sym_end] = {
    .visible = false,
    .named = true,
  },
  [sym_high_voltage_electrical_signal_represented_as_a_binary_number] = {
    .visible = true,
    .named = true,
  },
  [sym_low_voltage_electrical_signal_represented_as_a_binary_number] = {
    .visible = true,
    .named = true,
  },
  [sym_non_electrical_signal_represented_as_a_binary_number] = {
    .visible = true,
    .named = true,
  },
  [sym_source_file] = {
    .visible = true,
    .named = true,
  },
  [sym_electrical_signal_represented_as_a_binary_number] = {
    .visible = true,
    .named = true,
  },
  [aux_sym_source_file_repeat1] = {
    .visible = false,
    .named = false,
  },
};

static const TSSymbol ts_alias_sequences[PRODUCTION_ID_COUNT][MAX_ALIAS_SEQUENCE_LENGTH] = {
  [0] = {0},
};

static const uint16_t ts_non_terminal_alias_map[] = {
  0,
};

static const TSStateId ts_primary_state_ids[STATE_COUNT] = {
  [0] = 0,
  [1] = 1,
  [2] = 2,
  [3] = 3,
  [4] = 4,
  [5] = 5,
};

static bool ts_lex(TSLexer *lexer, TSStateId state) {
  START_LEXER();
  eof = lexer->eof(lexer);
  switch (state) {
    case 0:
      if (eof) ADVANCE(1);
      if (lookahead == '0') ADVANCE(3);
      if (lookahead == '1') ADVANCE(2);
      if (('\t' <= lookahead && lookahead <= '\r') ||
          lookahead == ' ') ADVANCE(4);
      if (lookahead != 0) ADVANCE(5);
      END_STATE();
    case 1:
      ACCEPT_TOKEN(ts_builtin_sym_end);
      END_STATE();
    case 2:
      ACCEPT_TOKEN(sym_high_voltage_electrical_signal_represented_as_a_binary_number);
      END_STATE();
    case 3:
      ACCEPT_TOKEN(sym_low_voltage_electrical_signal_represented_as_a_binary_number);
      END_STATE();
    case 4:
      ACCEPT_TOKEN(sym_non_electrical_signal_represented_as_a_binary_number);
      if (('\t' <= lookahead && lookahead <= '\r') ||
          lookahead == ' ') ADVANCE(4);
      if (lookahead != 0 &&
          lookahead != '0' &&
          lookahead != '1') ADVANCE(5);
      END_STATE();
    case 5:
      ACCEPT_TOKEN(sym_non_electrical_signal_represented_as_a_binary_number);
      if (lookahead != 0 &&
          lookahead != '0' &&
          lookahead != '1') ADVANCE(5);
      END_STATE();
    default:
      return false;
  }
}

static const TSLexMode ts_lex_modes[STATE_COUNT] = {
  [0] = {.lex_state = 0},
  [1] = {.lex_state = 0},
  [2] = {.lex_state = 0},
  [3] = {.lex_state = 0},
  [4] = {.lex_state = 0},
  [5] = {.lex_state = 0},
};

static const uint16_t ts_parse_table[LARGE_STATE_COUNT][SYMBOL_COUNT] = {
  [0] = {
    [ts_builtin_sym_end] = ACTIONS(1),
    [sym_high_voltage_electrical_signal_represented_as_a_binary_number] = ACTIONS(1),
    [sym_low_voltage_electrical_signal_represented_as_a_binary_number] = ACTIONS(1),
    [sym_non_electrical_signal_represented_as_a_binary_number] = ACTIONS(1),
  },
  [1] = {
    [sym_source_file] = STATE(5),
    [sym_electrical_signal_represented_as_a_binary_number] = STATE(2),
    [aux_sym_source_file_repeat1] = STATE(2),
    [ts_builtin_sym_end] = ACTIONS(3),
    [sym_high_voltage_electrical_signal_represented_as_a_binary_number] = ACTIONS(5),
    [sym_low_voltage_electrical_signal_represented_as_a_binary_number] = ACTIONS(5),
    [sym_non_electrical_signal_represented_as_a_binary_number] = ACTIONS(7),
  },
  [2] = {
    [sym_electrical_signal_represented_as_a_binary_number] = STATE(3),
    [aux_sym_source_file_repeat1] = STATE(3),
    [ts_builtin_sym_end] = ACTIONS(9),
    [sym_high_voltage_electrical_signal_represented_as_a_binary_number] = ACTIONS(5),
    [sym_low_voltage_electrical_signal_represented_as_a_binary_number] = ACTIONS(5),
    [sym_non_electrical_signal_represented_as_a_binary_number] = ACTIONS(11),
  },
  [3] = {
    [sym_electrical_signal_represented_as_a_binary_number] = STATE(3),
    [aux_sym_source_file_repeat1] = STATE(3),
    [ts_builtin_sym_end] = ACTIONS(13),
    [sym_high_voltage_electrical_signal_represented_as_a_binary_number] = ACTIONS(15),
    [sym_low_voltage_electrical_signal_represented_as_a_binary_number] = ACTIONS(15),
    [sym_non_electrical_signal_represented_as_a_binary_number] = ACTIONS(18),
  },
  [4] = {
    [ts_builtin_sym_end] = ACTIONS(21),
    [sym_high_voltage_electrical_signal_represented_as_a_binary_number] = ACTIONS(23),
    [sym_low_voltage_electrical_signal_represented_as_a_binary_number] = ACTIONS(23),
    [sym_non_electrical_signal_represented_as_a_binary_number] = ACTIONS(21),
  },
};

static const uint16_t ts_small_parse_table[] = {
  [0] = 1,
    ACTIONS(25), 1,
      ts_builtin_sym_end,
};

static const uint32_t ts_small_parse_table_map[] = {
  [SMALL_STATE(5)] = 0,
};

static const TSParseActionEntry ts_parse_actions[] = {
  [0] = {.entry = {.count = 0, .reusable = false}},
  [1] = {.entry = {.count = 1, .reusable = false}}, RECOVER(),
  [3] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_source_file, 0, 0, 0),
  [5] = {.entry = {.count = 1, .reusable = false}}, SHIFT(4),
  [7] = {.entry = {.count = 1, .reusable = true}}, SHIFT(2),
  [9] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_source_file, 1, 0, 0),
  [11] = {.entry = {.count = 1, .reusable = true}}, SHIFT(3),
  [13] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_source_file_repeat1, 2, 0, 0),
  [15] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym_source_file_repeat1, 2, 0, 0), SHIFT_REPEAT(4),
  [18] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_source_file_repeat1, 2, 0, 0), SHIFT_REPEAT(3),
  [21] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_electrical_signal_represented_as_a_binary_number, 1, 0, 0),
  [23] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_electrical_signal_represented_as_a_binary_number, 1, 0, 0),
  [25] = {.entry = {.count = 1, .reusable = true}},  ACCEPT_INPUT(),
};

#ifdef __cplusplus
extern "C" {
#endif
#ifdef TREE_SITTER_HIDE_SYMBOLS
#define TS_PUBLIC
#elif defined(_WIN32)
#define TS_PUBLIC __declspec(dllexport)
#else
#define TS_PUBLIC __attribute__((visibility("default")))
#endif

TS_PUBLIC const TSLanguage *tree_sitter_nc(void) {
  static const TSLanguage language = {
    .version = LANGUAGE_VERSION,
    .symbol_count = SYMBOL_COUNT,
    .alias_count = ALIAS_COUNT,
    .token_count = TOKEN_COUNT,
    .external_token_count = EXTERNAL_TOKEN_COUNT,
    .state_count = STATE_COUNT,
    .large_state_count = LARGE_STATE_COUNT,
    .production_id_count = PRODUCTION_ID_COUNT,
    .field_count = FIELD_COUNT,
    .max_alias_sequence_length = MAX_ALIAS_SEQUENCE_LENGTH,
    .parse_table = &ts_parse_table[0][0],
    .small_parse_table = ts_small_parse_table,
    .small_parse_table_map = ts_small_parse_table_map,
    .parse_actions = ts_parse_actions,
    .symbol_names = ts_symbol_names,
    .symbol_metadata = ts_symbol_metadata,
    .public_symbol_map = ts_symbol_map,
    .alias_map = ts_non_terminal_alias_map,
    .alias_sequences = &ts_alias_sequences[0][0],
    .lex_modes = ts_lex_modes,
    .lex_fn = ts_lex,
    .primary_state_ids = ts_primary_state_ids,
  };
  return &language;
}
#ifdef __cplusplus
}
#endif
