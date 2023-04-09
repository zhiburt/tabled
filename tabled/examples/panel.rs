//! This example demonstrates using the [`Panel`] [`TableOption`] to inject
//! table-length columns and rows into a [`Table`].
//!
//! * [`Panel`] supports four injection options:
//!     * Horizontal | manual index selection
//!     * Vertical | manual index selection
//!     * Header | before first row
//!     * Footer | after last row

use tabled::{
    settings::{
        object::Segment,
        style::{BorderSpanCorrection, Style},
        Alignment, Modify, Panel, Width,
    },
    Table, Tabled,
};

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
    let mut table = Table::new(DATA);
    table
        .with(Panel::header("Tabled Releases"))
        .with(Panel::footer(format!("N - {}", DATA.len())))
        .with(Panel::vertical(0, "Some text goes here"))
        .with(Panel::vertical(5, "Some text goes here"))
        .with(Modify::new((0, 0)).with(Width::wrap(1)))
        .with(Modify::new((0, 5)).with(Width::wrap(1)))
        .with(Modify::new(Segment::all()).with(Alignment::center()))
        .with(Style::modern())
        .with(BorderSpanCorrection);

    println!("{table}");
}
