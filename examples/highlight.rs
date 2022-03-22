//! The example can be run by this command
//! `cargo run --example highlight`

use tabled::{
    object::{Columns, Object, Rows},
    style::{Border, Style},
    Highlight, TableIteratorExt,
};

fn main() {
    let data = vec![["A", "B", "C"], ["D", "E", "F"], ["G", "H", "I"]];

    let table = data.table().with(Style::modern()).with(Highlight::new(
        Rows::first().and(Columns::single(1)),
        Border::filled('*'),
    ));

    println!("{}", table);

    assert_eq!(
        table.to_string(),
        "*************\n\
         * 0 │ 1 │ 2 *\n\
         *****───*****\n\
         │ A * B * C │\n\
         ├───*───*───┤\n\
         │ D * E * F │\n\
         ├───*───*───┤\n\
         │ G * H * I │\n\
         └───*****───┘\n"
    )
}
