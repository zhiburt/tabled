//! The example can be run by this command
//! `cargo run --example border_text`

use tabled::{
    object::Segment, style::HorizontalLine, Alignment, BorderText, ModifyObject, Style, Table,
};

fn main() {
    let data = [[5, 6, 7, 8, 9], [10, 11, 12, 13, 14]];

    let table = Table::new(&data)
        .with(
            Style::modern()
                .off_horizontal()
                .lines([HorizontalLine::new(1, Style::modern().get_horizontal())]),
        )
        .with(BorderText::first(" Numbers "))
        .with(BorderText::new(1, " More numbers "))
        .with(BorderText::last(" end. "))
        .with(Segment::all().modify().with(Alignment::left()));

    println!("{}", table);
}
