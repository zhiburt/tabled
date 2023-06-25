use static_table::pool_table;

use testing_table::test_table;

test_table!(
    pool_table,
    pool_table!([[1, 2, 123], [1, 2, 123], [1, 2, 123]]),
    "+---+---+-----+"
    "| 1 | 2 | 123 |"
    "+---+---+-----+"
    "| 1 | 2 | 123 |"
    "+---+---+-----+"
    "| 1 | 2 | 123 |"
    "+---+---+-----+"
);

test_table!(
    pool_table_empty,
    pool_table!([]),
    "+--+"
    "|  |"
    "+--+"
);

test_table!(
    pool_table_empty_row,
    pool_table!([[]]),
    "+--+"
    "|  |"
    "+--+"
);

test_table!(
    pool_table_one_row,
    pool_table!([[1, 2, 3]]),
    "+---+---+---+"
    "| 1 | 2 | 3 |"
    "+---+---+---+"
);

test_table!(
    pool_table_repeat_row,
    pool_table!([[1, 2, 3]; 5]),
    "+---+---+---+"
    "| 1 | 2 | 3 |"
    "+---+---+---+"
    "| 1 | 2 | 3 |"
    "+---+---+---+"
    "| 1 | 2 | 3 |"
    "+---+---+---+"
    "| 1 | 2 | 3 |"
    "+---+---+---+"
    "| 1 | 2 | 3 |"
    "+---+---+---+"
);

test_table!(
    pool_table_repeat_row_zero_times,
    pool_table!([[1, 2, 3]; 0]),
    "+--+"
    "|  |"
    "+--+"
);

test_table!(
    pool_table_repeat_column,
    pool_table!([["Hello World"; 2], ["hello", "world"]]),
    "+-------------+-------------+"
    "| Hello World | Hello World |"
    "+-------------+-------------+"
    "| hello       | world       |"
    "+-------------+-------------+"
);

test_table!(
    pool_table_repeat_column_zero_times,
    pool_table!([["Hello World"; 0]]),
    "+--+"
    "|  |"
    "+--+"
);

test_table!(
    pool_table_repeat_column_zero_times_with_other_row,
    pool_table!([["Hello World"; 0], ["hello", "world"]]),
    "+---------------+"
    "|               |"
    "+-------+-------+"
    "| hello | world |"
    "+-------+-------+"
);

test_table!(
    pool_table_byte_string,
    pool_table!([[b"abcd"], [b"0123"]]),
    "+-------------------+"
    "| [97, 98, 99, 100] |"
    "+-------------------+"
    "| [48, 49, 50, 51]  |"
    "+-------------------+"
);

test_table!(
    pool_table_byte_char,
    pool_table!([[b'a'], [b'b']]),
    "+----+"
    "| 97 |"
    "+----+"
    "| 98 |"
    "+----+"
);

test_table!(
    pool_table_char,
    pool_table!([['a'], ['b']]),
    "+---+"
    "| a |"
    "+---+"
    "| b |"
    "+---+"
);

test_table!(
    pool_table_bool,
    pool_table!([[true], [false]]),
    "+-------+"
    "| true  |"
    "+-------+"
    "| false |"
    "+-------+"
);

test_table!(
    pool_table_u64,
    pool_table!([[0u64], [1u64]]),
    "+---+"
    "| 0 |"
    "+---+"
    "| 1 |"
    "+---+"
);

test_table!(
    pool_table_u32,
    pool_table!([[0u32], [1u32]]),
    "+---+"
    "| 0 |"
    "+---+"
    "| 1 |"
    "+---+"
);

test_table!(
    pool_table_u16,
    pool_table!([[0u16], [1u16]]),
    "+---+"
    "| 0 |"
    "+---+"
    "| 1 |"
    "+---+"
);

test_table!(
    pool_table_u8,
    pool_table!([[0u8], [1u8]]),
    "+---+"
    "| 0 |"
    "+---+"
    "| 1 |"
    "+---+"
);

test_table!(
    pool_table_i64,
    pool_table!([[0i64], [1i64]]),
    "+---+"
    "| 0 |"
    "+---+"
    "| 1 |"
    "+---+"
);

test_table!(
    pool_table_i32,
    pool_table!([[0i32], [1i32]]),
    "+---+"
    "| 0 |"
    "+---+"
    "| 1 |"
    "+---+"
);

test_table!(
    pool_table_i16,
    pool_table!([[0i16], [1i16]]),
    "+---+"
    "| 0 |"
    "+---+"
    "| 1 |"
    "+---+"
);

test_table!(
    pool_table_i8,
    pool_table!([[0i8], [1i8]]),
    "+---+"
    "| 0 |"
    "+---+"
    "| 1 |"
    "+---+"
);

test_table!(
    pool_table_isize,
    pool_table!([[0isize], [1isize]]),
    "+---+"
    "| 0 |"
    "+---+"
    "| 1 |"
    "+---+"
);

test_table!(
    pool_table_isize_minus,
    pool_table!([[-0isize], [-1isize]]),
    "+----+"
    "| -0 |"
    "+----+"
    "| -1 |"
    "+----+"
);

test_table!(
    pool_table_usize,
    pool_table!([[0usize], [1usize]]),
    "+---+"
    "| 0 |"
    "+---+"
    "| 1 |"
    "+---+"
);

test_table!(
    pool_table_f32,
    pool_table!([[0.0f32], [0.1f32]]),
    "+-----+"
    "| 0.0 |"
    "+-----+"
    "| 0.1 |"
    "+-----+"
);

test_table!(
    pool_table_f32_minus,
    pool_table!([[-0.0f32], [-0.1f32]]),
    "+------+"
    "| -0.0 |"
    "+------+"
    "| -0.1 |"
    "+------+"
);

test_table!(
    pool_table_f64,
    pool_table!([[0.0f64], [0.1f64]]),
    "+-----+"
    "| 0.0 |"
    "+-----+"
    "| 0.1 |"
    "+-----+"
);

test_table!(
    pool_table_f64_minus,
    pool_table!([[-0.0f64], [-0.1f64]]),
    "+------+"
    "| -0.0 |"
    "+------+"
    "| -0.1 |"
    "+------+"
);

test_table!(
    pool_table_dims_differences_0,
    pool_table!([["Hello World"; 2], ["hello"]]),
    "+-------------+-------------+"
    "| Hello World | Hello World |"
    "+-------------+-------------+"
    "| hello                     |"
    "+---------------------------+"
);

test_table!(
    pool_table_dims_differences_1,
    pool_table!([["Hello World"], ["hello", "world"]]),
    "+---------------+"
    "| Hello World   |"
    "+-------+-------+"
    "| hello | world |"
    "+-------+-------+"
);

test_table!(
    pool_table_dims_differences_2,
    pool_table!([
        ["Hello World"],
        ["hello", "world", "!!!"],
        ["hello", "world"]
    ]),
    "+---------------------+"
    "| Hello World         |"
    "+-------+-------+-----+"
    "| hello | world | !!! |"
    "+-------+--+----+-----+"
    "| hello    | world    |"
    "+----------+----------+"
);

test_table!(
    pool_table_with_theme_padding_margin,
    pool_table!(
        [[1, 2, 123], [1, 2, 123], [1, 2, 123]],
        MARGIN = "1, 1, 1, 1",
        PADDING = "2, 2, 0, 0",
        THEME = "ROUNDED"
    ),
    "                       "
    " ╭─────┬─────┬───────╮ "
    " │  1  │  2  │  123  │ "
    "  ───── ───── ───────  "
    " │  1  │  2  │  123  │ "
    "  ───── ───── ───────  "
    " │  1  │  2  │  123  │ "
    " ╰─────┴─────┴───────╯ "
    "                       "
);

test_table!(
    pool_table_with_theme,
    pool_table!(
        [["Hello World"], [1, 2, 123], [1, 2, 3, 4, 5, 6, 7, 8, 9],],
        THEME = "EXTENDED"
    ),
    "╔═══════════════════════════════════╗"
    "║ Hello World                       ║"
    "╠═══════════╦══════════╦════════════╣"
    "║ 1         ║ 2        ║ 123        ║"
    "╠═══╦═══╦═══╬═══╦═══╦══╩╦═══╦═══╦═══╣"
    "║ 1 ║ 2 ║ 3 ║ 4 ║ 5 ║ 6 ║ 7 ║ 8 ║ 9 ║"
    "╚═══╩═══╩═══╩═══╩═══╩═══╩═══╩═══╩═══╝"
);
