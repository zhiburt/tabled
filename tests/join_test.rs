use tabled::{join::Join, Style, Table};
use util::create_vector;

mod util;

#[test]
fn table_join_vertical() {
    let data1 = create_vector::<2, 3>();
    let data2 = create_vector::<2, 3>();

    {
        let table1 = Table::new(&data1).with(Style::PSQL);
        let table2 = Table::new(&data2).with(Style::ASCII);
        let table3 = table1.with(Join::vertical(table2));

        let expected = concat!(
            "  N | column 0 | column 1 | column 2  \n",
            " ---+----------+----------+---------- \n",
            "  0 |   0-0    |   0-1    |   0-2     \n",
            "  1 |   1-0    |   1-1    |   1-2     \n",
            "+---+----------+----------+----------+\n",
            "| N | column 0 | column 1 | column 2 |\n",
            "+---+----------+----------+----------+\n",
            "| 0 |   0-0    |   0-1    |   0-2    |\n",
            "+---+----------+----------+----------+\n",
            "| 1 |   1-0    |   1-1    |   1-2    |\n",
            "+---+----------+----------+----------+\n",
        );

        assert_eq!(expected, table3.to_string());
    }

    {
        let table1 = Table::new(&data1).with(Style::PSQL);
        let table2 = Table::new(&data2).with(Style::ASCII);
        let table3 = table2.with(Join::vertical(table1));

        let expected = concat!(
            "+---+----------+----------+----------+\n",
            "| N | column 0 | column 1 | column 2 |\n",
            "+---+----------+----------+----------+\n",
            "| 0 |   0-0    |   0-1    |   0-2    |\n",
            "+---+----------+----------+----------+\n",
            "| 1 |   1-0    |   1-1    |   1-2    |\n",
            "+---+----------+----------+----------+\n",
            "  N | column 0 | column 1 | column 2  \n",
            " ---+----------+----------+---------- \n",
            "  0 |   0-0    |   0-1    |   0-2     \n",
            "  1 |   1-0    |   1-1    |   1-2     \n",
        );

        assert_eq!(table3.to_string(), expected);
    }
}

#[test]
fn table_join_horizontal() {
    let data1 = create_vector::<2, 3>();
    let data2 = create_vector::<2, 3>();

    {
        let table1 = Table::new(&data1).with(Style::PSQL);
        let table2 = Table::new(&data2).with(Style::ASCII);
        let table3 = table2.with(Join::horizontal(table1));

        let expected = concat!(
            "+---+----------+----------+----------+                                    \n",
            "| N | column 0 | column 1 | column 2 | N | column 0 | column 1 | column 2 \n",
            "+---+----------+----------+----------+---+----------+----------+----------\n",
            "| 0 |   0-0    |   0-1    |   0-2    | 0 |   0-0    |   0-1    |   0-2    \n",
            "+---+----------+----------+----------+                                    \n",
            "| 1 |   1-0    |   1-1    |   1-2    | 1 |   1-0    |   1-1    |   1-2    \n",
            "+---+----------+----------+----------+                                    \n",
        );

        assert_eq!(expected, table3.to_string());
    }

    {
        let table1 = Table::new(&data1).with(Style::PSQL);
        let table2 = Table::new(&data2).with(Style::ASCII);
        let table3 = table1.with(Join::horizontal(table2));

        let expected = concat!(
            "                                    +---+----------+----------+----------+\n",
            " N | column 0 | column 1 | column 2 | N | column 0 | column 1 | column 2 |\n",
            "---+----------+----------+----------+---+----------+----------+----------+\n",
            " 0 |   0-0    |   0-1    |   0-2    | 0 |   0-0    |   0-1    |   0-2    |\n",
            "                                    +---+----------+----------+----------+\n",
            " 1 |   1-0    |   1-1    |   1-2    | 1 |   1-0    |   1-1    |   1-2    |\n",
            "                                    +---+----------+----------+----------+\n",
        );

        assert_eq!(expected, table3.to_string());
    }
}

#[test]
fn table_join_vertical_different_size() {
    let data1 = create_vector::<2, 2>();
    let data2 = create_vector::<2, 3>();

    let table1 = Table::new(&data1).with(Style::PSQL);
    let table2 = Table::new(&data2).with(Style::PSQL);
    let table3 = table1.with(Join::vertical(table2));

    let expected = concat!(
        " N | column 0 | column 1            \n",
        "---+----------+----------           \n",
        " 0 |   0-0    |   0-1               \n",
        " 1 |   1-0    |   1-1               \n",
        " N | column 0 | column 1 | column 2 \n",
        "---+----------+----------+----------\n",
        " 0 |   0-0    |   0-1    |   0-2    \n",
        " 1 |   1-0    |   1-1    |   1-2    \n",
    );

    assert_eq!(expected, table3.to_string());
}

#[test]
fn table_join_horizontal_different_size() {
    let data1 = create_vector::<2, 3>();
    let data2 = create_vector::<3, 3>();

    let table1 = Table::new(&data1).with(Style::PSQL);
    let table2 = Table::new(&data2).with(Style::PSQL);
    let table3 = table1.with(Join::horizontal(table2));

    let expected = concat!(
        " N | column 0 | column 1 | column 2  N | column 0 | column 1 | column 2 \n",
        "---+----------+----------+-------------+----------+----------+----------\n",
        " 0 |   0-0    |   0-1    |   0-2     0 |   0-0    |   0-1    |   0-2    \n",
        " 1 |   1-0    |   1-1    |   1-2     1 |   1-0    |   1-1    |   1-2    \n",
        "                                     2 |   2-0    |   2-1    |   2-2    \n",
    );

    assert_eq!(expected, table3.to_string());
}
