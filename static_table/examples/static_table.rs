//! This example demonstrates using the powerful [`static_table!`] macro to translate
//! a sequence of arrays and several, optional settings to a static [`str`] table representation.
//!
//! * Note that [`static_table!`] is evaluated at compile time, resulting in highly efficient runtime performance.
//! * [`static_table!`] supports configuration of:
//!     * granular column and row span specification
//!     * [`THEME`](tabled::settings::Style)
//!     * [`ALIGNMENT`](tabled::settings::Alignment)
//!     * [`PADDING`](`tabled::settings::Padding`)
//!     * [`MARGIN`](tabled::settings::Margin)

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
