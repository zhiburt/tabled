use tabled::{Table, Tabled};

#[derive(Tabled)]
struct Vehicle {
    vtype: String,
    #[tabled(map = "String::from")]
    engine_type: EngineType,
}

enum EngineType {
    V8,
    V12,
}

impl From<&EngineType> for String {
    fn from(value: &EngineType) -> Self {
        match value {
            EngineType::V8 => "V8".to_string(),
            EngineType::V12 => "V12".to_string(),
        }
    }
}

fn main() {
    let data = [
        Vehicle {
            vtype: String::from("limousine"),
            engine_type: EngineType::V8,
        },
        Vehicle {
            vtype: String::from("sport"),
            engine_type: EngineType::V12,
        },
    ];

    let table = Table::new(data);

    println!("{table}");
}
