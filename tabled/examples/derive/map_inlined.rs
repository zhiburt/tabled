use tabled::{Table, Tabled};

#[derive(Tabled)]
struct Vehicle {
    vtype: String,
    #[tabled(inline("engine->"), map(Engine, "Self::parse_engine"))]
    engine: String,
}

#[derive(Tabled)]
struct Engine {
    cylinder_amount: u8,
    oil_change_cycle: u8,
    disel: bool,
}

impl Vehicle {
    fn new(vtype: &str, engine: &str) -> Self {
        Self {
            vtype: vtype.to_string(),
            engine: engine.to_string(),
        }
    }

    fn parse_engine(text: &str) -> Engine {
        let mut settings = text.split(";");
        let cylinder_amount = settings.next().unwrap().parse().unwrap();
        let oil_change_cycle = settings.next().unwrap().parse().unwrap();
        let disel = settings.next().unwrap().parse().unwrap();

        Engine {
            cylinder_amount,
            oil_change_cycle,
            disel,
        }
    }
}

fn main() {
    let data = [
        Vehicle::new("limousine", "3;12;false"),
        Vehicle::new("sport", "12;4;true"),
    ];

    let table = Table::new(data);

    println!("{table}");
}
