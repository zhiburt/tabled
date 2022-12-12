use tabled::{
    object::{Cell, Columns, Segment},
    Alignment, Border, Highlight, Modify, Padding, Panel, Span, Style, Table,
};

use crate::util::{create_table, init_table, new_table, static_table, test_table};

mod util;

test_table!(
    span_column_test_0,
    create_table::<3, 3>()
        .with(Style::psql())
        .with(Modify::new(Segment::all()).with(Alignment::left()))
        .with(Modify::new(Columns::single(0)).with(Span::column(2))),
    " N | column 1 | column 2 "
    "-+-+----------+----------"
    " 0 | 0-1      | 0-2      "
    " 1 | 1-1      | 1-2      "
    " 2 | 2-1      | 2-2      "
);

test_table!(
    span_column_test_1,
    create_table::<3, 3>()
        .with(Style::psql())
        .with(Modify::new(Segment::all()).with(Alignment::left()))
        .with(Modify::new(Columns::new(1..2)).with(Span::column(2))),
    " N | column 0 | column 2 "
    "---+-----+----+----------"
    " 0 | 0-0      | 0-2      "
    " 1 | 1-0      | 1-2      "
    " 2 | 2-0      | 2-2      "
);

test_table!(
    span_column_test_2,
    create_table::<3, 3>()
        .with(Style::psql())
        .with(Modify::new(Segment::all()).with(Alignment::left()))
        .with(Modify::new(Columns::single(0)).with(Span::column(4))),
    " N "
    "+++"
    " 0 "
    " 1 "
    " 2 "
);

test_table!(
    cell_span_test_0,
    create_table::<3, 3>()
        .with(Style::psql())
        .with(Modify::new(Segment::all()).with(Alignment::left()))
        .with(Modify::new(Cell(0, 0)).with(Span::column(2))),
    " N       | column 1 | column 2 "
    "---+-----+----------+----------"
    " 0 | 0-0 | 0-1      | 0-2      "
    " 1 | 1-0 | 1-1      | 1-2      "
    " 2 | 2-0 | 2-1      | 2-2      "
);

test_table!(
    cell_span_test_1,
    create_table::<3, 3>()
        .with(Style::psql())
        .with(Modify::new(Segment::all()).with(Alignment::left()))
        .with(Modify::new(Cell(1, 0)).with(Span::column(2))),
    " N | column 0 | column 1 | column 2 "
    "---+----------+----------+----------"
    " 0            | 0-1      | 0-2      "
    " 1 | 1-0      | 1-1      | 1-2      "
    " 2 | 2-0      | 2-1      | 2-2      "
);

test_table!(
    cell_span_test_2,
    create_table::<3, 3>()
        .with(Style::psql())
        .with(Modify::new(Cell(2, 0)).with(Span::column(2))),
    " N | column 0 | column 1 | column 2 "
    "---+----------+----------+----------"
    " 0 |   0-0    |   0-1    |   0-2    "
    "      1       |   1-1    |   1-2    "
    " 2 |   2-0    |   2-1    |   2-2    "
);

test_table!(
    cell_span_test_3,
    create_table::<3, 3>()
        .with(Style::psql())
        .with(Modify::new(Segment::all()).with(Alignment::left()))
        .with(Modify::new(Cell(3, 0)).with(Span::column(2))),
    " N | column 0 | column 1 | column 2 "
    "---+----------+----------+----------"
    " 0 | 0-0      | 0-1      | 0-2      "
    " 1 | 1-0      | 1-1      | 1-2      "
    " 2            | 2-1      | 2-2      "
);

test_table!(
    cell_span_test_4,
    create_table::<3, 3>()
        .with(Style::psql())
        .with(Modify::new(Segment::all()).with(Alignment::left()))
        .with(Modify::new(Cell(0, 1)).with(Span::column(2))),
    " N | column 0  | column 2 "
    "---+-----+-----+----------"
    " 0 | 0-0 | 0-1 | 0-2      "
    " 1 | 1-0 | 1-1 | 1-2      "
    " 2 | 2-0 | 2-1 | 2-2      "
);

test_table!(
    cell_span_test_5,
    create_table::<3, 3>()
        .with(Style::psql())
        .with(Modify::new(Segment::all()).with(Alignment::left()))
        .with(Modify::new(Cell(1, 1)).with(Span::column(2))),
    " N | column 0 | column 1 | column 2 "
    "---+----------+----------+----------"
    " 0 | 0-0                 | 0-2      "
    " 1 | 1-0      | 1-1      | 1-2      "
    " 2 | 2-0      | 2-1      | 2-2      "
);

test_table!(
    cell_span_test_6,
    create_table::<3, 3>()
        .with(Style::psql())
        .with(Modify::new(Segment::all()).with(Alignment::left()))
        .with(Modify::new(Cell(2, 1)).with(Span::column(2))),
    " N | column 0 | column 1 | column 2 "
    "---+----------+----------+----------"
    " 0 | 0-0      | 0-1      | 0-2      "
    " 1 | 1-0                 | 1-2      "
    " 2 | 2-0      | 2-1      | 2-2      "
);

test_table!(
    cell_span_test_7,
    create_table::<3, 3>()
        .with(Style::psql())
        .with(Modify::new(Segment::all()).with(Alignment::left()))
        .with(Modify::new(Cell(3, 1)).with(Span::column(2))),
    " N | column 0 | column 1 | column 2 "
    "---+----------+----------+----------"
    " 0 | 0-0      | 0-1      | 0-2      "
    " 1 | 1-0      | 1-1      | 1-2      "
    " 2 | 2-0                 | 2-2      "
);

test_table!(
    cell_span_test_8,
    create_table::<3, 3>()
        .with(Style::psql())
        .with(Modify::new(Cell(0, 2)).with(Span::column(2))),
    " N | column 0 | column 1  "
    "---+----------+-----+-----"
    " 0 |   0-0    | 0-1 | 0-2 "
    " 1 |   1-0    | 1-1 | 1-2 "
    " 2 |   2-0    | 2-1 | 2-2 "
);

test_table!(
    cell_span_test_9,
    create_table::<3, 3>()
        .with(Style::psql())
        .with(Modify::new(Cell(1, 2)).with(Span::column(2))),
    " N | column 0 | column 1 | column 2 "
    "---+----------+----------+----------"
    " 0 |   0-0    |         0-1         "
    " 1 |   1-0    |   1-1    |   1-2    "
    " 2 |   2-0    |   2-1    |   2-2    "
);

test_table!(
    cell_span_test_10,
    create_table::<3, 3>()
        .with(Style::psql())
        .with(Modify::new(Cell(2, 2)).with(Span::column(2))),
    " N | column 0 | column 1 | column 2 "
    "---+----------+----------+----------"
    " 0 |   0-0    |   0-1    |   0-2    "
    " 1 |   1-0    |         1-1         "
    " 2 |   2-0    |   2-1    |   2-2    "
);

test_table!(
    cell_span_test_11,
    create_table::<3, 3>()
        .with(Style::psql())
        .with(Modify::new(Cell(3, 2)).with(Span::column(2))),
    " N | column 0 | column 1 | column 2 "
    "---+----------+----------+----------"
    " 0 |   0-0    |   0-1    |   0-2    "
    " 1 |   1-0    |   1-1    |   1-2    "
    " 2 |   2-0    |         2-1         "
);

test_table!(
    span_multiline,
    init_table::<3, 3, _, _>([((2, 2), "https://\nwww\n.\nredhat\n.com\n/en")])
        .with(Style::psql())
        .with(Modify::new(Cell(3, 2)).with(Span::column(2))),
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
);

test_table!(
    indent_works_in_spaned_columns,
    create_table::<3, 3>()
        .with(Style::psql())
        .with(Modify::new(Segment::all()).with(Padding::new(3, 0, 0, 0)))
        .with(Modify::new(Segment::all()).with(Alignment::left()))
        .with(Modify::new(Cell(1, 1)).with(Span::column(3)))
        .with(Modify::new(Cell(3, 1)).with(Span::column(3))),
    "   N|   column 0|   column 1|   column 2"
    "----+-----------+-----------+-----------"
    "   0|   0-0                             "
    "   1|   1-0     |   1-1     |   1-2     "
    "   2|   2-0                             "
);

test_table!(
    spaned_columns_with_collision,
    new_table([["just 1 column"; 5]; 5])
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
        ),
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
);

test_table!(
    span_with_panel_test_0,
    new_table([[1, 2, 3]])
        .with(Panel::horizontal(0).text("Tabled Releases"))
        .with(Modify::new(Cell(1, 0)).with(Span::column(2)))
        .with(Style::ascii()),
    "+-----+-----+-----+"
    "| Tabled Releases |"
    "+-----+-----+-----+"
    "|     0     |  2  |"
    "+-----+-----+-----+"
    "|  1  |  2  |  3  |"
    "+-----+-----+-----+"
);

test_table!(
    span_with_panel_test_1,
    new_table([[1, 2, 3], [4, 5, 6]])
        .with(Panel::horizontal(0).text("Tabled Releases"))
        .with(Modify::new(Cell(2, 0)).with(Span::column(2)))
        .with(Style::ascii()),
    "+-----+-----+-----+"
    "| Tabled Releases |"
    "+-----+-----+-----+"
    "|  0  |  1  |  2  |"
    "+-----+-----+-----+"
    "|     1     |  3  |"
    "+-----+-----+-----+"
    "|  4  |  5  |  6  |"
    "+-----+-----+-----+"
);

test_table!(
    span_with_panel_test_2,
    new_table([[1, 2, 3], [4, 5, 6]])
        .with(Panel::horizontal(0).text("Tabled Releases"))
        .with(Modify::new(Cell(1, 0)).with(Span::column(2)))
        .with(Modify::new(Cell(2, 0)).with(Span::column(2)))
        .with(Style::ascii()),
    "+-----+-----+-----+"
    "| Tabled Releases |"
    "+-----+-----+-----+"
    "|     0     |  2  |"
    "+-----+-----+-----+"
    "|     1     |  3  |"
    "+-----+-----+-----+"
    "|  4  |  5  |  6  |"
    "+-----+-----+-----+"
);

test_table!(
    span_with_panel_with_correction_test_0,
    new_table([[1, 2, 3]])
        .with(Panel::horizontal(0).text("Tabled Releases"))
        .with(Modify::new(Cell(1, 0)).with(Span::column(2)))
        .with(Style::ascii())
        .with(Style::correct_spans()),
    "+-----------------+"
    "| Tabled Releases |"
    "+-----------+-----+"
    "|     0     |  2  |"
    "+-----+-----+-----+"
    "|  1  |  2  |  3  |"
    "+-----+-----+-----+"
);

test_table!(
    span_with_panel_with_correction_test_1,
    new_table([[1, 2, 3], [4, 5, 6]])
        .with(Panel::horizontal(0).text("Tabled Releases"))
        .with(Modify::new(Cell(2, 0)).with(Span::column(2)))
        .with(Style::ascii())
        .with(Style::correct_spans()),
    "+-----------------+"
    "| Tabled Releases |"
    "+-----+-----+-----+"
    "|  0  |  1  |  2  |"
    "+-----+-----+-----+"
    "|     1     |  3  |"
    "+-----+-----+-----+"
    "|  4  |  5  |  6  |"
    "+-----+-----+-----+"
);

test_table!(
    span_with_panel_with_correction_test_2,
    new_table([[1, 2, 3], [4, 5, 6]])
        .with(Panel::horizontal(0).text("Tabled Releases"))
        .with(Modify::new(Cell(1, 0)).with(Span::column(2)))
        .with(Modify::new(Cell(2, 0)).with(Span::column(2)))
        .with(Style::ascii())
        .with(Style::correct_spans()),
    "+-----------------+"
    "| Tabled Releases |"
    "+-----------+-----+"
    "|     0     |  2  |"
    "+-----------+-----+"
    "|     1     |  3  |"
    "+-----+-----+-----+"
    "|  4  |  5  |  6  |"
    "+-----+-----+-----+"
);

#[test]
#[should_panic]
#[ignore = "span zero not yet decided"]
fn span_column_exceeds_boundaries_test() {
    // todo: determine if it's the right behaiviour

    create_table::<3, 3>()
        .with(Modify::new(Columns::single(0)).with(Span::column(100)))
        .to_string();
}

#[test]
#[ignore = "span zero not yet decided"]
fn span_cell_exceeds_boundaries_test() {
    // these tests shows that exiding boundaries causes invalid behaiviour
    //
    // todo: determine if it's the right behaiviour

    let table = create_table::<3, 3>()
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

    let table = create_table::<3, 3>()
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

    let table = create_table::<3, 3>()
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
#[ignore = "span zero not yet decided"]
fn span_zero_test() {
    let table = create_table::<3, 3>()
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

    let table = create_table::<3, 3>()
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

    let table = create_table::<3, 3>()
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

    let table = create_table::<3, 3>()
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

    let table = create_table::<3, 3>()
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

    let table = create_table::<3, 3>()
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
    let table = create_table::<2, 2>()
        .with(Style::psql())
        .with(Modify::new(Segment::all()).with(Span::column(0)))
        .to_string();

    // todo: determine whether it's correct
    assert_eq!(table, static_table!("\n++\n\n\n"));
}

mod row {
    use tabled::object::Rows;

    use super::*;

    #[test]
    fn span_row_test() {
        let table = create_table::<3, 3>();
        {
            // let table_str = table
            //     .clone()
            //     .with(Style::ascii())
            //     .with(Modify::new(Segment::all()).with(Alignment::left()))
            //     .with(Modify::new(Rows::single(0)).with(Span::row(2)))
            //     .to_string();

            // assert_eq!(
            //     table_str,
            //     static_table!(
            //         "+---+----------+----------+----------+"
            //         "+ N + column 0 + column 1 + column 2 +"
            //         "+---+----------+----------+----------+"
            //         "| 1 | 1-0      | 1-1      | 1-2      |"
            //         "+---+----------+----------+----------+"
            //         "| 2 | 2-0      | 2-1      | 2-2      |"
            //         "+---+----------+----------+----------+"
            //     )
            // );

            let table = table
                .clone()
                .with(Style::psql())
                .with(Modify::new(Segment::all()).with(Alignment::left()))
                .with(Modify::new(Rows::single(0)).with(Span::row(2)))
                .to_string();

            assert_eq!(
                table,
                static_table!(
                    " N + column 0 + column 1 + column 2 "
                    " 1 | 1-0      | 1-1      | 1-2      "
                    " 2 | 2-0      | 2-1      | 2-2      "
                )
            );
        }
        {
            let table = table
                .clone()
                .with(Style::psql())
                .with(Modify::new(Segment::all()).with(Alignment::left()))
                .with(Modify::new(Rows::new(1..2)).with(Span::row(2)))
                .to_string();

            assert_eq!(
                table,
                static_table!(
                    " N | column 0 | column 1 | column 2 "
                    "---+----------+----------+----------"
                    " 0 | 0-0      | 0-1      | 0-2      "
                    " 2 | 2-0      | 2-1      | 2-2      "
                )
            );
        }
        {
            let table = table
                .clone()
                .with(Style::psql())
                .with(Modify::new(Segment::all()).with(Alignment::left()))
                .with(Modify::new(Rows::single(0)).with(Span::row(4)))
                .to_string();

            assert_eq!(table, " N + column 0 + column 1 + column 2 ");
        }
    }

    #[test]
    fn cell_span_test() {
        let table = create_table::<3, 3>();
        {
            // first column cells row span = 2

            {
                let table = table
                    .clone()
                    .with(Style::psql())
                    .with(Modify::new(Segment::all()).with(Alignment::left()))
                    .with(Modify::new(Cell(0, 0)).with(Span::row(2)))
                    .to_string();

                assert_eq!(
                    table,
                    static_table!(
                        " N | column 0 | column 1 | column 2 "
                        "   +----------+----------+----------"
                        "   | 0-0      | 0-1      | 0-2      "
                        " 1 | 1-0      | 1-1      | 1-2      "
                        " 2 | 2-0      | 2-1      | 2-2      "
                    )
                );
            }
            {
                let table = table
                    .clone()
                    .with(Style::psql())
                    .with(Modify::new(Segment::all()).with(Alignment::left()))
                    .with(Modify::new(Cell(1, 0)).with(Span::row(2)))
                    .to_string();

                assert_eq!(
                    table,
                    static_table!(
                        " N | column 0 | column 1 | column 2 "
                        "---+----------+----------+----------"
                        " 0 | 0-0      | 0-1      | 0-2      "
                        "   | 1-0      | 1-1      | 1-2      "
                        " 2 | 2-0      | 2-1      | 2-2      "
                    )
                );
            }
            {
                let table = table
                    .clone()
                    .with(Style::psql())
                    .with(Modify::new(Cell(2, 0)).with(Span::row(2)))
                    .to_string();

                assert_eq!(
                    table,
                    static_table!(
                        " N | column 0 | column 1 | column 2 "
                        "---+----------+----------+----------"
                        " 0 |   0-0    |   0-1    |   0-2    "
                        " 1 |   1-0    |   1-1    |   1-2    "
                        "   |   2-0    |   2-1    |   2-2    "
                    )
                );
            }
        }

        {
            // first row cells row span = 2

            {
                let table = table
                    .clone()
                    .with(Style::psql())
                    .with(Modify::new(Segment::all()).with(Alignment::left()))
                    .with(Modify::new(Cell(0, 1)).with(Span::row(2)))
                    .to_string();

                assert_eq!(
                    table,
                    static_table!(
                        " N | column 0 | column 1 | column 2 "
                        "---+          +----------+----------"
                        " 0 |          | 0-1      | 0-2      "
                        " 1 | 1-0      | 1-1      | 1-2      "
                        " 2 | 2-0      | 2-1      | 2-2      "
                    )
                );
            }
            {
                let table = table
                    .clone()
                    .with(Style::psql())
                    .with(Modify::new(Segment::all()).with(Alignment::left()))
                    .with(Modify::new(Cell(0, 2)).with(Span::row(2)))
                    .to_string();

                assert_eq!(
                    table,
                    static_table!(
                        " N | column 0 | column 1 | column 2 "
                        "---+----------+          +----------"
                        " 0 | 0-0      |          | 0-2      "
                        " 1 | 1-0      | 1-1      | 1-2      "
                        " 2 | 2-0      | 2-1      | 2-2      "
                    )
                );
            }
            {
                let table = table
                    .clone()
                    .with(Style::psql())
                    .with(Modify::new(Cell(0, 3)).with(Span::row(2)))
                    .to_string();

                assert_eq!(
                    table,
                    static_table!(
                        " N | column 0 | column 1 | column 2 "
                        "---+----------+----------+          "
                        " 0 |   0-0    |   0-1    |          "
                        " 1 |   1-0    |   1-1    |   1-2    "
                        " 2 |   2-0    |   2-1    |   2-2    "
                    )
                );
            }
        }

        {
            // second column span=2
            {
                let table = table
                    .clone()
                    .with(Style::psql())
                    .with(Modify::new(Segment::all()).with(Alignment::left()))
                    .with(Modify::new(Cell(1, 1)).with(Span::row(2)))
                    .to_string();

                assert_eq!(
                    table,
                    static_table!(
                        " N | column 0 | column 1 | column 2 "
                        "---+----------+----------+----------"
                        " 0 | 0-0      | 0-1      | 0-2      "
                        " 1 |          | 1-1      | 1-2      "
                        " 2 | 2-0      | 2-1      | 2-2      "
                    )
                );
            }
            {
                let table = table
                    .clone()
                    .with(Style::psql())
                    .with(Modify::new(Segment::all()).with(Alignment::left()))
                    .with(Modify::new(Cell(2, 1)).with(Span::row(2)))
                    .to_string();

                assert_eq!(
                    table,
                    static_table!(
                        " N | column 0 | column 1 | column 2 "
                        "---+----------+----------+----------"
                        " 0 | 0-0      | 0-1      | 0-2      "
                        " 1 | 1-0      | 1-1      | 1-2      "
                        " 2 |          | 2-1      | 2-2      "
                    )
                );
            }
        }
        {
            // 3rd column span=2
            {
                let table = table
                    .clone()
                    .with(Style::psql())
                    .with(Modify::new(Segment::all()).with(Alignment::left()))
                    .with(Modify::new(Cell(1, 2)).with(Span::row(2)))
                    .to_string();

                assert_eq!(
                    table,
                    static_table!(
                        " N | column 0 | column 1 | column 2 "
                        "---+----------+----------+----------"
                        " 0 | 0-0      | 0-1      | 0-2      "
                        " 1 | 1-0      |          | 1-2      "
                        " 2 | 2-0      | 2-1      | 2-2      "
                    )
                );
            }
            {
                let table = table
                    .clone()
                    .with(Style::psql())
                    .with(Modify::new(Segment::all()).with(Alignment::left()))
                    .with(Modify::new(Cell(2, 2)).with(Span::row(2)))
                    .to_string();

                assert_eq!(
                    table,
                    static_table!(
                        " N | column 0 | column 1 | column 2 "
                        "---+----------+----------+----------"
                        " 0 | 0-0      | 0-1      | 0-2      "
                        " 1 | 1-0      | 1-1      | 1-2      "
                        " 2 | 2-0      |          | 2-2      "
                    )
                );
            }
        }
        {
            // 4th column span=2
            {
                let table = table
                    .clone()
                    .with(Style::psql())
                    .with(Modify::new(Segment::all()).with(Alignment::left()))
                    .with(Modify::new(Cell(1, 3)).with(Span::row(2)))
                    .to_string();

                assert_eq!(
                    table,
                    static_table!(
                        " N | column 0 | column 1 | column 2 "
                        "---+----------+----------+----------"
                        " 0 | 0-0      | 0-1      | 0-2      "
                        " 1 | 1-0      | 1-1      |          "
                        " 2 | 2-0      | 2-1      | 2-2      "
                    )
                );
            }
            {
                let table = table
                    .clone()
                    .with(Style::psql())
                    .with(Modify::new(Segment::all()).with(Alignment::left()))
                    .with(Modify::new(Cell(2, 3)).with(Span::row(2)))
                    .to_string();

                assert_eq!(
                    table,
                    static_table!(
                        " N | column 0 | column 1 | column 2 "
                        "---+----------+----------+----------"
                        " 0 | 0-0      | 0-1      | 0-2      "
                        " 1 | 1-0      | 1-1      | 1-2      "
                        " 2 | 2-0      | 2-1      |          "
                    )
                );
            }
        }
    }

    #[test]
    fn span_with_panel_with_correction_test() {
        let data = [[1, 2, 3]];
        let table = Table::new(data)
            .with(Modify::new(Cell(0, 0)).with(Span::row(2)))
            .with(Style::ascii())
            .with(Style::correct_spans())
            .to_string();

        assert_eq!(
            table,
            static_table!(
                "+---+---+---+"
                "| 0 | 1 | 2 |"
                "|   +---+---+"
                "|   | 2 | 3 |"
                "+---+---+---+"
            )
        );

        let data = [[1, 2, 3], [4, 5, 6]];
        let table = Table::new(data)
            .with(Modify::new(Cell(1, 0)).with(Span::row(2)))
            .with(Modify::new(Cell(0, 2)).with(Span::row(3)))
            .with(Style::ascii())
            .with(Style::correct_spans())
            .to_string();

        assert_eq!(
            table,
            static_table!(
                "+---+---+---+"
                "| 0 | 1 | 2 |"
                "+---+---+   |"
                "| 1 | 2 |   |"
                "|   +---+   |"
                "|   | 5 |   |"
                "+---+---+---+"
            )
        );

        let data = [[1, 2, 3], [4, 5, 6]];
        let table = Table::new(data)
            .with(Modify::new(Cell(1, 0)).with(Span::row(2)))
            .with(Modify::new(Cell(0, 2)).with(Span::row(3)))
            .with(Modify::new(Cell(0, 1)).with(Span::row(2)))
            .with(Style::ascii())
            .with(Style::correct_spans())
            .to_string();

        assert_eq!(
            table,
            static_table!(
                "+---+---+---+"
                "| 0 | 1 | 2 |"
                "+---+   |   |"
                "| 1 +---+   |"
                "|   | 5 |   |"
                "+---+---+---+"
            )
        );

        let data = [[1, 2, 3], [4, 5, 6]];
        let table = Table::new(data)
            .with(Modify::new(Cell(1, 0)).with(Span::row(2)))
            .with(
                Modify::new(Cell(0, 1))
                    .with(Span::row(2))
                    .with(Span::column(2)),
            )
            .with(Style::ascii())
            .with(Style::correct_spans())
            .to_string();

        assert_eq!(
            table,
            static_table!(
                "+---+-------+"
                "| 0 | 1     |"
                "+---+       +"
                "| 1 +---+---+"
                "|   | 5 | 6 |"
                "+---+---+---+"
            )
        );
    }

    #[test]
    fn span_example_test() {
        let data = [["just 1 column"; 5]; 5];

        let h_span = |r, c, span| Modify::new(Cell(r, c)).with(Span::column(span));
        let v_span = |r, c, span| Modify::new(Cell(r, c)).with(Span::row(span));

        let table = Table::new(data)
            .with(h_span(0, 0, 5).with(String::from("span all 5 columns")))
            .with(h_span(1, 0, 4).with(String::from("span 4 columns")))
            .with(h_span(2, 0, 2).with(String::from("span 2 columns")))
            .with(v_span(2, 4, 4).with(String::from("just 1 column\nspan\n4\ncolumns")))
            .with(v_span(3, 1, 2).with(String::from("span 2 columns\nspan\n2\ncolumns")))
            .with(v_span(2, 3, 3).with(String::from("just 1 column\nspan\n3\ncolumns")))
            .with(h_span(3, 1, 2))
            .with(Style::modern())
            .with(Style::correct_spans())
            .with(Modify::new(Segment::all()).with(Alignment::center_vertical()))
            .to_string();

        assert_eq!(
            table,
            static_table!(
                "┌───────────────────────────────────────────────────────────────────────────────┐"
                "│ span all 5 columns                                                            │"
                "├───────────────────────────────────────────────────────────────┬───────────────┤"
                "│ span 4 columns                                                │ just 1 column │"
                "├───────────────────────────────┬───────────────┬───────────────┼───────────────┤"
                "│ span 2 columns                │ just 1 column │               │               │"
                "├───────────────┬───────────────┴───────────────┤ just 1 column │               │"
                "│ just 1 column │ span 2 columns                │ span          │ just 1 column │"
                "│               │ span                          │ 3             │ span          │"
                "├───────────────┤ 2                             │ columns       │ 4             │"
                "│ just 1 column │ columns                       │               │ columns       │"
                "├───────────────┼───────────────┬───────────────┼───────────────┤               │"
                "│ just 1 column │ just 1 column │ just 1 column │ just 1 column │               │"
                "└───────────────┴───────────────┴───────────────┴───────────────┴───────────────┘"
            )
        )
    }

    #[test]
    fn highlight_row_span_test() {
        let data = [
            ["1", "2\n2\n2\n2\n2\n2\n2\n2", "3"],
            ["4", "5", "6"],
            ["7", "8", "9"],
        ];
        let table = Table::new(data)
            .with(Modify::new(Cell(1, 1)).with(Span::row(3)))
            .with(Style::modern())
            .with(Highlight::new(Columns::single(1), Border::filled('*')))
            .to_string();

        assert_eq!(
            table,
            static_table!(
                "┌───*****───┐"
                "│ 0 * 1 * 2 │"
                "├───*───*───┤"
                "│ 1 * 2 * 3 │"
                "│   * 2 *   │"
                "├───* 2 *───┤"
                "│ 4 * 2 * 6 │"
                "│   * 2 *   │"
                "├───* 2 *───┤"
                "│ 7 * 2 * 9 │"
                "│   * 2 *   │"
                "└───*****───┘"
            )
        );
    }
}

#[test]
fn highlight_row_col_span_test() {
    let data = [
        ["1", "2\n2\n2\n2\n2\n2\n2\n2", "3", "0"],
        ["4", "5", "6", "0"],
        ["7", "8", "9", "0"],
    ];
    let table = Table::new(data)
        .with(
            Modify::new(Cell(1, 1))
                .with(Span::row(3))
                .with(Span::column(2)),
        )
        .with(Style::modern())
        .with(Highlight::new(Columns::new(1..3), Border::filled('*')))
        .to_string();

    assert_eq!(
        table,
        static_table!(
            "┌───*********───┐"
            "│ 0 * 1 │ 2 * 3 │"
            "├───*───┼───*───┤"
            "│ 1 * 2     * 0 │"
            "│   * 2     *   │"
            "├───* 2     *───┤"
            "│ 4 * 2     * 0 │"
            "│   * 2     *   │"
            "├───* 2     *───┤"
            "│ 7 * 2     * 0 │"
            "│   * 2     *   │"
            "└───*********───┘"
        )
    );
}

test_table!(
    column_span_bigger_then_max,
    create_table::<3, 3>().with(Modify::new(Cell(0, 0)).with(Span::column(100))),
    "+---+----------+----------+----------+"
    "| N | column 0 | column 1 | column 2 |"
    "+---+----------+----------+----------+"
    "| 0 |   0-0    |   0-1    |   0-2    |"
    "+---+----------+----------+----------+"
    "| 1 |   1-0    |   1-1    |   1-2    |"
    "+---+----------+----------+----------+"
    "| 2 |   2-0    |   2-1    |   2-2    |"
    "+---+----------+----------+----------+"
);

test_table!(
    row_span_bigger_then_max,
    create_table::<3, 3>().with(Modify::new(Cell(0, 0)).with(Span::row(100))),
    "+---+----------+----------+----------+"
    "| N | column 0 | column 1 | column 2 |"
    "+---+----------+----------+----------+"
    "| 0 |   0-0    |   0-1    |   0-2    |"
    "+---+----------+----------+----------+"
    "| 1 |   1-0    |   1-1    |   1-2    |"
    "+---+----------+----------+----------+"
    "| 2 |   2-0    |   2-1    |   2-2    |"
    "+---+----------+----------+----------+"
);

test_table!(
    column_span_invalid_position_row,
    create_table::<3, 3>().with(Modify::new(Cell(1000, 0)).with(Span::column(2))),
    "+---+----------+----------+----------+"
    "| N | column 0 | column 1 | column 2 |"
    "+---+----------+----------+----------+"
    "| 0 |   0-0    |   0-1    |   0-2    |"
    "+---+----------+----------+----------+"
    "| 1 |   1-0    |   1-1    |   1-2    |"
    "+---+----------+----------+----------+"
    "| 2 |   2-0    |   2-1    |   2-2    |"
    "+---+----------+----------+----------+"
);

test_table!(
    column_span_invalid_position_column,
    create_table::<3, 3>().with(Modify::new(Cell(0, 1000)).with(Span::column(2))),
    "+---+----------+----------+----------+"
    "| N | column 0 | column 1 | column 2 |"
    "+---+----------+----------+----------+"
    "| 0 |   0-0    |   0-1    |   0-2    |"
    "+---+----------+----------+----------+"
    "| 1 |   1-0    |   1-1    |   1-2    |"
    "+---+----------+----------+----------+"
    "| 2 |   2-0    |   2-1    |   2-2    |"
    "+---+----------+----------+----------+"
);

test_table!(
    column_span_invalid_position_row_and_column,
    create_table::<3, 3>().with(Modify::new(Cell(1000, 1000)).with(Span::column(2))),
    "+---+----------+----------+----------+"
    "| N | column 0 | column 1 | column 2 |"
    "+---+----------+----------+----------+"
    "| 0 |   0-0    |   0-1    |   0-2    |"
    "+---+----------+----------+----------+"
    "| 1 |   1-0    |   1-1    |   1-2    |"
    "+---+----------+----------+----------+"
    "| 2 |   2-0    |   2-1    |   2-2    |"
    "+---+----------+----------+----------+"
);

test_table!(
    row_span_invalid_position_row,
    create_table::<3, 3>().with(Modify::new(Cell(1000, 0)).with(Span::row(2))),
    "+---+----------+----------+----------+"
    "| N | column 0 | column 1 | column 2 |"
    "+---+----------+----------+----------+"
    "| 0 |   0-0    |   0-1    |   0-2    |"
    "+---+----------+----------+----------+"
    "| 1 |   1-0    |   1-1    |   1-2    |"
    "+---+----------+----------+----------+"
    "| 2 |   2-0    |   2-1    |   2-2    |"
    "+---+----------+----------+----------+"
);

test_table!(
    row_span_invalid_position_column,
    create_table::<3, 3>().with(Modify::new(Cell(0, 1000)).with(Span::row(2))),
    "+---+----------+----------+----------+"
    "| N | column 0 | column 1 | column 2 |"
    "+---+----------+----------+----------+"
    "| 0 |   0-0    |   0-1    |   0-2    |"
    "+---+----------+----------+----------+"
    "| 1 |   1-0    |   1-1    |   1-2    |"
    "+---+----------+----------+----------+"
    "| 2 |   2-0    |   2-1    |   2-2    |"
    "+---+----------+----------+----------+"
);

test_table!(
    row_span_invalid_position_row_and_column,
    create_table::<3, 3>().with(Modify::new(Cell(1000, 1000)).with(Span::row(2))),
    "+---+----------+----------+----------+"
    "| N | column 0 | column 1 | column 2 |"
    "+---+----------+----------+----------+"
    "| 0 |   0-0    |   0-1    |   0-2    |"
    "+---+----------+----------+----------+"
    "| 1 |   1-0    |   1-1    |   1-2    |"
    "+---+----------+----------+----------+"
    "| 2 |   2-0    |   2-1    |   2-2    |"
    "+---+----------+----------+----------+"
);
