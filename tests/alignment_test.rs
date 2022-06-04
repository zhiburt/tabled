use tabled::{
    object::{Columns, Rows, Segment},
    Alignment, Modify, Padding, Style, Table,
};

use crate::util::{create_vector, static_table};

mod util;

#[test]
fn full_alignment() {
    let data = create_vector::<3, 3>();
    let table = Table::new(&data)
        .with(Style::psql())
        .with(Modify::new(Segment::all()).with(Alignment::left()))
        .to_string();

    assert_eq!(
        table,
        static_table!(
            " N | column 0 | column 1 | column 2 "
            "---+----------+----------+----------"
            " 0 | 0-0      | 0-1      | 0-2      "
            " 1 | 1-0      | 1-1      | 1-2      "
            " 2 | 2-0      | 2-1      | 2-2      "
        )
    );
}

#[test]
fn head_and_data_alignment() {
    let data = create_vector::<3, 3>();
    let table = Table::new(&data)
        .with(Style::ascii())
        .with(Modify::new(Rows::first()).with(Alignment::left()))
        .with(Modify::new(Rows::new(1..)).with(Alignment::right()))
        .to_string();

    assert_eq!(
        table,
        static_table!(
            "+---+----------+----------+----------+"
            "| N | column 0 | column 1 | column 2 |"
            "+---+----------+----------+----------+"
            "| 0 |      0-0 |      0-1 |      0-2 |"
            "+---+----------+----------+----------+"
            "| 1 |      1-0 |      1-1 |      1-2 |"
            "+---+----------+----------+----------+"
            "| 2 |      2-0 |      2-1 |      2-2 |"
            "+---+----------+----------+----------+"
        )
    );
}

#[test]
fn full_alignment_multiline() {
    let mut data = create_vector::<3, 3>();
    data[2][2] = String::from("https://\nwww\n.\nredhat\n.com\n/en");

    let table = Table::new(&data)
        .with(Style::psql())
        .with(Modify::new(Segment::all()).with(Alignment::left()))
        .to_string();

    assert_eq!(
        table,
        static_table!(
            " N | column 0 | column 1 | column 2 "
            "---+----------+----------+----------"
            " 0 | 0-0      | 0-1      | 0-2      "
            " 1 | 1-0      | 1-1      | 1-2      "
            " 2 | 2-0      | https:// | 2-2      "
            "   |          | www      |          "
            "   |          | .        |          "
            "   |          | redhat   |          "
            "   |          | .com     |          "
            "   |          | /en      |          "
        )
    );
}

#[test]
fn vertical_alignment_test() {
    let mut data = create_vector::<3, 3>();
    data[1][2] = String::from("E\nnde\navou\nros");
    data[2][2] = String::from("Red\nHat");
    data[2][3] = String::from("https://\nwww\n.\nredhat\n.com\n/en");

    let table = Table::new(&data)
        .with(Style::psql())
        .with(Modify::new(Columns::new(1..)).with(Alignment::bottom()))
        .to_string();

    assert_eq!(
        table,
        static_table!(
            " N | column 0 | column 1 | column 2 "
            "---+----------+----------+----------"
            " 0 |   0-0    |   0-1    |   0-2    "
            " 1 |          |   E      |          "
            "   |          |   nde    |          "
            "   |          |   avou   |          "
            "   |   1-0    |   ros    |   1-2    "
            " 2 |          |          | https:// "
            "   |          |          | www      "
            "   |          |          | .        "
            "   |          |          | redhat   "
            "   |          |   Red    | .com     "
            "   |   2-0    |   Hat    | /en      "
        )
    );
}

#[test]
fn alignment_doesnt_change_padding() {
    let data = create_vector::<3, 3>();

    let table = Table::new(&data)
        .with(Style::psql())
        .with(Modify::new(Segment::all()).with(Padding::new(3, 0, 0, 0)))
        .with(Modify::new(Segment::all()).with(Alignment::left()))
        .to_string();

    assert_eq!(
        table,
        static_table!(
            "   N|   column 0|   column 1|   column 2"
            "----+-----------+-----------+-----------"
            "   0|   0-0     |   0-1     |   0-2     "
            "   1|   1-0     |   1-1     |   1-2     "
            "   2|   2-0     |   2-1     |   2-2     "
        )
    );
}
