//! The example can be run by this command
//! `cargo run --example custom_style`

use tabled::{
    object::Segment,
    style::{HorizontalLine, VerticalLine},
    Alignment, ModifyObject, Style, Table, Tabled,
};

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
    let data = [
        CodeEditor::new("Sublime Text 3", "2008", "Sublime HQ"),
        CodeEditor::new("Visual Studio Code", "2015", "Microsoft"),
        CodeEditor::new("Notepad++", "2003", "Don Ho"),
        CodeEditor::new("GNU Emacs", "1984", "Richard Stallman"),
        CodeEditor::new("Neovim", "2015", "Vim community"),
    ];

    let theme = Style::modern()
        .off_horizontal()
        .off_vertical()
        .horizontals([HorizontalLine::new(1, Style::modern().get_horizontal()).intersection(None)])
        .verticals([VerticalLine::new(1, Style::modern().get_vertical())]);

    let table = Table::new(&data)
        .with(theme)
        .with(Segment::all().modify().with(Alignment::left()));

    println!("{}", table);
}
