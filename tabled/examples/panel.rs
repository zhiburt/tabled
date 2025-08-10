use tabled::{
    settings::{style::Style, themes::BorderCorrection, Alignment, Panel},
    Table, Tabled,
};

#[derive(Tabled)]
struct Release<'a> {
    version: &'a str,
    published_date: &'a str,
    features: &'a str,
}

#[rustfmt::skip]
const DATA: [Release<'static>; 3] = [
    Release { version: "0.2.1", published_date: "2021-06-23", features: "#[header(inline)] attribute" },
    Release { version: "0.2.0", published_date: "2021-06-19", features: "API changes" },
    Release { version: "0.1.4", published_date: "2021-06-07", features: "display_with attribute" },
];

fn main() {
    let mut table = Table::new(DATA);
    table
        .with(Panel::header("Tabled Releases"))
        .with(Panel::footer(format!("N - {}", DATA.len())))
        .with(Panel::vertical(0, "Some text goes here").width(1))
        .with(Panel::vertical(5, "Some text goes here").width(1))
        .with(Alignment::center())
        .with(Style::modern())
        .with(BorderCorrection::span());

    println!("{table}");
}
