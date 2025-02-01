use tabled::Tabled;

#[derive(Debug)]
pub struct Country {
    name: String,
    capital: Option<String>,
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

#[derive(Tabled)]
#[tabled(display_type(Option, "display_option", "UNKNOWN"))]
pub struct CountryDisplay {
    name: String,
    capital: Option<String>,
}

impl From<Country> for CountryDisplay {
    fn from(country: Country) -> Self {
        Self {
            name: country.name,
            capital: country.capital,
        }
    }
}

fn main() {
    let data = vec![
        Country {
            name: "France".to_string(),
            capital: Some("Paris".to_string()),
        },
        Country {
            name: "Germany".to_string(),
            capital: Some("Berlin".to_string()),
        },
        Country {
            name: "Unknown".to_string(),
            capital: None,
        },
    ];

    let table_data = data.into_iter().map(CountryDisplay::from);
    let table = tabled::Table::new(table_data);
    println!("{}", table);
}
