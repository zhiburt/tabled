//! This example demonstrates using the [attribute macro](https://doc.rust-lang.org/reference/procedural-macros.html#attribute-macros)
//! [`format`] to beatifuly castomize the resulting values, be used for table contraction.

use tabled::{settings::Style, Table, Tabled};

#[derive(Tabled)]
struct Phone {
    #[tabled(format = "code {}")]
    code: String,
    #[tabled(rename = "")]
    #[tabled(format("{}/{}", self.alias.join(","), self.number))]
    number: String,
    #[tabled(skip)]
    alias: Vec<String>,
}

impl Phone {
    fn new(code: &str, number: &str, alias: &[&str]) -> Self {
        let alias = alias.iter().map(ToString::to_string).collect();

        Self {
            code: code.to_string(),
            number: number.to_string(),
            alias,
        }
    }
}

fn main() {
    let data = [
        Phone::new("AFN", "11111111", &["Mate"]),
        Phone::new("CAD", "22222222", &["Sara", "Football", "meetup"]),
        Phone::new("RUS", "33333333", &["Cris", "meetup"]),
        Phone::new("BLR", "44444444", &["Ham", "meetup"]),
    ];

    let mut table = Table::new(data);
    table.with(Style::modern_rounded().remove_horizontal());

    println!("{table}");
}
