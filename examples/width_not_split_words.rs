//! The example can be run by this command
//! `cargo run --example width_not_split_words`

use tabled::{object::Segment, Alignment, Modify, Style, TableIteratorExt, Width};

fn main() {
    let readme_text = include_str!("../CHANGELOG.md");
    let lines = readme_text.lines().filter(|s| !s.is_empty()).enumerate();

    let table = lines.table().with(Style::modern().horizontal_off()).with(
        Modify::new(Segment::all())
            .with(Width::wrap(30).keep_words())
            .with(Alignment::left()),
    );

    println!("{}", table);
}
