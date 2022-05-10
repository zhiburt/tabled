//! The example can be run by this command
//! `cargo run --example custom_style`

use tabled::{object::Full, Alignment, Modify, Style, Table, Tabled};

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

    let table = Table::new(&data)
        .with(Style::modern().horizontal_off().vertical_off().header(' '))
        .with(Modify::new(Full).with(Alignment::left()));

    println!("{}", table);
}
