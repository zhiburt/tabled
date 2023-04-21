use static_table::pool_table;

#[test]
fn pool_table() {
    let table = pool_table!([[1, 2, 123], [1, 2, 123], [1, 2, 123]]);
    let expected = "+---+---+-----+\n\
                          | 1 | 2 | 123 |\n\
                          +---+---+-----+\n\
                          | 1 | 2 | 123 |\n\
                          +---+---+-----+\n\
                          | 1 | 2 | 123 |\n\
                          +---+---+-----+";
    assert_eq!(table, expected);
}

#[test]
fn pool_table_empty() {
    let table = pool_table!([]);
    let expected = "+--+\n|  |\n+--+";
    assert_eq!(table, expected);
}

#[test]
fn pool_table_empty_row() {
    let table = pool_table!([[]]);
    let expected = "+--+\n|  |\n+--+";
    assert_eq!(table, expected);
}

#[test]
fn pool_table_one_row() {
    let table = pool_table!([[1, 2, 3]]);
    let expected = "+---+---+---+\n\
                          | 1 | 2 | 3 |\n\
                          +---+---+---+";
    assert_eq!(table, expected);
}

#[test]
fn pool_table_repeat_row() {
    let table = pool_table!([[1, 2, 3]; 5]);
    let expected = "+---+---+---+\n\
                          | 1 | 2 | 3 |\n\
                          +---+---+---+\n\
                          | 1 | 2 | 3 |\n\
                          +---+---+---+\n\
                          | 1 | 2 | 3 |\n\
                          +---+---+---+\n\
                          | 1 | 2 | 3 |\n\
                          +---+---+---+\n\
                          | 1 | 2 | 3 |\n\
                          +---+---+---+";
    assert_eq!(table, expected);
}

#[test]
fn pool_table_repeat_row_zero_times() {
    let table = pool_table!([[1, 2, 3]; 0]);
    let expected = "+--+\n|  |\n+--+";
    assert_eq!(table, expected);
}

#[test]
fn pool_table_repeat_column() {
    let table = pool_table!([["Hello World"; 2], ["hello", "world"]]);
    let expected = "+-------------+-------------+\n\
                          | Hello World | Hello World |\n\
                          +-------------+-------------+\n\
                          | hello       | world       |\n\
                          +-------------+-------------+";
    assert_eq!(table, expected);
}

#[test]
fn pool_table_repeat_column_zero_times() {
    let table = pool_table!([["Hello World"; 0]]);
    let expected = "+--+\n|  |\n+--+";
    assert_eq!(table, expected);
}

#[test]
fn pool_table_repeat_column_zero_times_with_other_row() {
    let table = pool_table!([["Hello World"; 0], ["hello", "world"]]);
    let expected = "+---------------+\n\
                          |               |\n\
                          +-------+-------+\n\
                          | hello | world |\n\
                          +-------+-------+";
    assert_eq!(table, expected);
}

#[test]
fn pool_table_byte_string() {
    let table = pool_table!([[b"abcd"], [b"0123"]]);
    let expected = "+-------------------+\n\
                          | [97, 98, 99, 100] |\n\
                          +-------------------+\n\
                          | [48, 49, 50, 51]  |\n\
                          +-------------------+";
    assert_eq!(table, expected);
}

#[test]
fn pool_table_byte_char() {
    let table = pool_table!([[b'a'], [b'b']]);
    let expected = "+----+\n\
                          | 97 |\n\
                          +----+\n\
                          | 98 |\n\
                          +----+";
    assert_eq!(table, expected);
}

#[test]
fn pool_table_char() {
    let table = pool_table!([['a'], ['b']]);
    let expected = "+---+\n\
                          | a |\n\
                          +---+\n\
                          | b |\n\
                          +---+";
    assert_eq!(table, expected);
}

#[test]
fn pool_table_bool() {
    let table = pool_table!([[true], [false]]);
    let expected = "+-------+\n\
                          | true  |\n\
                          +-------+\n\
                          | false |\n\
                          +-------+";
    assert_eq!(table, expected);
}

#[test]
fn pool_table_u64() {
    let table = pool_table!([[0u64], [1u64]]);
    let expected = "+---+\n\
                          | 0 |\n\
                          +---+\n\
                          | 1 |\n\
                          +---+";
    assert_eq!(table, expected);
}

#[test]
fn pool_table_u32() {
    let table = pool_table!([[0u32], [1u32]]);
    let expected = "+---+\n\
                          | 0 |\n\
                          +---+\n\
                          | 1 |\n\
                          +---+";
    assert_eq!(table, expected);
}

#[test]
fn pool_table_u16() {
    let table = pool_table!([[0u16], [1u16]]);
    let expected = "+---+\n\
                          | 0 |\n\
                          +---+\n\
                          | 1 |\n\
                          +---+";
    assert_eq!(table, expected);
}

#[test]
fn pool_table_u8() {
    let table = pool_table!([[0u8], [1u8]]);
    let expected = "+---+\n\
                          | 0 |\n\
                          +---+\n\
                          | 1 |\n\
                          +---+";
    assert_eq!(table, expected);
}

#[test]
fn pool_table_i64() {
    let table = pool_table!([[0u64], [1u64]]);
    let expected = "+---+\n\
                          | 0 |\n\
                          +---+\n\
                          | 1 |\n\
                          +---+";
    assert_eq!(table, expected);
}

#[test]
fn pool_table_i32() {
    let table = pool_table!([[0u32], [1u32]]);
    let expected = "+---+\n\
                          | 0 |\n\
                          +---+\n\
                          | 1 |\n\
                          +---+";
    assert_eq!(table, expected);
}

#[test]
fn pool_table_i16() {
    let table = pool_table!([[0u16], [1u16]]);
    let expected = "+---+\n\
                          | 0 |\n\
                          +---+\n\
                          | 1 |\n\
                          +---+";
    assert_eq!(table, expected);
}

#[test]
fn pool_table_i8() {
    let table = pool_table!([[0u8], [1u8]]);
    let expected = "+---+\n\
                          | 0 |\n\
                          +---+\n\
                          | 1 |\n\
                          +---+";
    assert_eq!(table, expected);
}

#[test]
fn pool_table_isize() {
    let table = pool_table!([[0isize], [1isize]]);
    let expected = "+---+\n\
                          | 0 |\n\
                          +---+\n\
                          | 1 |\n\
                          +---+";
    assert_eq!(table, expected);
}

#[test]
fn pool_table_isize_minus() {
    let table = pool_table!([[-0], [-1isize]]);
    let expected = "+----+\n\
                          | -0 |\n\
                          +----+\n\
                          | -1 |\n\
                          +----+";
    assert_eq!(table, expected);
}

#[test]
fn pool_table_usize() {
    let table = pool_table!([[0usize], [1usize]]);
    let expected = "+---+\n\
                          | 0 |\n\
                          +---+\n\
                          | 1 |\n\
                          +---+";
    assert_eq!(table, expected);
}

#[test]
fn pool_table_float32() {
    let table = pool_table!([[0.0f32], [0.1f32]]);
    let expected = "+-----+\n\
                          | 0.0 |\n\
                          +-----+\n\
                          | 0.1 |\n\
                          +-----+";
    assert_eq!(table, expected);
}

#[test]
fn pool_table_float64() {
    let table = pool_table!([[0.0f64], [0.1f64]]);
    let expected = "+-----+\n\
                          | 0.0 |\n\
                          +-----+\n\
                          | 0.1 |\n\
                          +-----+";
    assert_eq!(table, expected);
}

#[test]
fn pool_table_float64_minus() {
    let table = pool_table!([[-0.0f64], [0.1f64]]);
    let expected = "+------+\n\
                          | -0.0 |\n\
                          +------+\n\
                          | 0.1  |\n\
                          +------+";
    assert_eq!(table, expected);
}

#[test]
fn pool_table_dims_differences_0() {
    let table = pool_table!([["Hello World"; 2], ["hello"]]);
    let expected = "+-------------+-------------+\n\
                          | Hello World | Hello World |\n\
                          +-------------+-------------+\n\
                          | hello                     |\n\
                          +---------------------------+";
    assert_eq!(table, expected);
}

#[test]
fn pool_table_dims_differences_1() {
    let table = pool_table!([["Hello World"], ["hello", "world"]]);
    let expected = "+---------------+\n\
                          | Hello World   |\n\
                          +-------+-------+\n\
                          | hello | world |\n\
                          +-------+-------+";
    assert_eq!(table, expected);
}

#[test]
fn pool_table_dims_differences_2() {
    let table = pool_table!([
        ["Hello World"],
        ["hello", "world", "!!!"],
        ["hello", "world"]
    ]);
    let expected = "+---------------------+\n\
                          | Hello World         |\n\
                          +-------+-------+-----+\n\
                          | hello | world | !!! |\n\
                          +-------+--+----+-----+\n\
                          | hello    | world    |\n\
                          +----------+----------+";
    assert_eq!(table, expected);
}

#[test]
fn pool_table_with_theme_padding_margin() {
    let table = pool_table!(
        [[1, 2, 123], [1, 2, 123], [1, 2, 123]],
        MARGIN = "1, 1, 1, 1",
        PADDING = "2, 2, 0, 0",
        THEME = "ROUNDED"
    );
    let expected = concat!(
        "                       \n",
        " ╭─────┬─────┬───────╮ \n",
        " │  1  │  2  │  123  │ \n",
        "  ───── ───── ───────  \n",
        " │  1  │  2  │  123  │ \n",
        "  ───── ───── ───────  \n",
        " │  1  │  2  │  123  │ \n",
        " ╰─────┴─────┴───────╯ \n",
        "                       ",
    );
    assert_eq!(table, expected);
}

#[test]
fn pool_table_with_theme() {
    let table = pool_table!(
        [["Hello World"], [1, 2, 123], [1, 2, 3, 4, 5, 6, 7, 8, 9],],
        THEME = "EXTENDED"
    );
    let expected = concat!(
        "╔═══════════════════════════════════╗\n",
        "║ Hello World                       ║\n",
        "╠═══════════╦══════════╦════════════╣\n",
        "║ 1         ║ 2        ║ 123        ║\n",
        "╠═══╦═══╦═══╬═══╦═══╦══╩╦═══╦═══╦═══╣\n",
        "║ 1 ║ 2 ║ 3 ║ 4 ║ 5 ║ 6 ║ 7 ║ 8 ║ 9 ║\n",
        "╚═══╩═══╩═══╩═══╩═══╩═══╩═══╩═══╩═══╝",
    );
    assert_eq!(table, expected);
}
