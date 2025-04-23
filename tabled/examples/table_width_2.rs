use tabled::{
    settings::{object::Segment, Style, Width},
    Table,
};

fn main() {
    let readme_text = include_str!("../../CHANGELOG.md");
    let lines = readme_text.lines().filter(|s| !s.is_empty()).enumerate();

    let mut table = Table::new(lines);
    table.with(Style::ascii_rounded());
    table.modify(Segment::all(), Width::wrap(30).keep_words(true));

    println!("{table}");
}
