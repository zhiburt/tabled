//! The example can be run by this command
//! `cargo run --example inline`

use tabled::{
    Alignment, AlignmentHorizontal, Full, Head, Indent, Modify, Row, Style, Table, Tabled,
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
        .with(Style::pseudo())
        .with(Modify::new(Full).with(Indent::new(1, 1, 0, 0)))
        .with(Modify::new(Head).with(Alignment::Horizontal(AlignmentHorizontal::Left)))
        .with(Modify::new(Row(1..)).with(Alignment::Horizontal(AlignmentHorizontal::Center)));

    println!("{}", table);
}

#[derive(Tabled)]
struct User {
    id: usize,
    #[header(inline)]
    personal_information: Person,
    #[header(inline)]
    contact: Contact,
}

#[derive(Tabled)]
struct Person {
    name: &'static str,
    surname: &'static str,
    #[field(display_with = "display_age")]
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
    #[field(inline("telegram::"))]
    Telegram {
        username: &'static str,
        number: &'static str,
    },
    #[header(hidden)]
    No,
    #[header(inline)]
    Number(#[header("number")] &'static str),
}
