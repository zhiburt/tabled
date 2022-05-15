//! The example can be run by this command
//! `cargo run --example colored_borders`
//!
//! This example requires a `color` feature.

use papergrid::Border;
use tabled::{object::Segment, Alignment, Highlight, Modify, Style, Table, Tabled};

#[derive(Tabled)]
struct CodeEditor {
    name: &'static str,
    first_release: &'static str,
    developer: &'static str,
}

impl CodeEditor {
    fn new(name: &'static str, first_release: &'static str, developer: &'static str) -> Self {
        Self {
            name,
            first_release,
            developer,
        }
    }
}

fn main() {
    use owo_colors::OwoColorize;
    use papergrid::Symbol;

    let data = [
        CodeEditor::new("Sublime Text 3", "2008", "Sublime HQ"),
        CodeEditor::new("Visual Studio Code", "2015", "Microsoft"),
        CodeEditor::new("Notepad++", "2003", "Don Ho"),
        CodeEditor::new("GNU Emacs", "1984", "Richard Stallman"),
        CodeEditor::new("Neovim", "2015", "Vim community"),
    ];

    let table = Table::new(&data)
        .with(Style::extended())
        .with(Modify::new(Segment::all()).with(Alignment::left()))
        .with(Highlight::new(
            Segment::all(),
            Border::default()
                .top(Symbol::ansi("═".red().to_string()).unwrap())
                .bottom(Symbol::ansi("═".blue().to_string()).unwrap())
                .left(Symbol::ansi("║".green().to_string()).unwrap())
                .right(Symbol::ansi("║".yellow().to_string()).unwrap())
                .top_left_corner(Symbol::ansi("╔".red().to_string()).unwrap())
                .top_right_corner(Symbol::ansi("╗".red().to_string()).unwrap())
                .bottom_left_corner(Symbol::ansi("╚".blue().to_string()).unwrap())
                .bottom_right_corner(Symbol::ansi("╝".blue().to_string()).unwrap()),
        ));

    println!("{}", table);
}
