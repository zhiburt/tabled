//! This example demonstrates several [`JsonTable`] functions for rotating
//! the arrangement of keys and values

use json_to_table::{json_to_table, Orientation};

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

    let vtable = json_to_table(&json);

    let mut htable = json_to_table(&json);
    htable
        .array_orientation(Orientation::Row)
        .object_orientation(Orientation::Row);

    let ctable = json_to_table(&json);

    println!("Vertical mode\n{vtable}");
    println!("Horizontal mode\n{htable}");
    println!("Custom mode\n{ctable}");
}
