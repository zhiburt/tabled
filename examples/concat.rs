//! The example can be run by this command
//! `cargo run --example concat`

use tabled::{Concat, Style, Table, Tabled};

#[derive(Debug, Tabled)]
struct Weather {
    temperature_c: f64,
    wind_ms: f64,
}

#[derive(Debug, Tabled)]
struct Location {
    latitude: f64,
    longitude: f64,
}

fn main() {
    let weather_data = [
        Weather {
            temperature_c: 16.0,
            wind_ms: 3000.0,
        },
        Weather {
            temperature_c: -20.0,
            wind_ms: 300.0,
        },
        Weather {
            temperature_c: 40.0,
            wind_ms: 100.0,
        },
    ];

    let location_data = [
        Location {
            latitude: 111.111,
            longitude: 333.333,
        },
        Location {
            latitude: 5.111,
            longitude: 7282.1,
        },
        Location {
            latitude: 0.0,
            longitude: 0.0,
        },
    ];

    let weather_table = Table::new(weather_data);
    let location_table = Table::new(location_data);

    let data_table = weather_table
        .with(Concat::horizontal(location_table))
        .with(Style::PSEUDO_CLEAN);

    println!("data");
    println!("{}", data_table);
}
