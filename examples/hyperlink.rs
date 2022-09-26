//! To run this example:
//! `cargo run --features=color --example hyperlink`

use std::iter::FromIterator;

use tabled::{object::Segment, Alignment, ModifyObject, Style, Table, Tabled, Width};

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

fn main() {
    let multicolored_debian = "\x1b[30mDebian\x1b[0m\
    \x1b[31mDebian \x1b[0m\
    \x1b[32mDebian \x1b[0m\
    \x1b[33mDebian \x1b[0m\
    \x1b[34mDebian \x1b[0m\
    \x1b[35mDebian \x1b[0m\
    \x1b[36mDebian \x1b[0m\
    \x1b[37mDebian \x1b[0m\
    \x1b[40mDebian \x1b[0m\
    \x1b[41mDebian \x1b[0m\
    \x1b[42mDebian \x1b[0m\
    \x1b[43mDebian \x1b[0m\
    \x1b[44mDebian \x1b[0m";

    let data = [
        Distribution::new(
            format_osc8_hyperlink("https://www.debian.org/", "Debian"),
            true,
        ),
        Distribution::new(
            format!(
                "{}---- 1 link followed by text",
                format_osc8_hyperlink("https://www.debian.org/", "Debian"),
            ),
            true,
        ),
        Distribution::new(
            format!(
                "{} 2 links with intervening text {}",
                format_osc8_hyperlink("https://www.debian.org/", "Debian"),
                format_osc8_hyperlink("https://www.wikipedia.org/", "Debian"),
            ),
            true,
        ),
        Distribution::new(
            format!(
                "a link surrounded {} by text",
                format_osc8_hyperlink("https://www.debian.org/", "Debian"),
            ),
            true,
        ),
        Distribution::new("Debian".into(), false),
        Distribution::new(
            format_osc8_hyperlink("https://www.debian.org/", multicolored_debian),
            true,
        ),
        Distribution::new(
            "DebianDebianDebianDebianDebianDebianDebianDebianDebianDebianDebianDebianDebianDebian"
                .into(),
            false,
        ),
    ];

    let mut table = Table::from_iter(&data);
    table
        .with(Style::ascii_rounded())
        .with(Alignment::left())
        .with(Segment::all().modify().with(Width::wrap(16).keep_words()));

    println!("{}", table);

    let mut table = Table::from_iter(&data);
    table
        .with(Style::ascii_rounded())
        .with(Alignment::left())
        .with(Segment::all().modify().with(Width::wrap(16)));

    println!("{}", table);
}
