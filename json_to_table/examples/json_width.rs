//! This example demonstrates using the [`json_to_table::JsonTable::into_table`] function
//! to create a table and control the width of certain fields.

use tabled::settings::{Modify, Style, Width};

fn main() {
    let json = serde_json::json!({
        "key1": "Some loooooooooooong string",
        "key2": {
            "key1": 123,
            "key2": [1, 2, 3, 4, 5],
        },
        "key3": [
            {"key": 123.3},
            2,
            "asd"
        ],
    });

    let mut table = json_to_table::json_to_table(&json).into_table();
    table.with(Style::extended());
    table.with(Modify::new((0, 1)).with(Width::wrap(6)));

    println!("{table}");
}
