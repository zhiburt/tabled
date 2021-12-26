use tabled::join::{Join, JoinMode};
use tabled::{Alignment, Full, Modify, Style, Table};
use util::create_vector;

mod util;

#[test]
fn table_join_vertical_top() {
    let data1 = create_vector::<3, 3>();
    let data2 = create_vector::<3, 3>();

    let table1 = Table::new(&data1)
        .with(Style::PSQL)
        .with(Modify::new(Full).with(Alignment::left()));

    let table2 = Table::new(&data2)
        .with(Style::ASCII)
        .with(Modify::new(Full).with(Alignment::left()));

    let table3 = table1.with(Join {
        mode: JoinMode::Vertical(table2),
        strict_size: true,
    });

    let expected = concat!(
        "  N | column 0 | column 1 | column 2  \n",
        " ---+----------+----------+---------- \n",
        "  0 | 0-0      | 0-1      | 0-2       \n",
        "  1 | 1-0      | 1-1      | 1-2       \n",
        "  2 | 2-0      | 2-1      | 2-2       \n",
        "+---+----------+----------+----------+\n",
        "| N | column 0 | column 1 | column 2 |\n",
        "+---+----------+----------+----------+\n",
        "| 0 | 0-0      | 0-1      | 0-2      |\n",
        "+---+----------+----------+----------+\n",
        "| 1 | 1-0      | 1-1      | 1-2      |\n",
        "+---+----------+----------+----------+\n",
        "| 2 | 2-0      | 2-1      | 2-2      |\n",
        "+---+----------+----------+----------+\n",
    );

    assert_eq!(table3.to_string(), expected);
}

#[test]
fn table_join_vertical_bottom() {
    let data1 = create_vector::<3, 3>();
    let data2 = create_vector::<3, 3>();

    let table1 = Table::new(&data1)
        .with(Style::PSQL)
        .with(Modify::new(Full).with(Alignment::left()));

    let table2 = Table::new(&data2)
        .with(Style::ASCII)
        .with(Modify::new(Full).with(Alignment::left()));

    let table3 = table2.with(Join {
        mode: JoinMode::Vertical(table1),
        strict_size: true,
    });

    let expected = concat!(
        "+---+----------+----------+----------+\n",
        "| N | column 0 | column 1 | column 2 |\n",
        "+---+----------+----------+----------+\n",
        "| 0 | 0-0      | 0-1      | 0-2      |\n",
        "+---+----------+----------+----------+\n",
        "| 1 | 1-0      | 1-1      | 1-2      |\n",
        "+---+----------+----------+----------+\n",
        "| 2 | 2-0      | 2-1      | 2-2      |\n",
        "+---+----------+----------+----------+\n",
        "  N | column 0 | column 1 | column 2  \n",
        " ---+----------+----------+---------- \n",
        "  0 | 0-0      | 0-1      | 0-2       \n",
        "  1 | 1-0      | 1-1      | 1-2       \n",
        "  2 | 2-0      | 2-1      | 2-2       \n",
    );

    assert_eq!(table3.to_string(), expected);
}

#[test]
fn table_join_horizontal_left() {
    let data1 = create_vector::<3, 3>();
    let data2 = create_vector::<3, 3>();

    let table1 = Table::new(&data1)
        .with(Style::PSQL)
        .with(Modify::new(Full).with(Alignment::left()));

    let table2 = Table::new(&data2)
        .with(Style::ASCII)
        .with(Modify::new(Full).with(Alignment::left()));

    let table3 = table2.with(Join {
        mode: JoinMode::Horizontal(table1),
        strict_size: true,
    });

    let expected = concat!(
        "+---+----------+----------+----------+                                    \n",
        "| N | column 0 | column 1 | column 2 | N | column 0 | column 1 | column 2 \n",
        "+---+----------+----------+----------+---+----------+----------+----------\n",
        "| 0 | 0-0      | 0-1      | 0-2      | 0 | 0-0      | 0-1      | 0-2      \n",
        "+---+----------+----------+----------+                                    \n",
        "| 1 | 1-0      | 1-1      | 1-2      | 1 | 1-0      | 1-1      | 1-2      \n",
        "+---+----------+----------+----------+                                    \n",
        "| 2 | 2-0      | 2-1      | 2-2      | 2 | 2-0      | 2-1      | 2-2      \n",
        "+---+----------+----------+----------+                                    \n",
    );

    assert_eq!(table3.to_string(), expected);
}

#[test]
fn table_join_horizontal_right() {
    let data1 = create_vector::<3, 3>();
    let data2 = create_vector::<3, 3>();

    let table1 = Table::new(&data1)
        .with(Style::PSQL)
        .with(Modify::new(Full).with(Alignment::left()));

    let table2 = Table::new(&data2)
        .with(Style::ASCII)
        .with(Modify::new(Full).with(Alignment::left()));

    let table3 = table1.with(Join {
        mode: JoinMode::Horizontal(table2),
        strict_size: true,
    });

    let expected = concat!(
        "                                    +---+----------+----------+----------+\n",
        " N | column 0 | column 1 | column 2 | N | column 0 | column 1 | column 2 |\n",
        "---+----------+----------+----------+---+----------+----------+----------+\n",
        " 0 | 0-0      | 0-1      | 0-2      | 0 | 0-0      | 0-1      | 0-2      |\n",
        "                                    +---+----------+----------+----------+\n",
        " 1 | 1-0      | 1-1      | 1-2      | 1 | 1-0      | 1-1      | 1-2      |\n",
        "                                    +---+----------+----------+----------+\n",
        " 2 | 2-0      | 2-1      | 2-2      | 2 | 2-0      | 2-1      | 2-2      |\n",
        "                                    +---+----------+----------+----------+\n",
    );

    assert_eq!(table3.to_string(), expected);
}

#[test]
fn table_join_horizontal_different_size() {
    let data1 = create_vector::<3, 3>();
    let data2 = create_vector::<4, 3>();

    let table1 = Table::new(&data1)
        .with(Style::PSQL)
        .with(Modify::new(Full).with(Alignment::left()));

    let table2 = Table::new(&data2)
        .with(Style::PSQL)
        .with(Modify::new(Full).with(Alignment::left()));

    let table3 = table1.with(Join {
        mode: JoinMode::Horizontal(table2),
        strict_size: false,
    });

    let expected = concat!(
        " N | column 0 | column 1 | column 2  N | column 0 | column 1 | column 2 \n",
        "---+----------+----------+-------------+----------+----------+----------\n",
        " 0 | 0-0      | 0-1      | 0-2       0 | 0-0      | 0-1      | 0-2      \n",
        " 1 | 1-0      | 1-1      | 1-2       1 | 1-0      | 1-1      | 1-2      \n",
        " 2 | 2-0      | 2-1      | 2-2       2 | 2-0      | 2-1      | 2-2      \n",
        "                                     3 | 3-0      | 3-1      | 3-2      \n",
    );

    assert_eq!(table3.to_string(), expected);
}

#[test]
fn table_join_vertical_different_size() {
    let data1 = create_vector::<3, 3>();
    let data2 = create_vector::<3, 4>();

    let table1 = Table::new(&data1)
        .with(Style::PSQL)
        .with(Modify::new(Full).with(Alignment::left()));

    let table2 = Table::new(&data2)
        .with(Style::PSQL)
        .with(Modify::new(Full).with(Alignment::left()));

    let table3 = table1.with(
        Join {
        mode: JoinMode::Vertical(table2),
        strict_size: false,
    });

    println!("{}",table3.to_string());

    let expected = concat!(
        " N | column 0 | column 1 | column 2            \n",
        "---+----------+----------+----------           \n",
        " 0 | 0-0      | 0-1      | 0-2                 \n",
        " 1 | 1-0      | 1-1      | 1-2                 \n",
        " 2 | 2-0      | 2-1      | 2-2                 \n",
        " N | column 0 | column 1 | column 2 | column 3 \n",
        "---+----------+----------+----------+----------\n",
        " 0 | 0-0      | 0-1      | 0-2      | 0-3      \n",
        " 1 | 1-0      | 1-1      | 1-2      | 1-3      \n",
        " 2 | 2-0      | 2-1      | 2-2      | 2-3      \n",
    );

    assert_eq!(table3.to_string(), expected);
}
