use std::{iter::FromIterator, str::FromStr};

use tabled::settings::Alignment;
use testing_table::test_table;
use toml::{Table as TomlMap, Value};

use toml_to_table::{Orientation, TomlTable};

test_table!(
    test_string,
    build_ron_table(Value::String(String::from("123456789"))),
    "+-----------+"
    "| 123456789 |"
    "+-----------+"
);

test_table!(
    test_bool_false,
    build_ron_table(Value::Boolean(false)),
    "+-------+"
    "| false |"
    "+-------+"
);

test_table!(
    test_bool_true,
    build_ron_table(Value::Boolean(true)),
    "+------+"
    "| true |"
    "+------+"
);

test_table!(
    test_datetime,
    build_ron_table(Value::Datetime(toml::value::Datetime::from_str("1979-05-27T07:32:00Z").unwrap())),
    "+----------------------+"
    "| 1979-05-27T07:32:00Z |"
    "+----------------------+"
);

test_table!(
    test_int,
    build_ron_table(Value::Integer(123456789)),
    "+-----------+"
    "| 123456789 |"
    "+-----------+"
);

test_table!(
    test_float,
    build_ron_table(Value::Float(123.456789)),
    "+------------+"
    "| 123.456789 |"
    "+------------+"
);

test_table!(
    test_sequence_0,
    build_ron_table(Value::Array(vec![
        Value::String(String::from("Hello")),
        Value::String(String::from("World")),
    ])),
    "+-------+"
    "| Hello |"
    "+-------+"
    "| World |"
    "+-------+"
);

test_table!(
    test_sequence_1,
    build_ron_table(Value::Array(vec![
        Value::Array(vec![
            Value::String(String::from("Hello")),
            Value::String(String::from("World")),
        ]),
        Value::String(String::from("Hello")),
        Value::Array(vec![
            Value::String(String::from("Hello")),
            Value::String(String::from("World")),
        ]),
        Value::String(String::from("World")),
        Value::Array(vec![
            Value::String(String::from("Hello")),
            Value::String(String::from("World")),
        ]),
    ])),
    "+-------+"
    "| Hello |"
    "+-------+"
    "| World |"
    "+-------+"
    "| Hello |"
    "+-------+"
    "| Hello |"
    "+-------+"
    "| World |"
    "+-------+"
    "| World |"
    "+-------+"
    "| Hello |"
    "+-------+"
    "| World |"
    "+-------+"
);

test_table!(
    test_sequence_0_row,
    build_ron_table_orientation(
        Value::Array(vec![
            Value::String(String::from("Hello")),
            Value::String(String::from("World")),
        ]),
        Orientation::Row,
        Orientation::Column,
    ),
    "+-------+-------+"
    "| Hello | World |"
    "+-------+-------+"
);

test_table!(
    test_sequence_1_row,
    build_ron_table_orientation(
        Value::Array(vec![
            Value::Array(vec![
                Value::String(String::from("Hello")),
                Value::String(String::from("World")),
            ]),
            Value::String(String::from("Hello")),
            Value::Array(vec![
                Value::String(String::from("Hello")),
                Value::String(String::from("World")),
            ]),
            Value::String(String::from("World")),
            Value::Array(vec![
                Value::String(String::from("Hello")),
                Value::String(String::from("World")),
            ]),
        ]),
        Orientation::Row,
        Orientation::Column,
    ),
    "+-------+-------+-------+-------+-------+-------+-------+-------+"
    "| Hello | World | Hello | Hello | World | World | Hello | World |"
    "+-------+-------+-------+-------+-------+-------+-------+-------+"
);

test_table!(
    test_map_0,
    build_ron_table(Value::Table(TomlMap::from_iter([
        (String::from("Hello Key"), Value::String(String::from("World Value"))),
        (String::from("Hello Key 2"), Value::String(String::from("Value 1"))),
    ]))),
    "+-------------+-------------+"
    "| Hello Key   | World Value |"
    "+-------------+-------------+"
    "| Hello Key 2 | Value 1     |"
    "+-------------+-------------+"
);

test_table!(
    test_map_1,
    build_ron_table(Value::Table(TomlMap::from_iter([
        (String::from("Hello Key"), Value::String(String::from("World Value"))),
        (
            String::from("Hello Key 2"),
            Value::Array(vec![
                Value::String(String::from("Hello")),
                Value::String(String::from("World")),
            ]),
        ),
    ]))),
    "+-------------+-------------+"
    "| Hello Key   | World Value |"
    "+-------------+-------------+"
    "| Hello Key 2 | Hello       |"
    "|             +-------------+"
    "|             | World       |"
    "+-------------+-------------+"
);

test_table!(
    test_map_0_row,
    build_ron_table_orientation(
        Value::Table(TomlMap::from_iter([
            (String::from("Hello Key"), Value::String(String::from("World Value"))),
            (String::from("Hello Key 2"), Value::String(String::from("Value 1"))),
        ])),
        Orientation::Row,
        Orientation::Row,
    ),
    "+-------------+-------------+"
    "| Hello Key   | Hello Key 2 |"
    "+-------------+-------------+"
    "| World Value | Value 1     |"
    "+-------------+-------------+"
);

test_table!(
    test_map_1_row,
    build_ron_table_orientation(
        Value::Table(TomlMap::from_iter([
            (
                String::from("Hello Key"),
                Value::String(String::from("World Value")),
            ),
            (
                String::from("Hello Key 2"),
                Value::Array(vec![
                    Value::String(String::from("Hello")),
                    Value::String(String::from("World")),
                ]),
            ),
        ])),
        Orientation::Row,
        Orientation::Row,
    ),
    "+-------------+---------------+"
    "| Hello Key   | Hello Key 2   |"
    "+-------------+-------+-------+"
    "| World Value | Hello | World |"
    "+-------------+-------+-------+"
);

test_table!(
    test_map_1_row_column,
    build_ron_table_orientation(
        Value::Table(TomlMap::from_iter([
            (
                String::from("Hello Key"),
                Value::String(String::from("World Value")),
            ),
            (
                String::from("Hello Key 2"),
                Value::Array(vec![
                    Value::String(String::from("Hello")),
                    Value::String(String::from("World")),
                ]),
            ),
        ])),
        Orientation::Column,
        Orientation::Row,
    ),
    "+-------------+-------------+"
    "| Hello Key   | Hello Key 2 |"
    "+-------------+-------------+"
    "| World Value | Hello       |"
    "|             +-------------+"
    "|             | World       |"
    "+-------------+-------------+"
);

test_table!(
    test_sequence_row_column,
    build_ron_table_orientation(
        Value::Array(vec![
            Value::Table(TomlMap::from_iter([
                (
                    String::from("Hello Key"),
                    Value::String(String::from("World Value")),
                ),
                (
                    String::from("Hello Key 2"),
                    Value::Array(vec![
                        Value::String(String::from("Hello")),
                        Value::String(String::from("World")),
                    ]),
                ),
            ])),
            Value::String(String::from("Hello Key 2")),
            Value::String(String::from("Hello")),
            Value::Array(vec![
                Value::String(String::from("Wold")),
                Value::String(String::from("Wod")),
                Value::String(String::from("Wo")),
                Value::String(String::from("Wo")),
            ]),
        ]),
        Orientation::Column,
        Orientation::Row,
    ),
    "+-------------+-------------+"
    "| Hello Key   | Hello Key 2 |"
    "+-------------+-------------+"
    "| World Value | Hello       |"
    "|             +-------------+"
    "|             | World       |"
    "+-------------+-------------+"
    "| Hello Key 2               |"
    "+---------------------------+"
    "| Hello                     |"
    "+---------------------------+"
    "| Wold                      |"
    "+---------------------------+"
    "| Wod                       |"
    "+---------------------------+"
    "| Wo                        |"
    "+---------------------------+"
    "| Wo                        |"
    "+---------------------------+"
);

test_table!(
    test_sequence_row_column_1,
    build_ron_table_orientation(
        Value::Array(vec![
            Value::Table(TomlMap::from_iter([
                (
                    String::from("Hello Key"),
                    Value::String(String::from("World Value")),
                ),
                (
                    String::from("Hello Key 2"),
                    Value::Array(vec![
                        Value::String(String::from("Hello")),
                        Value::String(String::from("World")),
                    ]),
                ),
            ])),
            Value::String(String::from("Hello Key 2")),
            Value::String(String::from("Hello")),
            Value::Array(vec![
                Value::String(String::from("Wold")),
                Value::String(String::from("Wod")),
                Value::String(String::from("Wo")),
                Value::String(String::from("Wo")),
            ]),
        ]),
        Orientation::Row,
        Orientation::Column,
    ),
    "+-------------+---------------+-------------+-------+------+-----+----+----+"
    "| Hello Key   | World Value   | Hello Key 2 | Hello | Wold | Wod | Wo | Wo |"
    "+-------------+-------+-------+             |       |      |     |    |    |"
    "| Hello Key 2 | Hello | World |             |       |      |     |    |    |"
    "+-------------+-------+-------+-------------+-------+------+-----+----+----+"
);

test_table!(
    test_sequence_row_column_2,
    build_ron_table_orientation(
        Value::Array(vec![
            Value::Table(TomlMap::from_iter([
                (
                    String::from("Hello Key"),
                    Value::String(String::from("World Value")),
                ),
                (
                    String::from("Hello Key 2"),
                    Value::Array(vec![
                        Value::String(String::from("Hello")),
                        Value::String(String::from("World")),
                    ]),
                ),
            ])),
            Value::String(String::from("Hello Key 2")),
            Value::String(String::from("Hello")),
            Value::Array(vec![
                Value::String(String::from("Wold")),
                Value::String(String::from("Wod")),
                Value::String(String::from("Wo")),
                Value::String(String::from("Wo")),
            ]),
        ]),
        Orientation::Row,
        Orientation::Row,
    ),
    "+-------------+---------------+-------------+-------+------+-----+----+----+"
    "| Hello Key   | Hello Key 2   | Hello Key 2 | Hello | Wold | Wod | Wo | Wo |"
    "+-------------+-------+-------+             |       |      |     |    |    |"
    "| World Value | Hello | World |             |       |      |     |    |    |"
    "+-------------+-------+-------+-------------+-------+------+-----+----+----+"
);

test_table!(
    test_sequence_row_column_3,
    build_ron_table_orientation(
        Value::Array(vec![
            Value::Table(TomlMap::from_iter([
                (
                    String::from("Hello Key"),
                    Value::String(String::from("World Value")),
                ),
                (
                    String::from("Hello Key 2"),
                    Value::Array(vec![
                        Value::String(String::from("Hello")),
                        Value::String(String::from("World")),
                    ]),
                ),
            ])),
            Value::String(String::from("Hello Key 2")),
            Value::String(String::from("Hello")),
            Value::Array(vec![
                Value::String(String::from("Wold")),
                Value::String(String::from("Wod")),
                Value::String(String::from("Wo")),
                Value::String(String::from("Wo")),
            ]),
        ]),
        Orientation::Column,
        Orientation::Column,
    ),
    "+-------------+-------------+"
    "| Hello Key   | World Value |"
    "+-------------+-------------+"
    "| Hello Key 2 | Hello       |"
    "|             +-------------+"
    "|             | World       |"
    "+-------------+-------------+"
    "| Hello Key 2               |"
    "+---------------------------+"
    "| Hello                     |"
    "+---------------------------+"
    "| Wold                      |"
    "+---------------------------+"
    "| Wod                       |"
    "+---------------------------+"
    "| Wo                        |"
    "+---------------------------+"
    "| Wo                        |"
    "+---------------------------+"
);

test_table!(
    test_alignment_center,
    TomlTable::new(Value::Array(vec![
        Value::Table(TomlMap::from_iter([
            (
                String::from("Hello Key"),
                Value::String(String::from("World Value")),
            ),
            (
                String::from("Hello Key 2"),
                Value::Array(vec![
                    Value::String(String::from("Hello")),
                    Value::String(String::from("World")),
                ]),
            ),
        ])),
        Value::String(String::from("Hello Key 2")),
        Value::String(String::from("Hello")),
        Value::Array(vec![
            Value::String(String::from("Wold")),
            Value::String(String::from("Wod")),
            Value::String(String::from("Wo")),
            Value::String(String::from("Wo")),
        ]),
    ]))
        .with(Alignment::center_vertical())
        .with(Alignment::center())
        .collapse()
        .to_string(),
    "+-------------+-------------+"
    "|  Hello Key  | World Value |"
    "+-------------+-------------+"
    "|             |    Hello    |"
    "| Hello Key 2 +-------------+"
    "|             |    World    |"
    "+-------------+-------------+"
    "|        Hello Key 2        |"
    "+---------------------------+"
    "|           Hello           |"
    "+---------------------------+"
    "|           Wold            |"
    "+---------------------------+"
    "|            Wod            |"
    "+---------------------------+"
    "|            Wo             |"
    "+---------------------------+"
    "|            Wo             |"
    "+---------------------------+"
);

fn build_ron_table(value: Value) -> String {
    TomlTable::new(value).collapse().to_string()
}

fn build_ron_table_orientation(value: Value, seq: Orientation, map: Orientation) -> String {
    TomlTable::new(value)
        .collapse()
        .seq_orientation(seq)
        .map_orientation(map)
        .to_string()
}
