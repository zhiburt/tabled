//! This example demonstrates customizing one of the [`tabled`] default [styles](Style)
//! to create a unique [`Table`] display.
//!
//! * Note that all predesigned styles can be configured completely.
//! Styles can also be created from scratch!
//!
//! * Note that adding and removing borders with a [`Style`] theme doesn't affect the
//! number of functional columns and rows.

use tabled::{
    settings::{
        style::{HorizontalLine, Style, VerticalLine},
        Alignment,
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

fn main() {
    let data = [
        CodeEditor::new("Sublime Text 3", "2008", "Sublime HQ"),
        CodeEditor::new("Visual Studio Code", "2015", "Microsoft"),
        CodeEditor::new("Notepad++", "2003", "Don Ho"),
        CodeEditor::new("GNU Emacs", "1984", "Richard Stallman"),
        CodeEditor::new("Neovim", "2015", "Vim community"),
    ];

    // HorizontalLine::from_style(Style::modern());
    // Style::modern().get_horizontal_line();

    // StyleProjection::style(Style::modern()).left().right().middle()

    // The question is do we need the geters in style as it's SORT OF a builder...
    // I guess we have to... to -> char but not Option<char>

    // this is a good Idea; so we could remove all geters from Style.

    // todo: lines -> source of truce as we do with frame....

    // todo: maybe turn down custom iterators support and just stick with a const arrays..., custom ones will be achivable by RawStyle;
    //  this way we will able to remove the Iterators too...
    //  and have separate methods

    let theme = Style::modern().remove_horizontal().remove_vertical().lines(
        [(
            1,
            Style::modern().get_horizontal_line().remove_intersection(),
        )],
        [(1, Style::modern().get_vertical_line().remove_intersection())],
    ).;

    let mut table = Table::new(data);
    table.with(theme).with(Alignment::left());

    println!("{table}");
}
