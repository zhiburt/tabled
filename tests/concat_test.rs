use tabled::{Concat, Style, Table};

use util::{create_vector, static_table};

mod util;

#[test]
fn table_join_vertical() {
    let mut data1 = create_vector::<2, 3>();
    data1[0][0] = "123".to_string();

    let data2 = create_vector::<2, 3>();

    {
        let table1 = Table::new(&data1).with(Style::psql());
        let table2 = Table::new(&data2).with(Style::ascii());
        let table3 = table1.with(Concat::vertical(table2));

        assert_eq!(
            table3.to_string(),
            static_table!(
                "  N  | column 0 | column 1 | column 2 "
                "-----+----------+----------+----------"
                " 123 |   0-0    |   0-1    |   0-2    "
                "  1  |   1-0    |   1-1    |   1-2    "
                "  N  | column 0 | column 1 | column 2 "
                "  0  |   0-0    |   0-1    |   0-2    "
                "  1  |   1-0    |   1-1    |   1-2    "
            )
        );
    }

    {
        let table1 = Table::new(&data1).with(Style::psql());
        let table2 = Table::new(&data2).with(Style::ascii());
        let table3 = table2.with(Concat::vertical(table1));

        assert_eq!(
            table3.to_string(),
            static_table!(
                "+-----+----------+----------+----------+"
                "|  N  | column 0 | column 1 | column 2 |"
                "+-----+----------+----------+----------+"
                "|  0  |   0-0    |   0-1    |   0-2    |"
                "+-----+----------+----------+----------+"
                "|  1  |   1-0    |   1-1    |   1-2    |"
                "+-----+----------+----------+----------+"
                "|  N  | column 0 | column 1 | column 2 |"
                "+-----+----------+----------+----------+"
                "| 123 |   0-0    |   0-1    |   0-2    |"
                "+-----+----------+----------+----------+"
                "|  1  |   1-0    |   1-1    |   1-2    |"
                "+-----+----------+----------+----------+"
            )
        );
    }
}

#[test]
fn table_join_horizontal() {
    let data1 = create_vector::<2, 3>();
    let data2 = create_vector::<2, 3>();

    {
        let table1 = Table::new(&data1).with(Style::ascii());
        let table2 = Table::new(&data2).with(Style::psql());
        let table3 = table2.with(Concat::horizontal(table1));

        assert_eq!(
            table3.to_string(),
            static_table!(
                " N | column 0 | column 1 | column 2 | N | column 0 | column 1 | column 2 "
                "---+----------+----------+----------                                     "
                " 0 |   0-0    |   0-1    |   0-2    | 0 |   0-0    |   0-1    |   0-2    "
                " 1 |   1-0    |   1-1    |   1-2    | 1 |   1-0    |   1-1    |   1-2    "
            )
        );
    }

    {
        let table1 = Table::new(&data1).with(Style::ascii());
        let table2 = Table::new(&data2).with(Style::psql());
        let table3 = table1.with(Concat::horizontal(table2));

        assert_eq!(
            table3.to_string(),
            static_table!(
                "+---+----------+----------+----------+---+----------+----------+----------+"
                "| N | column 0 | column 1 | column 2 | N | column 0 | column 1 | column 2 |"
                "+---+----------+----------+----------+---+----------+----------+----------+"
                "| 0 |   0-0    |   0-1    |   0-2    | 0 |   0-0    |   0-1    |   0-2    |"
                "+---+----------+----------+----------+---+----------+----------+----------+"
                "| 1 |   1-0    |   1-1    |   1-2    | 1 |   1-0    |   1-1    |   1-2    |"
                "+---+----------+----------+----------+---+----------+----------+----------+"
            )
        );
    }
}

#[test]
fn table_join_vertical_different_size() {
    let data1 = create_vector::<2, 2>();
    let data2 = create_vector::<2, 3>();

    let table1 = Table::new(&data1).with(Style::psql());
    let table2 = Table::new(&data2).with(Style::psql());
    let table3 = table1.with(Concat::vertical(table2));

    assert_eq!(
        table3.to_string(),
        static_table!(
            " N | column 0 | column 1 |          "
            "---+----------+----------           "
            " 0 |   0-0    |   0-1    |          "
            " 1 |   1-0    |   1-1    |          "
            " N | column 0 | column 1 | column 2 "
            " 0 |   0-0    |   0-1    |   0-2    "
            " 1 |   1-0    |   1-1    |   1-2    "
        )
    );
}

#[test]
fn table_join_horizontal_different_size() {
    let data1 = create_vector::<2, 3>();
    let data2 = create_vector::<3, 3>();

    let table1 = Table::new(&data1).with(Style::psql());
    let table2 = Table::new(&data2).with(Style::psql());
    let table3 = table1.with(Concat::horizontal(table2));

    assert_eq!(
        table3.to_string(),
        static_table!(
            " N | column 0 | column 1 | column 2 | N | column 0 | column 1 | column 2 "
            "---+----------+----------+----------                                     "
            " 0 |   0-0    |   0-1    |   0-2    | 0 |   0-0    |   0-1    |   0-2    "
            " 1 |   1-0    |   1-1    |   1-2    | 1 |   1-0    |   1-1    |   1-2    "
            "   |          |          |          | 2 |   2-0    |   2-1    |   2-2    "
        )
    );
}

#[test]
fn table_join_horizontal_with_not_default_empty_string() {
    let data1 = create_vector::<2, 3>();
    let data2 = create_vector::<3, 3>();

    let table1 = Table::new(&data1).with(Style::psql());
    let table2 = Table::new(&data2).with(Style::psql());
    let table3 = table1.with(Concat::horizontal(table2).default_cell("NaN"));

    assert_eq!(
        table3.to_string(),
        static_table!(
            " N | column 0 | column 1 | column 2 | N | column 0 | column 1 | column 2 "
            "---+----------+----------+----------                                     "
            " 0 |   0-0    |   0-1    |   0-2    | 0 |   0-0    |   0-1    |   0-2    "
            " 1 |   1-0    |   1-1    |   1-2    | 1 |   1-0    |   1-1    |   1-2    "
            "NaN|NaN       |NaN       |NaN       | 2 |   2-0    |   2-1    |   2-2    "
        )
    );
}

#[test]
fn table_join_vertical_with_not_default_empty_string() {
    let data1 = create_vector::<2, 2>();
    let data2 = create_vector::<2, 3>();

    let table1 = Table::new(&data1).with(Style::psql());
    let table2 = Table::new(&data2).with(Style::psql());
    let table3 = table1.with(Concat::vertical(table2).default_cell("NaN"));

    assert_eq!(
        table3.to_string(),
        static_table!(
            " N | column 0 | column 1 |NaN       "
            "---+----------+----------           "
            " 0 |   0-0    |   0-1    |NaN       "
            " 1 |   1-0    |   1-1    |NaN       "
            " N | column 0 | column 1 | column 2 "
            " 0 |   0-0    |   0-1    |   0-2    "
            " 1 |   1-0    |   1-1    |   1-2    "
        )
    );
}
