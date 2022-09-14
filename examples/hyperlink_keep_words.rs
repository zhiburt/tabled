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
            name: format_osc8_hyperlink(
                "https://www.debian.org/",
                "DebianDebianDebianDebianDebianDebianDebianDebianDebianDebianDebianDebianDebianDebian",
            ),
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
