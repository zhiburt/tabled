//! This example can be run with the following command:
//!
//! `cargo run --example orientation`
//!
//! This example demonstrates several [`JsonTable`] functions for rotating
//! the arrangement of keys and values with the following functions:
//!
//! - [`JsonTable::set_array_mode`]
//! - [`JsonTable::set_object_mode`]
//! - [`JsonTable::set_mode_visitor`]
//!
//! ---
//!
//! * Note the shared [`Orientation`] enum between these methods.

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
        .array_orientation(Orientation::Horizontal)
        .object_orientation(Orientation::Horizontal);

    let ctable = json_to_table(&json);

    println!("Vertical mode\n{vtable}");
    println!("Horizontal mode\n{htable}");
    println!("Custom mode\n{ctable}");
}
