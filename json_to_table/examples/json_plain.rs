//! This example demonstrates using the [`json_to_table::parse`] function
//! to create a table as not recursive structure.

use tabled::settings::Style;

fn main() {
    let json = serde_json::json!({
        "key1": "value1",
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

    let mut table = json_to_table::parse(&json);
    table.with(Style::modern());

    println!("{table}");
}
