#![cfg(feature = "std")]

use std::iter::FromIterator;

use quickcheck::Arbitrary;

use tabled::grid::util::string::{string_width, string_width_multiline};
use tabled::{builder::Builder, settings::style::Style, Table};

use util::test_table;

mod util;

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
        let mut b = Builder::default().set_header(["1", "2", "3"]);
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
        let mut b = Builder::default().set_header(["1", "2", "3"]);
        b.push_record(["a", "b", "c"]);
        b.push_record(["d", "e", "f"]);
        let b = b.remove_header();

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
        let mut b = Builder::default().set_header(["1", "2", "3", "4", "5"]);
        b.push_record(["a", "b", "c"]);
        b.push_record(["d", "e", "f"]);
        let b = b.remove_header();

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
        let mut b = Builder::default().set_header(["1", "2"]);
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
        let mut b = Builder::default().set_header(["1", "2", "3"]);
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
        let mut b = Builder::default().set_header(["1"]);
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
        let mut b = Builder::default().set_header(["1", "2"]).set_default_text("NaN");
        b.push_record(["a", "b", "c"]);
        b.push_record(["d"]);

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
        let mut b = Builder::default().set_header(["1"]).set_default_text("NaN");
        b.push_record(["a", "b"]);
        b.push_record(["d", "e", "f"]);

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
    clean(Builder::from_iter([["1", "2", "3"], ["a", "b", "c"], ["d", "e", "f"]]).set_header(["col1", "col2", "col3"])),
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
    clean(Builder::from_iter([["", "2", "3"], ["", "b", "c"], ["", "e", "f"]]).set_header(["col1", "col2", "col3"])),
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
    clean(Builder::from_iter([["1", "", "3"], ["a", "", "c"], ["d", "", "f"]]).set_header(["col1", "col2", "col3"])),
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
    clean(Builder::from_iter([["1", "2", ""], ["a", "b", ""], ["d", "e", ""]]).set_header(["col1", "col2", "col3"])),
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
    clean(Builder::from_iter([["", "", "3"], ["", "", "c"], ["", "", "f"]]).set_header(["col1", "col2", "col3"])),
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
    clean(Builder::from_iter([["1", "", ""], ["a", "", ""], ["d", "", ""]]).set_header(["col1", "col2", "col3"])),
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
    clean(Builder::from_iter([["", "2", ""], ["", "b", ""], ["", "e", ""]]).set_header(["col1", "col2", "col3"])),
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
    ),
    ""
);

test_table!(
    clean_with_columns_8,
    clean(Builder::from_iter([["", "", ""], ["a", "b", "c"], ["d", "e", "f"]]).set_header(["col1", "col2", "col3"])),
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
    clean(Builder::from_iter([["1", "2", "3"], ["", "", ""], ["d", "e", "f"]]).set_header(["col1", "col2", "col3"])),
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
    clean(Builder::from_iter([["1", "2", "3"], ["a", "b", "c"], ["", "", ""]]).set_header(["col1", "col2", "col3"])),
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
    clean(Builder::from_iter([["", "", ""], ["", "", ""], ["d", "e", "f"]]).set_header(["col1", "col2", "col3"])),
    "+------+------+------+"
    "| col1 | col2 | col3 |"
    "+------+------+------+"
    "| d    | e    | f    |"
    "+------+------+------+"
);

test_table!(
    clean_with_columns_12,
    clean(Builder::from_iter([["1", "2", "3"], ["", "", ""], ["", "", ""]]).set_header(["col1", "col2", "col3"])),
    "+------+------+------+"
    "| col1 | col2 | col3 |"
    "+------+------+------+"
    "| 1    | 2    | 3    |"
    "+------+------+------+"
);

test_table!(
    clean_with_columns_13,
    clean(Builder::from_iter([["", "", ""], ["a", "b", "c"], ["", "", ""]]).set_header(["col1", "col2", "col3"])),
    "+------+------+------+"
    "| col1 | col2 | col3 |"
    "+------+------+------+"
    "| a    | b    | c    |"
    "+------+------+------+"
);

test_table!(
    clean_with_columns_14,
    clean(Builder::from_iter([["1", "", "3"], ["", "", ""], ["d", "", "f"]]).set_header(["col1", "col2", "col3"])),
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

#[quickcheck_macros::quickcheck]
#[ignore = "Quickcheck tests are a bit slow, so we don't run them all the time"]
fn qc_table_is_consistent(data: Vec<Vec<isize>>) -> bool {
    let mut builder = Builder::default();
    for row in data {
        let row = row.into_iter().map(|i| i.to_string()).collect::<Vec<_>>();
        builder.push_record(row);
    }

    let table = builder.build().to_string();

    let lines = table.lines().collect::<Vec<_>>();
    let lines_has_the_same_length = lines
        .iter()
        .map(|line| papergrid::util::string::string_width(line))
        .all(|line_width| line_width == lines[0].len());
    lines_has_the_same_length
}

fn clean(mut b: Builder) -> String {
    b.clean();
    b.build().to_string()
}
#[derive(Clone, Debug)]
struct TableStructure {
    pub rows: Vec<Line>,
    pub theme: ThemeFixture,
}

type Line = Vec<String>;

#[derive(Clone, Debug)]
pub enum ThemeFixture {
    Empty,
    Blank,
    Ascii,
    Psql,
    Markdown,
    Modern,
    Sharp,
    Rounded,
    Extended,
    Dots,
    RestructuredText,
    AsciiRounded,
}
impl Arbitrary for TableStructure {
    fn arbitrary(g: &mut quickcheck::Gen) -> Self {
        Self {
            rows: Arbitrary::arbitrary(g),
            theme: ThemeFixture::arbitrary(g),
        }
    }
}

impl Arbitrary for ThemeFixture {
    fn arbitrary(g: &mut quickcheck::Gen) -> Self {
        use ThemeFixture::*;
        g.choose(&[
            Empty,
            Blank,
            Ascii,
            Psql,
            Markdown,
            Modern,
            Sharp,
            Rounded,
            Extended,
            Dots,
            RestructuredText,
            AsciiRounded,
        ])
        .unwrap()
        .to_owned()
    }
}

#[quickcheck_macros::quickcheck]
#[ignore = "Quickcheck tests are a bit slow, so we don't run them all the time"]
fn qc_table_is_consistent_with_borders(table_structure: TableStructure) {
    let rows = table_structure.rows;
    let theme = table_structure.theme;

    let builder = Builder::from_iter(rows);

    let mut table = builder.build();
    set_theme(&mut table, theme);
    let output = table.to_string();

    if let Some(line) = output.lines().next() {
        assert_eq!(string_width(line), string_width_multiline(&output));
    }
}

fn set_theme(table: &mut Table, theme: ThemeFixture) {
    use ThemeFixture::*;
    match theme {
        Empty => {
            table.with(Style::empty());
        }
        Blank => {
            table.with(Style::blank());
        }
        Ascii => {
            table.with(Style::ascii());
        }
        Psql => {
            table.with(Style::psql());
        }
        Markdown => {
            table.with(Style::markdown());
        }
        Modern => {
            table.with(Style::modern());
        }
        Sharp => {
            table.with(Style::sharp());
        }
        Rounded => {
            table.with(Style::rounded());
        }
        Extended => {
            table.with(Style::extended());
        }
        Dots => {
            table.with(Style::dots());
        }
        RestructuredText => {
            table.with(Style::re_structured_text());
        }
        AsciiRounded => {
            table.with(Style::ascii_rounded());
        }
    }
}
