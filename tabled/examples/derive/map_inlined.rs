use tabled::{Table, Tabled};

#[derive(Tabled)]
struct Vehicle {
    vtype: String,
    #[tabled(inline("engine->"), map("Engine::parse", Engine))]
    engine: String,
}

#[derive(Tabled)]
struct Engine {
    cylinder_amount: u8,
    oil_change_cycle: u8,
    disel: bool,
}

impl Engine {
    fn parse(text: &str) -> Self {
        let mut settings = text.split(";");
        let cylinder_amount = settings.next().unwrap().parse().unwrap();
        let oil_change_cycle = settings.next().unwrap().parse().unwrap();
        let disel = settings.next().unwrap().parse().unwrap();

        Self {
            cylinder_amount,
            oil_change_cycle,
            disel,
        }
    }
}

fn main() {
    let data = [
        Vehicle {
            vtype: String::from("limousine"),
            engine: String::from("3;12;false"),
        },
        Vehicle {
            vtype: String::from("sport"),
            engine: String::from("12;4;true"),
        },
    ];

    let table = Table::new(data);

    println!("{table}");
}
