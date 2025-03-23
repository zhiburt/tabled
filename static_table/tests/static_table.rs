use static_table::static_table;

use tabled::assert::test_table;

test_table!(
    static_table,
    static_table!([[1, 2, 123], [1, 2, 123], [1, 2, 123]]),
    "+---+---+-----+"
    "| 1 | 2 | 123 |"
    "+---+---+-----+"
    "| 1 | 2 | 123 |"
    "+---+---+-----+"
    "| 1 | 2 | 123 |"
    "+---+---+-----+"
);

test_table!(static_table_empty, static_table!([]), "");

test_table!(static_table_empty_row, static_table!([[]]), "");

test_table!(
    static_table_one_row,
    static_table!([[1, 2, 3]]),
    "+---+---+---+"
    "| 1 | 2 | 3 |"
    "+---+---+---+"
);

test_table!(
    static_table_repeat_row,
    static_table!([[1, 2, 3]; 5]),
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
    static_table_repeat_row_zero_times,
    static_table!([[1, 2, 3]; 0]),
    ""
);

test_table!(
    static_table_repeat_column,
    static_table!([["Hello World"; 2], ["hello", "world"]]),
    "+-------------+-------------+"
    "| Hello World | Hello World |"
    "+-------------+-------------+"
    "| hello       | world       |"
    "+-------------+-------------+"
);

test_table!(
    static_table_repeat_column_zero_times,
    static_table!([["Hello World"; 0]]),
    ""
);

test_table!(
    static_table_repeat_column_zero_times_with_other_row,
    static_table!([["Hello World"; 0], ["hello", "world"]]),
    "+-------+-------+"
    "|       |       |"
    "+-------+-------+"
    "| hello | world |"
    "+-------+-------+"
);

test_table!(
    static_table_byte_string,
    static_table!([[b"abcd"], [b"0123"]]),
    "+-------------------+"
    "| [97, 98, 99, 100] |"
    "+-------------------+"
    "| [48, 49, 50, 51]  |"
    "+-------------------+"
);

test_table!(
    static_table_byte_char,
    static_table!([[b'a'], [b'b']]),
    "+----+"
    "| 97 |"
    "+----+"
    "| 98 |"
    "+----+"
);

test_table!(
    static_table_char,
    static_table!([['a'], ['b']]),
    "+---+"
    "| a |"
    "+---+"
    "| b |"
    "+---+"
);

test_table!(
    static_table_bool,
    static_table!([[true], [false]]),
    "+-------+"
    "| true  |"
    "+-------+"
    "| false |"
    "+-------+"
);

test_table!(
    static_table_u64,
    static_table!([[0u64], [1u64]]),
    "+---+"
    "| 0 |"
    "+---+"
    "| 1 |"
    "+---+"
);

test_table!(
    static_table_u32,
    static_table!([[0u32], [1u32]]),
    "+---+"
    "| 0 |"
    "+---+"
    "| 1 |"
    "+---+"
);

test_table!(
    static_table_u16,
    static_table!([[0u16], [1u16]]),
    "+---+"
    "| 0 |"
    "+---+"
    "| 1 |"
    "+---+"
);

test_table!(
    static_table_u8,
    static_table!([[0u8], [1u8]]),
    "+---+"
    "| 0 |"
    "+---+"
    "| 1 |"
    "+---+"
);

test_table!(
    static_table_i64,
    static_table!([[0i64], [1i64]]),
    "+---+"
    "| 0 |"
    "+---+"
    "| 1 |"
    "+---+"
);

test_table!(
    static_table_i32,
    static_table!([[0i32], [1i32]]),
    "+---+"
    "| 0 |"
    "+---+"
    "| 1 |"
    "+---+"
);

test_table!(
    static_table_i16,
    static_table!([[0i16], [1i16]]),
    "+---+"
    "| 0 |"
    "+---+"
    "| 1 |"
    "+---+"
);

test_table!(
    static_table_i8,
    static_table!([[0i8], [1i8]]),
    "+---+"
    "| 0 |"
    "+---+"
    "| 1 |"
    "+---+"
);

test_table!(
    static_table_isize,
    static_table!([[0isize], [1isize]]),
    "+---+"
    "| 0 |"
    "+---+"
    "| 1 |"
    "+---+"
);

test_table!(
    static_table_usize,
    static_table!([[0usize], [1usize]]),
    "+---+"
    "| 0 |"
    "+---+"
    "| 1 |"
    "+---+"
);

test_table!(
    static_table_isize_neg,
    static_table!([[-0isize], [-1isize]]),
    "+----+"
    "| -0 |"
    "+----+"
    "| -1 |"
    "+----+"
);

test_table!(
    static_table_f32,
    static_table!([[0.0f32], [0.1f32]]),
    "+-----+"
    "| 0.0 |"
    "+-----+"
    "| 0.1 |"
    "+-----+"
);

test_table!(
    static_table_f64,
    static_table!([[0.0f64], [0.1f64]]),
    "+-----+"
    "| 0.0 |"
    "+-----+"
    "| 0.1 |"
    "+-----+"
);

test_table!(
    static_table_f32_neg,
    static_table!([[-0.0f32], [-0.1f32]]),
    "+------+"
    "| -0.0 |"
    "+------+"
    "| -0.1 |"
    "+------+"
);

test_table!(
    static_table_f64_neg,
    static_table!([[-0.0f64], [-0.1f64]]),
    "+------+"
    "| -0.0 |"
    "+------+"
    "| -0.1 |"
    "+------+"
);

test_table!(
    static_table_dims_differences_0,
    static_table!([["Hello World"; 2], ["hello"]]),
    "+-------------+-------------+"
    "| Hello World | Hello World |"
    "+-------------+-------------+"
    "| hello       |             |"
    "+-------------+-------------+"
);

test_table!(
    static_table_dims_differences_1,
    static_table!([["Hello World"], ["hello", "world"]]),
    "+-------------+-------+"
    "| Hello World |       |"
    "+-------------+-------+"
    "| hello       | world |"
    "+-------------+-------+"
);

test_table!(
    static_table_dims_differences_2,
    static_table!([["Hello World"], ["hello", "world", "!!!"], ["hello", "world"]]),
    "+-------------+-------+-----+"
    "| Hello World |       |     |"
    "+-------------+-------+-----+"
    "| hello       | world | !!! |"
    "+-------------+-------+-----+"
    "| hello       | world |     |"
    "+-------------+-------+-----+"
);

test_table!(
    static_table_vspan_0,
    static_table!([
        [{
            "Hello World";
            2
        }],
        ["hello", "world", "!!!"],
        ["hello", "world"]
    ]),
    "+---------------+-----+"
    "| Hello World   |     |"
    "+-------+-------+-----+"
    "| hello | world | !!! |"
    "+-------+-------+-----+"
    "| hello | world |     |"
    "+-------+-------+-----+"
);

test_table!(
    static_table_vspan_1,
    static_table!([
        [{
            "Hello World";
            4
        }],
        [
            "hello",
            {
                "world";
                2
            },
            "!!!"
        ],
        ["hello", "world"]
    ]),
    "+------------------------+"
    "| Hello World            |"
    "+-------+----------+-----+"
    "| hello | world    | !!! |"
    "+-------+-------+--+-----+"
    "| hello | world |  |     |"
    "+-------+-------+--+-----+"
);

test_table!(
    static_table_vspan_2,
    static_table!([
        [{
            "Hello World";
            4
        }],
        [
            "hello",
            {
                "world";
                2
            },
            "!!!"
        ],
        [
            {
                "hello";
                2
            },
            {
                "world";
                2
            }
        ]
    ]),
    "+---------------------+"
    "| Hello World         |"
    "+-------+-------+-----+"
    "| hello | world | !!! |"
    "+-------+---+---+-----+"
    "| hello     | world   |"
    "+-----------+---------+"
);

test_table!(
    static_table_vspan_const,
    static_table!([
        [{
            "Hello World";
            4
        }; 2],
        ["hello", "world", "!!!"],
        ["hello", "world", "!!!"],
    ]),
    "+------------------------+-------------+"
    "| Hello World            | Hello World |"
    "+-------+-------+-----+--+----+--+--+--+"
    "| hello | world | !!! |  |    |  |  |  |"
    "+-------+-------+-----+--+----+--+--+--+"
    "| hello | world | !!! |  |    |  |  |  |"
    "+-------+-------+-----+--+----+--+--+--+"
);

test_table!(
    static_table_empty_layer_column,
    static_table!([[{}, 1, 2, {}], [{}, "world", "!!!"]]),
    "+--+-------+-----+--+"
    "|  | 1     | 2   |  |"
    "+--+-------+-----+--+"
    "|  | world | !!! |  |"
    "+--+-------+-----+--+"
);

test_table!(
    static_table_hspan_0,
    static_table!([
        [{ "Hello" }, "World"],
        [{}, "hello", "world", "!!!"],
        ["hello", "world"]
    ]),
    "+-------+-------+-------+-----+"
    "| Hello | World |       |     |"
    "|       +-------+-------+-----+"
    "|       | hello | world | !!! |"
    "+-------+-------+-------+-----+"
    "| hello | world |       |     |"
    "+-------+-------+-------+-----+"
);

test_table!(
    static_table_hspan_1,
    static_table!([
        [{ "Hello" }, "World"],
        [{}, "hello", "world", "!!!"],
        [{}, "hello", "world"]
    ]),
    "+-------+-------+-------+-----+"
    "| Hello | World |       |     |"
    "|       +-------+-------+-----+"
    "|       | hello | world | !!! |"
    "|       +-------+-------+-----+"
    "|       | hello | world |     |"
    "+-------+-------+-------+-----+"
);

test_table!(
    static_table_hspan_doesnot_for_for_layers,
    static_table!([
        [{"Hello", "World"}],
        [{}, "hello", "world", "!!!"],
        [{}, "hello", "world"]
    ]),
    "+-------+-------+-------+-----+"
    "| Hello | World |       |     |"
    "+-------+-------+-------+-----+"
    "|       | hello | world | !!! |"
    "+-------+-------+-------+-----+"
    "|       | hello | world |     |"
    "+-------+-------+-------+-----+"
);

test_table!(
    static_table_hspan_and_vspan_cell,
    static_table!([
        [
            {
                "Hello";
                2
            },
            "World"
        ],
        [{}, "hello", "world", "!!!"],
        [{}, "hello", "world"]
    ]),
    "+-------+-------+-----+"
    "| Hello | World |     |"
    "|       +-------+-----+"
    "|       | world | !!! |"
    "|       +-------+-----+"
    "|       | world |     |"
    "+---+---+-------+-----+"
);

test_table!(
    static_table_hspan_const,
    static_table!([[{ "Hello" }, { "World" }], [{}; 3], [{}, "hello", "world"]]),
    "+-------+-------+-------+"
    "| Hello | World |       |"
    "|       |       +-------+"
    "|       |       |       |"
    "|       +-------+-------+"
    "|       | hello | world |"
    "+-------+-------+-------+"
);

test_table!(
    static_table_vspan_const_row,
    static_table!([[{"Hello World!"; 3}]]),
    "+--------------+"
    "| Hello World! |"
    "+--------------+"
);

test_table!(
    static_table_with_theme_padding_margin,
    static_table!(
        [[1, 2, 123], [1, 2, 123], [1, 2, 123]],
        MARGIN = "1, 1, 1, 1",
        PADDING = "2, 2, 0, 0",
        THEME = "ROUNDED"
    ),
    "                       "
    " ╭─────┬─────┬───────╮ "
    " │  1  │  2  │  123  │ "
    " ├─────┼─────┼───────┤ "
    " │  1  │  2  │  123  │ "
    " │  1  │  2  │  123  │ "
    " ╰─────┴─────┴───────╯ "
    "                       "
);
