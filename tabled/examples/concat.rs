//! The example can be run by this command
//! `cargo run --example concat`

use tabled::{
    settings::{alignment::Alignment, concat::Concat, object::Segment, style::Style, Modify},
    Table, Tabled,
};

#[derive(Debug, Tabled)]
struct Weather {
    temperature_c: f64,
    wind_ms: f64,
}

#[derive(Debug, Tabled)]
struct Location(
    #[tabled(rename = "latitude")] f64,
    #[tabled(rename = "longitude")] f64,
);

fn main() {
    let weather_data = [
        Weather {
            temperature_c: 1.0,
            wind_ms: 3.0,
        },
        Weather {
            temperature_c: -20.0,
            wind_ms: 30.0,
        },
        Weather {
            temperature_c: 40.0,
            wind_ms: 100.0,
        },
    ];

    let location_data = [
        Location(111.111, 333.333),
        Location(5.111, 7282.1),
        Location(0.0, 0.0),
        Location(0.0, 0.0),
    ];

    let location_table = Table::new(location_data);

    let mut weather_table = Table::new(weather_data);
    weather_table
        .with(Concat::horizontal(location_table))
        .with(Style::empty())
        .with(Modify::new(Segment::all()).with(Alignment::left()));

    println!("{}", weather_table);
}
