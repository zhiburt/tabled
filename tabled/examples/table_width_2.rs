//! The example can be run by this command
//! `cargo run --example table_width_2`

use tabled::{
    settings::{object::Segment, Alignment, Modify, Style, Width},
    Table,
};

fn main() {
    let readme_text = include_str!("../../CHANGELOG.md");
    let lines = readme_text.lines().filter(|s| !s.is_empty()).enumerate();

    let mut table = Table::new(lines);
    table.with(Style::ascii_rounded()).with(
        Modify::new(Segment::all())
            .with(Width::wrap(30).keep_words())
            .with(Alignment::left()),
    );

    println!("{table}");
}
