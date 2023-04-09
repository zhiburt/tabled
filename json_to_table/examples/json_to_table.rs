//! This example demonstrates parsing a JSON literal to a [`Value`],
//! and then translating that value to a [`JsonTable`] struct.

use json_to_table::json_to_table;

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

    let table = json_to_table(&json);

    println!("{table}");
}
