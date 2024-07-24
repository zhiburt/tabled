//! This example demonstrates using [ExtendedTable], a [Table] alternative with
//! limited flexibility but a greater emphasis on large data displays.

use tabled::{tables::ExtendedTable, Tabled};

#[derive(Tabled)]
struct Distribution {
    name: String,
    based_on: String,
    is_active: bool,
    is_cool: bool,
}

impl Distribution {
    fn new(name: &str, based_on: &str, is_active: bool, is_cool: bool) -> Self {
        Self {
            name: name.to_string(),
            based_on: based_on.to_string(),
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
