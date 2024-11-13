#![cfg(feature = "std")]

use tabled::{
    builder::Builder,
    settings::{
        highlight::Highlight,
        object::{Cell, Columns, Frame, Object, Rows, Segment},
        style::{Border, Style},
    },
};

use crate::matrix::Matrix;
use testing_table::{static_table, test_table};

test_table!(
    highlingt_object_exceeds_boundaries,
    Matrix::new(3, 3).with(Style::modern()).with(Highlight::outline(Cell::new(1000, 0), '+')),
    "┌───┬──────────┬──────────┬──────────┐"
    "│ N │ column 0 │ column 1 │ column 2 │"
    "├───┼──────────┼──────────┼──────────┤"
    "│ 0 │   0-0    │   0-1    │   0-2    │"
    "├───┼──────────┼──────────┼──────────┤"
    "│ 1 │   1-0    │   1-1    │   1-2    │"
    "├───┼──────────┼──────────┼──────────┤"
    "│ 2 │   2-0    │   2-1    │   2-2    │"
    "└───┴──────────┴──────────┴──────────┘"
);

test_table!(
    highlingt_empty_table,
    Builder::default()
        .build()
        .with(Highlight::outline(Segment::all(), '+')),
    ""
);

test_table!(
    highlingt_cell,
    Matrix::new(3, 3)
        .with(Style::modern())
        .with(Highlight::outline(Cell::new(0, 0), '+'))
        .with(Highlight::outline(Cell::new(1, 1), '*')),
    "+++++──────────┬──────────┬──────────┐"
    "+ N + column 0 │ column 1 │ column 2 │"
    "++++************──────────┼──────────┤"
    "│ 0 *   0-0    *   0-1    │   0-2    │"
    "├───************──────────┼──────────┤"
    "│ 1 │   1-0    │   1-1    │   1-2    │"
    "├───┼──────────┼──────────┼──────────┤"
    "│ 2 │   2-0    │   2-1    │   2-2    │"
    "└───┴──────────┴──────────┴──────────┘"
);

test_table!(
    highlingt_row,
    Matrix::new(3, 3)
        .with(Style::modern())
        .with(Highlight::outline(Rows::single(0), '+'))
        .with(Highlight::outline(Rows::single(3), '*')),
    "++++++++++++++++++++++++++++++++++++++"
    "+ N │ column 0 │ column 1 │ column 2 +"
    "++++++++++++++++++++++++++++++++++++++"
    "│ 0 │   0-0    │   0-1    │   0-2    │"
    "├───┼──────────┼──────────┼──────────┤"
    "│ 1 │   1-0    │   1-1    │   1-2    │"
    "**************************************"
    "* 2 │   2-0    │   2-1    │   2-2    *"
    "**************************************"
);

test_table!(
    highlingt_column,
    Matrix::new(3, 3)
        .with(Style::modern())
        .with(Highlight::new(Columns::single(0)).border(Border::filled('+')))
        .with(Highlight::new(Columns::single(2)).border(Border::filled('*'))),
    "+++++──────────************──────────┐"
    "+ N + column 0 * column 1 * column 2 │"
    "+───+──────────*──────────*──────────┤"
    "+ 0 +   0-0    *   0-1    *   0-2    │"
    "+───+──────────*──────────*──────────┤"
    "+ 1 +   1-0    *   1-1    *   1-2    │"
    "+───+──────────*──────────*──────────┤"
    "+ 2 +   2-0    *   2-1    *   2-2    │"
    "+++++──────────************──────────┘"
);

test_table!(
    highlingt_row_range,
    Matrix::new(3, 3)
        .with(Style::modern())
        .with(Highlight::outline(Rows::new(1..3), '+')),
    "┌───┬──────────┬──────────┬──────────┐"
    "│ N │ column 0 │ column 1 │ column 2 │"
    "++++++++++++++++++++++++++++++++++++++"
    "+ 0 │   0-0    │   0-1    │   0-2    +"
    "+───┼──────────┼──────────┼──────────+"
    "+ 1 │   1-0    │   1-1    │   1-2    +"
    "++++++++++++++++++++++++++++++++++++++"
    "│ 2 │   2-0    │   2-1    │   2-2    │"
    "└───┴──────────┴──────────┴──────────┘"
);

test_table!(
    highlingt_column_range,
    Matrix::new(3, 3)
        .with(Style::modern())
        .with(Highlight::outline(Columns::new(..2), '+')),
    "++++++++++++++++──────────┬──────────┐"
    "+ N │ column 0 + column 1 │ column 2 │"
    "+───┼──────────+──────────┼──────────┤"
    "+ 0 │   0-0    +   0-1    │   0-2    │"
    "+───┼──────────+──────────┼──────────┤"
    "+ 1 │   1-0    +   1-1    │   1-2    │"
    "+───┼──────────+──────────┼──────────┤"
    "+ 2 │   2-0    +   2-1    │   2-2    │"
    "++++++++++++++++──────────┴──────────┘"
);

test_table!(
    highlingt_frame,
    Matrix::new(3, 3)
        .with(Style::modern())
        .with(Highlight::new(Frame).border(
            Border::filled('+')
                .corner_top_left('*')
                .corner_top_right('#')
                .corner_bottom_left('@')
                .corner_bottom_right('.'),
        )),
    "*++++++++++++++++++++++++++++++++++++#"
    "+ N │ column 0 │ column 1 │ column 2 +"
    "+───┼──────────┼──────────┼──────────+"
    "+ 0 │   0-0    │   0-1    │   0-2    +"
    "+───┼──────────┼──────────┼──────────+"
    "+ 1 │   1-0    │   1-1    │   1-2    +"
    "+───┼──────────┼──────────┼──────────+"
    "+ 2 │   2-0    │   2-1    │   2-2    +"
    "@++++++++++++++++++++++++++++++++++++."
);

test_table!(
    highlingt_full,
    Matrix::new(3, 3)
        .with(Style::modern())
        .with(Highlight::new(Segment::all()).border(
            Border::filled('+')
                .corner_top_left('*')
                .corner_top_right('#')
                .corner_bottom_left('@')
                .corner_bottom_right('.'),
        )),
    "*++++++++++++++++++++++++++++++++++++#"
    "+ N │ column 0 │ column 1 │ column 2 +"
    "+───┼──────────┼──────────┼──────────+"
    "+ 0 │   0-0    │   0-1    │   0-2    +"
    "+───┼──────────┼──────────┼──────────+"
    "+ 1 │   1-0    │   1-1    │   1-2    +"
    "+───┼──────────┼──────────┼──────────+"
    "+ 2 │   2-0    │   2-1    │   2-2    +"
    "@++++++++++++++++++++++++++++++++++++."
);

test_table!(
    highlingt_single_column,
    Matrix::table(3, 0)
        .with(Style::modern())
        .with(Highlight::new(Cell::new(0, 0)).border(Border::new().left('*').top('x')))
        .with(Highlight::new(Rows::new(1..3)).border(Border::new().left('n'))),
    "┌xxx┐"
    "* N │"
    "├───┤"
    "n 0 │"
    "n───┤"
    "n 1 │"
    "├───┤"
    "│ 2 │"
    "└───┘"
);

test_table!(
    highlingt_several_times,
    Matrix::new(3, 3)
        .with(Style::modern())
        .with(Highlight::outline(Frame, '*'))
        .with(Highlight::outline(Cell::new(1, 1), '#'))
        .with(Highlight::outline(Columns::single(3), 'x')),
    "**************************xxxxxxxxxxxx"
    "* N │ column 0 │ column 1 x column 2 x"
    "*───############──────────x──────────x"
    "* 0 #   0-0    #   0-1    x   0-2    x"
    "*───############──────────x──────────x"
    "* 1 │   1-0    │   1-1    x   1-2    x"
    "*───┼──────────┼──────────x──────────x"
    "* 2 │   2-0    │   2-1    x   2-2    x"
    "**************************xxxxxxxxxxxx"
);

// @todo
//
// #[test]
// fn highlingt_empty_border() {
//     let data = create_vector::<3, 3>();
//     let table = Table::new(&data)
//         .with(Style::modern())
//         .with(Highlight::border(Frame, Border::empty()))
//         .to_string();

//     let expected = static_table!(
//         " N │ column 0 │ column 1 │ column 2 "
//         "───                       ──────────"
//         " 0     0-0    │   0-1        0-2    "
//         "─── ──────────┼────────── ──────────"
//         " 1     1-0    │   1-1        1-2    "
//         "───                       ──────────"
//         " 2 │   2-0    │   2-1    │   2-2    "
//     );

//     assert_eq!(table, expected);
// }

#[test]
fn highlingt_complex_figures() {
    macro_rules! test_highlight {
        ($object:expr, $expected:expr,) => {
            let border = Border::filled('+')
                .corner_top_left('*')
                .corner_top_right('#')
                .corner_bottom_left('@')
                .corner_bottom_right('.');

            let table = Matrix::new(3, 3)
                .with(Style::modern())
                .with(Highlight::new($object).border(border))
                .to_string();

            assert_eq!(table, $expected);
        };
    }

    test_highlight!(
        Segment::all().not(Segment::new(2.., 1..3)),
        static_table!(
            "*++++++++++++++++++++++++++++++++++++#"
            "+ N │ column 0 │ column 1 │ column 2 +"
            "+───┼──────────┼──────────┼──────────+"
            "+ 0 │   0-0    │   0-1    │   0-2    +"
            "+───*+++++++++++++++++++++#──────────+"
            "+ 1 +   1-0    │   1-1    +   1-2    +"
            "+───+──────────┼──────────+──────────+"
            "+ 2 +   2-0    │   2-1    +   2-2    +"
            "@+++.──────────┴──────────@++++++++++."
        ),
    );

    test_highlight!(
        Segment::all()
            .not(Segment::new(0..1, 1..3))
            .not(Columns::single(0)),
        static_table!(
            "┌───┬──────────┬──────────*++++++++++#"
            "│ N │ column 0 │ column 1 + column 2 +"
            "├───*+++++++++++++++++++++.──────────+"
            "│ 0 +   0-0    │   0-1    │   0-2    +"
            "├───+──────────┼──────────┼──────────+"
            "│ 1 +   1-0    │   1-1    │   1-2    +"
            "├───+──────────┼──────────┼──────────+"
            "│ 2 +   2-0    │   2-1    │   2-2    +"
            "└───@++++++++++++++++++++++++++++++++."
        ),
    );

    test_highlight!(
        Segment::all().not(Segment::new(0..1, 1..3)),
        static_table!(
            "*+++#──────────┬──────────*++++++++++#"
            "+ N + column 0 │ column 1 + column 2 +"
            "+───@+++++++++++++++++++++.──────────+"
            "+ 0 │   0-0    │   0-1    │   0-2    +"
            "+───┼──────────┼──────────┼──────────+"
            "+ 1 │   1-0    │   1-1    │   1-2    +"
            "+───┼──────────┼──────────┼──────────+"
            "+ 2 │   2-0    │   2-1    │   2-2    +"
            "@++++++++++++++++++++++++++++++++++++."
        ),
    );

    test_highlight!(
        Segment::all().not(Segment::new(1..2, 1..3)),
        static_table!(
            "*++++++++++++++++++++++++++++++++++++#"
            "+ N │ column 0 │ column 1 │ column 2 +"
            "+───*+++++++++++++++++++++#──────────+"
            "+ 0 +   0-0    │   0-1    +   0-2    +"
            "+───@+++++++++++++++++++++.──────────+"
            "+ 1 │   1-0    │   1-1    │   1-2    +"
            "+───┼──────────┼──────────┼──────────+"
            "+ 2 │   2-0    │   2-1    │   2-2    +"
            "@++++++++++++++++++++++++++++++++++++."
        ),
    );

    test_highlight!(
        Cell::new(0, 0)
            .and(Cell::new(3, 3))
            .and(Cell::new(0, 3))
            .and(Cell::new(3, 0)),
        static_table!(
            "*+++#──────────┬──────────*++++++++++#"
            "+ N + column 0 │ column 1 + column 2 +"
            "@+++.──────────┼──────────@++++++++++."
            "│ 0 │   0-0    │   0-1    │   0-2    │"
            "├───┼──────────┼──────────┼──────────┤"
            "│ 1 │   1-0    │   1-1    │   1-2    │"
            "*+++#──────────┼──────────*++++++++++#"
            "+ 2 +   2-0    │   2-1    +   2-2    +"
            "@+++.──────────┴──────────@++++++++++."
        ),
    );

    test_highlight!(
        Rows::single(0).and(Rows::single(3)),
        static_table!(
            "*++++++++++++++++++++++++++++++++++++#"
            "+ N │ column 0 │ column 1 │ column 2 +"
            "@++++++++++++++++++++++++++++++++++++."
            "│ 0 │   0-0    │   0-1    │   0-2    │"
            "├───┼──────────┼──────────┼──────────┤"
            "│ 1 │   1-0    │   1-1    │   1-2    │"
            "*++++++++++++++++++++++++++++++++++++#"
            "+ 2 │   2-0    │   2-1    │   2-2    +"
            "@++++++++++++++++++++++++++++++++++++."
        ),
    );

    test_highlight!(
        Columns::single(0).and(Columns::single(3)),
        static_table!(
            "*+++#──────────┬──────────*++++++++++#"
            "+ N + column 0 │ column 1 + column 2 +"
            "+───+──────────┼──────────+──────────+"
            "+ 0 +   0-0    │   0-1    +   0-2    +"
            "+───+──────────┼──────────+──────────+"
            "+ 1 +   1-0    │   1-1    +   1-2    +"
            "+───+──────────┼──────────+──────────+"
            "+ 2 +   2-0    │   2-1    +   2-2    +"
            "@+++.──────────┴──────────@++++++++++."
        ),
    );

    test_highlight!(
        Segment::all().not(Cell::new(3, 1).and(Cell::new(3, 2))),
        static_table!(
            "*++++++++++++++++++++++++++++++++++++#"
            "+ N │ column 0 │ column 1 │ column 2 +"
            "+───┼──────────┼──────────┼──────────+"
            "+ 0 │   0-0    │   0-1    │   0-2    +"
            "+───┼──────────┼──────────┼──────────+"
            "+ 1 │   1-0    │   1-1    │   1-2    +"
            "+───*+++++++++++++++++++++#──────────+"
            "+ 2 +   2-0    │   2-1    +   2-2    +"
            "@+++.──────────┴──────────@++++++++++."
        ),
    );

    test_highlight!(
        Rows::single(0)
            .and(Cell::new(1, 1).and(Cell::new(1, 2)))
            .and(Cell::new(2, 3)),
        static_table!(
            "*++++++++++++++++++++++++++++++++++++#"
            "+ N │ column 0 │ column 1 │ column 2 +"
            "@+++#──────────┼──────────*++++++++++."
            "│ 0 +   0-0    │   0-1    +   0-2    │"
            "├───@+++++++++++++++++++++*++++++++++#"
            "│ 1 │   1-0    │   1-1    +   1-2    +"
            "├───┼──────────┼──────────@++++++++++."
            "│ 2 │   2-0    │   2-1    │   2-2    │"
            "└───┴──────────┴──────────┴──────────┘"
        ),
    );

    test_highlight!(
        Segment::all()
            .not(Segment::new(2.., 0..3))
            .not(Cell::new(1, 0)),
        static_table!(
            "*++++++++++++++++++++++++++++++++++++#"
            "+ N │ column 0 │ column 1 │ column 2 +"
            "@+++#──────────┼──────────┼──────────+"
            "│ 0 +   0-0    │   0-1    │   0-2    +"
            "├───@+++++++++++++++++++++#──────────+"
            "│ 1 │   1-0    │   1-1    +   1-2    +"
            "├───┼──────────┼──────────+──────────+"
            "│ 2 │   2-0    │   2-1    +   2-2    +"
            "└───┴──────────┴──────────@++++++++++."
        ),
    );

    test_highlight!(
        Segment::all()
            .not(Segment::new(..1, 1..))
            .not(Segment::new(1..2, 2..))
            .not(Cell::new(2, 3)),
        static_table!(
            "*+++#──────────┬──────────┬──────────┐"
            "+ N + column 0 │ column 1 │ column 2 │"
            "+───@++++++++++#──────────┼──────────┤"
            "+ 0 │   0-0    +   0-1    │   0-2    │"
            "+───┼──────────@++++++++++#──────────┤"
            "+ 1 │   1-0    │   1-1    +   1-2    │"
            "+───┼──────────┼──────────@++++++++++#"
            "+ 2 │   2-0    │   2-1    │   2-2    +"
            "@++++++++++++++++++++++++++++++++++++."
        ),
    );
}
