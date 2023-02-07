//! The example can be run by this command
//! `cargo run --example table_width_2`

use tabled::{object::Segment, Alignment, Modify, Style, TableIteratorExt, Width};

fn main() {
    let readme_text = include_str!("../CHANGELOG.md");
    let lines = readme_text.lines().filter(|s| !s.is_empty()).enumerate();

    let mut table = lines.table();
    table.with(Style::ascii_rounded()).with(
        Modify::new(Segment::all())
            .with(Width::wrap(30).keep_words())
            .with(Alignment::left()),
    );

    println!("{}", table);
}
