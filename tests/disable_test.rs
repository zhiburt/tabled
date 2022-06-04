use tabled::{object::Segment, Alignment, Disable, Modify, Style, Table};

use crate::util::{create_vector, static_table};

mod util;

#[test]
fn disable_rows() {
    let data = create_vector::<3, 3>();
    let table = Table::new(&data)
        .with(Style::ascii())
        .with(Modify::new(Segment::all()).with(Alignment::left()))
        .with(Disable::Row(1..=2))
        .to_string();

    assert_eq!(
        table,
        static_table!(
            "+---+----------+----------+----------+"
            "| N | column 0 | column 1 | column 2 |"
            "+---+----------+----------+----------+"
            "| 2 | 2-0      | 2-1      | 2-2      |"
            "+---+----------+----------+----------+"
        )
    );
}

#[test]
fn disable_header() {
    let data = create_vector::<3, 3>();
    let table = Table::new(&data)
        .with(Style::psql())
        .with(Modify::new(Segment::all()).with(Alignment::left()))
        .with(Disable::Row(..1))
        .to_string();

    assert_eq!(
        table,
        static_table!(
            "---+-----+-----+-----"
            " 0 | 0-0 | 0-1 | 0-2 "
            " 1 | 1-0 | 1-1 | 1-2 "
            " 2 | 2-0 | 2-1 | 2-2 "
        )
    );
}

#[test]
fn disable_all_table_via_rows() {
    let data = create_vector::<3, 3>();
    let table = Table::new(&data)
        .with(Style::psql())
        .with(Modify::new(Segment::all()).with(Alignment::left()))
        .with(Disable::Row(..))
        .to_string();

    assert_eq!(table, "");
}

#[test]
fn disable_header_with_new_styling() {
    let data = create_vector::<3, 3>();
    let table = Table::new(&data)
        .with(Modify::new(Segment::all()).with(Alignment::left()))
        .with(Disable::Row(..1))
        .with(Style::modern().horizontal_off())
        .to_string();

    assert_eq!(
        table,
        static_table!(
            "┌───┬─────┬─────┬─────┐"
            "│ 0 │ 0-0 │ 0-1 │ 0-2 │"
            "├───┼─────┼─────┼─────┤"
            "│ 1 │ 1-0 │ 1-1 │ 1-2 │"
            "│ 2 │ 2-0 │ 2-1 │ 2-2 │"
            "└───┴─────┴─────┴─────┘"
        )
    );
}

#[test]
fn disable_columns() {
    let data = create_vector::<3, 3>();
    let table = Table::new(&data)
        .with(Style::psql())
        .with(Disable::Column(..1))
        .to_string();

    assert_eq!(
        table,
        static_table!(
            "| column 0 | column 1 | column 2 "
            "+----------+----------+----------"
            "|   0-0    |   0-1    |   0-2    "
            "|   1-0    |   1-1    |   1-2    "
            "|   2-0    |   2-1    |   2-2    "
        )
    );
}

#[test]
fn disable_all_table_via_columns() {
    let data = create_vector::<3, 3>();
    let table = Table::new(&data)
        .with(Style::psql())
        .with(Modify::new(Segment::all()).with(Alignment::left()))
        .with(Disable::Column(..))
        .to_string();

    assert_eq!(table, "");
}
