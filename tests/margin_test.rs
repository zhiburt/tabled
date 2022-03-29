use crate::util::create_vector;
use tabled::{
    style::{Border, Style},
    Cell, Highlight, Margin, Modify, Span, Table,
};

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

    let expected = concat!(
        "VVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVV\n",
        ">+++++══════════╦══════════╦══════════╗<<\n",
        ">+ N + column 0 ║ column 1 ║ column 2 ║<<\n",
        ">++++************══════════╬══════════╣<<\n",
        ">║ 0 *   0-0    *   0-1    ║   0-2    ║<<\n",
        ">╠═══************══════════╬══════════╣<<\n",
        ">║ 1 ║   1-0    ║   1-1    ║   1-2    ║<<\n",
        ">╠═══╬══════════╬══════════╬══════════╣<<\n",
        ">║ 2 ║   2-0    ║   2-1    ║   2-2    ║<<\n",
        ">╚═══╩══════════╩══════════╩══════════╝<<\n",
        "^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n",
        "^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n",
    );

    println!("{}", table);

    assert_eq!(table, expected);
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

    let expected = concat!(
        "VVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVV\n",
        "> N | column 0 | column 1 | column 2 <\n",
        ">---+----------+----------+----------<\n",
        "> 0 |   0-0    |   0-1    |   0-2    <\n",
        "> 1 |   1-0    |   1-1    |   1-2    <\n",
        "> 2 |   2-0    |      https://       <\n",
        ">   |          |      www            <\n",
        ">   |          |      .              <\n",
        ">   |          |      redhat         <\n",
        ">   |          |      .com           <\n",
        ">   |          |      /en            <\n",
        "^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n",
    );

    println!("{}", table);

    assert_eq!(table, expected);
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

    println!("{}", table);

    assert_eq!(table, expected);
}
