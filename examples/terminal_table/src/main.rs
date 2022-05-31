//! The example shows how we could spread a table to the size of a terminal.

use tabled::{object::Segment, Alignment, Modify, Style, TableIteratorExt, Tabled, Width};

#[derive(Tabled)]
struct Release {
    version: &'static str,
    published_date: &'static str,
    is_active: bool,
    major_feature: &'static str,
}

const DATA: [Release; 3] = [
    Release {
        version: "0.2.1",
        published_date: "2021-06-23",
        is_active: true,
        major_feature: "#[header(inline)] attribute",
    },
    Release {
        version: "0.2.0",
        published_date: "2021-06-19",
        is_active: false,
        major_feature: "API changes",
    },
    Release {
        version: "0.1.4",
        published_date: "2021-06-07",
        is_active: false,
        major_feature: "display_with attribute",
    },
];

fn main() {
    let (terminal_size::Width(width), _) = terminal_size::terminal_size().unwrap();

    let table = DATA
        .table()
        .with(Style::extended())
        .with(Modify::new(Segment::all()).with(Alignment::left()))
        .with(Width::wrap(width as usize).keep_words())
        .with(Width::increase(width as usize));

    println!("{}", table);
}
