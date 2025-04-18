//! The example shows how we could spread a table to the size of a terminal.

use tabled::{
    settings::{peaker::Priority, Height, Width},
    Table,
};
use terminal_size::{terminal_size, Height as TerminalHeight, Width as TerminalWidth};

fn main() {
    let (width, height) = get_terminal_size();

    let data = [
        ["0.2.1", "2021-06-23", "true", "#[header(inline)] attribute"],
        ["0.2.0", "2021-06-19", "false", "API changes"],
        ["0.1.4", "2021-06-07", "false", "display_with attribute"],
    ];

    let mut table = Table::from_iter(data);
    table
        .with(Width::wrap(width).priority(Priority::right()))
        .with(Width::increase(width))
        .with(Height::limit(height))
        .with(Height::increase(height));

    println!("{table}");
}

fn get_terminal_size() -> (usize, usize) {
    let (TerminalWidth(width), TerminalHeight(height)) =
        terminal_size().expect("failed to obtain a terminal size");

    (width as usize, height as usize)
}
