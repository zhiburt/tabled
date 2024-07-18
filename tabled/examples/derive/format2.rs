//! This example demonstrates using the [attribute macro](https://doc.rust-lang.org/reference/procedural-macros.html#attribute-macros)
//! [`format`] to beatifuly castomize the resulting values, be used for table contraction.

use tabled::{settings::Style, Table, Tabled};

#[derive(Tabled)]
struct User {
    #[tabled(format("{}.{}.{}.{}", self.ip[0], self.ip[1], self.ip[2], self.ip[3]))]
    ip: [u8; 4],
    mask: String,
    password: String,
}

impl User {
    fn new(ip: [u8; 4], mask: &str, password: &str) -> Self {
        Self {
            ip,
            mask: mask.to_string(),
            password: password.to_string(),
        }
    }
}

fn main() {
    let data = [
        User::new([127, 0, 0, 1], "", "11111111"),
        User::new([127, 0, 0, 1], "", "22222222"),
        User::new([127, 0, 0, 1], "", "33333333"),
        User::new([127, 0, 0, 1], "", "44444444"),
    ];

    let mut table = Table::new(data);
    table.with(
        Style::modern_rounded()
            .remove_horizontal()
            .remove_vertical(),
    );

    println!("{table}");
}
