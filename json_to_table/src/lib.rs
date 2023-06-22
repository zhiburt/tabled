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
use tabled::{builder::Builder, Table};

mod table;

/// The function converts a given [`Value`] to a [`JsonTable`].
///
/// ```
/// let json = serde_json::json!({
///     "key1": "value1",
///     "key2": {
///         "key1": 123,
///         "key2": [1, 2, 3, 4, 5],
///     },
///     "key3": [
///         {"key": 123.3},
///         2,
///         "asd"
///     ],
///     "key4": 1234.567
/// });
///     
/// let table = json_to_table::json_to_table(&json).to_string();
///
/// assert_eq!(
///     table,
///     concat!(
///         "+------+-----------------------+\n",
///         "| key1 |  value1               |\n",
///         "+------+-----------------------+\n",
///         "| key2 | +------+---------+    |\n",
///         "|      | | key1 |  123    |    |\n",
///         "|      | +------+---------+    |\n",
///         "|      | | key2 | +-----+ |    |\n",
///         "|      | |      | |  1  | |    |\n",
///         "|      | |      | +-----+ |    |\n",
///         "|      | |      | |  2  | |    |\n",
///         "|      | |      | +-----+ |    |\n",
///         "|      | |      | |  3  | |    |\n",
///         "|      | |      | +-----+ |    |\n",
///         "|      | |      | |  4  | |    |\n",
///         "|      | |      | +-----+ |    |\n",
///         "|      | |      | |  5  | |    |\n",
///         "|      | |      | +-----+ |    |\n",
///         "|      | +------+---------+    |\n",
///         "+------+-----------------------+\n",
///         "| key3 | +-------------------+ |\n",
///         "|      | | +-----+---------+ | |\n",
///         "|      | | | key |  123.3  | | |\n",
///         "|      | | +-----+---------+ | |\n",
///         "|      | +-------------------+ |\n",
///         "|      | |  2                | |\n",
///         "|      | +-------------------+ |\n",
///         "|      | |  asd              | |\n",
///         "|      | +-------------------+ |\n",
///         "+------+-----------------------+\n",
///         "| key4 |  1234.567             |\n",
///         "+------+-----------------------+",
///     ),
/// )
/// ```
pub fn json_to_table(value: &Value) -> JsonTable<&Value> {
    JsonTable::new(value)
}

/// The function converts a given [`Value`] to a [`Table`].
///
/// It's quite different from [`json_to_table`], cause it is not recursive
/// and treats `json` `object` and `array` as string values.
///
/// ```
/// let json = serde_json::json!({
///     "key1": "value1",
///     "key2": {
///         "key1": 123,
///         "key2": [1, 2, 3, 4, 5],
///     },
///     "key3": [
///         {"key": 123.3},
///         2,
///         "asd"
///     ],
///     "key4": 1234.567
/// });
///     
/// let table = json_to_table::parse(&json).to_string();
///
/// assert_eq!(
///     table,
///     concat!(
///         "+------+---------------------------------+\n",
///         "| key1 | value1                          |\n",
///         "+------+---------------------------------+\n",
///         "| key2 | {\"key1\":123,\"key2\":[1,2,3,4,5]} |\n",
///         "+------+---------------------------------+\n",
///         "| key3 | [{\"key\":123.3},2,\"asd\"]         |\n",
///         "+------+---------------------------------+\n",
///         "| key4 | 1234.567                        |\n",
///         "+------+---------------------------------+",
///     ),
/// )
/// ```
///
/// [`Table`]: tabled::Table
pub fn parse(value: &Value) -> Table {
    json_into_table(value)
}

fn json_into_table(value: &Value) -> Table {
    match value {
        Value::Array(array) => {
            let list = array.iter().map(json_value_to_string).collect::<Vec<_>>();
            Builder::from(vec![list]).build()
        }
        Value::Object(map) => {
            let list = map
                .iter()
                .map(|(key, value)| vec![key.clone(), json_value_to_string(value)])
                .collect::<Vec<_>>();

            Builder::from(list).build()
        }
        Value::Null => Builder::default().build(),
        Value::Bool(value) => single_value_table(value),
        Value::Number(value) => single_value_table(value),
        Value::String(value) => single_value_table(value),
    }
}

fn single_value_table<V: ToString>(value: V) -> Table {
    Builder::from(vec![vec![value.to_string()]]).build()
}

fn json_value_to_string(value: &Value) -> String {
    match value {
        Value::Null => String::new(),
        Value::Bool(value) => value.to_string(),
        Value::Number(value) => value.to_string(),
        Value::String(value) => value.to_string(),
        Value::Array(_) | Value::Object(_) => {
            serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
        }
    }
}
