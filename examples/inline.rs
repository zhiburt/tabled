//! The example can be run by this command
//! `cargo run --example inline`

use tabled::{
    object::{Full, Rows},
    Alignment, Modify, Padding, Style, Table, Tabled,
};

fn main() {
    let data = [
        User {
            id: 0,
            personal_information: Person {
                name: "Cassie",
                surname: "Berge",
                age: Some(55),
            },
            contact: Contact::Number("+1233332221"),
        },
        User {
            id: 1,
            personal_information: Person {
                name: "Nora",
                surname: "Bailey",
                age: Some(22),
            },
            contact: Contact::No,
        },
        User {
            id: 10,
            personal_information: Person {
                name: "Maxim",
                surname: "Zhiburt",
                age: None,
            },
            contact: Contact::Telegram {
                number: "+12345678",
                username: "no2Presley",
            },
        },
    ];

    let table = Table::new(&data)
        .with(Style::modern())
        .with(Modify::new(Full).with(Padding::new(1, 1, 0, 0)))
        .with(Modify::new(Rows::first()).with(Alignment::left()))
        .with(Modify::new(Rows::new(1..)).with(Alignment::center()));

    println!("{}", table);
}

#[derive(Tabled)]
struct User {
    id: usize,
    #[tabled(inline)]
    personal_information: Person,
    #[tabled(inline)]
    contact: Contact,
}

#[derive(Tabled)]
struct Person {
    name: &'static str,
    surname: &'static str,
    #[tabled(display_with = "display_age")]
    age: Option<usize>,
}

fn display_age(age: &Option<usize>) -> String {
    match age {
        Some(age) => format!("{} age", age),
        None => "unknown".to_owned(),
    }
}

#[derive(Tabled)]
enum Contact {
    #[tabled(inline("telegram::"))]
    Telegram {
        username: &'static str,
        number: &'static str,
    },
    #[tabled(hidden)]
    No,
    #[tabled(inline)]
    Number(#[tabled("number")] &'static str),
}
