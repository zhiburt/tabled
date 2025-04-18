use std::iter::FromIterator;

use tabled::{
    settings::{Padding, Shadow, Style},
    Table,
};

fn main() {
    let mut data = vec![];
    for i in 0..4 {
        data.push((i..i + 5).map(|v| v.to_string()));
    }

    let mut table = Table::from_iter(data);
    table.with(Style::rounded().remove_horizontals().remove_vertical());
    table.with(Padding::new(2, 2, 0, 0));
    table.with(Shadow::new(3).set_offset(6));

    println!("{table}");
}
