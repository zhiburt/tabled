//! The example can be run by this command
//! `cargo run --example static_table`

use static_table::static_table;

static LANG_LIST: &str = static_table!([
    ["name", "designed by", "first release"],
    ["C", "Dennis Ritchie", "1972"],
    ["Go", "Rob Pike", "2009"],
    ["Rust", "Graydon Hoare", "2010"],
]);

fn main() {
    println!("{LANG_LIST}")
}
