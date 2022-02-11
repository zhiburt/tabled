//! The example can be run by this command
//! `cargo run --example basic`

use tabled::{Alignment, AlignmentHorizontal, Head, Modify, Row, Style, Table, Tabled};

#[derive(Tabled)]
struct Distribution {
    name: &'static str,
    #[tabled(display_with = "Self::display_based_on")]
    based_on: Option<&'static str>,
    is_active: bool,
    is_cool: bool,
}

impl Distribution {
    fn display_based_on(o: &Option<&'static str>) -> String {
        match o {
            &Some(s) => s.into(),
            None => "Independent".into(),
        }
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

    let table = Table::new(&data)
        .with(Style::modern())
        .with(Modify::new(Head).with(Alignment::Horizontal(AlignmentHorizontal::Center)))
        .with(Modify::new(Row(1..)).with(Alignment::Horizontal(AlignmentHorizontal::Left)));

    println!("{}", table);
}
