use std::iter::FromIterator;

use tabled::builder::Builder;

#[test]
fn builder_add_record() {
    let builder = Builder::default()
        .add_record(["1", "2", "3"])
        .add_record(["a", "b", "c"])
        .add_record(["d", "e", "f"]);
    let table = builder.build().to_string();

    assert_eq!(
        table,
        "+---+---+---+\n\
         | 1 | 2 | 3 |\n\
         +---+---+---+\n\
         | a | b | c |\n\
         +---+---+---+\n\
         | d | e | f |\n\
         +---+---+---+\n"
    );
}

#[test]
fn builder_add_record_can_has_different_types() {
    let builder = Builder::default()
        .add_record([1, 2, 3])
        .add_record(["a", "b", "c"])
        .add_record(['d', 'e', 'f']);
    let table = builder.build().to_string();

    assert_eq!(
        table,
        "+---+---+---+\n\
         | 1 | 2 | 3 |\n\
         +---+---+---+\n\
         | a | b | c |\n\
         +---+---+---+\n\
         | d | e | f |\n\
         +---+---+---+\n"
    );
}

#[test]
fn builder_header() {
    let builder = Builder::default()
        .add_record(["a", "b", "c"])
        .add_record(["d", "e", "f"])
        .set_columns(["1", "2", "3"]);
    let table = builder.build().to_string();

    assert_eq!(
        table,
        "+---+---+---+\n\
         | 1 | 2 | 3 |\n\
         +---+---+---+\n\
         | a | b | c |\n\
         +---+---+---+\n\
         | d | e | f |\n\
         +---+---+---+\n"
    );
}

#[test]
fn builder_from_iter() {
    let builder = Builder::from_iter([["n", "name"], ["0", "Dmitriy"], ["1", "Vladislav"]]);
    let table = builder.build().to_string();

    assert_eq!(
        table,
        "+---+-----------+\n\
         | n |   name    |\n\
         +---+-----------+\n\
         | 0 |  Dmitriy  |\n\
         +---+-----------+\n\
         | 1 | Vladislav |\n\
         +---+-----------+\n"
    );
}

#[test]
fn builder_used_with_different_number_of_columns() {
    let builder = Builder::default()
        .set_columns(["1", "2"])
        .add_record(["a", "b", "c"])
        .add_record(["d"]);
    let table = builder.build().to_string();

    assert_eq!(
        table,
        "+---+---+---+\n\
         | 1 | 2 |   |\n\
         +---+---+---+\n\
         | a | b | c |\n\
         +---+---+---+\n\
         | d |   |   |\n\
         +---+---+---+\n"
    );

    let builder = Builder::default()
        .set_columns(["1", "2", "3"])
        .add_record(["a", "b"])
        .add_record(["d"]);
    let table = builder.build().to_string();

    assert_eq!(
        table,
        "+---+---+---+\n\
         | 1 | 2 | 3 |\n\
         +---+---+---+\n\
         | a | b |   |\n\
         +---+---+---+\n\
         | d |   |   |\n\
         +---+---+---+\n"
    );

    let builder = Builder::default()
        .set_columns(["1"])
        .add_record(["a", "b"])
        .add_record(["d", "e", "f"]);
    let table = builder.build().to_string();

    assert_eq!(
        table,
        "+---+---+---+\n\
         | 1 |   |   |\n\
         +---+---+---+\n\
         | a | b |   |\n\
         +---+---+---+\n\
         | d | e | f |\n\
         +---+---+---+\n"
    );
}

#[test]
fn builder_with_default_cell() {
    let builder = Builder::default()
        .set_default_text("NaN")
        .set_columns(["1", "2"])
        .add_record(["a", "b", "c"])
        .add_record(["d"]);
    let table = builder.build().to_string();

    assert_eq!(
        table,
        "+---+-----+-----+\n\
         | 1 |  2  | NaN |\n\
         +---+-----+-----+\n\
         | a |  b  |  c  |\n\
         +---+-----+-----+\n\
         | d | NaN | NaN |\n\
         +---+-----+-----+\n"
    );

    let builder = Builder::default()
        .set_default_text("NaN")
        .set_columns(["1", "2", "3"])
        .add_record(["a", "b"])
        .add_record(["d"]);
    let table = builder.build().to_string();

    assert_eq!(
        table,
        "+---+-----+-----+\n\
         | 1 |  2  |  3  |\n\
         +---+-----+-----+\n\
         | a |  b  | NaN |\n\
         +---+-----+-----+\n\
         | d | NaN | NaN |\n\
         +---+-----+-----+\n"
    );

    let builder = Builder::default()
        .set_default_text("NaN")
        .set_columns(["1"])
        .add_record(["a", "b"])
        .add_record(["d", "e", "f"]);
    let table = builder.build().to_string();

    assert_eq!(
        table,
        "+---+-----+-----+\n\
         | 1 | NaN | NaN |\n\
         +---+-----+-----+\n\
         | a |  b  | NaN |\n\
         +---+-----+-----+\n\
         | d |  e  |  f  |\n\
         +---+-----+-----+\n"
    );
}

#[test]
fn builder_extend() {
    let mut builder = Builder::default();
    builder.extend([1, 2, 3]);
    builder.extend(['a', 'b', 'c']);
    builder.extend(["d", "e", "f"]);

    let table = builder.build().to_string();

    assert_eq!(
        table,
        "+---+---+---+\n\
         | 1 | 2 | 3 |\n\
         +---+---+---+\n\
         | a | b | c |\n\
         +---+---+---+\n\
         | d | e | f |\n\
         +---+---+---+\n"
    );
}

#[test]
fn builder_from_vector() {
    let data = vec![
        vec!["1".to_string(), "2".to_string(), "3".to_string()],
        vec!["a".to_string(), "b".to_string(), "c".to_string()],
        vec!["d".to_string(), "e".to_string(), "f".to_string()],
    ];

    let builder = Builder::from(data);
    let table = builder.build().to_string();

    assert_eq!(
        table,
        "+---+---+---+\n\
         | 1 | 2 | 3 |\n\
         +---+---+---+\n\
         | a | b | c |\n\
         +---+---+---+\n\
         | d | e | f |\n\
         +---+---+---+\n"
    );
}

#[test]
fn builder_from_with_empty_lines() {
    let data = vec![
        vec!["1".to_string(), "2".to_string(), "3".to_string()],
        vec![],
        vec![],
        vec!["d".to_string(), "e".to_string(), "f".to_string()],
    ];

    let builder = Builder::from(data);
    let table = builder.build().to_string();

    assert_eq!(
        table,
        "+---+---+---+\n\
         | 1 | 2 | 3 |\n\
         +---+---+---+\n\
         |   |   |   |\n\
         +---+---+---+\n\
         |   |   |   |\n\
         +---+---+---+\n\
         | d | e | f |\n\
         +---+---+---+\n"
    );

    let data = vec![
        vec!["1".to_string(), "2".to_string(), "3".to_string()],
        vec![],
        vec!["d".to_string(), "e".to_string(), "f".to_string()],
    ];

    let builder = Builder::from(data);
    let table = builder.build().to_string();

    assert_eq!(
        table,
        "+---+---+---+\n\
         | 1 | 2 | 3 |\n\
         +---+---+---+\n\
         |   |   |   |\n\
         +---+---+---+\n\
         | d | e | f |\n\
         +---+---+---+\n"
    );

    let data = vec![
        vec![],
        vec!["1".to_string(), "2".to_string(), "3".to_string()],
        vec!["d".to_string(), "e".to_string(), "f".to_string()],
    ];

    let builder = Builder::from(data);
    let table = builder.build().to_string();

    assert_eq!(
        table,
        "+---+---+---+\n\
         |   |   |   |\n\
         +---+---+---+\n\
         | 1 | 2 | 3 |\n\
         +---+---+---+\n\
         | d | e | f |\n\
         +---+---+---+\n"
    );

    let data = vec![
        vec!["1".to_string(), "2".to_string(), "3".to_string()],
        vec!["d".to_string(), "e".to_string(), "f".to_string()],
        vec![],
    ];

    let builder = Builder::from(data);
    let table = builder.build().to_string();

    assert_eq!(
        table,
        "+---+---+---+\n\
         | 1 | 2 | 3 |\n\
         +---+---+---+\n\
         | d | e | f |\n\
         +---+---+---+\n\
         |   |   |   |\n\
         +---+---+---+\n"
    );
}

#[test]
fn builder_clean() {
    let tests = [
        (
            Builder::from_iter([["1", "2", "3"], ["a", "b", "c"], ["d", "e", "f"]]),
            "+---+---+---+\n\
             | 1 | 2 | 3 |\n\
             +---+---+---+\n\
             | a | b | c |\n\
             +---+---+---+\n\
             | d | e | f |\n\
             +---+---+---+\n",
        ),
        (
            Builder::from_iter([["", "2", "3"], ["", "b", "c"], ["", "e", "f"]]),
            "+---+---+\n\
             | 2 | 3 |\n\
             +---+---+\n\
             | b | c |\n\
             +---+---+\n\
             | e | f |\n\
             +---+---+\n",
        ),
        (
            Builder::from_iter([["1", "", "3"], ["a", "", "c"], ["d", "", "f"]]),
            "+---+---+\n\
             | 1 | 3 |\n\
             +---+---+\n\
             | a | c |\n\
             +---+---+\n\
             | d | f |\n\
             +---+---+\n",
        ),
        (
            Builder::from_iter([["1", "2", ""], ["a", "b", ""], ["d", "e", ""]]),
            "+---+---+\n\
             | 1 | 2 |\n\
             +---+---+\n\
             | a | b |\n\
             +---+---+\n\
             | d | e |\n\
             +---+---+\n",
        ),
        (
            Builder::from_iter([["", "", "3"], ["", "", "c"], ["", "", "f"]]),
            "+---+\n\
             | 3 |\n\
             +---+\n\
             | c |\n\
             +---+\n\
             | f |\n\
             +---+\n",
        ),
        (
            Builder::from_iter([["1", "", ""], ["a", "", ""], ["d", "", ""]]),
            "+---+\n\
             | 1 |\n\
             +---+\n\
             | a |\n\
             +---+\n\
             | d |\n\
             +---+\n",
        ),
        (
            Builder::from_iter([["", "2", ""], ["", "b", ""], ["", "e", ""]]),
            "+---+\n\
             | 2 |\n\
             +---+\n\
             | b |\n\
             +---+\n\
             | e |\n\
             +---+\n",
        ),
        (
            Builder::from_iter([["", "", ""], ["", "", ""], ["", "", ""]]),
            "",
        ),
        (
            Builder::from_iter([["", "", ""], ["a", "b", "c"], ["d", "e", "f"]]),
            "+---+---+---+\n\
             | a | b | c |\n\
             +---+---+---+\n\
             | d | e | f |\n\
             +---+---+---+\n",
        ),
        (
            Builder::from_iter([["1", "2", "3"], ["", "", ""], ["d", "e", "f"]]),
            "+---+---+---+\n\
             | 1 | 2 | 3 |\n\
             +---+---+---+\n\
             | d | e | f |\n\
             +---+---+---+\n",
        ),
        (
            Builder::from_iter([["1", "2", "3"], ["a", "b", "c"], ["", "", ""]]),
            "+---+---+---+\n\
             | 1 | 2 | 3 |\n\
             +---+---+---+\n\
             | a | b | c |\n\
             +---+---+---+\n",
        ),
        (
            Builder::from_iter([["", "", ""], ["", "", ""], ["d", "e", "f"]]),
            "+---+---+---+\n\
             | d | e | f |\n\
             +---+---+---+\n",
        ),
        (
            Builder::from_iter([["1", "2", "3"], ["", "", ""], ["", "", ""]]),
            "+---+---+---+\n\
             | 1 | 2 | 3 |\n\
             +---+---+---+\n",
        ),
        (
            Builder::from_iter([["", "", ""], ["a", "b", "c"], ["", "", ""]]),
            "+---+---+---+\n\
             | a | b | c |\n\
             +---+---+---+\n",
        ),
        (
            Builder::from_iter([["1", "", "3"], ["", "", ""], ["d", "", "f"]]),
            "+---+---+\n\
             | 1 | 3 |\n\
             +---+---+\n\
             | d | f |\n\
             +---+---+\n",
        ),
    ];

    for (i, (builder, expected)) in tests.iter().enumerate() {
        assert_eq!(
            builder.clone().clean().build().to_string(),
            *expected,
            "index={}",
            i
        );
    }
}

#[test]
fn builder_clean_with_columns() {
    let tests = [
        (
            Builder::from_iter([["1", "2", "3"], ["a", "b", "c"], ["d", "e", "f"]])
                .set_columns(["col1", "col2", "col3"]),
            "+------+------+------+\n\
             | col1 | col2 | col3 |\n\
             +------+------+------+\n\
             |  1   |  2   |  3   |\n\
             +------+------+------+\n\
             |  a   |  b   |  c   |\n\
             +------+------+------+\n\
             |  d   |  e   |  f   |\n\
             +------+------+------+\n",
        ),
        (
            Builder::from_iter([["", "2", "3"], ["", "b", "c"], ["", "e", "f"]])
                .set_columns(["col1", "col2", "col3"]),
            "+------+------+\n\
             | col2 | col3 |\n\
             +------+------+\n\
             |  2   |  3   |\n\
             +------+------+\n\
             |  b   |  c   |\n\
             +------+------+\n\
             |  e   |  f   |\n\
             +------+------+\n",
        ),
        (
            Builder::from_iter([["1", "", "3"], ["a", "", "c"], ["d", "", "f"]])
                .set_columns(["col1", "col2", "col3"]),
            "+------+------+\n\
             | col1 | col3 |\n\
             +------+------+\n\
             |  1   |  3   |\n\
             +------+------+\n\
             |  a   |  c   |\n\
             +------+------+\n\
             |  d   |  f   |\n\
             +------+------+\n",
        ),
        (
            Builder::from_iter([["1", "2", ""], ["a", "b", ""], ["d", "e", ""]])
                .set_columns(["col1", "col2", "col3"]),
            "+------+------+\n\
             | col1 | col2 |\n\
             +------+------+\n\
             |  1   |  2   |\n\
             +------+------+\n\
             |  a   |  b   |\n\
             +------+------+\n\
             |  d   |  e   |\n\
             +------+------+\n",
        ),
        (
            Builder::from_iter([["", "", "3"], ["", "", "c"], ["", "", "f"]])
                .set_columns(["col1", "col2", "col3"]),
            "+------+\n\
             | col3 |\n\
             +------+\n\
             |  3   |\n\
             +------+\n\
             |  c   |\n\
             +------+\n\
             |  f   |\n\
             +------+\n",
        ),
        (
            Builder::from_iter([["1", "", ""], ["a", "", ""], ["d", "", ""]])
                .set_columns(["col1", "col2", "col3"]),
            "+------+\n\
             | col1 |\n\
             +------+\n\
             |  1   |\n\
             +------+\n\
             |  a   |\n\
             +------+\n\
             |  d   |\n\
             +------+\n",
        ),
        (
            Builder::from_iter([["", "2", ""], ["", "b", ""], ["", "e", ""]])
                .set_columns(["col1", "col2", "col3"]),
            "+------+\n\
             | col2 |\n\
             +------+\n\
             |  2   |\n\
             +------+\n\
             |  b   |\n\
             +------+\n\
             |  e   |\n\
             +------+\n",
        ),
        (
            Builder::from_iter([["", "", ""], ["", "", ""], ["", "", ""]])
                .set_columns(["col1", "col2", "col3"]),
            "",
        ),
        (
            Builder::from_iter([["", "", ""], ["a", "b", "c"], ["d", "e", "f"]])
                .set_columns(["col1", "col2", "col3"]),
            "+------+------+------+\n\
             | col1 | col2 | col3 |\n\
             +------+------+------+\n\
             |  a   |  b   |  c   |\n\
             +------+------+------+\n\
             |  d   |  e   |  f   |\n\
             +------+------+------+\n",
        ),
        (
            Builder::from_iter([["1", "2", "3"], ["", "", ""], ["d", "e", "f"]])
                .set_columns(["col1", "col2", "col3"]),
            "+------+------+------+\n\
             | col1 | col2 | col3 |\n\
             +------+------+------+\n\
             |  1   |  2   |  3   |\n\
             +------+------+------+\n\
             |  d   |  e   |  f   |\n\
             +------+------+------+\n",
        ),
        (
            Builder::from_iter([["1", "2", "3"], ["a", "b", "c"], ["", "", ""]])
                .set_columns(["col1", "col2", "col3"]),
            "+------+------+------+\n\
             | col1 | col2 | col3 |\n\
             +------+------+------+\n\
             |  1   |  2   |  3   |\n\
             +------+------+------+\n\
             |  a   |  b   |  c   |\n\
             +------+------+------+\n",
        ),
        (
            Builder::from_iter([["", "", ""], ["", "", ""], ["d", "e", "f"]])
                .set_columns(["col1", "col2", "col3"]),
            "+------+------+------+\n\
             | col1 | col2 | col3 |\n\
             +------+------+------+\n\
             |  d   |  e   |  f   |\n\
             +------+------+------+\n",
        ),
        (
            Builder::from_iter([["1", "2", "3"], ["", "", ""], ["", "", ""]])
                .set_columns(["col1", "col2", "col3"]),
            "+------+------+------+\n\
             | col1 | col2 | col3 |\n\
             +------+------+------+\n\
             |  1   |  2   |  3   |\n\
             +------+------+------+\n",
        ),
        (
            Builder::from_iter([["", "", ""], ["a", "b", "c"], ["", "", ""]])
                .set_columns(["col1", "col2", "col3"]),
            "+------+------+------+\n\
             | col1 | col2 | col3 |\n\
             +------+------+------+\n\
             |  a   |  b   |  c   |\n\
             +------+------+------+\n",
        ),
        (
            Builder::from_iter([["1", "", "3"], ["", "", ""], ["d", "", "f"]])
                .set_columns(["col1", "col2", "col3"]),
            "+------+------+\n\
             | col1 | col3 |\n\
             +------+------+\n\
             |  1   |  3   |\n\
             +------+------+\n\
             |  d   |  f   |\n\
             +------+------+\n",
        ),
    ];

    for (i, (builder, expected)) in tests.iter().enumerate() {
        let table = builder.clone().clean().build().to_string();

        assert_eq!(table, *expected, "index={}", i);
    }
}

#[test]
fn builder_clean_empty() {
    let data: [[usize; 0]; 0] = [[]; 0];
    let table = Builder::from_iter(data).clean().build().to_string();
    assert_eq!(table, "");

    let data: [[usize; 0]; 10] = [[]; 10];
    let table = Builder::from_iter(data).clean().build().to_string();
    assert_eq!(table, "");
}

#[quickcheck_macros::quickcheck]
#[ignore = "Quickcheck tests are a bit slow, so we don't run them all the time"]
fn qc_table_is_consistent(data: Vec<Vec<isize>>) -> bool {
    let mut builder = Builder::default();
    for row in data {
        builder = builder.add_record(row);
    }

    let table = builder.build().to_string();

    let lines = table.lines().collect::<Vec<_>>();
    let lines_has_the_same_length = lines
        .iter()
        .map(|line| papergrid::string_width(line))
        .all(|line_width| line_width == lines[0].len());
    lines_has_the_same_length
}
