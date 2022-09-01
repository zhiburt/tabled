use std::iter::FromIterator;

use tabled::builder::Builder;

use util::test_table;

mod util;

test_table!(
    add_record,
    Builder::default()
        .add_record(["1", "2", "3"])
        .add_record(["a", "b", "c"])
        .add_record(["d", "e", "f"])
        .clone()
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
    set_columns,
    Builder::default()
        .add_record(["a", "b", "c"])
        .add_record(["d", "e", "f"])
        .set_columns(["1", "2", "3"])
        .clone()
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
    header_remove_0,
    Builder::default()
        .add_record(["a", "b", "c"])
        .add_record(["d", "e", "f"])
        .set_columns(["1", "2", "3"])
        .remove_columns()
        .clone()
        .build(),
    "+---+---+---+"
    "| a | b | c |"
    "+---+---+---+"
    "| d | e | f |"
    "+---+---+---+"
);

test_table!(
    header_remove_1,
    Builder::default()
        .add_record(["a", "b", "c"])
        .add_record(["d", "e", "f"])
        .set_columns(["1", "2", "3", "4", "5"])
        .remove_columns()
        .clone()
        .build(),
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
    Builder::default().set_columns(["1", "2"]).add_record(["a", "b", "c"]).add_record(["d"]).clone().build(),
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
    Builder::default().set_columns(["1", "2", "3"]).add_record(["a", "b"]).add_record(["d"]).clone().build(),
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
    Builder::default().set_columns(["1"]).add_record(["a", "b"]).add_record(["d", "e", "f"]).clone().build(),
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
    Builder::default()
        .set_default_text("NaN")
        .set_columns(["1", "2"])
        .add_record(["a", "b", "c"])
        .add_record(["d"])
        .clone()
        .build(),
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
    Builder::default()
        .set_default_text("NaN")
        .set_columns(["1"])
        .add_record(["a", "b"])
        .add_record(["d", "e", "f"])
        .clone()
        .build(),
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
    Builder::from(vec![
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
    Builder::from(vec![
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
    Builder::from(vec![
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
    Builder::from(vec![
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
    Builder::from(vec![
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
    Builder::from_iter([["1", "2", "3"], ["a", "b", "c"], ["d", "e", "f"]]).clean().clone().build(),
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
    Builder::from_iter([["", "2", "3"], ["", "b", "c"], ["", "e", "f"]]).clean().clone().build(),
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
    Builder::from_iter([["1", "", "3"], ["a", "", "c"], ["d", "", "f"]]).clean().clone().build(),
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
    Builder::from_iter([["1", "2", ""], ["a", "b", ""], ["d", "e", ""]]).clean().clone().build(),
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
    Builder::from_iter([["", "", "3"], ["", "", "c"], ["", "", "f"]]).clean().clone().build(),
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
    Builder::from_iter([["1", "", ""], ["a", "", ""], ["d", "", ""]]).clean().clone().build(),
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
    Builder::from_iter([["", "2", ""], ["", "b", ""], ["", "e", ""]]).clean().clone().build(),
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
    Builder::from_iter([["", "", ""], ["a", "b", "c"], ["d", "e", "f"]]).clean().clone().build(),
    "+---+---+---+"
    "| a | b | c |"
    "+---+---+---+"
    "| d | e | f |"
    "+---+---+---+"
);

test_table!(
    clean_8,
    Builder::from_iter([["1", "2", "3"], ["", "", ""], ["d", "e", "f"]]).clean().clone().build(),
    "+---+---+---+"
    "| 1 | 2 | 3 |"
    "+---+---+---+"
    "| d | e | f |"
    "+---+---+---+"
);

test_table!(
    clean_9,
    Builder::from_iter([["1", "2", "3"], ["a", "b", "c"], ["", "", ""]]).clean().clone().build(),
    "+---+---+---+"
    "| 1 | 2 | 3 |"
    "+---+---+---+"
    "| a | b | c |"
    "+---+---+---+"
);

test_table!(
    clean_10,
    Builder::from_iter([["", "", ""], ["", "", ""], ["d", "e", "f"]]).clean().clone().build(),
    "+---+---+---+"
    "| d | e | f |"
    "+---+---+---+"
);

test_table!(
    clean_11,
    Builder::from_iter([["1", "2", "3"], ["", "", ""], ["", "", ""]]).clean().clone().build(),
    "+---+---+---+"
    "| 1 | 2 | 3 |"
    "+---+---+---+"
);

test_table!(
    clean_12,
    Builder::from_iter([["", "", ""], ["a", "b", "c"], ["", "", ""]]).clean().clone().build(),
    "+---+---+---+"
    "| a | b | c |"
    "+---+---+---+"
);

test_table!(
    clean_13,
    Builder::from_iter([["1", "", "3"], ["", "", ""], ["d", "", "f"]]).clean().clone().build(),
    "+---+---+"
    "| 1 | 3 |"
    "+---+---+"
    "| d | f |"
    "+---+---+"
);

test_table!(
    clean_with_columns_0,
    Builder::from_iter([["1", "2", "3"], ["a", "b", "c"], ["d", "e", "f"]]).set_columns(["col1", "col2", "col3"]).clean().clone().build(),
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
    Builder::from_iter([["", "2", "3"], ["", "b", "c"], ["", "e", "f"]]).set_columns(["col1", "col2", "col3"]).clean().clone().build(),
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
    Builder::from_iter([["1", "", "3"], ["a", "", "c"], ["d", "", "f"]]).set_columns(["col1", "col2", "col3"]).clean().clone().build(),
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
    Builder::from_iter([["1", "2", ""], ["a", "b", ""], ["d", "e", ""]]).set_columns(["col1", "col2", "col3"]).clean().clone().build(),
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
    Builder::from_iter([["", "", "3"], ["", "", "c"], ["", "", "f"]]).set_columns(["col1", "col2", "col3"]).clean().clone().build(),
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
    Builder::from_iter([["1", "", ""], ["a", "", ""], ["d", "", ""]]).set_columns(["col1", "col2", "col3"]).clean().clone().build(),
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
    Builder::from_iter([["", "2", ""], ["", "b", ""], ["", "e", ""]]).set_columns(["col1", "col2", "col3"]).clean().clone().build(),
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
    Builder::from_iter([["", "", ""], ["", "", ""], ["", "", ""]])
        .set_columns(["col1", "col2", "col3"])
        .clean()
        .clone()
        .build(),
    ""
);

test_table!(
    clean_with_columns_8,
    Builder::from_iter([["", "", ""], ["a", "b", "c"], ["d", "e", "f"]]).set_columns(["col1", "col2", "col3"]).clean().clone().build(),
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
    Builder::from_iter([["1", "2", "3"], ["", "", ""], ["d", "e", "f"]]).set_columns(["col1", "col2", "col3"]).clean().clone().build(),
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
    Builder::from_iter([["1", "2", "3"], ["a", "b", "c"], ["", "", ""]]).set_columns(["col1", "col2", "col3"]).clean().clone().build(),
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
    Builder::from_iter([["", "", ""], ["", "", ""], ["d", "e", "f"]]).set_columns(["col1", "col2", "col3"]).clean().clone().build(),
    "+------+------+------+"
    "| col1 | col2 | col3 |"
    "+------+------+------+"
    "| d    | e    | f    |"
    "+------+------+------+"
);

test_table!(
    clean_with_columns_12,
    Builder::from_iter([["1", "2", "3"], ["", "", ""], ["", "", ""]]).set_columns(["col1", "col2", "col3"]).clean().clone().build(),
    "+------+------+------+"
    "| col1 | col2 | col3 |"
    "+------+------+------+"
    "| 1    | 2    | 3    |"
    "+------+------+------+"
);

test_table!(
    clean_with_columns_13,
    Builder::from_iter([["", "", ""], ["a", "b", "c"], ["", "", ""]]).set_columns(["col1", "col2", "col3"]).clean().clone().build(),
    "+------+------+------+"
    "| col1 | col2 | col3 |"
    "+------+------+------+"
    "| a    | b    | c    |"
    "+------+------+------+"
);

test_table!(
    clean_with_columns_14,
    Builder::from_iter([["1", "", "3"], ["", "", ""], ["d", "", "f"]]).set_columns(["col1", "col2", "col3"]).clean().clone().build(),
    "+------+------+"
    "| col1 | col3 |"
    "+------+------+"
    "| 1    | 3    |"
    "+------+------+"
    "| d    | f    |"
    "+------+------+"
);

test_table!(
    clean_empty_0,
    Builder::from_iter([[""; 0]; 0]).clean().clone().build(),
    ""
);

test_table!(
    clean_empty_1,
    Builder::from_iter([[""; 0]; 10]).clean().clone().build(),
    ""
);

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
        .set_name(Some("A index name".to_owned()))
        .clone()
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
        .hide_index()
        .clone()
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
        .set_index(1)
        .clone()
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
        .set_index(1)
        .set_name(Some("Hello".to_owned()))
        .clone()
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
        .set_index(1)
        .set_name(None)
        .clone()
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
        .clone()
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

#[quickcheck_macros::quickcheck]
#[ignore = "Quickcheck tests are a bit slow, so we don't run them all the time"]
fn qc_table_is_consistent(data: Vec<Vec<isize>>) -> bool {
    let mut builder = Builder::default();
    for row in data {
        let row = row.into_iter().map(|i| i.to_string()).collect::<Vec<_>>();
        builder.add_record(row);
    }

    let table = builder.build().to_string();

    let lines = table.lines().collect::<Vec<_>>();
    let lines_has_the_same_length = lines
        .iter()
        .map(|line| papergrid::util::string_width(line))
        .all(|line_width| line_width == lines[0].len());
    lines_has_the_same_length
}
