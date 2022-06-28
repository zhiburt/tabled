//! The example can be run by this command
//! `cargo run --example margin`

use tabled::{Margin, Style, TableIteratorExt};

fn main() {
    let data = vec![["A", "B", "C"], ["D", "E", "F"], ["G", "H", "I"]];

    let table = data
        .table()
        .with(Style::re_structured_text())
        .with(Margin::new(3, 3, 1, 0).set_fill('x', 'y', 'j', ' '));

    println!("{}", table);

    assert_eq!(
        table.to_string(),
        "jjjjjjjjjjjjjjjjj\n\
         xxx=== === ===yyy\n\
         xxx 0   1   2 yyy\n\
         xxx=== === ===yyy\n\
         xxx A   B   C yyy\n\
         xxx D   E   F yyy\n\
         xxx G   H   I yyy\n\
         xxx=== === ===yyy"
    )
}
