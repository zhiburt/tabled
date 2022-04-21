use crate::util::create_vector;
use tabled::{
    object::{Full, Rows},
    Alignment, Modify, Padding, Style, Table,
};

mod util;

#[test]
fn padding() {
    let data = create_vector::<3, 3>();
    let table = Table::new(&data)
        .with(Style::psql())
        .with(Modify::new(Full).with(Alignment::left()))
        .with(Modify::new(Rows::new(1..)).with(Padding::new(1, 1, 0, 2)))
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
fn padding_with_set_characters() {
    let data = create_vector::<3, 3>();
    let table = Table::new(&data)
        .with(Style::psql())
        .with(Modify::new(Full).with(Padding::new(1, 2, 1, 1).set_fill('>', '<', 'V', '^')))
        .to_string();

    let expected = concat!(
        "VVVV|VVVVVVVVVVV|VVVVVVVVVVV|VVVVVVVVVVV\n",
        ">N<<|>column 0<<|>column 1<<|>column 2<<\n",
        "^^^^|^^^^^^^^^^^|^^^^^^^^^^^|^^^^^^^^^^^\n",
        "----+-----------+-----------+-----------\n",
        "VVVV|VVVVVVVVVVV|VVVVVVVVVVV|VVVVVVVVVVV\n",
        ">0<<|>  0-0   <<|>  0-1   <<|>  0-2   <<\n",
        "^^^^|^^^^^^^^^^^|^^^^^^^^^^^|^^^^^^^^^^^\n",
        "VVVV|VVVVVVVVVVV|VVVVVVVVVVV|VVVVVVVVVVV\n",
        ">1<<|>  1-0   <<|>  1-1   <<|>  1-2   <<\n",
        "^^^^|^^^^^^^^^^^|^^^^^^^^^^^|^^^^^^^^^^^\n",
        "VVVV|VVVVVVVVVVV|VVVVVVVVVVV|VVVVVVVVVVV\n",
        ">2<<|>  2-0   <<|>  2-1   <<|>  2-2   <<\n",
        "^^^^|^^^^^^^^^^^|^^^^^^^^^^^|^^^^^^^^^^^\n",
    );

    assert_eq!(table, expected);
}

#[test]
fn padding_with_set_characters_and_zero_ident() {
    let data = create_vector::<3, 3>();
    let table = Table::new(&data)
        .with(Style::psql())
        .with(Modify::new(Full).with(Padding::zero().set_fill('>', '<', '^', 'V')))
        .to_string();

    let expected = concat!(
        "N|column 0|column 1|column 2\n",
        "-+--------+--------+--------\n",
        "0|  0-0   |  0-1   |  0-2   \n",
        "1|  1-0   |  1-1   |  1-2   \n",
        "2|  2-0   |  2-1   |  2-2   \n",
    );

    assert_eq!(table, expected);
}

#[test]
fn padding_multiline() {
    let data = create_vector::<3, 3>();
    let table = Table::new(&data)
        .with(Style::psql())
        .with(Modify::new(Rows::new(1..)).with(Padding::new(1, 1, 1, 1)))
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
fn padding_multiline_with_vertical_alignment() {
    let data = create_vector::<3, 3>();
    let table = Table::new(&data)
        .with(Style::psql())
        .with(
            Modify::new(Full)
                .with(Alignment::center())
                .with(Alignment::center_vertical()),
        )
        .with(Modify::new(Rows::new(1..)).with(Padding::new(1, 1, 1, 1)))
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
