//! The example can be run by this command
//! `cargo run --example panel`

use tabled::{object::Segment, Alignment, Modify, Panel, Style, Table, Tabled};

#[derive(Tabled)]
struct Release {
    version: &'static str,
    published_date: &'static str,
    is_active: bool,
    major_feature: &'static str,
}

impl Release {
    const fn new(
        version: &'static str,
        published_date: &'static str,
        is_active: bool,
        major_feature: &'static str,
    ) -> Self {
        Self {
            version,
            published_date,
            is_active,
            major_feature,
        }
    }
}

const DATA: [Release; 3] = [
    Release::new("0.2.1", "2021-06-23", true, "#[header(inline)] attribute"),
    Release::new("0.2.0", "2021-06-19", false, "API changes"),
    Release::new("0.1.4", "2021-06-07", false, "display_with attribute"),
];

fn main() {
    let table = Table::new(DATA)
        .with(Panel::header("Tabled Releases"))
        .with(Panel::footer(format!("N - {}", DATA.len())))
        .with(Panel::vertical(0).text("Some text goes here").text_width(1))
        .with(Panel::vertical(5).text("Some text goes here").text_width(1))
        .with(Modify::new(Segment::all()).with(Alignment::center()))
        .with(Style::modern())
        .with(Style::correct_spans());

    println!("{}", table);
}
