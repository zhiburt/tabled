//! The crate contains a [`json_to_table`] function which builds a [`Table`] from an ordinary json.
//!
//! You can build the table either generally or in a squash.
//! See the examples below.
//!
//! ```
//! use serde_json::json;
//! use json_to_table::json_to_table;
//!
//! let value = json!(
//!     {
//!         "name": "John Doe",
//!         "age": 43,
//!         "address": {
//!             "street": "10 Downing Street",
//!             "city": "London"
//!         },
//!         "phones": [
//!             "+44 1234567",
//!             "+44 2345678"
//!         ]
//!     }
//! );
//!
//! // recursive table
//! let table = json_to_table(&value).to_string();
//!
//! println!("{}", table);
//!
//! assert_eq!(
//!     table,
//!     concat!(
//!         "+---------+----------------------------------+\n",
//!         "| address | +--------+---------------------+ |\n",
//!         "|         | | city   |  London             | |\n",
//!         "|         | +--------+---------------------+ |\n",
//!         "|         | | street |  10 Downing Street  | |\n",
//!         "|         | +--------+---------------------+ |\n",
//!         "+---------+----------------------------------+\n",
//!         "| age     |  43                              |\n",
//!         "+---------+----------------------------------+\n",
//!         "| name    |  John Doe                        |\n",
//!         "+---------+----------------------------------+\n",
//!         "| phones  | +---------------+                |\n",
//!         "|         | |  +44 1234567  |                |\n",
//!         "|         | +---------------+                |\n",
//!         "|         | |  +44 2345678  |                |\n",
//!         "|         | +---------------+                |\n",
//!         "+---------+----------------------------------+",
//!     ),
//! );
//!
//! // squash tables together
//! let table = json_to_table(&value).collapse().to_string();
//!
//! assert_eq!(
//!     table,
//!     concat!(
//!         "+---------+--------+-------------------+\n",
//!         "| address | city   | London            |\n",
//!         "|         +--------+-------------------+\n",
//!         "|         | street | 10 Downing Street |\n",
//!         "+---------+--------+-------------------+\n",
//!         "| age     | 43                         |\n",
//!         "+---------+----------------------------+\n",
//!         "| name    | John Doe                   |\n",
//!         "+---------+----------------------------+\n",
//!         "| phones  | +44 1234567                |\n",
//!         "|         +----------------------------+\n",
//!         "|         | +44 2345678                |\n",
//!         "+---------+----------------------------+",
//!     ),
//! );
//! ```
//!
//! [`Table`]: tabled::Table

#![deny(unused_must_use)]
#![warn(
    missing_docs,
    rust_2018_idioms,
    missing_debug_implementations,
    unreachable_pub
)]
#![allow(clippy::uninlined_format_args)]
#![doc(
    html_logo_url = "https://raw.githubusercontent.com/zhiburt/tabled/86ac146e532ce9f7626608d7fd05072123603a2e/assets/tabled-gear.svg"
)]

use serde_json::Value;

pub use table::{JsonTable, Orientation};

mod table;

/// The function converts a given [`Value`] to a [`Table`].
///
/// See the example in a module documentation.
///
/// [`Table`]: tabled::Table
pub fn json_to_table(value: &Value) -> JsonTable<&Value> {
    JsonTable::new(value)
}
