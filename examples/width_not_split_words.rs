//! The example can be run by this command
//! `cargo run --example width_not_split_words`

use tabled::{Alignment, Full, MaxWidth, Modify, Style, TableIteratorExt};

fn main() {
    let readme_text = include_str!("../CHANGELOG.md");
    let lines = readme_text.lines().filter(|s| !s.is_empty()).enumerate();

    let table = lines.table().with(Style::modern().horizontal_off()).with(
        Modify::new(Full)
            .with(MaxWidth::wrapping(30).keep_words())
            .with(Alignment::left()),
    );

    println!("{}", table);
}
