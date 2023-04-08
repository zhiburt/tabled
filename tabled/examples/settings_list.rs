//! This example demonstrates using the [`Settings`] [`TableOption`] to array
//! [`Table`] configurations in a separate step from instantiation.
//!
//! * Note how this methodoly can lead to huge performance gains
//! with compile-time constants.

use tabled::{
    settings::{
        object::{FirstRow, Rows},
        style::On,
        Alignment, Modify, ModifyList, Padding, Settings, Style,
    },
    Table, Tabled,
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

// unfortunately we can't leave it as a blank type, so we need to provide it.
type TableTheme = Settings<
    Settings<Settings<Settings, Style<On, On, On, On, On, On>>, Padding>,
    ModifyList<FirstRow, Alignment>,
>;

const THEME: TableTheme = Settings::empty()
    .with(Style::ascii())
    .with(Padding::new(1, 3, 0, 0))
    .with(Modify::list(Rows::first(), Alignment::center()));

fn main() {
    let data = [
        CodeEditor::new("Sublime Text 3", "2008", "Sublime HQ"),
        CodeEditor::new("Visual Studio Code", "2015", "Microsoft"),
        CodeEditor::new("Notepad++", "2003", "Don Ho"),
        CodeEditor::new("GNU Emacs", "1984", "Richard Stallman"),
        CodeEditor::new("Neovim", "2015", "Vim community"),
    ];

    let mut table = Table::new(data);
    table.with(THEME);

    println!("{table}");
}
