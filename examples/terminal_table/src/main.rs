//! The example shows how we could spread a table to the size of a terminal.

use tabled::{Style, TableIteratorExt, Tabled, Width};

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
    let (terminal_size::Width(width), _) = terminal_size::terminal_size().unwrap();

    let table = DATA
        .table()
        .with(Style::extended())
        .with(Width::wrap(width as usize))
        .with(Width::increase(width as usize));

    println!("{}", table);
}
