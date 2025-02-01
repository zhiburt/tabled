use tabled::Tabled;

#[derive(Tabled)]
#[tabled(display_type(Option, "display_option", "UNKNOWN"))]
pub struct Country {
    name: String,
    capital: Option<String>,
    currency: Option<String>,
}

fn display_option<T>(opt: &Option<T>, default: &str) -> String
where
    T: ToString,
{
    match opt {
        Some(val) => val.to_string().to_uppercase(),
        None => default.to_string(),
    }
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

    let table = tabled::Table::new(data);

    println!("{}", table);
}
