//! The example can be run by this command
//! `cargo run --example basic`

use tabled::{table, Alignment, Head, Row, Style, Tabled};

#[derive(Tabled)]
struct Distribution {
    name: &'static str,
    #[field(display_with = "display_based_on")]
    based_on: Option<&'static str>,
    is_active: bool,
    is_cool: bool,
}

fn display_based_on(o: &Option<&'static str>) -> String {
    match o {
        &Some(s) => s.into(),
        None => "Independent".into(),
    }
}

fn main() {
    let data = [
        Distribution {
            name: "Manjaro",
            based_on: Some("Arch"),
            is_cool: true,
            is_active: true,
        },
        Distribution {
            name: "Debian",
            based_on: None,
            is_cool: true,
            is_active: true,
        },
        Distribution {
            name: "Debian",
            based_on: None,
            is_cool: true,
            is_active: true,
        },
    ];

    let table = table!(
        &data,
        Style::pseudo(),
        Alignment::center_horizontal(Head),
        Alignment::left(Row(1..)),
    );

    println!("{}", table);
}
