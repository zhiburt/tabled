use tabled::{
    object::Cell,
    style::{Border, Style},
    Highlight, Margin, Modify, Span, Table, Width,
};

use crate::util::{create_vector, is_lines_equal, static_table};

mod util;

#[test]
fn margin_with_table_based_on_grid_borders() {
    let data = create_vector::<3, 3>();
    let table = Table::new(&data)
        .with(Style::extended())
        .with(Highlight::new(Cell(0, 0), Border::filled('+')))
        .with(Highlight::new(Cell(1, 1), Border::filled('*')))
        .with(Margin::new(1, 2, 1, 2).set_fill('>', '<', 'V', '^'))
        .to_string();

    assert_eq!(
        table,
        static_table!(
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
        )
    );
}

#[test]
fn margin_without_table_based_on_grid_borders() {
    let mut data = create_vector::<3, 3>();
    data[2][2] = String::from("https://\nwww\n.\nredhat\n.com\n/en");

    let table = Table::new(&data)
        .with(Style::psql())
        .with(Modify::new(Cell(3, 2)).with(Span::column(2)))
        .with(Margin::new(1, 1, 1, 1).set_fill('>', '<', 'V', '^'))
        .to_string();

    assert_eq!(
        table,
        static_table!(
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
        )
    );
}

#[test]
fn table_with_empty_margin() {
    let mut data = create_vector::<3, 3>();
    data[2][2] = String::from("https://\nwww\n.\nredhat\n.com\n/en");

    let table = Table::new(&data)
        .with(Style::psql())
        .with(Modify::new(Cell(3, 2)).with(Span::column(2)))
        .with(Margin::new(0, 0, 0, 0).set_fill('>', '<', 'V', '^'))
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
    assert!(is_lines_equal(&table, 20))
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
