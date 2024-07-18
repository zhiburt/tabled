//! This example demonstrates using the [attribute macro](https://doc.rust-lang.org/reference/procedural-macros.html#attribute-macros)
//! [`format`] to beatifuly castomize the resulting values, be used for table contraction.

use tabled::{settings::Style, Table, Tabled};

#[derive(Tabled)]
struct User {
    #[tabled(format("{}.{}.{}.{}", self.ip[0], self.ip[1], self.ip[2], self.ip[3]))]
    ip: [u8; 4],
    #[tabled(inline)]
    password: Password,
}

#[derive(Tabled)]
enum Password {
    #[tabled(inline)]
    Mask {
        #[tabled(format("T {}", str::to_uppercase(self.text)))]
        text: String,
        #[tabled(format = "F {}")]
        factor: usize,
    },
    #[tabled(inline)]
    Plain(String),
}

impl Password {
    fn mask(s: &str, f: usize) -> Self {
        Self::Mask {
            text: s.to_string(),
            factor: f,
        }
    }

    fn plain(s: &str) -> Self {
        Self::Plain(s.to_string())
    }
}

impl User {
    fn new(ip: [u8; 4], password: Password) -> Self {
        Self { ip, password }
    }
}

fn main() {
    let data = [
        User::new([127, 0, 0, 1], Password::mask("11111111", 0)),
        User::new([127, 0, 0, 1], Password::mask("1", 1000)),
        User::new([127, 0, 0, 3], Password::plain("3333")),
    ];

    let mut table = Table::new(data);
    table.with(
        Style::modern_rounded()
            .remove_horizontal()
            .remove_vertical(),
    );

    println!("{table}");
}
