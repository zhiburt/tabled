#![cfg(feature = "std")]

mod util;

use tabled::{
    settings::{Alignment, Margin, Padding, Style},
    tables::{PoolTable, TableValue},
};
use util::{create_matrix, test_table};

test_table!(
    pool_table,
    PoolTable::new(create_matrix::<3, 3>()),
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
    "│     │ 2-01               │      │"
    "├─────┼────────────────────┼──────┤"
    "│ 0-1 │ 2-02               │ 2-02 │"
    "│     ├────────────────────┼──────┤"
    "│     │ 2-03               │ 2-03 │"
    "├─────┼────────────────────┼──────┤"
    "│ 0-2 │ 1-1                │ 2-1  │"
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
    "│     │      │      │      ├──────┤"
    "│     │      │      │      │ 2    │"
    "│     ├──────┴──────┴──────┤ -    │"
    "│     │ 2-01               │ 0    │"
    "├─────┤                    │ 2    │"
    "│ 0-1 ├────────────────────┼──────┤"
    "│     │ 2-02               │ 2-03 │"
    "│     ├────────────────────┼──────┤"
    "│     │ 2-03               │ 2-1  │"
    "│     ├────────────────────┼──────┤"
    "│     │ 1-1                │ 2-0  │"
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
    "├─────────────┴──┬─────────┬┴─────────────┤"
    "│     Hello      │         │    Hello     │"
    "├─────────┬──────┴┬───────┬┴──────┬───────┤"
    "│    W    │   o   │   r   │   l   │   d   │"
    "└─────────┴───────┴───────┴───────┴───────┘"
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
    PoolTable::new(create_matrix::<3, 3>()).with(Padding::new(1, 2, 3, 4)),
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

test_table!(
    pool_table_margin,
    PoolTable::new(create_matrix::<3, 3>()).with(Margin::new(1, 2, 3, 4)),
    "                            "
    "                            "
    "                            "
    " +-----   +-----   +-----+  "
    " | 0-0    | 0-1    | 0-2 |  "
    "                            "
    "                            "
    "                            "
    "                            "
    "                            "
    "                            "
    "                            "
    " +-----   +-----   +-----+  "
    " | 1-0    | 1-1    | 1-2 |  "
    "                            "
    "                            "
    "                            "
    "                            "
    "                            "
    "                            "
    "                            "
    " +-----   +-----   +-----+  "
    " | 2-0    | 2-1    | 2-2 |  "
    " +-----   +-----   +-----+  "
    "                            "
    "                            "
    "                            "
    "                            "
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
    "| 1                      |    2     | 3 |"
    "|                        | 2        | 3 |"
    "|                        |          | 3 |"
    "+-------------+----------+-+--------+---+"
    "|      3      |     2      |     1      |"
    "|      3      |     2      |            |"
    "|      3      |            |            |"
    "+------+------+------------+-----+------+"
    "|  1   |            3            |  2   |"
    "|      |            3            |  2   |"
    "|      |  3                      |      |"
    "+------+-------------------------+------+"
);

test_table!(
    pool_table_style_empty,
    PoolTable::new(create_matrix::<3, 3>()).with(Style::empty()),
    " 0-0  0-1  0-2 "
    " 1-0  1-1  1-2 "
    " 2-0  2-1  2-2 "
);

test_table!(
    pool_table_style_markdown,
    PoolTable::new(create_matrix::<3, 3>()).with(Style::markdown()),
    "| 0-0 | 0-1 | 0-2 |"
    "| 1-0 | 1-1 | 1-2 |"
    "| 2-0 | 2-1 | 2-2 |"
    "|----- ----- -----|"
);

test_table!(
    pool_table_style_rounded,
    PoolTable::new(create_matrix::<3, 3>()).with(Style::rounded()),
    "╭─────┬─────┬─────╮"
    "│ 0-0 │ 0-1 │ 0-2 │"
    " ───── ───── ───── "
    "│ 1-0 │ 1-1 │ 1-2 │"
    " ───── ───── ───── "
    "│ 2-0 │ 2-1 │ 2-2 │"
    "├─────┴─────┴─────┤"
);
