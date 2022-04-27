//! The example can be run by this command
//! `cargo run --example index`

use tabled::{Style, Table, Tabled};

#[derive(Tabled)]
struct Distribution {
    name: &'static str,
    #[tabled(display_with = "display_based_on")]
    based_on: Option<&'static str>,
    is_active: bool,
    is_cool: bool,
}

fn display_based_on(o: &Option<&'static str>) -> String {
    match o {
        &Some(s) => s.into(),
        None => "Unknown".into(),
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

    let table = Table::builder(&data)
        .index()
        .set_index(0)
        .set_name(None)
        .transpose()
        .build()
        .with(Style::modern());

    println!("{}", table);
}
