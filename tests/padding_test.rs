use tabled::{
    object::{Rows, Segment},
    Alignment, Modify, Padding, Style, Table,
};

use crate::util::{create_vector, static_table};

mod util;

#[test]
fn padding() {
    let data = create_vector::<3, 3>();
    let table = Table::new(&data)
        .with(Style::psql())
        .with(Modify::new(Segment::all()).with(Alignment::left()))
        .with(Modify::new(Rows::new(1..)).with(Padding::new(1, 1, 0, 2)))
        .to_string();

    assert_eq!(
        table,
        static_table!(
            " N | column 0 | column 1 | column 2 "
            "---+----------+----------+----------"
            " 0 | 0-0      | 0-1      | 0-2      "
            "   |          |          |          "
            "   |          |          |          "
            " 1 | 1-0      | 1-1      | 1-2      "
            "   |          |          |          "
            "   |          |          |          "
            " 2 | 2-0      | 2-1      | 2-2      "
            "   |          |          |          "
            "   |          |          |          "
        )
    );
}

#[test]
fn padding_with_set_characters() {
    let data = create_vector::<3, 3>();
    let table = Table::new(&data)
        .with(Style::psql())
        .with(
            Modify::new(Segment::all()).with(Padding::new(1, 2, 1, 1).set_fill('>', '<', 'V', '^')),
        )
        .to_string();

    assert_eq!(
        table,
        static_table!(
            "VVVV|VVVVVVVVVVV|VVVVVVVVVVV|VVVVVVVVVVV"
            ">N<<|>column 0<<|>column 1<<|>column 2<<"
            "^^^^|^^^^^^^^^^^|^^^^^^^^^^^|^^^^^^^^^^^"
            "----+-----------+-----------+-----------"
            "VVVV|VVVVVVVVVVV|VVVVVVVVVVV|VVVVVVVVVVV"
            ">0<<|>  0-0   <<|>  0-1   <<|>  0-2   <<"
            "^^^^|^^^^^^^^^^^|^^^^^^^^^^^|^^^^^^^^^^^"
            "VVVV|VVVVVVVVVVV|VVVVVVVVVVV|VVVVVVVVVVV"
            ">1<<|>  1-0   <<|>  1-1   <<|>  1-2   <<"
            "^^^^|^^^^^^^^^^^|^^^^^^^^^^^|^^^^^^^^^^^"
            "VVVV|VVVVVVVVVVV|VVVVVVVVVVV|VVVVVVVVVVV"
            ">2<<|>  2-0   <<|>  2-1   <<|>  2-2   <<"
            "^^^^|^^^^^^^^^^^|^^^^^^^^^^^|^^^^^^^^^^^"
        )
    );
}

#[test]
fn padding_with_set_characters_and_zero_ident() {
    let data = create_vector::<3, 3>();
    let table = Table::new(&data)
        .with(Style::psql())
        .with(Modify::new(Segment::all()).with(Padding::zero().set_fill('>', '<', '^', 'V')))
        .to_string();

    assert_eq!(
        table,
        static_table!(
            "N|column 0|column 1|column 2"
            "-+--------+--------+--------"
            "0|  0-0   |  0-1   |  0-2   "
            "1|  1-0   |  1-1   |  1-2   "
            "2|  2-0   |  2-1   |  2-2   "
        )
    );
}

#[test]
fn padding_multiline() {
    let data = create_vector::<3, 3>();
    let table = Table::new(&data)
        .with(Style::psql())
        .with(Modify::new(Rows::new(1..)).with(Padding::new(1, 1, 1, 1)))
        .to_string();

    assert_eq!(
        table,
        static_table!(
            " N | column 0 | column 1 | column 2 "
            "---+----------+----------+----------"
            "   |          |          |          "
            " 0 |   0-0    |   0-1    |   0-2    "
            "   |          |          |          "
            "   |          |          |          "
            " 1 |   1-0    |   1-1    |   1-2    "
            "   |          |          |          "
            "   |          |          |          "
            " 2 |   2-0    |   2-1    |   2-2    "
            "   |          |          |          "
        )
    );
}

#[test]
fn padding_multiline_with_vertical_alignment() {
    let data = create_vector::<3, 3>();
    let table = Table::new(&data)
        .with(Style::psql())
        .with(
            Modify::new(Segment::all())
                .with(Alignment::center())
                .with(Alignment::center_vertical()),
        )
        .with(Modify::new(Rows::new(1..)).with(Padding::new(1, 1, 1, 1)))
        .to_string();

    assert_eq!(
        table,
        static_table!(
            " N | column 0 | column 1 | column 2 "
            "---+----------+----------+----------"
            "   |          |          |          "
            " 0 |   0-0    |   0-1    |   0-2    "
            "   |          |          |          "
            "   |          |          |          "
            " 1 |   1-0    |   1-1    |   1-2    "
            "   |          |          |          "
            "   |          |          |          "
            " 2 |   2-0    |   2-1    |   2-2    "
            "   |          |          |          "
        )
    );
}
