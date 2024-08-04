//! This example demonstrates using the [`RawStyle`] [setting](tabled::settings) to
//! to granularly specify border colors.
//!
//! * 🚩 This example requires the `color` feature.
//!
//! * Note how [`Color`] contains several helpful, const values covering
//!   a basic selection of foreground and background colors. [`Color`] also
//!   supports custom colors with [`Color::new()`].

use tabled::{
    settings::{style::Style, themes::Theme, Color},
    Table, Tabled,
};

#[derive(Tabled)]
struct CodeEditor {
    name: String,
    first_release: String,
    developer: String,
}

impl CodeEditor {
    fn new(name: &str, first_release: &str, developer: &str) -> Self {
        Self {
            name: name.to_string(),
            first_release: first_release.to_string(),
            developer: developer.to_string(),
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

    let mut style = Theme::from(Style::extended());
    style.set_colors_top(Color::FG_RED);
    style.set_colors_bottom(Color::FG_CYAN);
    style.set_colors_left(Color::FG_BLUE);
    style.set_colors_right(Color::FG_GREEN);
    style.set_colors_corner_top_left(Color::FG_BLUE);
    style.set_colors_corner_top_right(Color::FG_RED);
    style.set_colors_corner_bottom_left(Color::FG_CYAN);
    style.set_colors_corner_bottom_right(Color::FG_GREEN);
    style.set_colors_intersection_bottom(Color::FG_CYAN);
    style.set_colors_intersection_top(Color::FG_RED);
    style.set_colors_intersection_right(Color::FG_GREEN);
    style.set_colors_intersection_left(Color::FG_BLUE);
    style.set_colors_intersection(Color::FG_MAGENTA);
    style.set_colors_horizontal(Color::FG_MAGENTA);
    style.set_colors_vertical(Color::FG_MAGENTA);

    let table = Table::new(data).with(style).to_string();

    println!("{table}");
}
