#![cfg(feature = "std")]

use tabled::settings::{object::Cell, Border, Highlight, Margin, Modify, Span, Style, Width};

use crate::matrix::Matrix;
use testing_table::{is_lines_equal, static_table, test_table};

#[cfg(feature = "color")]
use ::{owo_colors::OwoColorize, std::convert::TryFrom, tabled::settings::Color};

test_table!(
    margin_with_table_based_on_grid_borders,
    Matrix::new(3, 3)
        .with(Style::extended())
        .with(Highlight::new(Cell::new(0, 0), Border::filled('+')))
        .with(Highlight::new(Cell::new(1, 1), Border::filled('*')))
        .with(Margin::new(1, 2, 1, 2).fill('>', '<', 'V', '^')),
    "VVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVV"
    ">+++++══════════╦══════════╦══════════╗<<"
    ">+ N + column 0 ║ column 1 ║ column 2 ║<<"
    ">++++************══════════╬══════════╣<<"
    ">║ 0 *   0-0    *   0-1    ║   0-2    ║<<"
    ">╠═══************══════════╬══════════╣<<"
    ">║ 1 ║   1-0    ║   1-1    ║   1-2    ║<<"
    ">╠═══╬══════════╬══════════╬══════════╣<<"
    ">║ 2 ║   2-0    ║   2-1    ║   2-2    ║<<"
    ">╚═══╩══════════╩══════════╩══════════╝<<"
    "^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^"
    "^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^"
);

test_table!(
    margin_without_table_based_on_grid_borders,
    Matrix::new(3, 3)
        .insert((3, 2), "https://\nwww\n.\nredhat\n.com\n/en")
        .with(Style::psql())
        .with(Modify::new(Cell::new(3, 2)).with(Span::column(2)))
        .with(Margin::new(1, 1, 1, 1).fill('>', '<', 'V', '^')),
    "VVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVV"
    "> N | column 0 | column 1 | column 2 <"
    ">---+----------+----------+----------<"
    "> 0 |   0-0    |   0-1    |   0-2    <"
    "> 1 |   1-0    |   1-1    |   1-2    <"
    "> 2 |   2-0    |      https://       <"
    ">   |          |      www            <"
    ">   |          |      .              <"
    ">   |          |      redhat         <"
    ">   |          |      .com           <"
    ">   |          |      /en            <"
    "^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^"
);

test_table!(
    table_with_empty_margin,
    Matrix::new(3, 3)
        .insert((3, 2), "https://\nwww\n.\nredhat\n.com\n/en")
        .with(Style::psql())
        .with(Modify::new(Cell::new(3, 2)).with(Span::column(2)))
        .with(Margin::new(0, 0, 0, 0).fill('>', '<', 'V', '^')),
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

#[test]
fn table_with_margin_and_min_width() {
    let table = Matrix::new(3, 3)
        .with(Style::psql())
        .with(Modify::new(Cell::new(1, 1)).with(Span::column(2)))
        .with(Margin::new(1, 1, 1, 1).fill('>', '<', 'V', '^'))
        .with(Width::truncate(20))
        .to_string();

    assert_eq!(
        table,
        static_table!(
            "VVVVVVVVVVVVVVVVVVVV"
            ">  | co | co | col <"
            ">--+----+----+-----<"
            ">  |   0-0   | 0-2 <"
            ">  | 1- | 1- | 1-2 <"
            ">  | 2- | 2- | 2-2 <"
            "^^^^^^^^^^^^^^^^^^^^"
        )
    );
    assert!(is_lines_equal(&table, 20));
}

#[test]
fn table_with_margin_and_max_width() {
    let table = Matrix::new(3, 3)
        .with(Style::psql())
        .with(Modify::new(Cell::new(1, 1)).with(Span::column(2)))
        .with(Margin::new(1, 1, 1, 1).fill('>', '<', 'V', '^'))
        .with(Width::increase(50))
        .to_string();

    assert_eq!(papergrid::util::string::string_width_multiline(&table), 50);
    assert_eq!(
        table,
        static_table!(
            "VVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVV"
            ">  N   |  column 0   |  column 1   |  column 2   <"
            ">------+-------------+-------------+-------------<"
            ">  0   |            0-0            |     0-2     <"
            ">  1   |     1-0     |     1-1     |     1-2     <"
            ">  2   |     2-0     |     2-1     |     2-2     <"
            "^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^"
        )
    );
}

#[test]
#[ignore = "It's not yet clear what to do with such spans"]
fn table_0_spanned_with_width() {
    let table = Matrix::table(0, 0)
        .with(Modify::new(Cell::new(0, 0)).with(Span::column(0)))
        .with(Width::increase(50))
        .to_string();

    assert_eq!(table, "++\n|\n++\n");

    let table = Matrix::table(0, 0)
        .with(Modify::new(Cell::new(0, 0)).with(Span::column(0)))
        .with(Width::truncate(50))
        .to_string();

    assert_eq!(table, "++\n|\n++\n");
}

#[test]
fn margin_color_test_not_colored_feature() {
    use tabled::settings::Color;

    let table = Matrix::new(3, 3)
        .with(Style::psql())
        .with(Margin::new(2, 2, 2, 2).fill('>', '<', 'V', '^').colorize(
            Color::BG_GREEN,
            Color::BG_YELLOW,
            Color::BG_RED,
            Color::BG_BLUE,
        ))
        .to_string();

    assert_eq!(
        table,
        static_table!(
            "\u{1b}[41mVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVV\u{1b}[49m"
            "\u{1b}[41mVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVV\u{1b}[49m"
            "\u{1b}[42m>>\u{1b}[49m N | column 0 | column 1 | column 2 \u{1b}[43m<<\u{1b}[49m"
            "\u{1b}[42m>>\u{1b}[49m---+----------+----------+----------\u{1b}[43m<<\u{1b}[49m"
            "\u{1b}[42m>>\u{1b}[49m 0 |   0-0    |   0-1    |   0-2    \u{1b}[43m<<\u{1b}[49m"
            "\u{1b}[42m>>\u{1b}[49m 1 |   1-0    |   1-1    |   1-2    \u{1b}[43m<<\u{1b}[49m"
            "\u{1b}[42m>>\u{1b}[49m 2 |   2-0    |   2-1    |   2-2    \u{1b}[43m<<\u{1b}[49m"
            "\u{1b}[44m^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\u{1b}[49m"
            "\u{1b}[44m^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\u{1b}[49m"
        )
    );
}

#[cfg(feature = "color")]
#[test]
fn margin_color_test() {
    let table = Matrix::new(3, 3)
        .with(Style::psql())
        .with(Margin::new(2, 2, 2, 2).fill('>', '<', 'V', '^').colorize(
            Color::try_from(" ".red().bold().to_string()).unwrap(),
            Color::try_from(" ".green().to_string()).unwrap(),
            Color::try_from(" ".on_blue().red().bold().to_string()).unwrap(),
            Color::try_from(" ".on_yellow().blue().to_string()).unwrap(),
        ))
        .to_string();

    assert_eq!(
        table,
        static_table!(
            "\u{1b}[1m\u{1b}[31m\u{1b}[44mVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVV\u{1b}[22m\u{1b}[39m\u{1b}[49m"
            "\u{1b}[1m\u{1b}[31m\u{1b}[44mVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVV\u{1b}[22m\u{1b}[39m\u{1b}[49m"
            "\u{1b}[1m\u{1b}[31m>>\u{1b}[22m\u{1b}[39m N | column 0 | column 1 | column 2 \u{1b}[32m<<\u{1b}[39m"
            "\u{1b}[1m\u{1b}[31m>>\u{1b}[22m\u{1b}[39m---+----------+----------+----------\u{1b}[32m<<\u{1b}[39m"
            "\u{1b}[1m\u{1b}[31m>>\u{1b}[22m\u{1b}[39m 0 |   0-0    |   0-1    |   0-2    \u{1b}[32m<<\u{1b}[39m"
            "\u{1b}[1m\u{1b}[31m>>\u{1b}[22m\u{1b}[39m 1 |   1-0    |   1-1    |   1-2    \u{1b}[32m<<\u{1b}[39m"
            "\u{1b}[1m\u{1b}[31m>>\u{1b}[22m\u{1b}[39m 2 |   2-0    |   2-1    |   2-2    \u{1b}[32m<<\u{1b}[39m"
            "\u{1b}[34m\u{1b}[43m^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\u{1b}[39m\u{1b}[49m"
            "\u{1b}[34m\u{1b}[43m^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\u{1b}[39m\u{1b}[49m"
        )
    );
}
