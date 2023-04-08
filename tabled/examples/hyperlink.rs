//! This example demonstrates how hyperlinks can be embedded into a [`Table`] display.
//!
//! While not a [`tabled`] specific implementation, it is helpful to know that
//! most users expect certain elements of interactivity based on the purpose of your display.
//!
//! * 🚩 This example requires the `color` feature.
//!
//! * ⚠️ Terminal interfaces may differ in how they parse links or make them interactive.
//! [`tabled`] doesn't have the final say on whether a link is clickable or not.

use tabled::{
    settings::{object::Segment, Alignment, Modify, Style, Width},
    Table, Tabled,
};

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
        Distribution::new(format!("{debian_link} a link followed by text"), true),
        Distribution::new(
            format!("{debian_link} links with intervening text {wiki_link}"),
            true,
        ),
        Distribution::new(format!("a link surrounded {debian_link} by text"), true),
        Distribution::new(debian_colored_link, true),
        Distribution::new(debian_repeat, false),
    ];

    let mut table = Table::new(&data);
    table
        .with(Style::ascii_rounded())
        .with(Alignment::left())
        .with(Modify::new(Segment::all()).with(Width::wrap(16).keep_words()));

    println!("{table}");

    let mut table = Table::new(&data);
    table
        .with(Style::ascii_rounded())
        .with(Alignment::left())
        .with(Modify::new(Segment::all()).with(Width::wrap(16)));

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

fn format_osc8_hyperlink(url: &str, text: &str) -> String {
    format!("\x1b]8;;{url}\x1b\\{text}\x1b]8;;\x1b\\",)
}
