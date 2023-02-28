//! The example can be run by this command
//! `cargo run --example collapse`

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

    let mut table = json_to_table(&json);
    table.collapse();

    println!("{table}");
}
