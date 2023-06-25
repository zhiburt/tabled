#![cfg(feature = "std")]

use std::iter::FromIterator;

use tabled::builder::Builder;

use testing_table::test_table;

test_table!(
    push_record,
    {
        let mut b = Builder::default();
        b.push_record(["1", "2", "3"]);
        b.push_record(["a", "b", "c"]);
        b.push_record(["d", "e", "f"]);
        b.build()
    },
    "+---+---+---+"
    "| 1 | 2 | 3 |"
    "+---+---+---+"
    "| a | b | c |"
    "+---+---+---+"
    "| d | e | f |"
    "+---+---+---+"
);

test_table!(
    set_header,
    {
        let mut b = Builder::default();
        b.set_header(["1", "2", "3"]);
        b.push_record(["a", "b", "c"]);
        b.push_record(["d", "e", "f"]);
        b.build()
    },
    "+---+---+---+"
    "| 1 | 2 | 3 |"
    "+---+---+---+"
    "| a | b | c |"
    "+---+---+---+"
    "| d | e | f |"
    "+---+---+---+"
);

test_table!(
    header_remove_0,
    {
        let mut b = Builder::default();
        b.set_header(["1", "2", "3"]);
        b.push_record(["a", "b", "c"]);
        b.push_record(["d", "e", "f"]);
        b.remove_header();
        b.build()
    },
    "+---+---+---+"
    "| a | b | c |"
    "+---+---+---+"
    "| d | e | f |"
    "+---+---+---+"
);

test_table!(
    header_remove_1,
    {
        let mut b = Builder::default();
        b.set_header(["1", "2", "3", "4", "5"]);
        b.push_record(["a", "b", "c"]);
        b.push_record(["d", "e", "f"]);
        b.remove_header();
        b.build()
    },
    "+---+---+---+"
    "| a | b | c |"
    "+---+---+---+"
    "| d | e | f |"
    "+---+---+---+"
);

test_table!(
    from_iter,
    Builder::from_iter([["n", "name"], ["0", "Dmitriy"], ["1", "Vladislav"]]).build(),
    "+---+-----------+"
    "| n | name      |"
    "+---+-----------+"
    "| 0 | Dmitriy   |"
    "+---+-----------+"
    "| 1 | Vladislav |"
    "+---+-----------+"
);

test_table!(
    used_with_different_number_of_columns_0,
    {
        let mut b = Builder::default();
        b.set_header(["1", "2"]);
        b.push_record(["a", "b", "c"]);
        b.push_record(["d"]);
        b.build()
    },
    "+---+---+---+"
    "| 1 | 2 |   |"
    "+---+---+---+"
    "| a | b | c |"
    "+---+---+---+"
    "| d |   |   |"
    "+---+---+---+"
);

test_table!(
    used_with_different_number_of_columns_1,
    {
        let mut b = Builder::default();
        b.set_header(["1", "2", "3"]);
        b.push_record(["a", "b"]);
        b.push_record(["d"]);
        b.build()
    },
    "+---+---+---+"
    "| 1 | 2 | 3 |"
    "+---+---+---+"
    "| a | b |   |"
    "+---+---+---+"
    "| d |   |   |"
    "+---+---+---+"
);

test_table!(
    used_with_different_number_of_columns_2,
    {
        let mut b = Builder::default();
        b.set_header(["1"]);
        b.push_record(["a", "b"]);
        b.push_record(["d", "e", "f"]);
        b.build()
    },
    "+---+---+---+"
    "| 1 |   |   |"
    "+---+---+---+"
    "| a | b |   |"
    "+---+---+---+"
    "| d | e | f |"
    "+---+---+---+"
);

test_table!(
    with_default_cell_0,
    {
        let mut b = Builder::default();
        b.set_header(["1", "2"]).set_default_text("NaN").push_record(["a", "b", "c"]).push_record(["d"]);
        b.build()
    },
    "+---+-----+-----+"
    "| 1 | 2   | NaN |"
    "+---+-----+-----+"
    "| a | b   | c   |"
    "+---+-----+-----+"
    "| d | NaN | NaN |"
    "+---+-----+-----+"
);

test_table!(
    with_default_cell_1,
    {
        let mut b = Builder::default();
        b.set_header(["1"]).set_default_text("NaN").push_record(["a", "b"]).push_record(["d", "e", "f"]);
        b.build()
    },
    "+---+-----+-----+"
    "| 1 | NaN | NaN |"
    "+---+-----+-----+"
    "| a | b   | NaN |"
    "+---+-----+-----+"
    "| d | e   | f   |"
    "+---+-----+-----+"
);

test_table!(
    extend,
    {
        let mut b = Builder::default();
        b.extend(["1", "2", "3"]);
        b.extend(["a", "b", "c"]);
        b.extend(["d", "e", "f"]);
        b.build()
    },
    "+---+---+---+"
    "| 1 | 2 | 3 |"
    "+---+---+---+"
    "| a | b | c |"
    "+---+---+---+"
    "| d | e | f |"
    "+---+---+---+"
);

test_table!(
    from_vector_0,
    Builder::from_iter(vec![
        vec!["1".to_string(), "2".to_string(), "3".to_string()],
        vec!["a".to_string(), "b".to_string(), "c".to_string()],
        vec!["d".to_string(), "e".to_string(), "f".to_string()],
    ])
    .build(),
    "+---+---+---+"
    "| 1 | 2 | 3 |"
    "+---+---+---+"
    "| a | b | c |"
    "+---+---+---+"
    "| d | e | f |"
    "+---+---+---+"
);

test_table!(
    from_with_empty_lines_0,
    Builder::from_iter(vec![
        vec!["1".to_string(), "2".to_string(), "3".to_string()],
        vec![],
        vec![],
        vec!["d".to_string(), "e".to_string(), "f".to_string()],
    ])
    .build(),
    "+---+---+---+"
    "| 1 | 2 | 3 |"
    "+---+---+---+"
    "|   |   |   |"
    "+---+---+---+"
    "|   |   |   |"
    "+---+---+---+"
    "| d | e | f |"
    "+---+---+---+"
);

test_table!(
    from_with_empty_lines_1,
    Builder::from_iter(vec![
        vec!["1".to_string(), "2".to_string(), "3".to_string()],
        vec![],
        vec!["d".to_string(), "e".to_string(), "f".to_string()],
    ])
    .build(),
    "+---+---+---+"
    "| 1 | 2 | 3 |"
    "+---+---+---+"
    "|   |   |   |"
    "+---+---+---+"
    "| d | e | f |"
    "+---+---+---+"
);

test_table!(
    from_with_empty_lines_2,
    Builder::from_iter(vec![
        vec![],
        vec!["1".to_string(), "2".to_string(), "3".to_string()],
        vec!["d".to_string(), "e".to_string(), "f".to_string()],
    ])
    .build(),
    "+---+---+---+"
    "|   |   |   |"
    "+---+---+---+"
    "| 1 | 2 | 3 |"
    "+---+---+---+"
    "| d | e | f |"
    "+---+---+---+"
);

test_table!(
    from_with_empty_lines_3,
    Builder::from_iter(vec![
        vec!["1".to_string(), "2".to_string(), "3".to_string()],
        vec!["d".to_string(), "e".to_string(), "f".to_string()],
        vec![],
    ])
    .build(),
    "+---+---+---+"
    "| 1 | 2 | 3 |"
    "+---+---+---+"
    "| d | e | f |"
    "+---+---+---+"
    "|   |   |   |"
    "+---+---+---+"
);

test_table!(
    clean_0,
    clean(Builder::from_iter([["1", "2", "3"], ["a", "b", "c"], ["d", "e", "f"]])),
    "+---+---+---+"
    "| 1 | 2 | 3 |"
    "+---+---+---+"
    "| a | b | c |"
    "+---+---+---+"
    "| d | e | f |"
    "+---+---+---+"
);

test_table!(
    clean_1,
    clean(Builder::from_iter([["", "2", "3"], ["", "b", "c"], ["", "e", "f"]])),
    "+---+---+"
    "| 2 | 3 |"
    "+---+---+"
    "| b | c |"
    "+---+---+"
    "| e | f |"
    "+---+---+"
);

test_table!(
    clean_2,
    clean(Builder::from_iter([["1", "", "3"], ["a", "", "c"], ["d", "", "f"]])),
    "+---+---+"
    "| 1 | 3 |"
    "+---+---+"
    "| a | c |"
    "+---+---+"
    "| d | f |"
    "+---+---+"
);

test_table!(
    clean_3,
    clean(Builder::from_iter([["1", "2", ""], ["a", "b", ""], ["d", "e", ""]])),
    "+---+---+"
    "| 1 | 2 |"
    "+---+---+"
    "| a | b |"
    "+---+---+"
    "| d | e |"
    "+---+---+"
);

test_table!(
    clean_4,
    clean(Builder::from_iter([["", "", "3"], ["", "", "c"], ["", "", "f"]])),
    "+---+"
    "| 3 |"
    "+---+"
    "| c |"
    "+---+"
    "| f |"
    "+---+"
);

test_table!(
    clean_5,
    clean(Builder::from_iter([["1", "", ""], ["a", "", ""], ["d", "", ""]])),
    "+---+"
    "| 1 |"
    "+---+"
    "| a |"
    "+---+"
    "| d |"
    "+---+"
);

test_table!(
    clean_6,
    clean(Builder::from_iter([["", "2", ""], ["", "b", ""], ["", "e", ""]])),
    "+---+"
    "| 2 |"
    "+---+"
    "| b |"
    "+---+"
    "| e |"
    "+---+"
);

test_table!(
    clean_7,
    clean(Builder::from_iter([["", "", ""], ["a", "b", "c"], ["d", "e", "f"]])),
    "+---+---+---+"
    "| a | b | c |"
    "+---+---+---+"
    "| d | e | f |"
    "+---+---+---+"
);

test_table!(
    clean_8,
    clean(Builder::from_iter([["1", "2", "3"], ["", "", ""], ["d", "e", "f"]])),
    "+---+---+---+"
    "| 1 | 2 | 3 |"
    "+---+---+---+"
    "| d | e | f |"
    "+---+---+---+"
);

test_table!(
    clean_9,
    clean(Builder::from_iter([["1", "2", "3"], ["a", "b", "c"], ["", "", ""]])),
    "+---+---+---+"
    "| 1 | 2 | 3 |"
    "+---+---+---+"
    "| a | b | c |"
    "+---+---+---+"
);

test_table!(
    clean_10,
    clean(Builder::from_iter([["", "", ""], ["", "", ""], ["d", "e", "f"]])),
    "+---+---+---+"
    "| d | e | f |"
    "+---+---+---+"
);

test_table!(
    clean_11,
    clean(Builder::from_iter([["1", "2", "3"], ["", "", ""], ["", "", ""]])),
    "+---+---+---+"
    "| 1 | 2 | 3 |"
    "+---+---+---+"
);

test_table!(
    clean_12,
    clean(Builder::from_iter([["", "", ""], ["a", "b", "c"], ["", "", ""]])),
    "+---+---+---+"
    "| a | b | c |"
    "+---+---+---+"
);

test_table!(
    clean_13,
    clean(Builder::from_iter([["1", "", "3"], ["", "", ""], ["d", "", "f"]])),
    "+---+---+"
    "| 1 | 3 |"
    "+---+---+"
    "| d | f |"
    "+---+---+"
);

test_table!(
    clean_with_columns_0,
    clean(Builder::from_iter([["1", "2", "3"], ["a", "b", "c"], ["d", "e", "f"]]).set_header(["col1", "col2", "col3"]).clone()),
    "+------+------+------+"
    "| col1 | col2 | col3 |"
    "+------+------+------+"
    "| 1    | 2    | 3    |"
    "+------+------+------+"
    "| a    | b    | c    |"
    "+------+------+------+"
    "| d    | e    | f    |"
    "+------+------+------+"
);

test_table!(
    clean_with_columns_1,
    clean(Builder::from_iter([["", "2", "3"], ["", "b", "c"], ["", "e", "f"]]).set_header(["col1", "col2", "col3"]).clone()),
    "+------+------+"
    "| col2 | col3 |"
    "+------+------+"
    "| 2    | 3    |"
    "+------+------+"
    "| b    | c    |"
    "+------+------+"
    "| e    | f    |"
    "+------+------+"
);

test_table!(
    clean_with_columns_2,
    clean(Builder::from_iter([["1", "", "3"], ["a", "", "c"], ["d", "", "f"]]).set_header(["col1", "col2", "col3"]).clone()),
    "+------+------+"
    "| col1 | col3 |"
    "+------+------+"
    "| 1    | 3    |"
    "+------+------+"
    "| a    | c    |"
    "+------+------+"
    "| d    | f    |"
    "+------+------+"
);

test_table!(
    clean_with_columns_3,
    clean(Builder::from_iter([["1", "2", ""], ["a", "b", ""], ["d", "e", ""]]).set_header(["col1", "col2", "col3"]).clone()),
    "+------+------+"
    "| col1 | col2 |"
    "+------+------+"
    "| 1    | 2    |"
    "+------+------+"
    "| a    | b    |"
    "+------+------+"
    "| d    | e    |"
    "+------+------+"
);

test_table!(
    clean_with_columns_4,
    clean(Builder::from_iter([["", "", "3"], ["", "", "c"], ["", "", "f"]]).set_header(["col1", "col2", "col3"]).clone()),
    "+------+"
    "| col3 |"
    "+------+"
    "| 3    |"
    "+------+"
    "| c    |"
    "+------+"
    "| f    |"
    "+------+"
);

test_table!(
    clean_with_columns_5,
    clean(Builder::from_iter([["1", "", ""], ["a", "", ""], ["d", "", ""]]).set_header(["col1", "col2", "col3"]).clone()),
    "+------+"
    "| col1 |"
    "+------+"
    "| 1    |"
    "+------+"
    "| a    |"
    "+------+"
    "| d    |"
    "+------+"
);

test_table!(
    clean_with_columns_6,
    clean(Builder::from_iter([["", "2", ""], ["", "b", ""], ["", "e", ""]]).set_header(["col1", "col2", "col3"]).clone()),
    "+------+"
    "| col2 |"
    "+------+"
    "| 2    |"
    "+------+"
    "| b    |"
    "+------+"
    "| e    |"
    "+------+"
);

test_table!(
    clean_with_columns_7,
    clean(
        Builder::from_iter([["", "", ""], ["", "", ""], ["", "", ""]])
            .set_header(["col1", "col2", "col3"])
            .clone()
    ),
    ""
);

test_table!(
    clean_with_columns_8,
    clean(Builder::from_iter([["", "", ""], ["a", "b", "c"], ["d", "e", "f"]]).set_header(["col1", "col2", "col3"]).clone()),
    "+------+------+------+"
    "| col1 | col2 | col3 |"
    "+------+------+------+"
    "| a    | b    | c    |"
    "+------+------+------+"
    "| d    | e    | f    |"
    "+------+------+------+"
);

test_table!(
    clean_with_columns_9,
    clean(Builder::from_iter([["1", "2", "3"], ["", "", ""], ["d", "e", "f"]]).set_header(["col1", "col2", "col3"]).clone()),
    "+------+------+------+"
    "| col1 | col2 | col3 |"
    "+------+------+------+"
    "| 1    | 2    | 3    |"
    "+------+------+------+"
    "| d    | e    | f    |"
    "+------+------+------+"
);

test_table!(
    clean_with_columns_10,
    clean(Builder::from_iter([["1", "2", "3"], ["a", "b", "c"], ["", "", ""]]).set_header(["col1", "col2", "col3"]).clone()),
    "+------+------+------+"
    "| col1 | col2 | col3 |"
    "+------+------+------+"
    "| 1    | 2    | 3    |"
    "+------+------+------+"
    "| a    | b    | c    |"
    "+------+------+------+"
);

test_table!(
    clean_with_columns_11,
    clean(Builder::from_iter([["", "", ""], ["", "", ""], ["d", "e", "f"]]).set_header(["col1", "col2", "col3"]).clone()),
    "+------+------+------+"
    "| col1 | col2 | col3 |"
    "+------+------+------+"
    "| d    | e    | f    |"
    "+------+------+------+"
);

test_table!(
    clean_with_columns_12,
    clean(Builder::from_iter([["1", "2", "3"], ["", "", ""], ["", "", ""]]).set_header(["col1", "col2", "col3"]).clone()),
    "+------+------+------+"
    "| col1 | col2 | col3 |"
    "+------+------+------+"
    "| 1    | 2    | 3    |"
    "+------+------+------+"
);

test_table!(
    clean_with_columns_13,
    clean(Builder::from_iter([["", "", ""], ["a", "b", "c"], ["", "", ""]]).set_header(["col1", "col2", "col3"]).clone()),
    "+------+------+------+"
    "| col1 | col2 | col3 |"
    "+------+------+------+"
    "| a    | b    | c    |"
    "+------+------+------+"
);

test_table!(
    clean_with_columns_14,
    clean(Builder::from_iter([["1", "", "3"], ["", "", ""], ["d", "", "f"]]).set_header(["col1", "col2", "col3"]).clone()),
    "+------+------+"
    "| col1 | col3 |"
    "+------+------+"
    "| 1    | 3    |"
    "+------+------+"
    "| d    | f    |"
    "+------+------+"
);

test_table!(clean_empty_0, clean(Builder::from_iter([[""; 0]; 0])), "");

test_table!(clean_empty_1, clean(Builder::from_iter([[""; 0]; 10])), "");

test_table!(
    index,
    Builder::from_iter([["n", "name"], ["0", "Dmitriy"], ["1", "Vladislav"]]).index().build(),
    "+---+---+-----------+"
    "|   | 0 | 1         |"
    "+---+---+-----------+"
    "| 0 | n | name      |"
    "+---+---+-----------+"
    "| 1 | 0 | Dmitriy   |"
    "+---+---+-----------+"
    "| 2 | 1 | Vladislav |"
    "+---+---+-----------+"
);

test_table!(
    index_set_name,
    Builder::from_iter([["n", "name"], ["0", "Dmitriy"], ["1", "Vladislav"]])
        .index()
        .name(Some("A index name".into()))
        .build(),
    "+--------------+---+-----------+"
    "|              | 0 | 1         |"
    "+--------------+---+-----------+"
    "| A index name |   |           |"
    "+--------------+---+-----------+"
    "| 0            | n | name      |"
    "+--------------+---+-----------+"
    "| 1            | 0 | Dmitriy   |"
    "+--------------+---+-----------+"
    "| 2            | 1 | Vladislav |"
    "+--------------+---+-----------+"
);

test_table!(
    index_enumeration,
    Builder::from_iter([["n", "name"], ["0", "Dmitriy"], ["1", "Vladislav"]])
        .index()
        .hide()
        .build(),
    "+---+-----------+"
    "| 0 | 1         |"
    "+---+-----------+"
    "| n | name      |"
    "+---+-----------+"
    "| 0 | Dmitriy   |"
    "+---+-----------+"
    "| 1 | Vladislav |"
    "+---+-----------+"
);

test_table!(
    set_index,
    Builder::from_iter([["n", "name"], ["0", "Dmitriy"], ["1", "Vladislav"]])
        .index()
        .column(1)
        .build(),
    "+-----------+---+"
    "|           | 0 |"
    "+-----------+---+"
    "| 1         |   |"
    "+-----------+---+"
    "| name      | n |"
    "+-----------+---+"
    "| Dmitriy   | 0 |"
    "+-----------+---+"
    "| Vladislav | 1 |"
    "+-----------+---+"
);

test_table!(
    set_index_and_set_index_name_0,
    Builder::from_iter([["n", "name"], ["0", "Dmitriy"], ["1", "Vladislav"]])
        .index()
        .column(1)
        .name(Some("Hello".into()))
        .build(),
    "+-----------+---+"
    "|           | 0 |"
    "+-----------+---+"
    "| Hello     |   |"
    "+-----------+---+"
    "| name      | n |"
    "+-----------+---+"
    "| Dmitriy   | 0 |"
    "+-----------+---+"
    "| Vladislav | 1 |"
    "+-----------+---+"
);

test_table!(
    set_index_and_set_index_name_1,
    Builder::from_iter([["n", "name"], ["0", "Dmitriy"], ["1", "Vladislav"]])
        .index()
        .column(1)
        .name(None)
        .build(),
    "+-----------+---+"
    "|           | 0 |"
    "+-----------+---+"
    "| name      | n |"
    "+-----------+---+"
    "| Dmitriy   | 0 |"
    "+-----------+---+"
    "| Vladislav | 1 |"
    "+-----------+---+"
);

test_table!(
    index_transpose,
    Builder::from_iter([["n", "name", "zz"], ["0", "Dmitriy", "123"], ["1", "Vladislav", "123"]])
        .index()
        .transpose()
        .build(),
    "+---+------+---------+-----------+"
    "|   | 0    | 1       | 2         |"
    "+---+------+---------+-----------+"
    "| 0 | n    | 0       | 1         |"
    "+---+------+---------+-----------+"
    "| 1 | name | Dmitriy | Vladislav |"
    "+---+------+---------+-----------+"
    "| 2 | zz   | 123     | 123       |"
    "+---+------+---------+-----------+"
);

fn clean(mut b: Builder) -> String {
    b.clean();
    b.build().to_string()
}
