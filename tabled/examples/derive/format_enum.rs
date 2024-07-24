//! This example demonstrates using the [attribute macro](https://doc.rust-lang.org/reference/procedural-macros.html#attribute-macros)
//! [`format`] to beatifuly castomize the resulting values, be used for table contraction.

use tabled::{Table, Tabled};

#[derive(Tabled)]
enum Vehicle {
    #[tabled(inline)]
    Car(#[tabled(format = "car->{}", rename = "cars")] String),
    #[tabled(inline)]
    Boat(#[tabled(format = "boat->{}", rename = "boats")] String),
}

fn main() {
    let data = [
        Vehicle::Car("bmw".into()),
        Vehicle::Car("audi".into()),
        Vehicle::Car("volkswagen".into()),
        Vehicle::Boat("ford".into()),
    ];

    let table = Table::new(data);

    println!("{table}");
}
