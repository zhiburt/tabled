use std::iter;

use tabled::{
    builder::Builder,
    settings::{style::Style, themes::Colorization, Color},
};

fn main() {
    let mut b = Builder::new();
    b.extend(["♜", "♞", "♝", "♛", "♚", "♝", "♞", "♜"]);
    b.extend(["♟", "♟", "♟", "♟", "♟", "♟", "♟", "♟"]);
    b.extend(iter::empty::<String>());
    b.extend(iter::empty::<String>());
    b.extend(iter::empty::<String>());
    b.extend(iter::empty::<String>());
    b.extend(["♙", "♙", "♙", "♙", "♙", "♙", "♙", "♙"]);
    b.extend(["♖", "♘", "♗", "♕", "♔", "♗", "♘", "♖"]);

    let mut board = b.build();
    board
        .with(Style::empty())
        .with(Colorization::chess(Color::BG_WHITE, Color::BG_BLACK));

    println!("{board}");
}
