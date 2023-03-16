//! The example can be run by this command
//! `cargo run --example border_text`

use tabled::{
    settings::{
        object::Rows,
        style::{BorderText, HorizontalLine, Style},
    },
    Table,
};

fn main() {
    let data = [[5, 6, 7, 8, 9], [10, 11, 12, 13, 14]];

    let table = Table::new(data)
        .with(
            Style::modern()
                .remove_horizontal()
                .horizontals([HorizontalLine::new(1, Style::modern().get_horizontal())]),
        )
        .with(BorderText::new(" Numbers ").horizontal(Rows::first()))
        .with(BorderText::new(" More numbers ").horizontal(1))
        .with(BorderText::new(" end. ").horizontal(Rows::last()))
        .to_string();

    println!("{table}");
}
