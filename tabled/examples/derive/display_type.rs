use tabled::{Table, Tabled};

#[derive(Tabled)]
#[tabled(display(Option, "tabled::derive::display::option", "UNKNOWN"))]
pub struct Country {
    name: String,
    capital: Option<String>,
    currency: Option<String>,
}

fn main() {
    let data = vec![
        Country {
            name: String::from("France"),
            capital: Some(String::from("Paris")),
            currency: Some(String::from("EUR")),
        },
        Country {
            name: String::from("Germany"),
            capital: Some(String::from("Berlin")),
            currency: None,
        },
        Country {
            name: String::from("Unknown"),
            capital: None,
            currency: Some(String::from("01")),
        },
    ];

    let table = Table::new(data);

    println!("{}", table);
}
