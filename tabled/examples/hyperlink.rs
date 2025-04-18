use tabled::{
    settings::{object::Segment, Color, Style, Width},
    Table, Tabled,
};

fn main() {
    let colors = [
        Color::FG_BLACK,
        Color::FG_BLUE,
        Color::FG_GREEN,
        Color::FG_RED,
        Color::FG_MAGENTA,
        Color::FG_CYAN,
        Color::BG_BLACK,
        Color::BG_BLUE,
        Color::BG_GREEN,
        Color::BG_RED,
        Color::BG_MAGENTA,
        Color::BG_CYAN,
    ];

    let debian_multicolored = colors
        .iter()
        .map(|color| color.colorize("Debian"))
        .collect::<Vec<_>>()
        .join(" ");

    let debian_big = std::iter::repeat_n("Debian", 12).collect::<String>();

    let debian_multicolored_link = hyperlink("https://www.debian.org/", &debian_multicolored);
    let debian_link = hyperlink("https://www.debian.org/", "Debian");
    let wiki_link = hyperlink("https://www.wikipedia.org/", "Debian");

    let data = [
        Distribution::new("Debian".into(), false),
        Distribution::new(debian_link.clone(), true),
        Distribution::new(format!("{debian_link} a link followed by text"), true),
        Distribution::new(
            format!("{debian_link} links with intervening text {wiki_link}"),
            true,
        ),
        Distribution::new(format!("a link surrounded {debian_link} by text"), true),
        Distribution::new(debian_multicolored_link, true),
        Distribution::new(debian_big, false),
    ];

    let mut table = Table::new(&data);
    table.with(Style::ascii().remove_horizontal());
    table.modify(Segment::all(), Width::wrap(20).keep_words(true));
    println!("{table}");

    let mut table = Table::new(&data);
    table.with(Style::ascii().remove_horizontal());
    table.modify(Segment::all(), Width::wrap(20));
    println!("{table}");
}

#[derive(Tabled)]
struct Distribution {
    name: String,
    is_hyperlink: bool,
}

impl Distribution {
    fn new(name: String, is_hyperlink: bool) -> Self {
        Self { name, is_hyperlink }
    }
}

fn hyperlink(url: &str, text: &str) -> String {
    format!("\x1b]8;;{url}\x1b\\{text}\x1b]8;;\x1b\\",)
}
