//! This example can be run with the following command:
//!
//! `cat some_interesting_data.json | cargo run --example reader -- --collapse`
//!
//! This example demonstrates parsing json from the [`Stdin`]\(standard input) to a [`JsonTable`].
//!
//! * Check out [`serde_json`] for other helpful ways to import json data into your project.
//! * Check out [`std::env`] and [`std::io`] for more examples of dealing with system streams.
//! * Windows alternatives for the unix `cat` command:
//!     - `type` for command prompt
//!     - `get-content` for powershell

use std::{env, io};

use json_to_table::json_to_table;

fn main() {
    let use_collapse = env::args().any(|arg| &arg == "--collapse");

    let stdin = io::stdin();
    let value = serde_json::from_reader(stdin).expect("failed to read stdin");

    let mut table = json_to_table(&value);

    if use_collapse {
        table.collapse();
    }

    println!("{table}");
}
