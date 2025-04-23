use tabled::{settings::Extract, Table, Tabled};

#[derive(Tabled)]
struct Album {
    artist: String,
    name: String,
    released: String,
    #[tabled(format = "{:?}")]
    level: Greatness,
}

#[derive(Debug)]
enum Greatness {
    Supreme,
    Outstanding,
    Unparalleled,
}

fn main() {
    use Greatness::*;

    #[rustfmt::skip]
    let data = [
        Album { artist: String::from("Pink Floyd"), name: String::from("The Dark Side of the Moon"), released: String::from("01 March 1973"), level: Unparalleled },
        Album { artist: String::from("Fleetwood Mac"), name: String::from("Rumours"), released: String::from("04 February 1977"), level: Outstanding },
        Album { artist: String::from("Led Zeppelin"), name: String::from("Led Zeppelin IV"), released: String::from("08 November 1971"), level: Supreme },
    ];

    let mut table = Table::new(data);

    println!("Full table");
    println!("{table}");

    table.with(Extract::segment(1..=2, 1..));

    println!("Segment: row=(1..=2) column=(1..)");
    println!("{table}");
}
