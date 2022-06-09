use tabled::{
    object::{Cell, Columns, Segment},
    Alignment, Modify, Padding, Panel, Span, Style, Table,
};

use crate::util::{create_vector, static_table};

mod util;

#[test]
fn span_column_test() {
    let data = create_vector::<3, 3>();
    {
        let table = Table::new(&data)
            .with(Style::psql())
            .with(Modify::new(Segment::all()).with(Alignment::left()))
            .with(Modify::new(Columns::single(0)).with(Span::column(2)))
            .to_string();

        assert_eq!(
            table,
            static_table!(
                " N | column 1 | column 2 "
                "-+-+----------+----------"
                " 0 | 0-1      | 0-2      "
                " 1 | 1-1      | 1-2      "
                " 2 | 2-1      | 2-2      "
            )
        );
    }
    {
        let table = Table::new(&data)
            .with(Style::psql())
            .with(Modify::new(Segment::all()).with(Alignment::left()))
            .with(Modify::new(Columns::new(1..2)).with(Span::column(2)))
            .to_string();

        assert_eq!(
            table,
            static_table!(
                " N | column 0 | column 2 "
                "---+-----+----+----------"
                " 0 | 0-0      | 0-2      "
                " 1 | 1-0      | 1-2      "
                " 2 | 2-0      | 2-2      "
            )
        );
    }
    {
        let table = Table::new(&data)
            .with(Style::psql())
            .with(Modify::new(Segment::all()).with(Alignment::left()))
            .with(Modify::new(Columns::single(0)).with(Span::column(data.len() + 1)))
            .to_string();

        assert_eq!(table, static_table!(" N " "+++" " 0 " " 1 " " 2 "));
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
                .with(Modify::new(Segment::all()).with(Alignment::left()))
                .with(Modify::new(Cell(0, 0)).with(Span::column(2)))
                .to_string();

            assert_eq!(
                table,
                static_table!(
                    " N       | column 1 | column 2 "
                    "---+-----+----------+----------"
                    " 0 | 0-0 | 0-1      | 0-2      "
                    " 1 | 1-0 | 1-1      | 1-2      "
                    " 2 | 2-0 | 2-1      | 2-2      "
                )
            );
        }
        {
            let table = Table::new(&data)
                .with(Style::psql())
                .with(Modify::new(Segment::all()).with(Alignment::left()))
                .with(Modify::new(Cell(1, 0)).with(Span::column(2)))
                .to_string();

            assert_eq!(
                table,
                static_table!(
                    " N | column 0 | column 1 | column 2 "
                    "---+----------+----------+----------"
                    " 0            | 0-1      | 0-2      "
                    " 1 | 1-0      | 1-1      | 1-2      "
                    " 2 | 2-0      | 2-1      | 2-2      "
                )
            );
        }
        {
            let table = Table::new(&data)
                .with(Style::psql())
                .with(Modify::new(Cell(2, 0)).with(Span::column(2)))
                .to_string();

            assert_eq!(
                table,
                static_table!(
                    " N | column 0 | column 1 | column 2 "
                    "---+----------+----------+----------"
                    " 0 |   0-0    |   0-1    |   0-2    "
                    "      1       |   1-1    |   1-2    "
                    " 2 |   2-0    |   2-1    |   2-2    "
                )
            );
        }
        {
            let table = Table::new(&data)
                .with(Style::psql())
                .with(Modify::new(Segment::all()).with(Alignment::left()))
                .with(Modify::new(Cell(3, 0)).with(Span::column(2)))
                .to_string();

            assert_eq!(
                table,
                static_table!(
                    " N | column 0 | column 1 | column 2 "
                    "---+----------+----------+----------"
                    " 0 | 0-0      | 0-1      | 0-2      "
                    " 1 | 1-0      | 1-1      | 1-2      "
                    " 2            | 2-1      | 2-2      "
                )
            );
        }
    }

    {
        // second column span=2
        {
            let table = Table::new(&data)
                .with(Style::psql())
                .with(Modify::new(Segment::all()).with(Alignment::left()))
                .with(Modify::new(Cell(0, 1)).with(Span::column(2)))
                .to_string();

            assert_eq!(
                table,
                static_table!(
                    " N | column 0  | column 2 "
                    "---+-----+-----+----------"
                    " 0 | 0-0 | 0-1 | 0-2      "
                    " 1 | 1-0 | 1-1 | 1-2      "
                    " 2 | 2-0 | 2-1 | 2-2      "
                )
            );
        }
        {
            let table = Table::new(&data)
                .with(Style::psql())
                .with(Modify::new(Segment::all()).with(Alignment::left()))
                .with(Modify::new(Cell(1, 1)).with(Span::column(2)))
                .to_string();

            assert_eq!(
                table,
                static_table!(
                    " N | column 0 | column 1 | column 2 "
                    "---+----------+----------+----------"
                    " 0 | 0-0                 | 0-2      "
                    " 1 | 1-0      | 1-1      | 1-2      "
                    " 2 | 2-0      | 2-1      | 2-2      "
                )
            );
        }
        {
            let table = Table::new(&data)
                .with(Style::psql())
                .with(Modify::new(Segment::all()).with(Alignment::left()))
                .with(Modify::new(Cell(2, 1)).with(Span::column(2)))
                .to_string();

            assert_eq!(
                table,
                static_table!(
                    " N | column 0 | column 1 | column 2 "
                    "---+----------+----------+----------"
                    " 0 | 0-0      | 0-1      | 0-2      "
                    " 1 | 1-0                 | 1-2      "
                    " 2 | 2-0      | 2-1      | 2-2      "
                )
            );
        }
        {
            let table = Table::new(&data)
                .with(Style::psql())
                .with(Modify::new(Segment::all()).with(Alignment::left()))
                .with(Modify::new(Cell(3, 1)).with(Span::column(2)))
                .to_string();

            assert_eq!(
                table,
                static_table!(
                    " N | column 0 | column 1 | column 2 "
                    "---+----------+----------+----------"
                    " 0 | 0-0      | 0-1      | 0-2      "
                    " 1 | 1-0      | 1-1      | 1-2      "
                    " 2 | 2-0                 | 2-2      "
                )
            );
        }
    }

    {
        // 3rd column span = 1

        {
            let table = Table::new(&data)
                .with(Style::psql())
                .with(Modify::new(Cell(0, 2)).with(Span::column(2)))
                .to_string();

            assert_eq!(
                table,
                static_table!(
                    " N | column 0 | column 1  "
                    "---+----------+-----+-----"
                    " 0 |   0-0    | 0-1 | 0-2 "
                    " 1 |   1-0    | 1-1 | 1-2 "
                    " 2 |   2-0    | 2-1 | 2-2 "
                )
            );
        }
        {
            let table = Table::new(&data)
                .with(Style::psql())
                .with(Modify::new(Cell(1, 2)).with(Span::column(2)))
                .to_string();

            assert_eq!(
                table,
                static_table!(
                    " N | column 0 | column 1 | column 2 "
                    "---+----------+----------+----------"
                    " 0 |   0-0    |         0-1         "
                    " 1 |   1-0    |   1-1    |   1-2    "
                    " 2 |   2-0    |   2-1    |   2-2    "
                )
            );
        }
        {
            let table = Table::new(&data)
                .with(Style::psql())
                .with(Modify::new(Cell(2, 2)).with(Span::column(2)))
                .to_string();

            assert_eq!(
                table,
                static_table!(
                    " N | column 0 | column 1 | column 2 "
                    "---+----------+----------+----------"
                    " 0 |   0-0    |   0-1    |   0-2    "
                    " 1 |   1-0    |         1-1         "
                    " 2 |   2-0    |   2-1    |   2-2    "
                )
            );
        }
        {
            let table = Table::new(&data)
                .with(Style::psql())
                .with(Modify::new(Cell(3, 2)).with(Span::column(2)))
                .to_string();

            assert_eq!(
                table,
                static_table!(
                    " N | column 0 | column 1 | column 2 "
                    "---+----------+----------+----------"
                    " 0 |   0-0    |   0-1    |   0-2    "
                    " 1 |   1-0    |   1-1    |   1-2    "
                    " 2 |   2-0    |         2-1         "
                )
            );
        }
    }
}

#[test]
#[should_panic]
#[ignore = "span zero not yet decided"]
fn span_column_exceeds_boundries_test() {
    // todo: determine if it's the right behaiviour

    let data = create_vector::<3, 3>();
    Table::new(&data)
        .with(Modify::new(Columns::single(0)).with(Span::column(100)))
        .to_string();
}

#[test]
#[ignore = "span zero not yet decided"]
fn span_cell_exceeds_boundries_test() {
    // these tests shows that exiding boundaries causes invalid behaiviour
    //
    // todo: determine if it's the right behaiviour

    let data = create_vector::<3, 3>();
    let table = Table::new(&data)
        .with(Style::psql())
        .with(Modify::new(Segment::all()).with(Alignment::left()))
        .with(Modify::new(Cell(0, 0)).with(Span::column(20)))
        .to_string();

    assert_eq!(
        table,
        static_table!(
            " N "
            "---+-----+-----+-----"
            " 0 | 0-0 | 0-1 | 0-2 "
            " 1 | 1-0 | 1-1 | 1-2 "
            " 2 | 2-0 | 2-1 | 2-2 "
        )
    );

    let data = create_vector::<3, 3>();
    let table = Table::new(&data)
        .with(Style::psql())
        .with(Modify::new(Segment::all()).with(Alignment::left()))
        .with(Modify::new(Cell(1, 1)).with(Span::column(20)))
        .to_string();

    assert_eq!(
        table,
        static_table!(
            " N | column 0 | column 1 | column 2 "
            "---+----------+----------+----------"
            " 0 | 0-0 "
            " 1 | 1-0      | 1-1      | 1-2      "
            " 2 | 2-0      | 2-1      | 2-2      "
        )
    );

    let data = create_vector::<3, 3>();
    let table = Table::new(&data)
        .with(Style::psql())
        .with(Modify::new(Segment::all()).with(Alignment::left()))
        .with(Modify::new(Cell(1, 0)).with(Span::column(20)))
        .to_string();

    assert_eq!(
        table,
        static_table!(
            " N | column 0 | column 1 | column 2 "
            "---+----------+----------+----------"
            " 0 "
            " 1 | 1-0      | 1-1      | 1-2      "
            " 2 | 2-0      | 2-1      | 2-2      "
        )
    );
}

#[test]
fn span_multiline() {
    let mut data = create_vector::<3, 3>();
    data[2][2] = String::from("https://\nwww\n.\nredhat\n.com\n/en");

    let table = Table::new(&data)
        .with(Style::psql())
        .with(Modify::new(Cell(3, 2)).with(Span::column(2)))
        .to_string();

    assert_eq!(
        table,
        static_table!(
            " N | column 0 | column 1 | column 2 "
            "---+----------+----------+----------"
            " 0 |   0-0    |   0-1    |   0-2    "
            " 1 |   1-0    |   1-1    |   1-2    "
            " 2 |   2-0    |      https://       "
            "   |          |      www            "
            "   |          |      .              "
            "   |          |      redhat         "
            "   |          |      .com           "
            "   |          |      /en            "
        )
    );
}

#[test]
fn indent_works_in_spaned_columns() {
    let data = create_vector::<3, 3>();

    let table = Table::new(&data)
        .with(Style::psql())
        .with(Modify::new(Segment::all()).with(Padding::new(3, 0, 0, 0)))
        .with(Modify::new(Segment::all()).with(Alignment::left()))
        .with(Modify::new(Cell(1, 1)).with(Span::column(3)))
        .with(Modify::new(Cell(3, 1)).with(Span::column(3)))
        .to_string();

    assert_eq!(
        table,
        static_table!(
            "   N|   column 0|   column 1|   column 2"
            "----+-----------+-----------+-----------"
            "   0|   0-0                             "
            "   1|   1-0     |   1-1     |   1-2     "
            "   2|   2-0                             "
        )
    );
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

    assert_eq!(
        table,
        static_table!(
            "┌───────────────┬───────────────┬───────────────┬───────────────┬───────────────┐"
            "│                              span all 5 columns                               │"
            "├───────────────┼───────────────┼───────────────┼───────────────┼───────────────┤"
            "│                        span 4 columns                         │ just 1 column │"
            "├───────────────┼───────────────┼───────────────┼───────────────┼───────────────┤"
            "│                span 3 columns                 │        span 2 columns         │"
            "├───────────────┼───────────────┼───────────────┼───────────────┼───────────────┤"
            "│        span 3 columns         │                span 3 columns                 │"
            "├───────────────┼───────────────┼───────────────┼───────────────┼───────────────┤"
            "│ just 1 column │                        span 4 columns                         │"
            "├───────────────┼───────────────┼───────────────┼───────────────┼───────────────┤"
            "│ just 1 column │ just 1 column │ just 1 column │ just 1 column │ just 1 column │"
            "└───────────────┴───────────────┴───────────────┴───────────────┴───────────────┘"
        )
    );
}

#[test]
#[ignore = "span zero not yet decided"]
fn span_zero_test() {
    let data = create_vector::<3, 3>();

    let table = Table::new(&data)
        .with(Style::psql())
        .with(Modify::new(Cell(0, 0)).with(Span::column(0)))
        .to_string();

    assert_eq!(
        table,
        static_table!(
            " column 0 | column 1 | column 2 "
            "----+-----+----------+----------"
            " 0  | 0-0 |   0-1    |   0-2    "
            " 1  | 1-0 |   1-1    |   1-2    "
            " 2  | 2-0 |   2-1    |   2-2    "
        )
    );

    let table = Table::new(&data)
        .with(Style::psql())
        .with(Modify::new(Cell(0, 1)).with(Span::column(0)))
        .to_string();

    assert_eq!(
        table,
        static_table!(
            "    N    | column 1 | column 2 "
            "---+-----+----------+----------"
            " 0 | 0-0 |   0-1    |   0-2    "
            " 1 | 1-0 |   1-1    |   1-2    "
            " 2 | 2-0 |   2-1    |   2-2    "
        )
    );

    let table = Table::new(&data)
        .with(Style::psql())
        .with(Modify::new(Cell(0, 2)).with(Span::column(0)))
        .to_string();

    assert_eq!(
        table,
        static_table!(
            " N | column 0  | column 2 "
            "---+-----+-----+----------"
            " 0 | 0-0 | 0-1 |   0-2    "
            " 1 | 1-0 | 1-1 |   1-2    "
            " 2 | 2-0 | 2-1 |   2-2    "
        )
    );

    let table = Table::new(&data)
        .with(Style::psql())
        .with(Modify::new(Cell(0, 3)).with(Span::column(0)))
        .to_string();

    assert_eq!(
        table,
        static_table!(
            " N | column 0 | column 1  "
            "---+----------+-----+-----"
            " 0 |   0-0    | 0-1 | 0-2 "
            " 1 |   1-0    | 1-1 | 1-2 "
            " 2 |   2-0    | 2-1 | 2-2 "
        )
    );

    let table = Table::new(&data)
        .with(Style::psql())
        .with(Modify::new(Cell(0, 4)).with(Span::column(0)))
        .to_string();

    assert_eq!(
        table,
        static_table!(
            " N | column 0 | column 1 | column 2 "
            "---+----------+----------+----------"
            " 0 |   0-0    |   0-1    |   0-2    "
            " 1 |   1-0    |   1-1    |   1-2    "
            " 2 |   2-0    |   2-1    |   2-2    "
        )
    );

    let table = Table::new(&data)
        .with(Style::psql())
        .with(Modify::new(Cell(0, 0)).with(Span::column(0)))
        .with(Modify::new(Cell(1, 1)).with(Span::column(0)))
        .with(Modify::new(Cell(2, 2)).with(Span::column(0)))
        .with(Modify::new(Cell(3, 2)).with(Span::column(0)))
        .with(Modify::new(Cell(3, 1)).with(Span::column(0)))
        .to_string();

    assert_eq!(
        table,
        static_table!(
            " column 0 | column 1 | column 2 "
            "------+-------+------+----------"
            "    0     |   0-1    |   0-2    "
            "  1   |     1-0      |   1-2    "
            "          2          |   2-2    "
        )
    );
}

#[test]
#[ignore = "span zero not yet decided"]
fn span_all_table_to_zero_test() {
    let data = create_vector::<2, 2>();

    let table = Table::new(&data)
        .with(Style::psql())
        .with(Modify::new(Segment::all()).with(Span::column(0)))
        .to_string();

    // todo: determine whether it's correct
    assert_eq!(table, static_table!("\n++\n\n\n"));
}

#[test]
fn span_with_panel_test() {
    let data = [[1, 2, 3]];
    let table = Table::new(data)
        .with(Panel("Tabled Releases", 0))
        .with(Modify::new(Cell(1, 0)).with(Span::column(2)))
        .with(Style::ascii())
        .to_string();

    assert_eq!(
        table,
        static_table!(
            "+-----+----+----+"
            "|Tabled Releases|"
            "+-----+----+----+"
            "|    0     | 2  |"
            "+-----+----+----+"
            "|  1  | 2  | 3  |"
            "+-----+----+----+"
        )
    );

    let data = [[1, 2, 3], [4, 5, 6]];
    let table = Table::new(data)
        .with(Panel("Tabled Releases", 0))
        .with(Modify::new(Cell(2, 0)).with(Span::column(2)))
        .with(Style::ascii())
        .to_string();

    assert_eq!(
        table,
        static_table!(
            "+-----+----+----+"
            "|Tabled Releases|"
            "+-----+----+----+"
            "|  0  | 1  | 2  |"
            "+-----+----+----+"
            "|    1     | 3  |"
            "+-----+----+----+"
            "|  4  | 5  | 6  |"
            "+-----+----+----+"
        )
    );

    let data = [[1, 2, 3], [4, 5, 6]];
    let table = Table::new(data)
        .with(Panel("Tabled Releases", 0))
        .with(Modify::new(Cell(1, 0)).with(Span::column(2)))
        .with(Modify::new(Cell(2, 0)).with(Span::column(2)))
        .with(Style::ascii())
        .to_string();

    assert_eq!(
        table,
        static_table!(
            "+-----+----+----+"
            "|Tabled Releases|"
            "+-----+----+----+"
            "|    0     | 2  |"
            "+-----+----+----+"
            "|    1     | 3  |"
            "+-----+----+----+"
            "|  4  | 5  | 6  |"
            "+-----+----+----+"
        )
    );
}

#[test]
fn span_with_panel_with_correction_test() {
    let data = [[1, 2, 3]];
    let table = Table::new(data)
        .with(Panel("Tabled Releases", 0))
        .with(Modify::new(Cell(1, 0)).with(Span::column(2)))
        .with(Style::ascii())
        .with(Style::correct_spans())
        .to_string();

    assert_eq!(
        table,
        static_table!(
            "+---------------+"
            "|Tabled Releases|"
            "+----------+----+"
            "|    0     | 2  |"
            "+-----+----+----+"
            "|  1  | 2  | 3  |"
            "+-----+----+----+"
        )
    );

    let data = [[1, 2, 3], [4, 5, 6]];
    let table = Table::new(data)
        .with(Panel("Tabled Releases", 0))
        .with(Modify::new(Cell(2, 0)).with(Span::column(2)))
        .with(Style::ascii())
        .with(Style::correct_spans())
        .to_string();

    assert_eq!(
        table,
        static_table!(
            "+---------------+"
            "|Tabled Releases|"
            "+-----+----+----+"
            "|  0  | 1  | 2  |"
            "+-----+----+----+"
            "|    1     | 3  |"
            "+-----+----+----+"
            "|  4  | 5  | 6  |"
            "+-----+----+----+"
        )
    );

    let data = [[1, 2, 3], [4, 5, 6]];
    let table = Table::new(data)
        .with(Panel("Tabled Releases", 0))
        .with(Modify::new(Cell(1, 0)).with(Span::column(2)))
        .with(Modify::new(Cell(2, 0)).with(Span::column(2)))
        .with(Style::ascii())
        .with(Style::correct_spans())
        .to_string();

    assert_eq!(
        table,
        static_table!(
            "+---------------+"
            "|Tabled Releases|"
            "+----------+----+"
            "|    0     | 2  |"
            "+----------+----+"
            "|    1     | 3  |"
            "+-----+----+----+"
            "|  4  | 5  | 6  |"
            "+-----+----+----+"
        )
    );
}
