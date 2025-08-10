use std::iter::FromIterator;

use tabled::{grid::config::Offset, settings::Reverse, Table};

fn main() {
    let data = [
        ["string", "support", "wood", "station"],
        ["applied", "sense", "use", "shoe"],
        ["shout", "noun", "rear", "crowd"],
        ["coal", "flag", "current", "heading"],
    ];

    let mut table = Table::from_iter(data);
    table.with(Reverse::rows(1).limit(Offset::End(1)));
    table.with(Reverse::columns(0));

    println!("{table}");
}
