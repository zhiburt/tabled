use crate::util::create_vector;
use tabled::{Alignment, Full, Indent, Modify, Row, Style, Table};

mod util;

#[test]
fn indent() {
    let data = create_vector::<3, 3>();
    let table = Table::new(&data)
        .with(Style::PSQL)
        .with(Modify::new(Full).with(Alignment::left()))
        .with(Modify::new(Row(1..)).with(Indent::new(1, 1, 0, 2)))
        .to_string();

    let expected = concat!(
        " N | column 0 | column 1 | column 2 \n",
        "---+----------+----------+----------\n",
        " 0 | 0-0      | 0-1      | 0-2      \n",
        "   |          |          |          \n",
        "   |          |          |          \n",
        " 1 | 1-0      | 1-1      | 1-2      \n",
        "   |          |          |          \n",
        "   |          |          |          \n",
        " 2 | 2-0      | 2-1      | 2-2      \n",
        "   |          |          |          \n",
        "   |          |          |          \n",
    );

    assert_eq!(table, expected);
}

#[test]
fn indent_multiline() {
    let data = create_vector::<3, 3>();
    let table = Table::new(&data)
        .with(Style::PSQL)
        .with(Modify::new(Row(1..)).with(Indent::new(1, 1, 1, 1)))
        .to_string();

    let expected = concat!(
        " N | column 0 | column 1 | column 2 \n",
        "---+----------+----------+----------\n",
        "   |          |          |          \n",
        " 0 |   0-0    |   0-1    |   0-2    \n",
        "   |          |          |          \n",
        "   |          |          |          \n",
        " 1 |   1-0    |   1-1    |   1-2    \n",
        "   |          |          |          \n",
        "   |          |          |          \n",
        " 2 |   2-0    |   2-1    |   2-2    \n",
        "   |          |          |          \n",
    );

    assert_eq!(table, expected);
}

#[test]
fn indent_multiline_with_vertical_alignment() {
    let data = create_vector::<3, 3>();
    let table = Table::new(&data)
        .with(Style::PSQL)
        .with(
            Modify::new(Full)
                .with(Alignment::center_horizontal())
                .with(Alignment::center_vertical()),
        )
        .with(Modify::new(Row(1..)).with(Indent::new(1, 1, 1, 1)))
        .to_string();

    let expected = concat!(
        " N | column 0 | column 1 | column 2 \n",
        "---+----------+----------+----------\n",
        "   |          |          |          \n",
        " 0 |   0-0    |   0-1    |   0-2    \n",
        "   |          |          |          \n",
        "   |          |          |          \n",
        " 1 |   1-0    |   1-1    |   1-2    \n",
        "   |          |          |          \n",
        "   |          |          |          \n",
        " 2 |   2-0    |   2-1    |   2-2    \n",
        "   |          |          |          \n",
    );

    assert_eq!(table, expected);
}
