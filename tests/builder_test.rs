use std::iter::FromIterator;

use tabled::builder::Builder;

#[test]
fn builder_add_row() {
    let builder = Builder::default()
        .add_row(["1", "2", "3"])
        .add_row(["a", "b", "c"])
        .add_row(["d", "e", "f"]);
    let table = builder.build().to_string();
    let expected = "+---+---+---+\n\
                         | 1 | 2 | 3 |\n\
                         +---+---+---+\n\
                         | a | b | c |\n\
                         +---+---+---+\n\
                         | d | e | f |\n\
                         +---+---+---+\n";

    assert_eq!(table, expected);
}

#[test]
fn builder_add_row_can_has_different_types() {
    let builder = Builder::default()
        .add_row([1, 2, 3])
        .add_row(["a", "b", "c"])
        .add_row(['d', 'e', 'f']);
    let table = builder.build().to_string();
    let expected = "+---+---+---+\n\
                         | 1 | 2 | 3 |\n\
                         +---+---+---+\n\
                         | a | b | c |\n\
                         +---+---+---+\n\
                         | d | e | f |\n\
                         +---+---+---+\n";

    assert_eq!(table, expected);
}

#[test]
fn builder_header() {
    let builder = Builder::default()
        .add_row(["a", "b", "c"])
        .add_row(["d", "e", "f"])
        .set_header(["1", "2", "3"]);
    let table = builder.build().to_string();
    let expected = "+---+---+---+\n\
                         | 1 | 2 | 3 |\n\
                         +---+---+---+\n\
                         | a | b | c |\n\
                         +---+---+---+\n\
                         | d | e | f |\n\
                         +---+---+---+\n";

    assert_eq!(table, expected);
}

#[test]
fn builder_from_iter() {
    let builder = Builder::from_iter([["n", "name"], ["0", "Dmitriy"], ["1", "Vladislav"]]);
    let table = builder.build().to_string();
    let expected = "+---+-----------+\n\
                         | n |   name    |\n\
                         +---+-----------+\n\
                         | 0 |  Dmitriy  |\n\
                         +---+-----------+\n\
                         | 1 | Vladislav |\n\
                         +---+-----------+\n";

    assert_eq!(table, expected);
}

#[test]
fn builder_used_with_different_number_of_columns() {
    let builder = Builder::default()
        .set_header(["1", "2"])
        .add_row(["a", "b", "c"])
        .add_row(["d"]);
    let table = builder.build().to_string();
    let expected = "+---+---+---+\n\
                         | 1 | 2 |   |\n\
                         +---+---+---+\n\
                         | a | b | c |\n\
                         +---+---+---+\n\
                         | d |   |   |\n\
                         +---+---+---+\n";
    assert_eq!(table, expected);

    let builder = Builder::default()
        .set_header(["1", "2", "3"])
        .add_row(["a", "b"])
        .add_row(["d"]);
    let table = builder.build().to_string();
    let expected = "+---+---+---+\n\
                         | 1 | 2 | 3 |\n\
                         +---+---+---+\n\
                         | a | b |   |\n\
                         +---+---+---+\n\
                         | d |   |   |\n\
                         +---+---+---+\n";
    assert_eq!(table, expected);

    let builder = Builder::default()
        .set_header(["1"])
        .add_row(["a", "b"])
        .add_row(["d", "e", "f"]);
    let table = builder.build().to_string();
    let expected = "+---+---+---+\n\
                         | 1 |   |   |\n\
                         +---+---+---+\n\
                         | a | b |   |\n\
                         +---+---+---+\n\
                         | d | e | f |\n\
                         +---+---+---+\n";
    assert_eq!(table, expected);
}

#[test]
fn builder_with_default_cell() {
    let builder = Builder::default()
        .set_default_text("NaN")
        .set_header(["1", "2"])
        .add_row(["a", "b", "c"])
        .add_row(["d"]);
    let table = builder.build().to_string();
    let expected = "+---+-----+-----+\n\
                         | 1 |  2  | NaN |\n\
                         +---+-----+-----+\n\
                         | a |  b  |  c  |\n\
                         +---+-----+-----+\n\
                         | d | NaN | NaN |\n\
                         +---+-----+-----+\n";
    assert_eq!(table, expected);

    let builder = Builder::default()
        .set_default_text("NaN")
        .set_header(["1", "2", "3"])
        .add_row(["a", "b"])
        .add_row(["d"]);
    let table = builder.build().to_string();
    let expected = "+---+-----+-----+\n\
                         | 1 |  2  |  3  |\n\
                         +---+-----+-----+\n\
                         | a |  b  | NaN |\n\
                         +---+-----+-----+\n\
                         | d | NaN | NaN |\n\
                         +---+-----+-----+\n";
    assert_eq!(table, expected);

    let builder = Builder::default()
        .set_default_text("NaN")
        .set_header(["1"])
        .add_row(["a", "b"])
        .add_row(["d", "e", "f"]);
    let table = builder.build().to_string();
    let expected = "+---+-----+-----+\n\
                         | 1 | NaN | NaN |\n\
                         +---+-----+-----+\n\
                         | a |  b  | NaN |\n\
                         +---+-----+-----+\n\
                         | d |  e  |  f  |\n\
                         +---+-----+-----+\n";
    assert_eq!(table, expected);
}

#[test]
fn builder_extend() {
    let mut builder = Builder::default();
    builder.extend([1, 2, 3]);
    builder.extend(['a', 'b', 'c']);
    builder.extend(["d", "e", "f"]);

    let table = builder.build().to_string();

    let expected = "+---+---+---+\n\
                         | 1 | 2 | 3 |\n\
                         +---+---+---+\n\
                         | a | b | c |\n\
                         +---+---+---+\n\
                         | d | e | f |\n\
                         +---+---+---+\n";

    assert_eq!(table, expected);
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

    let expected = "+---+---+---+\n\
                         | 1 | 2 | 3 |\n\
                         +---+---+---+\n\
                         | a | b | c |\n\
                         +---+---+---+\n\
                         | d | e | f |\n\
                         +---+---+---+\n";

    assert_eq!(table, expected);
}

#[quickcheck_macros::quickcheck]
#[ignore = "Quickcheck tests are a bit slow, so we don't run them all the time"]
fn qc_table_is_consistent(data: Vec<Vec<isize>>) -> bool {
    let mut builder = Builder::default();
    for row in data {
        builder = builder.add_row(row);
    }

    let table = builder.build().to_string();

    let lines = table.lines().collect::<Vec<_>>();
    let lines_has_the_same_length = lines
        .iter()
        .map(|line| papergrid::string_width(line))
        .all(|line_width| line_width == lines[0].len());
    lines_has_the_same_length
}
