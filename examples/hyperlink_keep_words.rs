//! To run this example:
//! `cargo run --features=color --example hyperlink`

use std::iter::FromIterator;

use tabled::{object::Segment, Alignment, ModifyObject, Style, Table, Tabled, Width};

#[derive(Tabled)]
struct Distribution {
    name: String,
    is_hyperlink: bool,
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
    // Use cfg macro to check that 'color' feature is enabled
    assert!(
        cfg!(feature = "color"),
        "This example requires the 'color' feature"
    );

    let multicolored_debian = "\x1b[30mDebian\x1b[0m\
    \x1b[31mDebian\x1b[0m\
    \x1b[32mDebian\x1b[0m\
    \x1b[33mDebian\x1b[0m\
    \x1b[34mDebian\x1b[0m\
    \x1b[35mDebian\x1b[0m\
    \x1b[36mDebian\x1b[0m\
    \x1b[37mDebian\x1b[0m\
    \x1b[40mDebian\x1b[0m\
    \x1b[41mDebian\x1b[0m\
    \x1b[42mDebian\x1b[0m\
    \x1b[43mDebian\x1b[0m\
    \x1b[44mDebian\x1b[0m";

    let data = [
        Distribution {
            name: format_osc8_hyperlink("https://www.debian.org/", "Debian"),
            is_hyperlink: true,
        },
        Distribution {
            name: "Debian".into(),
            is_hyperlink: false,
        },
        Distribution {
            name: format_osc8_hyperlink("https://www.debian.org/", multicolored_debian),
            is_hyperlink: true,
        },
        Distribution {
            name: "DebianDebianDebianDebianDebianDebianDebianDebianDebianDebianDebianDebianDebianDebian".into(),
            is_hyperlink: false,
        },
    ];

    let table = Table::from_iter(&data).with(Style::ascii_rounded()).with(
        Segment::all()
            .modify()
            .with(Width::wrap(30).keep_words())
            .with(Alignment::left()),
    );

    println!("{}", table);
}
