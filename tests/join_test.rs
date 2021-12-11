use tabled::{Style, Table, join, Full, Modify, Alignment};
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
        
    let table3 = table1.with(join::Join::Vertical(table2));

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
        
    let table3 = table2.with(join::Join::Vertical(table1));

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
        
    let table3 = table2.with(join::Join::Horizontal(table1));

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
        
    let table3 = table1.with(join::Join::Horizontal(table2));

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
