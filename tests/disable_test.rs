use crate::util::create_vector;
use tabled::{Alignment, Disable, Full, Modify, Style, Table};

mod util;

#[test]
fn disable_rows() {
    let data = create_vector::<3, 3>();
    let table = Table::new(&data)
        .with(Style::ASCII)
        .with(Modify::new(Full).with(Alignment::left()))
        .with(Disable::Row(1..=2))
        .to_string();

    let expected = concat!(
        "+---+----------+----------+----------+\n",
        "| N | column 0 | column 1 | column 2 |\n",
        "+---+----------+----------+----------+\n",
        "| 2 | 2-0      | 2-1      | 2-2      |\n",
        "+---+----------+----------+----------+\n",
    );

    assert_eq!(table, expected);
}

#[test]
fn disable_header() {
    let data = create_vector::<3, 3>();
    let table = Table::new(&data)
        .with(Style::PSQL)
        .with(Modify::new(Full).with(Alignment::left()))
        .with(Disable::Row(..1))
        .to_string();

    let expected = concat!(
        "---+-----+-----+-----\n",
        " 0 | 0-0 | 0-1 | 0-2 \n",
        " 1 | 1-0 | 1-1 | 1-2 \n",
        " 2 | 2-0 | 2-1 | 2-2 \n",
    );

    assert_eq!(table, expected);
}

#[test]
fn disable_all_table_via_rows() {
    let data = create_vector::<3, 3>();
    let table = Table::new(&data)
        .with(Style::PSQL)
        .with(Modify::new(Full).with(Alignment::left()))
        .with(Disable::Row(..))
        .to_string();

    assert_eq!(table, "");
}

#[test]
fn disable_header_with_new_styling() {
    let data = create_vector::<3, 3>();
    let table = Table::new(&data)
        .with(Modify::new(Full).with(Alignment::left()))
        .with(Disable::Row(..1))
        .with(Style::PSEUDO_CLEAN)
        .to_string();

    let expected = concat!(
        "┌───┬─────┬─────┬─────┐\n",
        "│ 0 │ 0-0 │ 0-1 │ 0-2 │\n",
        "├───┼─────┼─────┼─────┤\n",
        "│ 1 │ 1-0 │ 1-1 │ 1-2 │\n",
        "│ 2 │ 2-0 │ 2-1 │ 2-2 │\n",
        "└───┴─────┴─────┴─────┘\n",
    );

    assert_eq!(table, expected);
}

#[test]
fn disable_columns() {
    let data = create_vector::<3, 3>();
    let table = Table::new(&data)
        .with(Style::PSQL)
        .with(Disable::Column(..1))
        .to_string();

    let expected = concat!(
        "| column 0 | column 1 | column 2 \n",
        "+----------+----------+----------\n",
        "|   0-0    |   0-1    |   0-2    \n",
        "|   1-0    |   1-1    |   1-2    \n",
        "|   2-0    |   2-1    |   2-2    \n",
    );

    assert_eq!(table, expected);
}

#[test]
fn disable_all_table_via_columns() {
    let data = create_vector::<3, 3>();
    let table = Table::new(&data)
        .with(Style::PSQL)
        .with(Modify::new(Full).with(Alignment::left()))
        .with(Disable::Column(..))
        .to_string();

    assert_eq!(table, "");
}
