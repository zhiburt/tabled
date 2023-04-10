use std::iter::FromIterator;

use ron::{value::Float, Map, Number, Value};
use ron_to_table::{Orientation, RonTable};
use tabled::settings::Alignment;

#[test]
fn test_unit() {
    let table = build_ron_table(Value::Unit);
    assert_eq!(table, "");
}

#[test]
fn test_string() {
    let table = build_ron_table(Value::String(String::from("123456789")));
    assert_eq!(table, "+-----------+\n| 123456789 |\n+-----------+");
}

#[test]
fn test_bool() {
    let table = build_ron_table(Value::Bool(true));
    assert_eq!(table, "+------+\n| true |\n+------+");

    let table = build_ron_table(Value::Bool(false));
    assert_eq!(table, "+-------+\n| false |\n+-------+");
}

#[test]
fn test_char() {
    let table = build_ron_table(Value::Char('a'));
    assert_eq!(table, "+---+\n| a |\n+---+");
}

#[test]
fn test_number() {
    let table = build_ron_table(Value::Number(Number::Integer(123456789)));
    assert_eq!(table, "+-----------+\n| 123456789 |\n+-----------+");

    let table = build_ron_table(Value::Number(Number::Float(Float::new(123.456789))));
    assert_eq!(table, "+------------+\n| 123.456789 |\n+------------+");
}

#[test]
fn test_sequence_0() {
    let table = build_ron_table(Value::Seq(vec![
        Value::String(String::from("Hello")),
        Value::String(String::from("World")),
    ]));
    assert_eq!(
        table,
        "+---------+\n\
         |  Hello  |\n\
         +---------+\n\
         |  World  |\n\
         +---------+"
    );
}

#[test]
fn test_sequence_1() {
    let table = build_ron_table(Value::Seq(vec![
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
    ]));
    assert_eq!(
        table,
        "+-------------+\n\
         | +---------+ |\n\
         | |  Hello  | |\n\
         | +---------+ |\n\
         | |  World  | |\n\
         | +---------+ |\n\
         +-------------+\n\
         |  Hello      |\n\
         +-------------+\n\
         | +---------+ |\n\
         | |  Hello  | |\n\
         | +---------+ |\n\
         | |  World  | |\n\
         | +---------+ |\n\
         +-------------+\n\
         |  World      |\n\
         +-------------+\n\
         | +---------+ |\n\
         | |  Hello  | |\n\
         | +---------+ |\n\
         | |  World  | |\n\
         | +---------+ |\n\
         +-------------+"
    );
}

#[test]
fn test_sequence_0_row() {
    let table = build_ron_table_orientation(
        Value::Seq(vec![
            Value::String(String::from("Hello")),
            Value::String(String::from("World")),
        ]),
        Orientation::Row,
        Orientation::Column,
    );
    assert_eq!(
        table,
        "+---------+---------+\n\
         |  Hello  |  World  |\n\
         +---------+---------+"
    );
}

#[test]
fn test_sequence_1_row() {
    let table = build_ron_table_orientation(
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
    );
    assert_eq!(
        table,
        "+-----------------------+---------+-----------------------+---------+-----------------------+\n\
         | +---------+---------+ |  Hello  | +---------+---------+ |  World  | +---------+---------+ |\n\
         | |  Hello  |  World  | |         | |  Hello  |  World  | |         | |  Hello  |  World  | |\n\
         | +---------+---------+ |         | +---------+---------+ |         | +---------+---------+ |\n\
         +-----------------------+---------+-----------------------+---------+-----------------------+"
    );
}

#[test]
fn test_map_0() {
    let table = build_ron_table(Value::Map(Map::from_iter([
        (
            Value::String(String::from("Hello Key")),
            Value::String(String::from("World Value")),
        ),
        (
            Value::String(String::from("Hello Key 2")),
            Value::String(String::from("Value 1")),
        ),
    ])));

    assert_eq!(
        table,
        "+---------------+---------------+\n\
         |  Hello Key    |  World Value  |\n\
         +---------------+---------------+\n\
         |  Hello Key 2  |  Value 1      |\n\
         +---------------+---------------+",
    );
}

#[test]
fn test_map_1() {
    let table = build_ron_table(Value::Map(Map::from_iter([
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
    ])));
    assert_eq!(
        table,
        "+---------------+---------------+\n\
         |  Hello Key    |  World Value  |\n\
         +---------------+---------------+\n\
         |  Hello Key 2  | +---------+   |\n\
         |               | |  Hello  |   |\n\
         |               | +---------+   |\n\
         |               | |  World  |   |\n\
         |               | +---------+   |\n\
         +---------------+---------------+"
    );
}

#[test]
fn test_map_0_row() {
    let table = build_ron_table_orientation(
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
    );
    assert_eq!(
        table,
        "+---------------+---------------+\n\
         |  Hello Key    |  Hello Key 2  |\n\
         +---------------+---------------+\n\
         |  World Value  |  Value 1      |\n\
         +---------------+---------------+"
    );
}

#[test]
fn test_map_1_row() {
    let table = build_ron_table_orientation(
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
    );
    assert_eq!(
        table,
        "+---------------+-----------------------+\n\
         |  Hello Key    |  Hello Key 2          |\n\
         +---------------+-----------------------+\n\
         |  World Value  | +---------+---------+ |\n\
         |               | |  Hello  |  World  | |\n\
         |               | +---------+---------+ |\n\
         +---------------+-----------------------+"
    );
}

#[test]
fn test_map_1_row_column() {
    let table = build_ron_table_orientation(
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
    );
    assert_eq!(
        table,
        "+---------------+---------------+\n\
         |  Hello Key    |  Hello Key 2  |\n\
         +---------------+---------------+\n\
         |  World Value  | +---------+   |\n\
         |               | |  Hello  |   |\n\
         |               | +---------+   |\n\
         |               | |  World  |   |\n\
         |               | +---------+   |\n\
         +---------------+---------------+"
    );
}

#[test]
fn test_sequence_row_column() {
    let table = build_ron_table_orientation(
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
    );
    assert_eq!(
        table,
        "+-----------------------------------+\n\
         | +---------------+---------------+ |\n\
         | |  Hello Key    |  Hello Key 2  | |\n\
         | +---------------+---------------+ |\n\
         | |  World Value  | +---------+   | |\n\
         | |               | |  Hello  |   | |\n\
         | |               | +---------+   | |\n\
         | |               | |  World  |   | |\n\
         | |               | +---------+   | |\n\
         | +---------------+---------------+ |\n\
         +-----------------------------------+\n\
         |  Hello Key 2                      |\n\
         +-----------------------------------+\n\
         |  Hello                            |\n\
         +-----------------------------------+\n\
         | +--------+                        |\n\
         | |  Wold  |                        |\n\
         | +--------+                        |\n\
         | |  Wod   |                        |\n\
         | +--------+                        |\n\
         | |  Wo    |                        |\n\
         | +--------+                        |\n\
         | |  Wo    |                        |\n\
         | +--------+                        |\n\
         +-----------------------------------+"
    );
}

#[test]
fn test_sequence_row_column_1() {
    let table = build_ron_table_orientation(
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
    );
    assert_eq!(
        table,
        "+-------------------------------------------+---------------+---------+----------------------------------+\n\
         | +---------------+-----------------------+ |  Hello Key 2  |  Hello  | +--------+-------+------+------+ |\n\
         | |  Hello Key    |  World Value          | |               |         | |  Wold  |  Wod  |  Wo  |  Wo  | |\n\
         | +---------------+-----------------------+ |               |         | +--------+-------+------+------+ |\n\
         | |  Hello Key 2  | +---------+---------+ | |               |         |                                  |\n\
         | |               | |  Hello  |  World  | | |               |         |                                  |\n\
         | |               | +---------+---------+ | |               |         |                                  |\n\
         | +---------------+-----------------------+ |               |         |                                  |\n\
         +-------------------------------------------+---------------+---------+----------------------------------+"
    );
}

#[test]
fn test_sequence_row_column_2() {
    let table = build_ron_table_orientation(
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
    );
    assert_eq!(
        table,
        "+-------------------------------------------+---------------+---------+----------------------------------+\n\
         | +---------------+-----------------------+ |  Hello Key 2  |  Hello  | +--------+-------+------+------+ |\n\
         | |  Hello Key    |  Hello Key 2          | |               |         | |  Wold  |  Wod  |  Wo  |  Wo  | |\n\
         | +---------------+-----------------------+ |               |         | +--------+-------+------+------+ |\n\
         | |  World Value  | +---------+---------+ | |               |         |                                  |\n\
         | |               | |  Hello  |  World  | | |               |         |                                  |\n\
         | |               | +---------+---------+ | |               |         |                                  |\n\
         | +---------------+-----------------------+ |               |         |                                  |\n\
         +-------------------------------------------+---------------+---------+----------------------------------+"
    );
}

#[test]
fn test_sequence_row_column_3() {
    let table = build_ron_table_orientation(
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
    );
    assert_eq!(
        table,
        "+-----------------------------------+\n\
         | +---------------+---------------+ |\n\
         | |  Hello Key    |  World Value  | |\n\
         | +---------------+---------------+ |\n\
         | |  Hello Key 2  | +---------+   | |\n\
         | |               | |  Hello  |   | |\n\
         | |               | +---------+   | |\n\
         | |               | |  World  |   | |\n\
         | |               | +---------+   | |\n\
         | +---------------+---------------+ |\n\
         +-----------------------------------+\n\
         |  Hello Key 2                      |\n\
         +-----------------------------------+\n\
         |  Hello                            |\n\
         +-----------------------------------+\n\
         | +--------+                        |\n\
         | |  Wold  |                        |\n\
         | +--------+                        |\n\
         | |  Wod   |                        |\n\
         | +--------+                        |\n\
         | |  Wo    |                        |\n\
         | +--------+                        |\n\
         | |  Wo    |                        |\n\
         | +--------+                        |\n\
         +-----------------------------------+"
    );
}

#[test]
fn test_option_0() {
    let table = build_ron_table(Value::Option(Some(Box::new(Value::String(String::from(
        "123",
    ))))));

    assert_eq!(
        table,
        "+-----+\n\
         | 123 |\n\
         +-----+",
    );
}

#[test]
fn test_option_1() {
    let table = build_ron_table(Value::Option(None));

    assert_eq!(table, "");
}

#[test]
fn test_option_2() {
    let table = build_ron_table(Value::Map(Map::from_iter([
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
    ])));

    assert_eq!(
        table,
        "+---------------+---------------+\n\
         |  Hello        | +-----------+ |\n\
         |               | |  123      | |\n\
         |               | +-----------+ |\n\
         |               | |  1        | |\n\
         |               | +-----------+ |\n\
         |               | |  xasdasd  | |\n\
         |               | +-----------+ |\n\
         +---------------+---------------+\n\
         |  Hello Key    |  World Value  |\n\
         +---------------+---------------+\n\
         |  Hello Key 2  |  Value 1      |\n\
         +---------------+---------------+"
    );
}

#[test]
fn test_option_3() {
    let table = build_ron_table(Value::Map(Map::from_iter([
        (
            Value::String(String::from("Hello Key")),
            Value::String(String::from("World Value")),
        ),
        (Value::String(String::from("Hello")), Value::Option(None)),
        (
            Value::String(String::from("Hello Key 2")),
            Value::String(String::from("Value 1")),
        ),
    ])));

    assert_eq!(
        table,
        "+---------------+---------------+\n\
         |  Hello        |               |\n\
         +---------------+---------------+\n\
         |  Hello Key    |  World Value  |\n\
         +---------------+---------------+\n\
         |  Hello Key 2  |  Value 1      |\n\
         +---------------+---------------+"
    );
}

#[test]
fn test_alignment_center() {
    let table = RonTable::default()
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
        ]));
    assert_eq!(
        table,
        "+-----------------------------------+\n\
         | +---------------+---------------+ |\n\
         | |   Hello Key   |  World Value  | |\n\
         | +---------------+---------------+ |\n\
         | |               |  +---------+  | |\n\
         | |               |  |  Hello  |  | |\n\
         | |  Hello Key 2  |  +---------+  | |\n\
         | |               |  |  World  |  | |\n\
         | |               |  +---------+  | |\n\
         | +---------------+---------------+ |\n\
         +-----------------------------------+\n\
         |            Hello Key 2            |\n\
         +-----------------------------------+\n\
         |               Hello               |\n\
         +-----------------------------------+\n\
         |            +--------+             |\n\
         |            |  Wold  |             |\n\
         |            +--------+             |\n\
         |            |  Wod   |             |\n\
         |            +--------+             |\n\
         |            |   Wo   |             |\n\
         |            +--------+             |\n\
         |            |   Wo   |             |\n\
         |            +--------+             |\n\
         +-----------------------------------+"
    );
}

fn build_ron_table(value: Value) -> String {
    RonTable::default().build(&value)
}

fn build_ron_table_orientation(value: Value, seq: Orientation, map: Orientation) -> String {
    RonTable::default()
        .seq_orientation(seq)
        .map_orientation(map)
        .build(&value)
}
