//! The example can be run by this command
//! `cargo run --example table_width_2`

use tabled::{object::Segment, Alignment, ModifyObject, Style, TableIteratorExt, Width};

fn main() {
    let readme_text = include_str!("../CHANGELOG.md");
    let lines = readme_text.lines().filter(|s| !s.is_empty()).enumerate();

    let table = lines.table().with(Style::ascii_rounded()).with(
        Segment::all()
            .modify()
            .with(Width::wrap(30).keep_words())
            .with(Alignment::left()),
    );

    println!("{}", table);
}
