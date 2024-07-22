use std::iter::FromIterator;

use tabled::{
    settings::{Padding, Reverse, Shadow, Style},
    Table,
};

fn main() {
    let data = [
        ["string", "support", "wood", "station", "should", "height"],
        ["shaking", "sweet", "answer", "could", "over", "rough"],
        ["equator", "save", "pig", "camera", "alone", "office"],
        ["eight", "act", "image", "attached", "gone", "zero"],
        ["applied", "sense", "use", "shoe", "born", "care"],
        ["easier", "shout", "noun", "applied", "rear", "crowd"],
        ["coal", "flag", "current", "nearby", "expect", "heading"],
    ];

    let mut table = Table::from_iter(data);
    table.with(Style::rounded().remove_horizontals().remove_vertical());
    table.with(Padding::new(2, 2, 0, 0));
    table.with(Shadow::new(3).set_offset(6));
    table.with(Reverse::rows(1, 0));
    table.with(Reverse::columns(0, 0));

    println!("{table}");
}
