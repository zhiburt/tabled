//! To run this example:
//! `cargo run --features=color --example hyperlink`

use std::iter::FromIterator;

use tabled::{object::Segment, Alignment, Modify, Style, Table, Tabled, Width};

fn main() {
    let multicolored_debian = "\x1b[30mDebian\x1b[0m\
    \x1b[31m Debian\x1b[0m\
    \x1b[32m Debian\x1b[0m\
    \x1b[33m Debian\x1b[0m\
    \x1b[34m Debian\x1b[0m\
    \x1b[35m Debian\x1b[0m\
    \x1b[36m Debian\x1b[0m\
    \x1b[37m Debian\x1b[0m\
    \x1b[40m Debian\x1b[0m\
    \x1b[41m Debian\x1b[0m\
    \x1b[42m Debian\x1b[0m\
    \x1b[43m Debian\x1b[0m\
    \x1b[44m Debian\x1b[0m";

    let debian_repeat =
        "DebianDebianDebianDebianDebianDebianDebianDebianDebianDebianDebianDebianDebianDebian"
            .to_string();

    let debian_colored_link = format_osc8_hyperlink("https://www.debian.org/", multicolored_debian);
    let debian_link = format_osc8_hyperlink("https://www.debian.org/", "Debian");
    let wiki_link = format_osc8_hyperlink("https://www.wikipedia.org/", "Debian");

    let data = [
        Distribution::new("Debian".into(), false),
        Distribution::new(debian_link.clone(), true),
        Distribution::new(format!("{} a link followed by text", debian_link), true),
        Distribution::new(
            format!("{} links with intervening text {}", debian_link, wiki_link),
            true,
        ),
        Distribution::new(format!("a link surrounded {} by text", debian_link), true),
        Distribution::new(debian_colored_link, true),
        Distribution::new(debian_repeat, false),
    ];

    let mut table = Table::from_iter(&data);
    table
        .with(Style::ascii_rounded())
        .with(Alignment::left())
        .with(Modify::new(Segment::all()).with(Width::wrap(16).keep_words()));

    println!("{}", table);

    let mut table = Table::from_iter(&data);
    table
        .with(Style::ascii_rounded())
        .with(Alignment::left())
        .with(Modify::new(Segment::all()).with(Width::wrap(16)));

    println!("{}", table);
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

fn format_osc8_hyperlink(url: &str, text: &str) -> String {
    format!(
        "{osc}8;;{url}{st}{text}{osc}8;;{st}",
        url = url,
        text = text,
        osc = "\x1b]",
        st = "\x1b\\"
    )
}
