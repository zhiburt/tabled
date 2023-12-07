#![cfg(feature = "std")]

use std::iter::FromIterator;

use crate::matrix::Matrix;
use tabled::{builder::Builder, Table};
use testing_table::test_table;

test_table!(
    builder_index,
    Table::builder(Matrix::list::<3, 2>()).index().build(),
    "+---+---+----------+----------+"
    "|   | N | column 0 | column 1 |"
    "+---+---+----------+----------+"
    "| 0 | 0 | 0-0      | 0-1      |"
    "+---+---+----------+----------+"
    "| 1 | 1 | 1-0      | 1-1      |"
    "+---+---+----------+----------+"
    "| 2 | 2 | 2-0      | 2-1      |"
    "+---+---+----------+----------+"
);

test_table!(
    builder_index_transpose,
    Table::builder(Matrix::list::<4, 2>()).index().transpose().build(),
    "+----------+-----+-----+-----+-----+"
    "|          | 0   | 1   | 2   | 3   |"
    "+----------+-----+-----+-----+-----+"
    "| N        | 0   | 1   | 2   | 3   |"
    "+----------+-----+-----+-----+-----+"
    "| column 0 | 0-0 | 1-0 | 2-0 | 3-0 |"
    "+----------+-----+-----+-----+-----+"
    "| column 1 | 0-1 | 1-1 | 2-1 | 3-1 |"
    "+----------+-----+-----+-----+-----+"
);

test_table!(
    builder_index_0,
    Table::builder(Matrix::list::<4, 2>()).index().column(0).build(),
    "+---+----------+----------+"
    "|   | column 0 | column 1 |"
    "+---+----------+----------+"
    "| N |          |          |"
    "+---+----------+----------+"
    "| 0 | 0-0      | 0-1      |"
    "+---+----------+----------+"
    "| 1 | 1-0      | 1-1      |"
    "+---+----------+----------+"
    "| 2 | 2-0      | 2-1      |"
    "+---+----------+----------+"
    "| 3 | 3-0      | 3-1      |"
    "+---+----------+----------+"
);

test_table!(
    builder_index_0_no_name,
    Table::builder(Matrix::list::<4, 2>()).index().column(0).name(None).build(),
    "+---+----------+----------+"
    "|   | column 0 | column 1 |"
    "+---+----------+----------+"
    "| 0 | 0-0      | 0-1      |"
    "+---+----------+----------+"
    "| 1 | 1-0      | 1-1      |"
    "+---+----------+----------+"
    "| 2 | 2-0      | 2-1      |"
    "+---+----------+----------+"
    "| 3 | 3-0      | 3-1      |"
    "+---+----------+----------+"
);

test_table!(
    builder_index_0_name,
    Table::builder(Matrix::list::<4, 2>()).index().column(0).name(Some("Hello World".into())).build(),
    "+-------------+----------+----------+"
    "|             | column 0 | column 1 |"
    "+-------------+----------+----------+"
    "| Hello World |          |          |"
    "+-------------+----------+----------+"
    "| 0           | 0-0      | 0-1      |"
    "+-------------+----------+----------+"
    "| 1           | 1-0      | 1-1      |"
    "+-------------+----------+----------+"
    "| 2           | 2-0      | 2-1      |"
    "+-------------+----------+----------+"
    "| 3           | 3-0      | 3-1      |"
    "+-------------+----------+----------+"
);

test_table!(
    builder_index_0_name_transpose,
    Table::builder(Matrix::list::<4, 2>()).index().column(0).name(Some("Hello World".into())).transpose().build(),
    "+-------------+-----+-----+-----+-----+"
    "| Hello World | 0   | 1   | 2   | 3   |"
    "+-------------+-----+-----+-----+-----+"
    "| column 0    | 0-0 | 1-0 | 2-0 | 3-0 |"
    "+-------------+-----+-----+-----+-----+"
    "| column 1    | 0-1 | 1-1 | 2-1 | 3-1 |"
    "+-------------+-----+-----+-----+-----+"
);

test_table!(
    builder_index_with_no_columns,
    Builder::from_iter([["1", "2", "3"], ["a", "b", "c"], ["d", "e", "f"]]).index().build(),
    "+---+---+---+---+"
    "|   | 1 | 2 | 3 |"
    "+---+---+---+---+"
    "| 0 | a | b | c |"
    "+---+---+---+---+"
    "| 1 | d | e | f |"
    "+---+---+---+---+"
);

test_table!(
    builder_index_with_no_columns_and_name,
    Builder::from_iter([["1", "2", "3"], ["a", "b", "c"], ["d", "e", "f"]])
        .index()
        .name(Some(String::from("Hello World")))
        .build(),
    "+-------------+---+---+---+"
    "|             | 1 | 2 | 3 |"
    "+-------------+---+---+---+"
    "| Hello World |   |   |   |"
    "+-------------+---+---+---+"
    "| 0           | a | b | c |"
    "+-------------+---+---+---+"
    "| 1           | d | e | f |"
    "+-------------+---+---+---+"
);

test_table!(
    builder_index_with_no_columns_transpose,
    Builder::from_iter([["1", "2", "3"], ["a", "b", "c"], ["d", "e", "f"]])
        .index()
        .transpose()
        .build(),
    "+---+---+---+"
    "|   | 0 | 1 |"
    "+---+---+---+"
    "| 1 | a | d |"
    "+---+---+---+"
    "| 2 | b | e |"
    "+---+---+---+"
    "| 3 | c | f |"
    "+---+---+---+"
);

test_table!(builder_index_empty, Builder::default().index().build(), "");

test_table!(
    builder_index_transpose_empty,
    Builder::default().index().transpose().build(),
    ""
);

test_table!(
    builder_index_invalid_doesnt_panic,
    Builder::default().index().column(100).build(),
    ""
);

test_table!(
    builder_index_name_doesnt_shown_when_empty,
    Builder::default()
        .index()
        .name(Some("Hello World".into()))
        .build(),
    ""
);

test_table!(
    builder_index_with_header_but_no_data,
    {
        let mut b = Builder::default();
        b.push_record(["one", "two", "three"]);
        b.index().build()
    },
    "+--+-----+-----+-------+"
    "|  | one | two | three |"
    "+--+-----+-----+-------+"
);

#[test]
fn builder_index_transpose_transpose() {
    let data = Matrix::list::<4, 2>();
    let builder = Table::builder(data).index();

    let orig_table = builder.clone().build().to_string();
    let two_times_transposed_table = builder.transpose().transpose().build().to_string();

    assert_eq!(orig_table, two_times_transposed_table,);
}

#[test]
fn builder_index_no_name_transpose_transpose() {
    let data = Matrix::list::<4, 2>();
    let builder = Table::builder(data).index().name(None);

    let orig_table = builder.clone().build().to_string();
    let two_times_transposed_table = builder.transpose().transpose().build().to_string();

    assert_eq!(orig_table, two_times_transposed_table,);
}

test_table!(
    builder_index_convert_back_to_builder,
    {
        let mut b = Builder::new();
        b.push_record(["one", "two", "three"]);
        Builder::from(b.clone().index().hide()).build()
    },
    "+-----+-----+-------+"
    "| one | two | three |"
    "+-----+-----+-------+"
);

test_table!(
    index_column_3_times,
    Table::builder(Matrix::list::<3, 2>()).index().column(0).column(0).column(0).build(),
    "+----------+"
    "|          |"
    "+----------+"
    "| column 1 |"
    "+----------+"
    "| 0-1      |"
    "+----------+"
    "| 1-1      |"
    "+----------+"
    "| 2-1      |"
    "+----------+"
);

test_table!(
    index_transpose_with_no_name0,
    Builder::from_iter([["col1", "col2", "col3"], ["a", "b", "c"], ["d", "e", "f"]])
        .index()
        .transpose()
        .name(Some(String::from("hello world")))
        .transpose()
        .build(),
    "+-------------+------+------+------+"
    "|             | col1 | col2 | col3 |"
    "+-------------+------+------+------+"
    "| hello world |      |      |      |"
    "+-------------+------+------+------+"
    "| 0           | a    | b    | c    |"
    "+-------------+------+------+------+"
    "| 1           | d    | e    | f    |"
    "+-------------+------+------+------+"
);

test_table!(
    index_transpose_with_no_name1,
    Builder::from_iter([["col1", "col2", "col3"], ["a", "b", "c"], ["d", "e", "f"]])
        .index()
        .transpose()
        .name(Some(String::from("hello world")))
        .build(),
    "+-------------+---+---+"
    "| hello world | 0 | 1 |"
    "+-------------+---+---+"
    "| col1        | a | d |"
    "+-------------+---+---+"
    "| col2        | b | e |"
    "+-------------+---+---+"
    "| col3        | c | f |"
    "+-------------+---+---+"
);

test_table!(
    index_transpose_with_no_name2,
    Builder::from_iter([["col1", "col2", "col3"], ["a", "b", "c"], ["d", "e", "f"]])
        .index()
        .transpose()
        .build(),
    "+------+---+---+"
    "|      | 0 | 1 |"
    "+------+---+---+"
    "| col1 | a | d |"
    "+------+---+---+"
    "| col2 | b | e |"
    "+------+---+---+"
    "| col3 | c | f |"
    "+------+---+---+"
);
