use tabled::{
    settings::{
        object::{FirstRow, Rows},
        style::{On, Style},
        Alignment, Modify, ModifyList, Padding, Settings,
    },
    Table, Tabled,
};

#[derive(Tabled)]
struct Editor {
    name: &'static str,
    developer: &'static str,
    first_release: usize,
}

const THEME: Settings<
    Settings<Settings<Settings, Style<On, On, On, On, On, On, 0, 0>>, Padding>,
    ModifyList<FirstRow, Alignment>,
> = Settings::empty()
    .with(Style::ascii())
    .with(Padding::new(1, 3, 0, 0))
    .with(Modify::list(Rows::first(), Alignment::center()));

fn main() {
    #[rustfmt::skip]
    let data = [
        Editor { name: "Sublime Text 3", developer: "Sublime HQ", first_release: 2008 },
        Editor { name: "Visual Studio Code", developer: "Microsoft", first_release: 2015 },
        Editor { name: "Notepad++", developer: "Don Ho", first_release: 2003 },
        Editor { name: "GNU Emacs", developer: "Richard Stallman", first_release: 1984 },
        Editor { name: "Neovim", developer: "Vim community", first_release: 2015 },
    ];

    let mut table = Table::new(data);
    table.with(THEME);

    println!("{table}");
}
