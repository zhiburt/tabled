use tabled::{
    object::Cell,
    style::{Border, Style},
    Highlight, Margin, Modify, Span, Table, Width,
};

use crate::util::{create_vector, is_lines_equal, static_table, test_table};

mod util;

test_table!(
    margin_with_table_based_on_grid_borders,
    Table::new(create_vector::<3, 3>())
        .with(Style::extended())
        .with(Highlight::new(Cell(0, 0), Border::filled('+')))
        .with(Highlight::new(Cell(1, 1), Border::filled('*')))
        .with(Margin::new(1, 2, 1, 2).set_fill('>', '<', 'V', '^')),
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
    Table::new({
            let mut data = create_vector::<3, 3>();
            data[2][2] = String::from("https://\nwww\n.\nredhat\n.com\n/en");
            data
        })
        .with(Style::psql())
        .with(Modify::new(Cell(3, 2)).with(Span::column(2)))
        .with(Margin::new(1, 1, 1, 1).set_fill('>', '<', 'V', '^')),
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
    Table::new({
            let mut data = create_vector::<3, 3>();
            data[2][2] = String::from("https://\nwww\n.\nredhat\n.com\n/en");
            data
        })
        .with(Style::psql())
        .with(Modify::new(Cell(3, 2)).with(Span::column(2)))
        .with(Margin::new(0, 0, 0, 0).set_fill('>', '<', 'V', '^')),
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
    let data = create_vector::<3, 3>();

    let table = Table::new(&data)
        .with(Style::psql())
        .with(Modify::new(Cell(1, 1)).with(Span::column(2)))
        .with(Margin::new(1, 1, 1, 1).set_fill('>', '<', 'V', '^'))
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
    let data = create_vector::<3, 3>();

    let table = Table::new(&data)
        .with(Style::psql())
        .with(Modify::new(Cell(1, 1)).with(Span::column(2)))
        .with(Margin::new(1, 1, 1, 1).set_fill('>', '<', 'V', '^'))
        .with(Width::increase(50))
        .to_string();

    assert_eq!(papergrid::string_width_multiline(&table), 50);
    assert_eq!(
        table,
        static_table!(
            "VVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVV"
            "> N    | column 0    | column 1    | column 2    <"
            ">------+-------------+-------------+-------------<"
            "> 0    | 0-0                       | 0-2         <"
            "> 1    | 1-0         | 1-1         | 1-2         <"
            "> 2    | 2-0         | 2-1         | 2-2         <"
            "^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^"
        )
    );
}

#[test]
#[ignore = "It's not yet clear what to do with such spans"]
fn table_0_spanned_with_width() {
    let data = create_vector::<0, 0>();

    let table = Table::new(&data)
        .with(Modify::new(Cell(0, 0)).with(Span::column(0)))
        .with(Width::increase(50))
        .to_string();

    assert_eq!(table, "++\n|\n++\n");

    let table = Table::new(&data)
        .with(Modify::new(Cell(0, 0)).with(Span::column(0)))
        .with(Width::truncate(50))
        .to_string();

    assert_eq!(table, "++\n|\n++\n");
}

#[cfg(feature = "color")]
#[test]
fn margin_color_test() {
    use owo_colors::OwoColorize;
    use std::convert::TryFrom;
    use tabled::{margin::MarginColor, style::Color};

    let table = Table::new(&create_vector::<3, 3>())
        .with(Style::psql())
        .with(Margin::new(2, 2, 2, 2).set_fill('>', '<', 'V', '^'))
        .with(MarginColor::new(
            Color::try_from(" ".on_blue().red().bold().to_string()).unwrap(),
            Color::try_from(" ".on_yellow().blue().to_string()).unwrap(),
            Color::try_from(" ".red().bold().to_string()).unwrap(),
            Color::try_from(" ".green().to_string()).unwrap(),
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
