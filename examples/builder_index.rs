//! The example can be run by this command
//! `cargo run --example builder_index`

use tabled::{Style, Table, Tabled};

#[derive(Tabled)]
struct Distribution {
    name: &'static str,
    based_on: &'static str,
    is_active: bool,
    is_cool: bool,
}

impl Distribution {
    fn new(name: &'static str, based_on: &'static str, is_active: bool, is_cool: bool) -> Self {
        Self {
            name,
            based_on,
            is_active,
            is_cool,
        }
    }
}

fn main() {
    let data = [
        Distribution::new("Manjaro", "Arch", true, true),
        Distribution::new("Arch", "None", true, true),
        Distribution::new("Debian", "None", true, true),
    ];

    let mut builder = Table::builder(&data).index();
    builder.set_index(0).set_name(None).transpose();

    let mut table = builder.build();
    table.with(Style::modern());

    println!("{table}");
}
