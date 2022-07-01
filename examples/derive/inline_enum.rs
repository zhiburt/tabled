//! The example can be run by this command
//! `cargo run --example inline_enum`

use tabled::{Table, Tabled};

#[derive(Tabled)]
enum Contact {
    #[tabled(inline("telegram::"))]
    Telegram {
        username: &'static str,
        #[tabled(inline("telegram::"))]
        number: Number,
    },
    #[tabled(inline)]
    Local(#[tabled(inline("local::"))] Number),
}

#[derive(Tabled)]
struct Number {
    number: &'static str,
    code: usize,
}

impl Number {
    fn new(number: &'static str, code: usize) -> Self {
        Self { number, code }
    }
}

fn main() {
    let data = [
        Contact::Local(Number::new("654321", 123)),
        Contact::Telegram {
            username: "no2Presley",
            number: Number::new("123456", 123),
        },
    ];

    let table = Table::new(&data);

    println!("{}", table);
}
