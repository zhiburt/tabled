#![cfg(feature = "std")]
#![allow(clippy::redundant_clone)]

use std::iter::FromIterator;

use tabled::{
    builder::Builder,
    grid::config::Position,
    settings::{
        object::{Columns, Segment},
        style::{Border, BorderSpanCorrection, Style},
        Alignment, Highlight, Modify, Padding, Panel, Span,
    },
    Table,
};

use crate::matrix::Matrix;
use testing_table::{static_table, test_table};

test_table!(
    span_column_test_0,
    Matrix::new(3, 3)
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
    Matrix::new(3, 3)
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
    Matrix::new(3, 3)
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
    Matrix::new(3, 3)
        .with(Style::psql())
        .with(Modify::new(Segment::all()).with(Alignment::left()))
        .with(Modify::new((0, 0)).with(Span::column(2))),
    " N       | column 1 | column 2 "
    "---+-----+----------+----------"
    " 0 | 0-0 | 0-1      | 0-2      "
    " 1 | 1-0 | 1-1      | 1-2      "
    " 2 | 2-0 | 2-1      | 2-2      "
);

test_table!(
    cell_span_test_1,
    Matrix::new(3, 3)
        .with(Style::psql())
        .with(Modify::new(Segment::all()).with(Alignment::left()))
        .with(Modify::new((1, 0)).with(Span::column(2))),
    " N | column 0 | column 1 | column 2 "
    "---+----------+----------+----------"
    " 0            | 0-1      | 0-2      "
    " 1 | 1-0      | 1-1      | 1-2      "
    " 2 | 2-0      | 2-1      | 2-2      "
);

test_table!(
    cell_span_test_2,
    Matrix::new(3, 3)
        .with(Style::psql())
        .with(Modify::new((2, 0)).with(Span::column(2))),
    " N | column 0 | column 1 | column 2 "
    "---+----------+----------+----------"
    " 0 |   0-0    |   0-1    |   0-2    "
    "      1       |   1-1    |   1-2    "
    " 2 |   2-0    |   2-1    |   2-2    "
);

test_table!(
    cell_span_test_3,
    Matrix::new(3, 3)
        .with(Style::psql())
        .with(Modify::new(Segment::all()).with(Alignment::left()))
        .with(Modify::new((3, 0)).with(Span::column(2))),
    " N | column 0 | column 1 | column 2 "
    "---+----------+----------+----------"
    " 0 | 0-0      | 0-1      | 0-2      "
    " 1 | 1-0      | 1-1      | 1-2      "
    " 2            | 2-1      | 2-2      "
);

test_table!(
    cell_span_test_4,
    Matrix::new(3, 3)
        .with(Style::psql())
        .with(Modify::new(Segment::all()).with(Alignment::left()))
        .with(Modify::new((0, 1)).with(Span::column(2))),
    " N | column 0  | column 2 "
    "---+-----+-----+----------"
    " 0 | 0-0 | 0-1 | 0-2      "
    " 1 | 1-0 | 1-1 | 1-2      "
    " 2 | 2-0 | 2-1 | 2-2      "
);

test_table!(
    cell_span_test_5,
    Matrix::new(3, 3)
        .with(Style::psql())
        .with(Modify::new(Segment::all()).with(Alignment::left()))
        .with(Modify::new((1, 1)).with(Span::column(2))),
    " N | column 0 | column 1 | column 2 "
    "---+----------+----------+----------"
    " 0 | 0-0                 | 0-2      "
    " 1 | 1-0      | 1-1      | 1-2      "
    " 2 | 2-0      | 2-1      | 2-2      "
);

test_table!(
    cell_span_test_6,
    Matrix::new(3, 3)
        .with(Style::psql())
        .with(Modify::new(Segment::all()).with(Alignment::left()))
        .with(Modify::new((2, 1)).with(Span::column(2))),
    " N | column 0 | column 1 | column 2 "
    "---+----------+----------+----------"
    " 0 | 0-0      | 0-1      | 0-2      "
    " 1 | 1-0                 | 1-2      "
    " 2 | 2-0      | 2-1      | 2-2      "
);

test_table!(
    cell_span_test_7,
    Matrix::new(3, 3)
        .with(Style::psql())
        .with(Modify::new(Segment::all()).with(Alignment::left()))
        .with(Modify::new((3, 1)).with(Span::column(2))),
    " N | column 0 | column 1 | column 2 "
    "---+----------+----------+----------"
    " 0 | 0-0      | 0-1      | 0-2      "
    " 1 | 1-0      | 1-1      | 1-2      "
    " 2 | 2-0                 | 2-2      "
);

test_table!(
    cell_span_test_8,
    Matrix::new(3, 3)
        .with(Style::psql())
        .with(Modify::new((0, 2)).with(Span::column(2))),
    " N | column 0 | column 1  "
    "---+----------+-----+-----"
    " 0 |   0-0    | 0-1 | 0-2 "
    " 1 |   1-0    | 1-1 | 1-2 "
    " 2 |   2-0    | 2-1 | 2-2 "
);

test_table!(
    cell_span_test_9,
    Matrix::new(3, 3)
        .with(Style::psql())
        .with(Modify::new((1, 2)).with(Span::column(2))),
    " N | column 0 | column 1 | column 2 "
    "---+----------+----------+----------"
    " 0 |   0-0    |         0-1         "
    " 1 |   1-0    |   1-1    |   1-2    "
    " 2 |   2-0    |   2-1    |   2-2    "
);

test_table!(
    cell_span_test_10,
    Matrix::new(3, 3)
        .with(Style::psql())
        .with(Modify::new((2, 2)).with(Span::column(2))),
    " N | column 0 | column 1 | column 2 "
    "---+----------+----------+----------"
    " 0 |   0-0    |   0-1    |   0-2    "
    " 1 |   1-0    |         1-1         "
    " 2 |   2-0    |   2-1    |   2-2    "
);

test_table!(
    cell_span_test_11,
    Matrix::new(3, 3)
        .with(Style::psql())
        .with(Modify::new((3, 2)).with(Span::column(2))),
    " N | column 0 | column 1 | column 2 "
    "---+----------+----------+----------"
    " 0 |   0-0    |   0-1    |   0-2    "
    " 1 |   1-0    |   1-1    |   1-2    "
    " 2 |   2-0    |         2-1         "
);

test_table!(
    span_multiline,
    Matrix::new(3, 3)
        .insert((3, 2), "https://\nwww\n.\nredhat\n.com\n/en")
        .with(Style::psql())
        .with(Modify::new((3, 2)).with(Span::column(2))),
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
    Matrix::new(3, 3)
        .with(Style::psql())
        .with(Modify::new(Segment::all()).with(Padding::new(3, 0, 0, 0)))
        .with(Modify::new(Segment::all()).with(Alignment::left()))
        .with(Modify::new((1, 1)).with(Span::column(3)))
        .with(Modify::new((3, 1)).with(Span::column(3))),
    "   N|   column 0|   column 1|   column 2"
    "----+-----------+-----------+-----------"
    "   0|   0-0                             "
    "   1|   1-0     |   1-1     |   1-2     "
    "   2|   2-0                             "
);

test_table!(
    spaned_columns_with_collision,
    Matrix::iter([["just 1 column"; 5]; 5])
        .with(Style::modern())
        .with(
            Modify::new((0, 0))
                .with(Span::column(5))
                .with("span all 5 columns"),
        )
        .with(
            Modify::new((1, 0))
                .with(Span::column(4))
                .with("span 4 columns"),
        )
        .with(
            Modify::new((2, 0))
                .with(Span::column(3))
                .with("span 3 columns"),
        )
        .with(
            Modify::new((2, 3))
                .with(Span::column(2))
                .with("span 2 columns"),
        )
        .with(
            Modify::new((3, 0))
                .with(Span::column(2))
                .with("span 3 columns"),
        )
        .with(
            Modify::new((3, 2))
                .with(Span::column(3))
                .with("span 3 columns"),
        )
        .with(
            Modify::new((4, 1))
                .with(Span::column(4))
                .with("span 4 columns"),
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
    Matrix::iter([[1, 2, 3]])
        .with(Panel::horizontal(0,"Tabled Releases"))
        .with(Modify::new((1, 0)).with(Span::column(2)))
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
    Matrix::iter([[1, 2, 3], [4, 5, 6]])
        .with(Panel::horizontal(0,"Tabled Releases"))
        .with(Modify::new((2, 0)).with(Span::column(2)))
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
    Matrix::iter([[1, 2, 3], [4, 5, 6]])
        .with(Panel::horizontal(0,"Tabled Releases"))
        .with(Modify::new((1, 0)).with(Span::column(2)))
        .with(Modify::new((2, 0)).with(Span::column(2)))
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
    Matrix::iter([[1, 2, 3]])
        .with(Panel::horizontal(0,"Tabled Releases"))
        .with(Modify::new((1, 0)).with(Span::column(2)))
        .with(Style::ascii())
        .with(BorderSpanCorrection),
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
    Matrix::iter([[1, 2, 3], [4, 5, 6]])
        .with(Panel::horizontal(0,"Tabled Releases"))
        .with(Modify::new((2, 0)).with(Span::column(2)))
        .with(Style::ascii())
        .with(BorderSpanCorrection),
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
    Matrix::iter([[1, 2, 3], [4, 5, 6]])
        .with(Panel::horizontal(0,"Tabled Releases"))
        .with(Modify::new((1, 0)).with(Span::column(2)))
        .with(Modify::new((2, 0)).with(Span::column(2)))
        .with(Style::ascii())
        .with(BorderSpanCorrection),
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

    Matrix::new(3, 3)
        .with(Modify::new(Columns::single(0)).with(Span::column(100)))
        .to_string();
}

#[test]
#[ignore = "span zero not yet decided"]
fn span_cell_exceeds_boundaries_test() {
    // these tests shows that exiding boundaries causes invalid behaiviour
    //
    // todo: determine if it's the right behaiviour

    let table = Matrix::new(3, 3)
        .with(Style::psql())
        .with(Modify::new(Segment::all()).with(Alignment::left()))
        .with(Modify::new((0, 0)).with(Span::column(20)))
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

    let table = Matrix::new(3, 3)
        .with(Style::psql())
        .with(Modify::new(Segment::all()).with(Alignment::left()))
        .with(Modify::new((1, 1)).with(Span::column(20)))
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

    let table = Matrix::new(3, 3)
        .with(Style::psql())
        .with(Modify::new(Segment::all()).with(Alignment::left()))
        .with(Modify::new((1, 0)).with(Span::column(20)))
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
    let table = Matrix::new(3, 3)
        .with(Style::psql())
        .with(Modify::new((0, 0)).with(Span::column(0)))
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

    let table = Matrix::new(3, 3)
        .with(Style::psql())
        .with(Modify::new((0, 1)).with(Span::column(0)))
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

    let table = Matrix::new(3, 3)
        .with(Style::psql())
        .with(Modify::new((0, 2)).with(Span::column(0)))
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

    let table = Matrix::new(3, 3)
        .with(Style::psql())
        .with(Modify::new((0, 3)).with(Span::column(0)))
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

    let table = Matrix::new(3, 3)
        .with(Style::psql())
        .with(Modify::new((0, 4)).with(Span::column(0)))
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

    let table = Matrix::new(3, 3)
        .with(Style::psql())
        .with(Modify::new((0, 0)).with(Span::column(0)))
        .with(Modify::new((1, 1)).with(Span::column(0)))
        .with(Modify::new((2, 2)).with(Span::column(0)))
        .with(Modify::new((3, 2)).with(Span::column(0)))
        .with(Modify::new((3, 1)).with(Span::column(0)))
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
    let table = Matrix::table(2, 2)
        .with(Style::psql())
        .with(Modify::new(Segment::all()).with(Span::column(0)))
        .to_string();

    // todo: determine whether it's correct
    assert_eq!(table, static_table!("\n++\n\n\n"));
}

mod row {
    use tabled::settings::object::Rows;

    use super::*;

    #[test]
    fn span_row_test() {
        let table = Matrix::new(3, 3);
        {
            let table_str = table
                .clone()
                .with(Style::ascii())
                .with(Modify::new(Segment::all()).with(Alignment::left()))
                .with(Modify::new(Rows::single(0)).with(Span::row(2)))
                .to_string();

            assert_eq!(
                table_str,
                static_table!(
                    "+---+----------+----------+----------+"
                    "+ N + column 0 + column 1 + column 2 +"
                    "+---+----------+----------+----------+"
                    "| 1 | 1-0      | 1-1      | 1-2      |"
                    "+---+----------+----------+----------+"
                    "| 2 | 2-0      | 2-1      | 2-2      |"
                    "+---+----------+----------+----------+"
                )
            );

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
        let table = Matrix::new(3, 3);
        {
            // first column cells row span = 2

            {
                let table = table
                    .clone()
                    .with(Style::psql())
                    .with(Modify::new(Segment::all()).with(Alignment::left()))
                    .with(Modify::new((0, 0)).with(Span::row(2)))
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
                    .with(Modify::new((1, 0)).with(Span::row(2)))
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
                    .with(Modify::new((2, 0)).with(Span::row(2)))
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
                    .with(Modify::new((0, 1)).with(Span::row(2)))
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
                    .with(Modify::new((0, 2)).with(Span::row(2)))
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
                    .with(Modify::new((0, 3)).with(Span::row(2)))
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
                    .with(Modify::new((1, 1)).with(Span::row(2)))
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
                    .with(Modify::new((2, 1)).with(Span::row(2)))
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
                    .with(Modify::new((1, 2)).with(Span::row(2)))
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
                    .with(Modify::new((2, 2)).with(Span::row(2)))
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
                    .with(Modify::new((1, 3)).with(Span::row(2)))
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
                    .with(Modify::new((2, 3)).with(Span::row(2)))
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
            .with(Modify::new((0, 0)).with(Span::row(2)))
            .with(Style::ascii())
            .with(BorderSpanCorrection)
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
            .with(Modify::new((1, 0)).with(Span::row(2)))
            .with(Modify::new((0, 2)).with(Span::row(3)))
            .with(Style::ascii())
            .with(BorderSpanCorrection)
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
            .with(Modify::new((1, 0)).with(Span::row(2)))
            .with(Modify::new((0, 2)).with(Span::row(3)))
            .with(Modify::new((0, 1)).with(Span::row(2)))
            .with(Style::ascii())
            .with(BorderSpanCorrection)
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
            .with(Modify::new((1, 0)).with(Span::row(2)))
            .with(Modify::new((0, 1)).with(Span::row(2)).with(Span::column(2)))
            .with(Style::ascii())
            .with(BorderSpanCorrection)
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

        let h_span = |r, c, span| Modify::new((r, c)).with(Span::column(span));
        let v_span = |r, c, span| Modify::new((r, c)).with(Span::row(span));

        let table = Table::new(data)
            .with(h_span(0, 0, 5).with(String::from("span all 5 columns")))
            .with(h_span(1, 0, 4).with(String::from("span 4 columns")))
            .with(h_span(2, 0, 2).with(String::from("span 2 columns")))
            .with(v_span(2, 4, 4).with(String::from("just 1 column\nspan\n4\ncolumns")))
            .with(v_span(3, 1, 2).with(String::from("span 2 columns\nspan\n2\ncolumns")))
            .with(v_span(2, 3, 3).with(String::from("just 1 column\nspan\n3\ncolumns")))
            .with(h_span(3, 1, 2))
            .with(Style::modern())
            .with(BorderSpanCorrection)
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
            .with(Modify::new((1, 1)).with(Span::row(3)))
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
        .with(Modify::new((1, 1)).with(Span::row(3)).with(Span::column(2)))
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
    Matrix::new(3, 3).with(Modify::new((0, 0)).with(Span::column(100))),
    "+---+-----+-----+-----+"
    "|          N          |"
    "+---+-----+-----+-----+"
    "| 0 | 0-0 | 0-1 | 0-2 |"
    "+---+-----+-----+-----+"
    "| 1 | 1-0 | 1-1 | 1-2 |"
    "+---+-----+-----+-----+"
    "| 2 | 2-0 | 2-1 | 2-2 |"
    "+---+-----+-----+-----+"
);

test_table!(
    row_span_bigger_then_max,
    Matrix::new(3, 3).with(Modify::new((0, 0)).with(Span::row(100))),
    "+---+----------+----------+----------+"
    "| N | column 0 | column 1 | column 2 |"
    "+   +----------+----------+----------+"
    "|   |   0-0    |   0-1    |   0-2    |"
    "+   +----------+----------+----------+"
    "|   |   1-0    |   1-1    |   1-2    |"
    "+   +----------+----------+----------+"
    "|   |   2-0    |   2-1    |   2-2    |"
    "+---+----------+----------+----------+"
);

test_table!(
    column_span_invalid_position_row,
    Matrix::new(3, 3).with(Modify::new((1000, 0)).with(Span::column(2))),
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
    Matrix::new(3, 3).with(Modify::new((0, 1000)).with(Span::column(2))),
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
    Matrix::new(3, 3).with(Modify::new((1000, 1000)).with(Span::column(2))),
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
    Matrix::new(3, 3).with(Modify::new((1000, 0)).with(Span::row(2))),
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
    Matrix::new(3, 3).with(Modify::new((0, 1000)).with(Span::row(2))),
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
    Matrix::new(3, 3).with(Modify::new((1000, 1000)).with(Span::row(2))),
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
    fix_qc_0,
    {
        let data: [[i64; 39]; 2] = [[2542785870, 2382388818, 2879895075, 2885436543, 2331131758, 219892320, 2503640226, 3754929678, 2206481860, 686909682, 3456499235, 931699300, 1556722454, 958179233, 3896072307, 2042612749, 3354379549, 3272539286, 3926297167, 4294967295, 1650407458, 3322068437, 4294967295, 446762625, 829020202, 4150192304, 3430619243, 3460609391, 2992017103, 513091574, 1514148367, 2166549688, 1401371431, 2854075038, 1286733939, 2959901405, 4152658371, 0, 4224074215], [360331598, 3736108702, 2948800064, 2121584548, 1609988995, 469935087, 3974876615, 2193609088, 3568111892, 732365859, 0, 4294967295, 2994498036, 198522721, 1784359340, 1, 2732726754, 592359359, 3016729802, 878533877, 2997437699, 3573361662, 1111570515, 4294967295, 2245782848, 1383106893, 0, 0, 2869976103, 1611436878, 1682224972, 3249055253, 1562255501, 1370527728, 240481955, 334260406, 2247343342, 3000635978, 395723768]];
        let row_spans = [2, 1, 27, 111, 226, 221, 121, 22, 252, 30, 115, 85, 255, 126, 26, 245, 36, 50, 255, 211, 47, 114, 174, 173, 145, 138, 78, 198, 253, 229, 151, 243, 242, 30, 52, 116, 177, 25, 1, 32, 28, 48, 225, 103, 17, 243, 0, 128, 69, 206, 221, 105, 239, 74, 184, 48, 178, 237, 120, 228, 184, 1, 132, 118, 14, 187];
        let col_spans = [7, 91, 56, 246, 73];

        let data = data.iter().map(|row| row.iter().map(ToString::to_string));
        let rspans = create_span_list(2, 39).zip(row_spans.iter()).map(|(pos, span)| Modify::new(pos).with(Span::column(*span))).collect::<Vec<_>>();
        let cspans = create_span_list(2, 39).zip(col_spans.iter()).map(|(pos, span)| Modify::new(pos).with(Span::row(*span))).collect::<Vec<_>>();

        Builder::from_iter(data).build().with(Style::ascii()).with(rspans).with(cspans).to_string()
    },
    "+------+-----++++++++++++++++++++++++++++------------+------------+------------+------------+------------+-----------+-----------+------------+------------+-----------+"
    "| 2542785870 | 2879895075               | 513091574                                                                                                                    |"
    "+            +                          +------------+------------+------------+------------+------------+-----------+-----------+------------+------------+-----------+"
    "|            |                          | 1611436878 | 1682224972 | 3249055253 | 1562255501 | 1370527728 | 240481955 | 334260406 | 2247343342 | 3000635978 | 395723768 |"
    "+------+-----++++++++++++++++++++++++++++------------+------------+------------+------------+------------+-----------+-----------+------------+------------+-----------+"
);

fn create_span_list(count_rows: usize, count_cols: usize) -> impl Iterator<Item = Position> {
    (0..count_rows).flat_map(move |r| (0..count_cols).map(move |c| (r, c)))
}
