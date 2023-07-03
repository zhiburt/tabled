#![cfg(feature = "std")]

use tabled::{
    grid::dimension::{DimensionPriority, PoolTableDimension},
    settings::{formatting::AlignmentStrategy, Alignment, Margin, Padding, Style},
    tables::{PoolTable, TableValue},
};

use crate::matrix::Matrix;
use testing_table::test_table;

#[cfg(feature = "color")]
use tabled::grid::color::StaticColor;

test_table!(
    pool_table,
    PoolTable::new(Matrix::with_no_frame(3, 3).to_vec()),
    "+-----+-----+-----+"
    "| 0-0 | 0-1 | 0-2 |"
    "+-----+-----+-----+"
    "| 1-0 | 1-1 | 1-2 |"
    "+-----+-----+-----+"
    "| 2-0 | 2-1 | 2-2 |"
    "+-----+-----+-----+"
);

test_table!(
    pool_table_1,
    PoolTable::new([vec!["111111", "222"], vec!["111", "2233", "1", "2", "3"]]),
    "+-------------+----------+"
    "| 111111      | 222      |"
    "+-----+------++--+---+---+"
    "| 111 | 2233 | 1 | 2 | 3 |"
    "+-----+------+---+---+---+"
);

test_table!(
    pool_table_2,
    PoolTable::new([vec!["111", "2233", "1", "2", "3"], vec!["111111", "222"]]),
    "+-----+------+---+---+---+"
    "| 111 | 2233 | 1 | 2 | 3 |"
    "+-----+------++--+---+---+"
    "| 111111      | 222      |"
    "+-------------+----------+"
);

test_table!(
    pool_table_3,
    PoolTable::new([vec!["1\n11", "2\n2\n3\n3", "1", "\n2\n", "3"], vec!["11\n111\n1", "2\n2\n2"]]),
    "+----+---+---+---+---+"
    "| 1  | 2 | 1 |   | 3 |"
    "| 11 | 2 |   | 2 |   |"
    "|    | 3 |   |   |   |"
    "|    | 3 |   |   |   |"
    "+----+---+--++---+---+"
    "| 11        | 2      |"
    "| 111       | 2      |"
    "| 1         | 2      |"
    "+-----------+--------+"
);

test_table!(
    pool_table_4,
    PoolTable::new([vec!["11\n111\n1", "2\n2\n2"], vec!["1\n11", "2\n2\n3\n3", "1", "\n2\n", "3"]]),
    "+-----------+--------+"
    "| 11        | 2      |"
    "| 111       | 2      |"
    "| 1         | 2      |"
    "+----+---+--++---+---+"
    "| 1  | 2 | 1 |   | 3 |"
    "| 11 | 2 |   | 2 |   |"
    "|    | 3 |   |   |   |"
    "|    | 3 |   |   |   |"
    "+----+---+---+---+---+"
);

test_table!(
    pool_table_multiline,
    PoolTable::new([
        ["1", "2\n2", "3\n3\n3"],
        ["3\n3\n3", "2\n2", "1"],
        ["1", "3\n3\n3", "2\n2"]
    ]),
    "+---+---+---+"
    "| 1 | 2 | 3 |"
    "|   | 2 | 3 |"
    "|   |   | 3 |"
    "+---+---+---+"
    "| 3 | 2 | 1 |"
    "| 3 | 2 |   |"
    "| 3 |   |   |"
    "+---+---+---+"
    "| 1 | 3 | 2 |"
    "|   | 3 | 2 |"
    "|   | 3 |   |"
    "+---+---+---+"
);

test_table!(
    pool_table_value,
    PoolTable::from(TableValue::Row(vec![
        TableValue::Column(vec![TableValue::Cell(String::from("0-0")), TableValue::Cell(String::from("0-1")), TableValue::Cell(String::from("0-2"))]),
        TableValue::Column(vec![TableValue::Cell(String::from("1-0")), TableValue::Cell(String::from("1-1")), TableValue::Cell(String::from("1-2"))]),
        TableValue::Column(vec![TableValue::Cell(String::from("2-0")), TableValue::Cell(String::from("2-1")), TableValue::Cell(String::from("2-2"))]),
    ]))
    .with(Style::modern()),
    "┌─────┬─────┬─────┐"
    "│ 0-0 │ 1-0 │ 2-0 │"
    "├─────┼─────┼─────┤"
    "│ 0-1 │ 1-1 │ 2-1 │"
    "├─────┼─────┼─────┤"
    "│ 0-2 │ 1-2 │ 2-2 │"
    "└─────┴─────┴─────┘"
);

test_table!(
    pool_table_value_1,
    PoolTable::from(TableValue::Row(vec![
        TableValue::Column(vec![TableValue::Cell(String::from("0-0")), TableValue::Cell(String::from("0-1")), TableValue::Cell(String::from("0-2"))]),
        TableValue::Column(vec![TableValue::Cell(String::from("1-0")), TableValue::Cell(String::from("1-1")), TableValue::Cell(String::from("1-2"))]),
        TableValue::Column(vec![
            TableValue::Column(vec![TableValue::Cell(String::from("2-01")), TableValue::Cell(String::from("2-02")), TableValue::Cell(String::from("2-03"))]),
            TableValue::Cell(String::from("2-1")),
            TableValue::Cell(String::from("2-2")),
        ]),
    ]))
    .with(Style::modern()),
    "┌─────┬─────┬──────┐"
    "│ 0-0 │ 1-0 │ 2-01 │"
    "│     │     ├──────┤"
    "│     │     │ 2-02 │"
    "├─────┼─────┼──────┤"
    "│ 0-1 │ 1-1 │ 2-03 │"
    "│     │     ├──────┤"
    "├─────┼─────┤ 2-1  │"
    "│ 0-2 │ 1-2 ├──────┤"
    "│     │     │ 2-2  │"
    "└─────┴─────┴──────┘"
);

test_table!(
    pool_table_value_2,
    PoolTable::from(TableValue::Row(vec![
        TableValue::Column(vec![TableValue::Cell(String::from("0-0")), TableValue::Cell(String::from("0-1")), TableValue::Cell(String::from("0-2"))]),
        TableValue::Column(vec![
            TableValue::Row(vec![TableValue::Cell(String::from("2-01")), TableValue::Cell(String::from("2-02")), TableValue::Cell(String::from("2-03"))]),
            TableValue::Column(vec![TableValue::Cell(String::from("2-01")), TableValue::Cell(String::from("2-02")), TableValue::Cell(String::from("2-03"))]),
            TableValue::Cell(String::from("1-1")),
            TableValue::Cell(String::from("1-2"))
        ]),
        TableValue::Column(vec![
            TableValue::Column(vec![TableValue::Cell(String::from("2-01")), TableValue::Cell(String::from("2-02")), TableValue::Cell(String::from("2-03"))]),
            TableValue::Cell(String::from("2-1")),
            TableValue::Cell(String::from("2-2"))
        ]),
    ]))
    .with(Style::modern()),
    "┌─────┬──────┬──────┬──────┬──────┐"
    "│ 0-0 │ 2-01 │ 2-02 │ 2-03 │ 2-01 │"
    "│     ├──────┴──────┴──────┤      │"
    "│     │ 2-01               ├──────┤"
    "├─────┼────────────────────┤ 2-02 │"
    "│ 0-1 │ 2-02               ├──────┤"
    "│     ├────────────────────┤ 2-03 │"
    "│     │ 2-03               ├──────┤"
    "├─────┼────────────────────┤ 2-1  │"
    "│ 0-2 │ 1-1                │      │"
    "│     ├────────────────────┼──────┤"
    "│     │ 1-2                │ 2-2  │"
    "└─────┴────────────────────┴──────┘"
);

test_table!(
    pool_table_value_3,
    PoolTable::from(TableValue::Row(vec![
        TableValue::Column(vec![TableValue::Cell(String::from("0-0")), TableValue::Cell(String::from("0-1")), TableValue::Cell(String::from("0-2"))]),
        TableValue::Column(vec![
            TableValue::Row(vec![TableValue::Cell(String::from("2-01")), TableValue::Cell(String::from("2-02")), TableValue::Cell(String::from("2-03"))]),
            TableValue::Column(vec![TableValue::Cell(String::from("2-01")), TableValue::Cell(String::from("2-02")), TableValue::Cell(String::from("2-03"))]),
            TableValue::Cell(String::from("1-1")),
            TableValue::Row(vec![TableValue::Cell(String::from("2-01")), TableValue::Cell(String::from("2-02")), TableValue::Cell(String::from("2-03"))]),
            TableValue::Cell(String::from("1-2"))
        ]),
        TableValue::Column(vec![
            TableValue::Column(vec![TableValue::Cell(String::from("2-\n0\n1")), TableValue::Cell(String::from("2\n-\n0\n2")), TableValue::Cell(String::from("2-03"))]),
            TableValue::Cell(String::from("2-1")),
            TableValue::Column(vec![TableValue::Cell(String::from("2-0\n1")), TableValue::Cell(String::from("2-02")), TableValue::Cell(String::from("2-03"))]),
            TableValue::Cell(String::from("2-2"))
        ]),
    ]))
    .with(Style::modern()),
    "┌─────┬──────┬──────┬──────┬──────┐"
    "│ 0-0 │ 2-01 │ 2-02 │ 2-03 │ 2-   │"
    "│     │      │      │      │ 0    │"
    "│     │      │      │      │ 1    │"
    "│     ├──────┴──────┴──────┼──────┤"
    "│     │ 2-01               │ 2    │"
    "│     │                    │ -    │"
    "│     ├────────────────────┤ 0    │"
    "├─────┤ 2-02               │ 2    │"
    "│ 0-1 │                    ├──────┤"
    "│     ├────────────────────┤ 2-03 │"
    "│     │ 2-03               ├──────┤"
    "│     ├────────────────────┤ 2-1  │"
    "│     │ 1-1                ├──────┤"
    "│     │                    │ 2-0  │"
    "├─────┤                    │ 1    │"
    "│ 0-2 ├──────┬──────┬──────┼──────┤"
    "│     │ 2-01 │ 2-02 │ 2-03 │ 2-02 │"
    "│     │      │      │      ├──────┤"
    "│     ├──────┴──────┴──────┤ 2-03 │"
    "│     │ 1-2                ├──────┤"
    "│     │                    │ 2-2  │"
    "└─────┴────────────────────┴──────┘"
);

test_table!(
    pool_table_example,
    {
        let data = vec![
            vec!["Hello World", "Hello World", "Hello World"],
            vec!["Hello", "", "Hello"],
            vec!["W", "o", "r", "l", "d"],
        ];

        let data = TableValue::Column(
            data.into_iter()
                .map(|row| {
                    TableValue::Row(
                        row.into_iter()
                            .map(|text| TableValue::Cell(text.to_owned()))
                            .collect(),
                    )
                })
                .collect(),
        );

        PoolTable::from(data)
            .with(Style::modern())
            .with(Alignment::center())
            .to_string()
    },
    "┌─────────────┬─────────────┬─────────────┐"
    "│ Hello World │ Hello World │ Hello World │"
    "├─────────────┴─┬──────────┬┴─────────────┤"
    "│     Hello     │          │    Hello     │"
    "├────────┬──────┴─┬───────┬┴──────┬───────┤"
    "│   W    │   o    │   r   │   l   │   d   │"
    "└────────┴────────┴───────┴───────┴───────┘"
);

test_table!(
    pool_table_value_empty_row,
    PoolTable::from(TableValue::Row(vec![]))
    .with(Style::modern()),
    "┌──┐"
    "│  │"
    "└──┘"
);

test_table!(
    pool_table_value_empty_column,
    PoolTable::from(TableValue::Column(vec![]))
    .with(Style::modern()),
    "┌──┐"
    "│  │"
    "└──┘"
);

test_table!(
    pool_table_value_empty_cell,
    PoolTable::from(TableValue::Cell(String::from("")))
    .with(Style::modern()),
    "┌──┐"
    "│  │"
    "└──┘"
);

test_table!(
    pool_table_padding,
    PoolTable::new(Matrix::with_no_frame(3, 3).to_vec()).with(Padding::new(1, 2, 3, 4)),
    "+------+------+------+"
    "|      |      |      |"
    "|      |      |      |"
    "|      |      |      |"
    "| 0-0  | 0-1  | 0-2  |"
    "|      |      |      |"
    "|      |      |      |"
    "|      |      |      |"
    "|      |      |      |"
    "+------+------+------+"
    "|      |      |      |"
    "|      |      |      |"
    "|      |      |      |"
    "| 1-0  | 1-1  | 1-2  |"
    "|      |      |      |"
    "|      |      |      |"
    "|      |      |      |"
    "|      |      |      |"
    "+------+------+------+"
    "|      |      |      |"
    "|      |      |      |"
    "|      |      |      |"
    "| 2-0  | 2-1  | 2-2  |"
    "|      |      |      |"
    "|      |      |      |"
    "|      |      |      |"
    "|      |      |      |"
    "+------+------+------+"
);

#[cfg(feature = "color")]
test_table!(
    pool_table_padding_2,
    PoolTable::new(Matrix::with_no_frame(3, 3).to_vec())
        .with(Padding::new(1, 2, 3, 4)
            .fill('!', '@', '#', '$')
            .colorize(
                StaticColor::new("\u{1b}[34m", "\u{1b}[39m"),
                StaticColor::new("\u{1b}[34m", "\u{1b}[39m"),
                StaticColor::new("\u{1b}[34m", "\u{1b}[39m"),
                StaticColor::new("\u{1b}[34m", "\u{1b}[39m"),
            )
        ),
    "+------+------+------+"
    "|\u{1b}[34m######\u{1b}[39m|\u{1b}[34m######\u{1b}[39m|\u{1b}[34m######\u{1b}[39m|"
    "|\u{1b}[34m######\u{1b}[39m|\u{1b}[34m######\u{1b}[39m|\u{1b}[34m######\u{1b}[39m|"
    "|\u{1b}[34m######\u{1b}[39m|\u{1b}[34m######\u{1b}[39m|\u{1b}[34m######\u{1b}[39m|"
    "|\u{1b}[34m!\u{1b}[39m0-0\u{1b}[34m@@\u{1b}[39m|\u{1b}[34m!\u{1b}[39m0-1\u{1b}[34m@@\u{1b}[39m|\u{1b}[34m!\u{1b}[39m0-2\u{1b}[34m@@\u{1b}[39m|"
    "|\u{1b}[34m$$$$$$\u{1b}[39m|\u{1b}[34m$$$$$$\u{1b}[39m|\u{1b}[34m$$$$$$\u{1b}[39m|"
    "|\u{1b}[34m$$$$$$\u{1b}[39m|\u{1b}[34m$$$$$$\u{1b}[39m|\u{1b}[34m$$$$$$\u{1b}[39m|"
    "|\u{1b}[34m$$$$$$\u{1b}[39m|\u{1b}[34m$$$$$$\u{1b}[39m|\u{1b}[34m$$$$$$\u{1b}[39m|"
    "|\u{1b}[34m$$$$$$\u{1b}[39m|\u{1b}[34m$$$$$$\u{1b}[39m|\u{1b}[34m$$$$$$\u{1b}[39m|"
    "+------+------+------+"
    "|\u{1b}[34m######\u{1b}[39m|\u{1b}[34m######\u{1b}[39m|\u{1b}[34m######\u{1b}[39m|"
    "|\u{1b}[34m######\u{1b}[39m|\u{1b}[34m######\u{1b}[39m|\u{1b}[34m######\u{1b}[39m|"
    "|\u{1b}[34m######\u{1b}[39m|\u{1b}[34m######\u{1b}[39m|\u{1b}[34m######\u{1b}[39m|"
    "|\u{1b}[34m!\u{1b}[39m1-0\u{1b}[34m@@\u{1b}[39m|\u{1b}[34m!\u{1b}[39m1-1\u{1b}[34m@@\u{1b}[39m|\u{1b}[34m!\u{1b}[39m1-2\u{1b}[34m@@\u{1b}[39m|"
    "|\u{1b}[34m$$$$$$\u{1b}[39m|\u{1b}[34m$$$$$$\u{1b}[39m|\u{1b}[34m$$$$$$\u{1b}[39m|"
    "|\u{1b}[34m$$$$$$\u{1b}[39m|\u{1b}[34m$$$$$$\u{1b}[39m|\u{1b}[34m$$$$$$\u{1b}[39m|"
    "|\u{1b}[34m$$$$$$\u{1b}[39m|\u{1b}[34m$$$$$$\u{1b}[39m|\u{1b}[34m$$$$$$\u{1b}[39m|"
    "|\u{1b}[34m$$$$$$\u{1b}[39m|\u{1b}[34m$$$$$$\u{1b}[39m|\u{1b}[34m$$$$$$\u{1b}[39m|"
    "+------+------+------+"
    "|\u{1b}[34m######\u{1b}[39m|\u{1b}[34m######\u{1b}[39m|\u{1b}[34m######\u{1b}[39m|"
    "|\u{1b}[34m######\u{1b}[39m|\u{1b}[34m######\u{1b}[39m|\u{1b}[34m######\u{1b}[39m|"
    "|\u{1b}[34m######\u{1b}[39m|\u{1b}[34m######\u{1b}[39m|\u{1b}[34m######\u{1b}[39m|"
    "|\u{1b}[34m!\u{1b}[39m2-0\u{1b}[34m@@\u{1b}[39m|\u{1b}[34m!\u{1b}[39m2-1\u{1b}[34m@@\u{1b}[39m|\u{1b}[34m!\u{1b}[39m2-2\u{1b}[34m@@\u{1b}[39m|"
    "|\u{1b}[34m$$$$$$\u{1b}[39m|\u{1b}[34m$$$$$$\u{1b}[39m|\u{1b}[34m$$$$$$\u{1b}[39m|"
    "|\u{1b}[34m$$$$$$\u{1b}[39m|\u{1b}[34m$$$$$$\u{1b}[39m|\u{1b}[34m$$$$$$\u{1b}[39m|"
    "|\u{1b}[34m$$$$$$\u{1b}[39m|\u{1b}[34m$$$$$$\u{1b}[39m|\u{1b}[34m$$$$$$\u{1b}[39m|"
    "|\u{1b}[34m$$$$$$\u{1b}[39m|\u{1b}[34m$$$$$$\u{1b}[39m|\u{1b}[34m$$$$$$\u{1b}[39m|"
    "+------+------+------+"
);

test_table!(
    pool_table_margin,
    PoolTable::new(Matrix::with_no_frame(3, 3).to_vec()).with(Margin::new(1, 2, 3, 4).fill('!', '@', '#', '$')),
    "!###################@@"
    "!###################@@"
    "!###################@@"
    "!+-----+-----+-----+@@"
    "!| 0-0 | 0-1 | 0-2 |@@"
    "!+-----+-----+-----+@@"
    "!| 1-0 | 1-1 | 1-2 |@@"
    "!+-----+-----+-----+@@"
    "!| 2-0 | 2-1 | 2-2 |@@"
    "!+-----+-----+-----+@@"
    "!$$$$$$$$$$$$$$$$$$$@@"
    "!$$$$$$$$$$$$$$$$$$$@@"
    "!$$$$$$$$$$$$$$$$$$$@@"
    "!$$$$$$$$$$$$$$$$$$$@@"
);

test_table!(
    pool_table_alignment_bottom,
    PoolTable::new([
        ["1", "2\n2", "3\n3\n3"],
        ["3\n3\n3", "2\n2", "1"],
        ["1", "3\n3\n3", "2\n2"]
    ])
    .with(Alignment::bottom()),
    "+---+---+---+"
    "|   |   | 3 |"
    "|   | 2 | 3 |"
    "| 1 | 2 | 3 |"
    "+---+---+---+"
    "| 3 |   |   |"
    "| 3 | 2 |   |"
    "| 3 | 2 | 1 |"
    "+---+---+---+"
    "|   | 3 |   |"
    "|   | 3 | 2 |"
    "| 1 | 3 | 2 |"
    "+---+---+---+"
);

test_table!(
    pool_table_alignment_center_vertical,
    PoolTable::new([
        ["1", "2\n2", "3\n3\n3"],
        ["3\n3\n3", "2\n2", "1"],
        ["1", "3\n3\n3", "2\n2"]
    ])
    .with(Alignment::center_vertical()),
    "+---+---+---+"
    "|   | 2 | 3 |"
    "| 1 | 2 | 3 |"
    "|   |   | 3 |"
    "+---+---+---+"
    "| 3 | 2 |   |"
    "| 3 | 2 | 1 |"
    "| 3 |   |   |"
    "+---+---+---+"
    "|   | 3 | 2 |"
    "| 1 | 3 | 2 |"
    "|   | 3 |   |"
    "+---+---+---+"
);

test_table!(
    pool_table_alignment_right,
    PoolTable::new([
        ["1                     ", "2\n2       ", "3\n3\n3"],
        ["3\n3\n3", "2\n2", "1"],
        ["1", "3\n3\n3                   ", "2\n2"]
    ])
    .with(Alignment::right()),
    "+------------------------+----------+---+"
    "| 1                      | 2        | 3 |"
    "|                        | 2        | 3 |"
    "|                        |          | 3 |"
    "+-------------+----------+-+--------+---+"
    "|           3 |          2 |          1 |"
    "|           3 |          2 |            |"
    "|           3 |            |            |"
    "+------+------+------------+-----+------+"
    "|    1 |    3                    |    2 |"
    "|      |    3                    |    2 |"
    "|      |    3                    |      |"
    "+------+-------------------------+------+"
);

test_table!(
    pool_table_alignment_right_per_line,
    PoolTable::new([
        ["1                     ", "2\n2       ", "3\n3\n3"],
        ["3\n3\n3", "2\n2", "1"],
        ["1", "3\n3\n3                   ", "2\n2"]
    ])
    .with(Alignment::right())
    .with(AlignmentStrategy::PerLine),
    "+------------------------+----------+---+"
    "| 1                      |        2 | 3 |"
    "|                        | 2        | 3 |"
    "|                        |          | 3 |"
    "+-------------+----------+-+--------+---+"
    "|           3 |          2 |          1 |"
    "|           3 |          2 |            |"
    "|           3 |            |            |"
    "+------+------+------------+-----+------+"
    "|    1 |                       3 |    2 |"
    "|      |                       3 |    2 |"
    "|      |    3                    |      |"
    "+------+-------------------------+------+"
);

test_table!(
    pool_table_alignment_center_horizontal,
    PoolTable::new([
        ["1                     ", "2\n2       ", "3\n3\n3"],
        ["3\n3\n3", "2\n2", "1"],
        ["1", "3\n3\n3                   ", "2\n2"]
    ])
    .with(Alignment::center()),
    "+------------------------+----------+---+"
    "| 1                      | 2        | 3 |"
    "|                        | 2        | 3 |"
    "|                        |          | 3 |"
    "+-------------+----------+-+--------+---+"
    "|      3      |     2      |     1      |"
    "|      3      |     2      |            |"
    "|      3      |            |            |"
    "+------+------+------------+-----+------+"
    "|  1   |  3                      |  2   |"
    "|      |  3                      |  2   |"
    "|      |  3                      |      |"
    "+------+-------------------------+------+"
);

test_table!(
    pool_table_alignment_center_horizontal_line_strategy,
    PoolTable::new([
        ["1                     ", "2\n22222", "3\n3\n3"],
        ["3\n3\n3", "2\n2", "1"],
        ["1", "3\n3\n3                   ", "2\n2"]
    ])
    .with(Alignment::center())
    .with(AlignmentStrategy::PerLine),
    "+------------------------+-------+---+"
    "| 1                      |   2   | 3 |"
    "|                        | 22222 | 3 |"
    "|                        |       | 3 |"
    "+------------+-----------+-------+---+"
    "|     3      |     2     |     1     |"
    "|     3      |     2     |           |"
    "|     3      |           |           |"
    "+-----+------+-----------+-----+-----+"
    "|  1  |           3            |  2  |"
    "|     |           3            |  2  |"
    "|     |  3                     |     |"
    "+-----+------------------------+-----+"
);

test_table!(
    pool_table_style_empty,
    PoolTable::new(Matrix::with_no_frame(3, 3).to_vec()).with(Style::empty()),
    " 0-0  0-1  0-2 "
    " 1-0  1-1  1-2 "
    " 2-0  2-1  2-2 "
);

test_table!(
    pool_table_style_markdown,
    PoolTable::new(Matrix::with_no_frame(3, 3).to_vec()).with(Style::markdown()),
    "| 0-0 | 0-1 | 0-2 |"
    "| 1-0 | 1-1 | 1-2 |"
    "| 2-0 | 2-1 | 2-2 |"
);

test_table!(
    pool_table_style_rounded,
    PoolTable::new(Matrix::with_no_frame(3, 3).to_vec()).with(Style::rounded()),
    "╭─────┬─────┬─────╮"
    "│ 0-0 │ 0-1 │ 0-2 │"
    " ───── ───── ───── "
    "│ 1-0 │ 1-1 │ 1-2 │"
    " ───── ───── ───── "
    "│ 2-0 │ 2-1 │ 2-2 │"
    "╰─────┴─────┴─────╯"
);

test_table!(
    pool_table_dim_ctrl_0,
    PoolTable::from(TableValue::Row(vec![
        TableValue::Column(vec![TableValue::Cell(String::from("0-0")), TableValue::Cell(String::from("0-1")), TableValue::Cell(String::from("0-2"))]),
        TableValue::Column(vec![TableValue::Cell(String::from("1-0")), TableValue::Cell(String::from("1-1")), TableValue::Cell(String::from("1-2"))]),
        TableValue::Column(vec![
            TableValue::Column(vec![TableValue::Cell(String::from("2-01")), TableValue::Cell(String::from("2-02")), TableValue::Cell(String::from("2-03"))]),
            TableValue::Cell(String::from("2-1")),
            TableValue::Cell(String::from("2-2")),
        ]),
    ]))
    .with(PoolTableDimension::new(DimensionPriority::Last, DimensionPriority::Last)),
    "+-----+-----+------+"
    "| 0-0 | 1-0 | 2-01 |"
    "+-----+-----+------+"
    "| 0-1 | 1-1 | 2-02 |"
    "+-----+-----+------+"
    "| 0-2 | 1-2 | 2-03 |"
    "|     |     +------+"
    "|     |     | 2-1  |"
    "|     |     +------+"
    "|     |     | 2-2  |"
    "+-----+-----+------+"
);

test_table!(
    pool_table_dim_ctrl_1,
    PoolTable::new([
        ["1                     ", "2\n2       ", "3\n3\n3"],
        ["3\n3\n3", "2\n2", "1"],
        ["1", "3\n3\n3                   ", "2\n2"]
    ])
    .with(PoolTableDimension::new(DimensionPriority::List, DimensionPriority::List)),
    "+------------------------+----------+---+"
    "| 1                      | 2        | 3 |"
    "|                        | 2        | 3 |"
    "|                        |          | 3 |"
    "+-------------+----------+-+--------+---+"
    "| 3           | 2          | 1          |"
    "| 3           | 2          |            |"
    "| 3           |            |            |"
    "+------+------+------------+-----+------+"
    "| 1    | 3                       | 2    |"
    "|      | 3                       | 2    |"
    "|      | 3                       |      |"
    "+------+-------------------------+------+"
);

test_table!(
    pool_table_2_columns_1_cell,
    PoolTable::from(TableValue::Row(vec![
        TableValue::Column(vec![
            TableValue::Cell(String::from("0-0")),
            TableValue::Cell(String::from("0-1")),
            TableValue::Cell(String::from("0-2")),
            TableValue::Cell(String::from("0-3")),
            TableValue::Cell(String::from("0-4")),
            TableValue::Cell(String::from("0-5")),
        ]),
        TableValue::Column(vec![
            TableValue::Cell(String::from("1-0")),
            TableValue::Cell(String::from("1-1")),
            TableValue::Cell(String::from("1-2")),
            TableValue::Cell(String::from("1-3")),
            TableValue::Cell(String::from("1-4")),
            TableValue::Cell(String::from("1-6")),
            TableValue::Cell(String::from("1-7")),
            TableValue::Cell(String::from("1-8")),
            TableValue::Cell(String::from("1-9")),
        ]),
        TableValue::Cell(String::from("2-0")),
    ]))
    .with(PoolTableDimension::new(DimensionPriority::Last, DimensionPriority::Last)),
    "+-----+-----+-----+"
    "| 0-0 | 1-0 | 2-0 |"
    "+-----+-----+     |"
    "| 0-1 | 1-1 |     |"
    "+-----+-----+     |"
    "| 0-2 | 1-2 |     |"
    "+-----+-----+     |"
    "| 0-3 | 1-3 |     |"
    "+-----+-----+     |"
    "| 0-4 | 1-4 |     |"
    "+-----+-----+     |"
    "| 0-5 | 1-6 |     |"
    "|     +-----+     |"
    "|     | 1-7 |     |"
    "|     +-----+     |"
    "|     | 1-8 |     |"
    "|     +-----+     |"
    "|     | 1-9 |     |"
    "+-----+-----+-----+"
);
