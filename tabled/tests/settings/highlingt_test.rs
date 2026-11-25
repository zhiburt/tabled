#![cfg(feature = "std")]
#![cfg(feature = "assert")]

use std::iter::FromIterator;

use tabled::{
    assert::{static_table, test_table},
    builder::Builder,
    settings::{
        highlight::Highlight,
        object::{Cell, Columns, Frame, Object, Rows, Segment},
        style::{Border, BorderColor, Style},
        themes::BorderCorrection,
        Color, Span,
    },
    Table,
};

use crate::util::Matrix;

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
        .with(Highlight::outline(Rows::one(0), '+'))
        .with(Highlight::outline(Rows::one(3), '*')),
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
        .with(Highlight::new(Columns::one(0)).border(Border::filled('+')))
        .with(Highlight::new(Columns::one(2)).border(Border::filled('*'))),
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
        .with(Highlight::outline(Columns::one(3), 'x')),
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
            .not(Columns::one(0)),
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
        Rows::one(0).and(Rows::one(3)),
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
        Columns::one(0).and(Columns::one(3)),
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
        Rows::one(0)
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

#[test]
fn highlingt_reverse_cutout() {
    let data = [
        ["0", "1", "2", "3"],
        ["4", "5", "6", "7"],
        ["8", "9", "x", ""],
        ["c", "d", "", ""],
    ];

    let mut ordinary = Table::from_iter(data);
    ordinary
        .with(Style::modern())
        .modify((2, 2), (Span::column(2), Span::row(2)))
        .with(BorderCorrection::span())
        .with(Highlight::new(Segment::new(2.., 2..)).border(Border::inherit(Style::modern())));

    assert_eq!(
        ordinary.to_string(),
        static_table!(
            "┌───┬───┬───┬───┐"
            "│ 0 │ 1 │ 2 │ 3 │"
            "├───┼───┼───┼───┤"
            "│ 4 │ 5 │ 6 │ 7 │"
            "├───┼───┌───────┐"
            "│ 8 │ 9 │ x     │"
            "├───┼───│       │"
            "│ c │ d │       │"
            "└───┴───└───────┘"
        ),
    );

    let mut reversed = Table::from_iter(data);
    reversed
        .with(Style::modern())
        .modify((2, 2), (Span::column(2), Span::row(2)))
        .with(BorderCorrection::span())
        .with(
            Highlight::new(Segment::new(2.., 2..))
                .border(Border::inherit(Style::modern()))
                .reverse(),
        );

    assert_eq!(
        reversed.to_string(),
        static_table!(
            "┌───┬───┬───┬───┐"
            "│ 0 │ 1 │ 2 │ 3 │"
            "├───┼───┼───┼───┤"
            "│ 4 │ 5 │ 6 │ 7 │"
            "├───┼───┼───┴───┘"
            "│ 8 │ 9 │ x     │"
            "├───┼───┤       │"
            "│ c │ d │       │"
            "└───┴───┘───────┘"
        ),
    );
}

#[test]
fn highlingt_reverse_cutout_blanked() {
    let data = [
        ["0", "1", "2", "3"],
        ["4", "5", "6", "7"],
        ["8", "9", "x", ""],
        ["c", "d", "", ""],
    ];

    let mut table = Table::from_iter(data);
    table
        .with(Style::modern())
        .modify((2, 2), (Span::column(2), Span::row(2)))
        .with(BorderCorrection::span())
        .modify(Segment::new(2.., 2..), Border::filled(' '))
        .with(
            Highlight::new(Segment::new(2.., 2..))
                .border(Border::inherit(Style::modern()))
                .reverse(),
        );

    assert_eq!(
        table.to_string(),
        static_table!(
            "┌───┬───┬───┬───┐"
            "│ 0 │ 1 │ 2 │ 3 │"
            "├───┼───┼───┼───┤"
            "│ 4 │ 5 │ 6 │ 7 │"
            "├───┼───┼───┴───┘"
            "│ 8 │ 9 │ x      "
            "├───┼───┤        "
            "│ c │ d │        "
            "└───┴───┘        "
        ),
    );
}

#[test]
fn highlingt_reverse_empty_and_disconnected() {
    let base = Table::from_iter([["a", "b", "c"], ["d", "e", "f"], ["g", "h", "i"]])
        .with(Style::modern())
        .to_string();

    let empty_segment = Table::from_iter([["a", "b", "c"], ["d", "e", "f"], ["g", "h", "i"]])
        .with(Style::modern())
        .with(Highlight::outline(Segment::new(0..0, 0..0), '*').reverse())
        .to_string();
    assert_eq!(empty_segment, base);

    let oob_cell = Table::from_iter([["a", "b", "c"], ["d", "e", "f"], ["g", "h", "i"]])
        .with(Style::modern())
        .with(Highlight::outline(Cell::new(3, 0), '*').reverse())
        .to_string();
    assert_eq!(oob_cell, base);

    let empty_table = Builder::default()
        .build()
        .with(Highlight::outline(Segment::all(), '*').reverse())
        .to_string();
    assert_eq!(empty_table, "");

    let disconnected = Table::from_iter([["a", "b", "c"], ["d", "e", "f"], ["g", "h", "i"]])
        .with(Style::modern())
        .with(
            Highlight::new(Cell::new(0, 0).and(Cell::new(2, 2)))
                .border(Border::inherit(Style::modern()))
                .reverse(),
        )
        .to_string();
    assert_eq!(
        disconnected,
        static_table!(
            "┌───┌───┬───┐"
            "│ a │ b │ c │"
            "┌───┼───┼───┤"
            "│ d │ e │ f │"
            "├───┼───┼───┘"
            "│ g │ h │ i │"
            "└───┴───┘───┘"
        ),
    );

    let diagonal = Table::from_iter([["a", "b"], ["c", "d"]])
        .with(Style::modern())
        .modify(Cell::new(0, 0).and(Cell::new(1, 1)), Border::filled(' '))
        .with(
            Highlight::new(Cell::new(0, 0).and(Cell::new(1, 1)))
                .border(Border::inherit(Style::modern()))
                .reverse(),
        )
        .to_string();
    assert_eq!(
        diagonal,
        static_table!(
            "    ┌───┐"
            "  a │ b │"
            "┌───┼───┘"
            "│ c │ d  "
            "└───┘    "
        ),
    );
}

#[test]
fn highlingt_reverse_setter_order_and_color() {
    let data = [
        ["0", "1", "2", "3"],
        ["4", "5", "6", "7"],
        ["8", "9", "x", ""],
        ["c", "d", "", ""],
    ];

    let border_then_reverse = Table::from_iter(data)
        .with(Style::modern())
        .modify((2, 2), (Span::column(2), Span::row(2)))
        .with(BorderCorrection::span())
        .with(
            Highlight::new(Segment::new(2.., 2..))
                .border(Border::inherit(Style::modern()))
                .reverse(),
        )
        .to_string();

    let reverse_then_border = Table::from_iter(data)
        .with(Style::modern())
        .modify((2, 2), (Span::column(2), Span::row(2)))
        .with(BorderCorrection::span())
        .with(
            Highlight::new(Segment::new(2.., 2..))
                .reverse()
                .border(Border::inherit(Style::modern())),
        )
        .to_string();

    assert_eq!(border_then_reverse, reverse_then_border);

    let color_then_reverse = Table::from_iter([["a", "b"], ["c", "d"]])
        .with(Style::modern())
        .with(
            Highlight::new(Cell::new(1, 1))
                .color(BorderColor::filled(Color::FG_RED))
                .reverse(),
        )
        .to_string();

    let reverse_then_color = Table::from_iter([["a", "b"], ["c", "d"]])
        .with(Style::modern())
        .with(
            Highlight::new(Cell::new(1, 1))
                .reverse()
                .color(BorderColor::filled(Color::FG_RED)),
        )
        .to_string();

    assert_eq!(color_then_reverse, reverse_then_color);
    assert_eq!(
        color_then_reverse,
        static_table!(
            "┌───┬───┐"
            "│ a │ b │"
            "├───\u{1b}[31m┼───┤\u{1b}[39m"
            "│ c \u{1b}[31m│\u{1b}[39m d │"
            "└───\u{1b}[31m┴\u{1b}[39m───┘"
        ),
    );
}
