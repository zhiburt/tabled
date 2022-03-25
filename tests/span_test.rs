use crate::util::create_vector;
use tabled::{
    object::{Cell, Columns, Full},
    Alignment, Modify, Padding, Span, Style, Table,
};

mod util;

#[test]
fn span_column_test() {
    let data = create_vector::<3, 3>();
    {
        let table = Table::new(&data)
            .with(Style::psql())
            .with(Modify::new(Full).with(Alignment::left()))
            .with(Modify::new(Columns::single(0)).with(Span::column(2)))
            .to_string();

        let expected = concat!(
            " N | column 1 | column 2 \n",
            "-+-+----------+----------\n",
            " 0 | 0-1      | 0-2      \n",
            " 1 | 1-1      | 1-2      \n",
            " 2 | 2-1      | 2-2      \n",
        );

        assert_eq!(table, expected);
    }
    {
        let table = Table::new(&data)
            .with(Style::psql())
            .with(Modify::new(Full).with(Alignment::left()))
            .with(Modify::new(Columns::new(1..2)).with(Span::column(2)))
            .to_string();

        let expected = concat!(
            " N | column 0 | column 2 \n",
            "---+-----+----+----------\n",
            " 0 | 0-0      | 0-2      \n",
            " 1 | 1-0      | 1-2      \n",
            " 2 | 2-0      | 2-2      \n",
        );

        assert_eq!(table, expected);
    }
    {
        let table = Table::new(&data)
            .with(Style::psql())
            .with(Modify::new(Full).with(Alignment::left()))
            .with(Modify::new(Columns::single(0)).with(Span::column(data.len() + 1)))
            .to_string();

        let expected = concat!(" N \n", "+++\n", " 0 \n", " 1 \n", " 2 \n");

        assert_eq!(table, expected);
    }
}

#[test]
fn cell_span_test() {
    let data = create_vector::<3, 3>();
    {
        // first column span=2

        {
            let table = Table::new(&data)
                .with(Style::psql())
                .with(Modify::new(Full).with(Alignment::left()))
                .with(Modify::new(Cell(0, 0)).with(Span::column(2)))
                .to_string();

            let expected = concat!(
                " N       | column 1 | column 2 \n",
                "---+-----+----------+----------\n",
                " 0 | 0-0 | 0-1      | 0-2      \n",
                " 1 | 1-0 | 1-1      | 1-2      \n",
                " 2 | 2-0 | 2-1      | 2-2      \n",
            );

            assert_eq!(table, expected);
        }
        {
            let table = Table::new(&data)
                .with(Style::psql())
                .with(Modify::new(Full).with(Alignment::left()))
                .with(Modify::new(Cell(1, 0)).with(Span::column(2)))
                .to_string();

            let expected = concat!(
                " N | column 0 | column 1 | column 2 \n",
                "---+----------+----------+----------\n",
                " 0            | 0-1      | 0-2      \n",
                " 1 | 1-0      | 1-1      | 1-2      \n",
                " 2 | 2-0      | 2-1      | 2-2      \n",
            );

            assert_eq!(table, expected);
        }
        {
            let table = Table::new(&data)
                .with(Style::psql())
                .with(Modify::new(Cell(2, 0)).with(Span::column(2)))
                .to_string();

            let expected = concat!(
                " N | column 0 | column 1 | column 2 \n",
                "---+----------+----------+----------\n",
                " 0 |   0-0    |   0-1    |   0-2    \n",
                "      1       |   1-1    |   1-2    \n",
                " 2 |   2-0    |   2-1    |   2-2    \n",
            );

            assert_eq!(table, expected);
        }
        {
            let table = Table::new(&data)
                .with(Style::psql())
                .with(Modify::new(Full).with(Alignment::left()))
                .with(Modify::new(Cell(3, 0)).with(Span::column(2)))
                .to_string();

            let expected = concat!(
                " N | column 0 | column 1 | column 2 \n",
                "---+----------+----------+----------\n",
                " 0 | 0-0      | 0-1      | 0-2      \n",
                " 1 | 1-0      | 1-1      | 1-2      \n",
                " 2            | 2-1      | 2-2      \n",
            );

            assert_eq!(table, expected);
        }
    }

    {
        // second column span=2
        {
            let table = Table::new(&data)
                .with(Style::psql())
                .with(Modify::new(Full).with(Alignment::left()))
                .with(Modify::new(Cell(0, 1)).with(Span::column(2)))
                .to_string();

            let expected = concat!(
                " N | column 0  | column 2 \n",
                "---+-----+-----+----------\n",
                " 0 | 0-0 | 0-1 | 0-2      \n",
                " 1 | 1-0 | 1-1 | 1-2      \n",
                " 2 | 2-0 | 2-1 | 2-2      \n",
            );

            assert_eq!(table, expected);
        }
        {
            let table = Table::new(&data)
                .with(Style::psql())
                .with(Modify::new(Full).with(Alignment::left()))
                .with(Modify::new(Cell(1, 1)).with(Span::column(2)))
                .to_string();

            let expected = concat!(
                " N | column 0 | column 1 | column 2 \n",
                "---+----------+----------+----------\n",
                " 0 | 0-0                 | 0-2      \n",
                " 1 | 1-0      | 1-1      | 1-2      \n",
                " 2 | 2-0      | 2-1      | 2-2      \n",
            );

            assert_eq!(table, expected);
        }
        {
            let table = Table::new(&data)
                .with(Style::psql())
                .with(Modify::new(Full).with(Alignment::left()))
                .with(Modify::new(Cell(2, 1)).with(Span::column(2)))
                .to_string();

            let expected = concat!(
                " N | column 0 | column 1 | column 2 \n",
                "---+----------+----------+----------\n",
                " 0 | 0-0      | 0-1      | 0-2      \n",
                " 1 | 1-0                 | 1-2      \n",
                " 2 | 2-0      | 2-1      | 2-2      \n",
            );

            assert_eq!(table, expected);
        }
        {
            let table = Table::new(&data)
                .with(Style::psql())
                .with(Modify::new(Full).with(Alignment::left()))
                .with(Modify::new(Cell(3, 1)).with(Span::column(2)))
                .to_string();

            let expected = concat!(
                " N | column 0 | column 1 | column 2 \n",
                "---+----------+----------+----------\n",
                " 0 | 0-0      | 0-1      | 0-2      \n",
                " 1 | 1-0      | 1-1      | 1-2      \n",
                " 2 | 2-0                 | 2-2      \n",
            );

            assert_eq!(table, expected);
        }
    }

    {
        // 3rd column span = 1

        {
            let table = Table::new(&data)
                .with(Style::psql())
                .with(Modify::new(Cell(0, 2)).with(Span::column(2)))
                .to_string();

            let expected = concat!(
                " N | column 0 | column 1  \n",
                "---+----------+-----+-----\n",
                " 0 |   0-0    | 0-1 | 0-2 \n",
                " 1 |   1-0    | 1-1 | 1-2 \n",
                " 2 |   2-0    | 2-1 | 2-2 \n",
            );

            assert_eq!(table, expected);
        }
        {
            let table = Table::new(&data)
                .with(Style::psql())
                .with(Modify::new(Cell(1, 2)).with(Span::column(2)))
                .to_string();

            let expected = concat!(
                " N | column 0 | column 1 | column 2 \n",
                "---+----------+----------+----------\n",
                " 0 |   0-0    |         0-1         \n",
                " 1 |   1-0    |   1-1    |   1-2    \n",
                " 2 |   2-0    |   2-1    |   2-2    \n",
            );

            assert_eq!(table, expected);
        }
        {
            let table = Table::new(&data)
                .with(Style::psql())
                .with(Modify::new(Cell(2, 2)).with(Span::column(2)))
                .to_string();

            let expected = concat!(
                " N | column 0 | column 1 | column 2 \n",
                "---+----------+----------+----------\n",
                " 0 |   0-0    |   0-1    |   0-2    \n",
                " 1 |   1-0    |         1-1         \n",
                " 2 |   2-0    |   2-1    |   2-2    \n",
            );

            assert_eq!(table, expected);
        }
        {
            let table = Table::new(&data)
                .with(Style::psql())
                .with(Modify::new(Cell(3, 2)).with(Span::column(2)))
                .to_string();

            let expected = concat!(
                " N | column 0 | column 1 | column 2 \n",
                "---+----------+----------+----------\n",
                " 0 |   0-0    |   0-1    |   0-2    \n",
                " 1 |   1-0    |   1-1    |   1-2    \n",
                " 2 |   2-0    |         2-1         \n",
            );

            assert_eq!(table, expected);
        }
    }
}

#[test]
#[should_panic]
fn span_column_exceeds_boundries_test() {
    // todo: determine if it's the right behaiviour

    let data = create_vector::<3, 3>();
    Table::new(&data)
        .with(Modify::new(Columns::single(0)).with(Span::column(100)))
        .to_string();
}

#[test]
fn span_cell_exceeds_boundries_test() {
    // these tests shows that exiding boundries causes invalid behaiviour
    //
    // todo: determine if it's the right behaiviour

    let data = create_vector::<3, 3>();
    let table = Table::new(&data)
        .with(Style::psql())
        .with(Modify::new(Full).with(Alignment::left()))
        .with(Modify::new(Cell(0, 0)).with(Span::column(20)))
        .to_string();

    let expected = " N \n---+-----+-----+-----\n 0 | 0-0 | 0-1 | 0-2 \n 1 | 1-0 | 1-1 | 1-2 \n 2 | 2-0 | 2-1 | 2-2 \n";

    assert_eq!(table, expected);

    let data = create_vector::<3, 3>();
    let table = Table::new(&data)
        .with(Style::psql())
        .with(Modify::new(Full).with(Alignment::left()))
        .with(Modify::new(Cell(1, 1)).with(Span::column(20)))
        .to_string();

    let expected = " N | column 0 | column 1 | column 2 \n---+----------+----------+----------\n 0 | 0-0 \n 1 | 1-0      | 1-1      | 1-2      \n 2 | 2-0      | 2-1      | 2-2      \n";

    assert_eq!(table, expected);

    let data = create_vector::<3, 3>();
    let table = Table::new(&data)
        .with(Style::psql())
        .with(Modify::new(Full).with(Alignment::left()))
        .with(Modify::new(Cell(1, 0)).with(Span::column(20)))
        .to_string();

    let expected = " N | column 0 | column 1 | column 2 \n---+----------+----------+----------\n 0 \n 1 | 1-0      | 1-1      | 1-2      \n 2 | 2-0      | 2-1      | 2-2      \n";

    assert_eq!(table, expected);
}

#[test]
fn span_multiline() {
    let mut data = create_vector::<3, 3>();
    data[2][2] = String::from("https://\nwww\n.\nredhat\n.com\n/en");

    let table = Table::new(&data)
        .with(Style::psql())
        .with(Modify::new(Cell(3, 2)).with(Span::column(2)))
        .to_string();

    let expected = concat!(
        " N | column 0 | column 1 | column 2 \n",
        "---+----------+----------+----------\n",
        " 0 |   0-0    |   0-1    |   0-2    \n",
        " 1 |   1-0    |   1-1    |   1-2    \n",
        " 2 |   2-0    |      https://       \n",
        "   |          |      www            \n",
        "   |          |      .              \n",
        "   |          |      redhat         \n",
        "   |          |      .com           \n",
        "   |          |      /en            \n",
    );

    assert_eq!(table, expected);
}

#[test]
fn indent_works_in_spaned_columns() {
    let data = create_vector::<3, 3>();

    let table = Table::new(&data)
        .with(Style::psql())
        .with(Modify::new(Full).with(Padding::new(3, 0, 0, 0)))
        .with(Modify::new(Full).with(Alignment::left()))
        .with(Modify::new(Cell(1, 1)).with(Span::column(3)))
        .with(Modify::new(Cell(3, 1)).with(Span::column(3)))
        .to_string();

    let expected = concat!(
        "   N|   column 0|   column 1|   column 2\n",
        "----+-----------+-----------+-----------\n",
        "   0|   0-0                             \n",
        "   1|   1-0     |   1-1     |   1-2     \n",
        "   2|   2-0                             \n",
    );

    assert_eq!(table, expected);
}

#[test]
fn spaned_columns_with_colision() {
    let data = [["just 1 column"; 5]; 5];

    let table = Table::new(data)
        .with(Style::modern())
        .with(
            Modify::new(Cell(0, 0))
                .with(Span::column(5))
                .with(|_: &str| "span all 5 columns".to_string()),
        )
        .with(
            Modify::new(Cell(1, 0))
                .with(Span::column(4))
                .with(|_: &str| "span 4 columns".to_string()),
        )
        .with(
            Modify::new(Cell(2, 0))
                .with(Span::column(3))
                .with(|_: &str| "span 3 columns".to_string()),
        )
        .with(
            Modify::new(Cell(2, 3))
                .with(Span::column(2))
                .with(|_: &str| "span 2 columns".to_string()),
        )
        .with(
            Modify::new(Cell(3, 0))
                .with(Span::column(2))
                .with(|_: &str| "span 3 columns".to_string()),
        )
        .with(
            Modify::new(Cell(3, 2))
                .with(Span::column(3))
                .with(|_: &str| "span 3 columns".to_string()),
        )
        .with(
            Modify::new(Cell(4, 1))
                .with(Span::column(4))
                .with(|_: &str| "span 4 columns".to_string()),
        )
        .to_string();

    let expected = concat!(
        "┌───────────────┬───────────────┬───────────────┬───────────────┬───────────────┐\n",
        "│                              span all 5 columns                               │\n",
        "├───────────────┼───────────────┼───────────────┼───────────────┼───────────────┤\n",
        "│                        span 4 columns                         │ just 1 column │\n",
        "├───────────────┼───────────────┼───────────────┼───────────────┼───────────────┤\n",
        "│                span 3 columns                 │        span 2 columns         │\n",
        "├───────────────┼───────────────┼───────────────┼───────────────┼───────────────┤\n",
        "│        span 3 columns         │                span 3 columns                 │\n",
        "├───────────────┼───────────────┼───────────────┼───────────────┼───────────────┤\n",
        "│ just 1 column │                        span 4 columns                         │\n",
        "├───────────────┼───────────────┼───────────────┼───────────────┼───────────────┤\n",
        "│ just 1 column │ just 1 column │ just 1 column │ just 1 column │ just 1 column │\n",
        "└───────────────┴───────────────┴───────────────┴───────────────┴───────────────┘\n",
    );

    assert_eq!(table, expected);
}
