//! This example demonstrates using the [attribute macro](https://doc.rust-lang.org/reference/procedural-macros.html#attribute-macros)
//! [`format`] to beatifuly castomize the resulting values, be used for table constraction.

use tabled::{settings::Style, Table, Tabled};

#[derive(Tabled)]
struct Phone {
    #[tabled(format = "code {}")]
    code: String,
    #[tabled(skip)]
    alias: String,
    #[tabled(format("{}/{}", self.alias, self.number))]
    number: String,
}

impl Phone {
    fn new(code: &str, alias: &str, number: &str) -> Self {
        Self {
            code: code.to_string(),
            alias: alias.to_string(),
            number: number.to_string(),
        }
    }
}

fn main() {
    let data = [
        Phone::new("AFN", "Mate", "11111111"),
        Phone::new("CAD", "Sara", "22222222"),
        Phone::new("RUS", "Cris", "33333333"),
        Phone::new("BLR", "Ham", "44444444"),
    ];

    let mut table = Table::new(data);
    table.with(Style::modern_rounded().remove_horizontal());

    println!("{table}");
}
