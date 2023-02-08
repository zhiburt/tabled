//! The example can be run by this command
//! `cargo run --example table_width`

use tabled::{
    settings::{
        alignment::Alignment, measurement::Percent, object::Segment, style::Style, width::Width,
        Modify,
    },
    Table,
};

fn main() {
    let data = [
        ["Hello World!!!", "3.3.22.2"],
        ["Guten Morgen", "1.1.1.1"],
        ["Добры вечар", "127.0.0.1"],
        ["Bonjour le monde", ""],
        ["Ciao mondo", ""],
    ];

    let mut table = Table::builder(data).build();
    table
        .with(Style::markdown())
        .with(Alignment::left());

    println!("Original table\n{}\n", table);

    table.with(Width::truncate(20).suffix("..."));

    println!("Truncated table\n{}\n", table);

    table.with(Modify::new(Segment::all()).with(Width::wrap(5)));

    println!("Wrapped table\n{}\n", table);

    table.with(Width::increase(Percent(200)));

    println!("Widen table\n{}", table);
}
