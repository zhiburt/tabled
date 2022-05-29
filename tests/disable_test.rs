use crate::util::create_vector;
use tabled::{object::Segment, Alignment, Disable, Modify, Style, Table};

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
        concat!(
            "+---+----------+----------+----------+\n",
            "| N | column 0 | column 1 | column 2 |\n",
            "+---+----------+----------+----------+\n",
            "| 2 | 2-0      | 2-1      | 2-2      |\n",
            "+---+----------+----------+----------+\n",
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
        concat!(
            "---+-----+-----+-----\n",
            " 0 | 0-0 | 0-1 | 0-2 \n",
            " 1 | 1-0 | 1-1 | 1-2 \n",
            " 2 | 2-0 | 2-1 | 2-2 \n",
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
        concat!(
            "┌───┬─────┬─────┬─────┐\n",
            "│ 0 │ 0-0 │ 0-1 │ 0-2 │\n",
            "├───┼─────┼─────┼─────┤\n",
            "│ 1 │ 1-0 │ 1-1 │ 1-2 │\n",
            "│ 2 │ 2-0 │ 2-1 │ 2-2 │\n",
            "└───┴─────┴─────┴─────┘\n",
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
        concat!(
            "| column 0 | column 1 | column 2 \n",
            "+----------+----------+----------\n",
            "|   0-0    |   0-1    |   0-2    \n",
            "|   1-0    |   1-1    |   1-2    \n",
            "|   2-0    |   2-1    |   2-2    \n",
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
