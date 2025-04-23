use tabled::{
    settings::{style::Style, themes::Theme, Color},
    Table, Tabled,
};

#[derive(Tabled)]
struct Editor {
    name: String,
    dev: String,
    release: usize,
}
fn main() {
    #[rustfmt::skip]
    let data = [
        Editor { name: String::from("Sublime Text 3"),      release: 2008, dev: String::from("Sublime HQ") },
        Editor { name: String::from("Visual Studio Code"),  release: 2015, dev: String::from("Microsoft") },
        Editor { name: String::from("Notepad++"),           release: 2003, dev: String::from("Don Ho") },
        Editor { name: String::from("GNU Emacs"),           release: 1984, dev: String::from("Richard Stallman") },
        Editor { name: String::from("Neovim"),              release: 2015, dev: String::from("Vim community") },
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
