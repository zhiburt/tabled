use std::iter::FromIterator;

use ron::{value::F32, Map, Number, Value};
use tabled::assert::test_table;
use tabled::settings::Alignment;

use ron_to_table::{Orientation, RonTable};

test_table!(test_unit, build_ron_table(Value::Unit).to_string(), "");

test_table!(
    test_string,
    build_ron_table(Value::String(String::from("123456789"))),
    "+-----------+"
    "| 123456789 |"
    "+-----------+"
);

test_table!(
    test_bool_true,
    build_ron_table(Value::Bool(true)),
    "+------+"
    "| true |"
    "+------+"
);

test_table!(
    test_bool_false,
    build_ron_table(Value::Bool(false)),
    "+-------+"
    "| false |"
    "+-------+"
);

test_table!(
    test_char,
    build_ron_table(Value::Char('a')),
    "+---+"
    "| a |"
    "+---+"
);

test_table!(
    test_int,
    build_ron_table(Value::Number(Number::U32(123456789))),
    "+-----------+"
    "| 123456789 |"
    "+-----------+"
);

test_table!(
    test_float,
    build_ron_table(Value::Number(Number::F32(F32(123.456789)))),
    "+-----------+"
    "| 123.45679 |"
    "+-----------+"
);

test_table!(
    test_bytes,
    build_ron_table(Value::Bytes(vec![1, 2, 3, 4, 5])),
    "+-----------------+"
    "| [1, 2, 3, 4, 5] |"
    "+-----------------+"
);

test_table!(
    test_sequence_0,
    build_ron_table(Value::Seq(vec![
        Value::String(String::from("Hello")),
        Value::String(String::from("World")),
    ])),
    "+---------+"
    "|  Hello  |"
    "+---------+"
    "|  World  |"
    "+---------+"
);

test_table!(
    test_sequence_1,
    build_ron_table(Value::Seq(vec![
        Value::Seq(vec![
            Value::String(String::from("Hello")),
            Value::String(String::from("World")),
        ]),
        Value::String(String::from("Hello")),
        Value::Seq(vec![
            Value::String(String::from("Hello")),
            Value::String(String::from("World")),
        ]),
        Value::String(String::from("World")),
        Value::Seq(vec![
            Value::String(String::from("Hello")),
            Value::String(String::from("World")),
        ]),
    ])),
    "+-------------+"
    "| +---------+ |"
    "| |  Hello  | |"
    "| +---------+ |"
    "| |  World  | |"
    "| +---------+ |"
    "+-------------+"
    "|  Hello      |"
    "+-------------+"
    "| +---------+ |"
    "| |  Hello  | |"
    "| +---------+ |"
    "| |  World  | |"
    "| +---------+ |"
    "+-------------+"
    "|  World      |"
    "+-------------+"
    "| +---------+ |"
    "| |  Hello  | |"
    "| +---------+ |"
    "| |  World  | |"
    "| +---------+ |"
    "+-------------+"
);

test_table!(
    test_sequence_0_row,
    build_ron_table_orientation(
        Value::Seq(vec![
            Value::String(String::from("Hello")),
            Value::String(String::from("World")),
        ]),
        Orientation::Row,
        Orientation::Column,
    ),
    "+---------+---------+"
    "|  Hello  |  World  |"
    "+---------+---------+"
);

test_table!(
    test_sequence_1_row,
    build_ron_table_orientation(
        Value::Seq(vec![
            Value::Seq(vec![
                Value::String(String::from("Hello")),
                Value::String(String::from("World")),
            ]),
            Value::String(String::from("Hello")),
            Value::Seq(vec![
                Value::String(String::from("Hello")),
                Value::String(String::from("World")),
            ]),
            Value::String(String::from("World")),
            Value::Seq(vec![
                Value::String(String::from("Hello")),
                Value::String(String::from("World")),
            ]),
        ]),
        Orientation::Row,
        Orientation::Column,
    ),
    "+-----------------------+---------+-----------------------+---------+-----------------------+"
    "| +---------+---------+ |  Hello  | +---------+---------+ |  World  | +---------+---------+ |"
    "| |  Hello  |  World  | |         | |  Hello  |  World  | |         | |  Hello  |  World  | |"
    "| +---------+---------+ |         | +---------+---------+ |         | +---------+---------+ |"
    "+-----------------------+---------+-----------------------+---------+-----------------------+"
);

test_table!(
    test_map_0,
    build_ron_table(Value::Map(Map::from_iter([
        (
            Value::String(String::from("Hello Key")),
            Value::String(String::from("World Value"))
        ),
        (
            Value::String(String::from("Hello Key 2")),
            Value::String(String::from("Value 1")),
        ),
    ]))),
    "+---------------+---------------+"
    "|  Hello Key    |  World Value  |"
    "+---------------+---------------+"
    "|  Hello Key 2  |  Value 1      |"
    "+---------------+---------------+"
);

test_table!(
    test_map_1,
    build_ron_table(Value::Map(Map::from_iter([
        (
            Value::String(String::from("Hello Key")),
            Value::String(String::from("World Value")),
        ),
        (
            Value::String(String::from("Hello Key 2")),
            Value::Seq(vec![
                Value::String(String::from("Hello")),
                Value::String(String::from("World")),
            ]),
        ),
    ]))),
    "+---------------+---------------+"
    "|  Hello Key    |  World Value  |"
    "+---------------+---------------+"
    "|  Hello Key 2  | +---------+   |"
    "|               | |  Hello  |   |"
    "|               | +---------+   |"
    "|               | |  World  |   |"
    "|               | +---------+   |"
    "+---------------+---------------+"
);

test_table!(
    test_map_0_row,
    build_ron_table_orientation(
        Value::Map(Map::from_iter([
            (
                Value::String(String::from("Hello Key")),
                Value::String(String::from("World Value")),
            ),
            (
                Value::String(String::from("Hello Key 2")),
                Value::String(String::from("Value 1")),
            ),
        ])),
        Orientation::Row,
        Orientation::Row,
    ),
    "+---------------+---------------+"
    "|  Hello Key    |  Hello Key 2  |"
    "+---------------+---------------+"
    "|  World Value  |  Value 1      |"
    "+---------------+---------------+"
);

test_table!(
    test_map_1_row,
    build_ron_table_orientation(
        Value::Map(Map::from_iter([
            (
                Value::String(String::from("Hello Key")),
                Value::String(String::from("World Value")),
            ),
            (
                Value::String(String::from("Hello Key 2")),
                Value::Seq(vec![
                    Value::String(String::from("Hello")),
                    Value::String(String::from("World")),
                ]),
            ),
        ])),
        Orientation::Row,
        Orientation::Row,
    ),
    "+---------------+-----------------------+"
    "|  Hello Key    |  Hello Key 2          |"
    "+---------------+-----------------------+"
    "|  World Value  | +---------+---------+ |"
    "|               | |  Hello  |  World  | |"
    "|               | +---------+---------+ |"
    "+---------------+-----------------------+"
);

test_table!(
    test_map_1_row_column,
    build_ron_table_orientation(
        Value::Map(Map::from_iter([
            (
                Value::String(String::from("Hello Key")),
                Value::String(String::from("World Value")),
            ),
            (
                Value::String(String::from("Hello Key 2")),
                Value::Seq(vec![
                    Value::String(String::from("Hello")),
                    Value::String(String::from("World")),
                ]),
            ),
        ])),
        Orientation::Column,
        Orientation::Row,
    ),
    "+---------------+---------------+"
    "|  Hello Key    |  Hello Key 2  |"
    "+---------------+---------------+"
    "|  World Value  | +---------+   |"
    "|               | |  Hello  |   |"
    "|               | +---------+   |"
    "|               | |  World  |   |"
    "|               | +---------+   |"
    "+---------------+---------------+"
);

test_table!(
    test_sequence_row_column,
    build_ron_table_orientation(
        Value::Seq(vec![
            Value::Map(Map::from_iter([
                (
                    Value::String(String::from("Hello Key")),
                    Value::String(String::from("World Value")),
                ),
                (
                    Value::String(String::from("Hello Key 2")),
                    Value::Seq(vec![
                        Value::String(String::from("Hello")),
                        Value::String(String::from("World")),
                    ]),
                ),
            ])),
            Value::String(String::from("Hello Key 2")),
            Value::String(String::from("Hello")),
            Value::Seq(vec![
                Value::String(String::from("Wold")),
                Value::String(String::from("Wod")),
                Value::String(String::from("Wo")),
                Value::String(String::from("Wo")),
            ]),
        ]),
        Orientation::Column,
        Orientation::Row,
    ),
    "+-----------------------------------+"
    "| +---------------+---------------+ |"
    "| |  Hello Key    |  Hello Key 2  | |"
    "| +---------------+---------------+ |"
    "| |  World Value  | +---------+   | |"
    "| |               | |  Hello  |   | |"
    "| |               | +---------+   | |"
    "| |               | |  World  |   | |"
    "| |               | +---------+   | |"
    "| +---------------+---------------+ |"
    "+-----------------------------------+"
    "|  Hello Key 2                      |"
    "+-----------------------------------+"
    "|  Hello                            |"
    "+-----------------------------------+"
    "| +--------+                        |"
    "| |  Wold  |                        |"
    "| +--------+                        |"
    "| |  Wod   |                        |"
    "| +--------+                        |"
    "| |  Wo    |                        |"
    "| +--------+                        |"
    "| |  Wo    |                        |"
    "| +--------+                        |"
    "+-----------------------------------+"
);

test_table!(
    test_sequence_row_column_1,
    build_ron_table_orientation(
        Value::Seq(vec![
            Value::Map(Map::from_iter([
                (
                    Value::String(String::from("Hello Key")),
                    Value::String(String::from("World Value")),
                ),
                (
                    Value::String(String::from("Hello Key 2")),
                    Value::Seq(vec![
                        Value::String(String::from("Hello")),
                        Value::String(String::from("World")),
                    ]),
                ),
            ])),
            Value::String(String::from("Hello Key 2")),
            Value::String(String::from("Hello")),
            Value::Seq(vec![
                Value::String(String::from("Wold")),
                Value::String(String::from("Wod")),
                Value::String(String::from("Wo")),
                Value::String(String::from("Wo")),
            ]),
        ]),
        Orientation::Row,
        Orientation::Column,
    ),
    "+-------------------------------------------+---------------+---------+----------------------------------+"
    "| +---------------+-----------------------+ |  Hello Key 2  |  Hello  | +--------+-------+------+------+ |"
    "| |  Hello Key    |  World Value          | |               |         | |  Wold  |  Wod  |  Wo  |  Wo  | |"
    "| +---------------+-----------------------+ |               |         | +--------+-------+------+------+ |"
    "| |  Hello Key 2  | +---------+---------+ | |               |         |                                  |"
    "| |               | |  Hello  |  World  | | |               |         |                                  |"
    "| |               | +---------+---------+ | |               |         |                                  |"
    "| +---------------+-----------------------+ |               |         |                                  |"
    "+-------------------------------------------+---------------+---------+----------------------------------+"
);

test_table!(
    test_sequence_row_column_2,
    build_ron_table_orientation(
        Value::Seq(vec![
            Value::Map(Map::from_iter([
                (
                    Value::String(String::from("Hello Key")),
                    Value::String(String::from("World Value")),
                ),
                (
                    Value::String(String::from("Hello Key 2")),
                    Value::Seq(vec![
                        Value::String(String::from("Hello")),
                        Value::String(String::from("World")),
                    ]),
                ),
            ])),
            Value::String(String::from("Hello Key 2")),
            Value::String(String::from("Hello")),
            Value::Seq(vec![
                Value::String(String::from("Wold")),
                Value::String(String::from("Wod")),
                Value::String(String::from("Wo")),
                Value::String(String::from("Wo")),
            ]),
        ]),
        Orientation::Row,
        Orientation::Row,
    ),
    "+-------------------------------------------+---------------+---------+----------------------------------+"
    "| +---------------+-----------------------+ |  Hello Key 2  |  Hello  | +--------+-------+------+------+ |"
    "| |  Hello Key    |  Hello Key 2          | |               |         | |  Wold  |  Wod  |  Wo  |  Wo  | |"
    "| +---------------+-----------------------+ |               |         | +--------+-------+------+------+ |"
    "| |  World Value  | +---------+---------+ | |               |         |                                  |"
    "| |               | |  Hello  |  World  | | |               |         |                                  |"
    "| |               | +---------+---------+ | |               |         |                                  |"
    "| +---------------+-----------------------+ |               |         |                                  |"
    "+-------------------------------------------+---------------+---------+----------------------------------+"
);

test_table!(
    test_sequence_row_column_3,
    build_ron_table_orientation(
        Value::Seq(vec![
            Value::Map(Map::from_iter([
                (
                    Value::String(String::from("Hello Key")),
                    Value::String(String::from("World Value")),
                ),
                (
                    Value::String(String::from("Hello Key 2")),
                    Value::Seq(vec![
                        Value::String(String::from("Hello")),
                        Value::String(String::from("World")),
                    ]),
                ),
            ])),
            Value::String(String::from("Hello Key 2")),
            Value::String(String::from("Hello")),
            Value::Seq(vec![
                Value::String(String::from("Wold")),
                Value::String(String::from("Wod")),
                Value::String(String::from("Wo")),
                Value::String(String::from("Wo")),
            ]),
        ]),
        Orientation::Column,
        Orientation::Column,
    ),
    "+-----------------------------------+"
    "| +---------------+---------------+ |"
    "| |  Hello Key    |  World Value  | |"
    "| +---------------+---------------+ |"
    "| |  Hello Key 2  | +---------+   | |"
    "| |               | |  Hello  |   | |"
    "| |               | +---------+   | |"
    "| |               | |  World  |   | |"
    "| |               | +---------+   | |"
    "| +---------------+---------------+ |"
    "+-----------------------------------+"
    "|  Hello Key 2                      |"
    "+-----------------------------------+"
    "|  Hello                            |"
    "+-----------------------------------+"
    "| +--------+                        |"
    "| |  Wold  |                        |"
    "| +--------+                        |"
    "| |  Wod   |                        |"
    "| +--------+                        |"
    "| |  Wo    |                        |"
    "| +--------+                        |"
    "| |  Wo    |                        |"
    "| +--------+                        |"
    "+-----------------------------------+"
);

test_table!(
    test_option_0,
    build_ron_table(Value::Option(Some(Box::new(Value::String(String::from(
        "123",
    )))))),
    "+-----+"
    "| 123 |"
    "+-----+"
);

test_table!(test_option_1, build_ron_table(Value::Option(None)), "");

test_table!(
    test_option_2,
    build_ron_table(Value::Map(Map::from_iter([
        (
            Value::String(String::from("Hello Key")),
            Value::String(String::from("World Value")),
        ),
        (
            Value::String(String::from("Hello")),
            Value::Option(Some(Box::new(Value::Seq(vec![
                Value::String(String::from("123")),
                Value::String(String::from("1")),
                Value::String(String::from("xasdasd")),
            ])))),
        ),
        (
            Value::String(String::from("Hello Key 2")),
            Value::String(String::from("Value 1")),
        ),
    ]))),
    "+---------------+---------------+"
    "|  Hello        | +-----------+ |"
    "|               | |  123      | |"
    "|               | +-----------+ |"
    "|               | |  1        | |"
    "|               | +-----------+ |"
    "|               | |  xasdasd  | |"
    "|               | +-----------+ |"
    "+---------------+---------------+"
    "|  Hello Key    |  World Value  |"
    "+---------------+---------------+"
    "|  Hello Key 2  |  Value 1      |"
    "+---------------+---------------+"
);

test_table!(
    test_option_3,
    build_ron_table(Value::Map(Map::from_iter([
        (
            Value::String(String::from("Hello Key")),
            Value::String(String::from("World Value")),
        ),
        (Value::String(String::from("Hello")), Value::Option(None)),
        (
            Value::String(String::from("Hello Key 2")),
            Value::String(String::from("Value 1")),
        ),
    ]))),
    "+---------------+---------------+"
    "|  Hello        |               |"
    "+---------------+---------------+"
    "|  Hello Key    |  World Value  |"
    "+---------------+---------------+"
    "|  Hello Key 2  |  Value 1      |"
    "+---------------+---------------+"
);

test_table!(
    test_alignment_center,
    RonTable::default()
        .with(Alignment::center_vertical())
        .with(Alignment::center())
        .build(&Value::Seq(vec![
            Value::Map(Map::from_iter([
                (
                    Value::String(String::from("Hello Key")),
                    Value::String(String::from("World Value")),
                ),
                (
                    Value::String(String::from("Hello Key 2")),
                    Value::Seq(vec![
                        Value::String(String::from("Hello")),
                        Value::String(String::from("World")),
                    ]),
                ),
            ])),
            Value::String(String::from("Hello Key 2")),
            Value::String(String::from("Hello")),
            Value::Seq(vec![
                Value::String(String::from("Wold")),
                Value::String(String::from("Wod")),
                Value::String(String::from("Wo")),
                Value::String(String::from("Wo")),
            ]),
        ])),
    "+-----------------------------------+"
    "| +---------------+---------------+ |"
    "| |   Hello Key   |  World Value  | |"
    "| +---------------+---------------+ |"
    "| |               |  +---------+  | |"
    "| |               |  |  Hello  |  | |"
    "| |  Hello Key 2  |  +---------+  | |"
    "| |               |  |  World  |  | |"
    "| |               |  +---------+  | |"
    "| +---------------+---------------+ |"
    "+-----------------------------------+"
    "|            Hello Key 2            |"
    "+-----------------------------------+"
    "|               Hello               |"
    "+-----------------------------------+"
    "|            +--------+             |"
    "|            |  Wold  |             |"
    "|            +--------+             |"
    "|            |  Wod   |             |"
    "|            +--------+             |"
    "|            |   Wo   |             |"
    "|            +--------+             |"
    "|            |   Wo   |             |"
    "|            +--------+             |"
    "+-----------------------------------+"
);

fn build_ron_table(value: Value) -> String {
    RonTable::default().build(&value)
}

fn build_ron_table_orientation(value: Value, seq: Orientation, map: Orientation) -> String {
    RonTable::default()
        .seq_orientation(seq)
        .map_orientation(map)
        .build(&value)
}
