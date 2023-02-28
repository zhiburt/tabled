//! The example can be run by this command
//! `cargo run --example extended_display`

use tabled::{tables::extended::ExtendedTable, Tabled};

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
    let data = vec![
        Distribution::new("Manjaro", "Arch", true, true),
        Distribution::new("Arch", "", true, true),
        Distribution::new("Debian", "", true, true),
    ];

    let table = ExtendedTable::new(data);

    println!("{table}");
}
