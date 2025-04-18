use tabled::{
    settings::{
        style::{HorizontalLine, Style, VerticalLine},
        Alignment,
    },
    Table, Tabled,
};

#[derive(Tabled)]
struct Editor<'a> {
    name: &'a str,
    developer: &'a str,
    first_release: usize,
}

fn main() {
    #[rustfmt::skip]
    let data = [
        Editor { name: "Sublime Text 3", first_release: 2008, developer: "Sublime HQ" },
        Editor { name: "Visual Studio Code", first_release: 2015, developer: "Microsoft" },
        Editor { name: "Notepad++", first_release: 2003, developer: "Don Ho" },
        Editor { name: "GNU Emacs", first_release: 1984, developer: "Richard Stallman" },
        Editor { name: "Neovim", first_release: 2015, developer: "Vim community" },
    ];

    let mut table = Table::new(data);
    table.with(
        Style::modern()
            .horizontals([(1, HorizontalLine::inherit(Style::modern()))])
            .verticals([(1, VerticalLine::inherit(Style::modern()))])
            .remove_horizontal()
            .remove_vertical(),
    );
    table.with(Alignment::center());

    println!("{table}");
}
