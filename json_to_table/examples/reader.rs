//! This example reads a json from STDIN and build a table out of it.
//!
//! Example usage.
//!
//! ```
//! cat some_interesting_data.json | cargo run --example reader -- --collapse
//! ```

use std::io;

use json_to_table::json_to_table;

fn main() {
    let use_collapse = std::env::args().any(|arg| &arg == "--collapse");

    let stdin = io::stdin();
    let value = serde_json::from_reader(stdin).expect("failed to read stdin");

    let mut table = json_to_table(&value);

    if use_collapse {
        table.collapse();
    }

    println!("{table}");
}
