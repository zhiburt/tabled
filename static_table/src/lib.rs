//! This library provides a macros to build a table at compile time.
//!
//! It contains 2 macros:
//!
//! - [`static_table`]
//! - [`pool_table`]
//!
//! # Get started
//!
//! ## [`static_table`]
//! ```
//! use static_table::static_table;
//!
//! const INTRO_TABLE: &str = static_table!([
//!     ["name", "designed by", "first release"],
//!     ["C", "Dennis Ritchie", "1972"],
//!     ["Go", "Rob Pike", "2009"],
//!     ["Rust", "Graydon Hoare", "2010"],
//! ]);
//!
//! assert_eq!(
//!     INTRO_TABLE,
//!     "+------+----------------+---------------+\n\
//!      | name | designed by    | first release |\n\
//!      +------+----------------+---------------+\n\
//!      | C    | Dennis Ritchie | 1972          |\n\
//!      +------+----------------+---------------+\n\
//!      | Go   | Rob Pike       | 2009          |\n\
//!      +------+----------------+---------------+\n\
//!      | Rust | Graydon Hoare  | 2010          |\n\
//!      +------+----------------+---------------+"
//! );
//! ```
//!
//! ## [`pool_table`]
//!
//! ```
//! use static_table::pool_table;
//!
//! const INTRO_TABLE: &str = pool_table!([
//!     ["name", "designed by", "first release"],
//!     ["C", "Dennis Ritchie", "1972"],
//!     ["Go", "Rob Pike", "2009"],
//!     ["Rust", "Graydon Hoare", "2010"],
//! ]);
//!
//! assert_eq!(
//!     INTRO_TABLE,
//!     "+------+-------------+---------------+\n\
//!      | name | designed by | first release |\n\
//!      +------+-------------+-----+---------+\n\
//!      | C    | Dennis Ritchie    | 1972    |\n\
//!      +------+--+---------------++---------+\n\
//!      | Go      | Rob Pike      | 2009     |\n\
//!      +---------+---------------+-+--------+\n\
//!      | Rust    | Graydon Hoare   | 2010   |\n\
//!      +---------+-----------------+--------+"
//! );
//! ```
//!
//! ## Configuration
//!
//! ### Span
//!
//! You can configure a span for a cell.
//!
//! ```
//! use static_table::static_table;
//!
//! // see here we added a row with a span
//! let table = static_table!([
//!     [{"programming languages"; 3}],
//!     ["name", "designed by", "first release"],
//!     ["C", "Dennis Ritchie", "1972"],
//!     ["Go", "Rob Pike", "2009"],
//!     ["Rust", "Graydon Hoare", "2010"],
//! ]);
//!
//! assert_eq!(
//!     table,
//!     "+------+----------------+---------------+\n\
//!      | programming languages                 |\n\
//!      +------+----------------+---------------+\n\
//!      | name | designed by    | first release |\n\
//!      +------+----------------+---------------+\n\
//!      | C    | Dennis Ritchie | 1972          |\n\
//!      +------+----------------+---------------+\n\
//!      | Go   | Rob Pike       | 2009          |\n\
//!      +------+----------------+---------------+\n\
//!      | Rust | Graydon Hoare  | 2010          |\n\
//!      +------+----------------+---------------+"
//! );
//!
//! // see here we added a column which is fully spanned
//! let table = static_table!([
//!     [{"\n\nprogramming\nlanguages"}, "name", "designed by", "first release"],
//!     [{},               "C", "Dennis Ritchie", "1972"],
//!     [{},               "Go", "Rob Pike", "2009"],
//!     [{},               "Rust", "Graydon Hoare", "2010"],
//! ]);
//!
//! assert_eq!(
//!     table,
//!     "+-------------+------+----------------+---------------+\n\
//!      |             | name | designed by    | first release |\n\
//!      +             +------+----------------+---------------+\n\
//!      | programming | C    | Dennis Ritchie | 1972          |\n\
//!      + languages   +------+----------------+---------------+\n\
//!      |             | Go   | Rob Pike       | 2009          |\n\
//!      +             +------+----------------+---------------+\n\
//!      |             | Rust | Graydon Hoare  | 2010          |\n\
//!      +-------------+------+----------------+---------------+"
//! );
//! ```
//!
//! ### Settings
//!
//! You can change a table table settings, such as `THEME`, `MARGIN`, `PADDING` and `ALIGNMENT`, by using a comma separated `KEY=VALUE` pairs syntax.
//!
//! ```
//! use static_table::static_table;
//!
//! const INTRO_TABLE: &str = static_table!(
//!     [
//!         ["name", "designed by", "first release"],
//!         ["C", "Dennis Ritchie", "1972"],
//!         ["Go", "Rob Pike", "2009"],
//!         ["Rust", "Graydon Hoare", "2010"]
//!     ],
//!     THEME = "EXTENDED",
//!     ALIGNMENT = "RIGHT",
//!     PADDING = "3, 0, 1, 0",
//! );
//!
//! assert_eq!(
//!     INTRO_TABLE,
//!     "╔═══════╦═════════════════╦════════════════╗\n\
//!      ║       ║                 ║                ║\n\
//!      ║   name║      designed by║   first release║\n\
//!      ╠═══════╬═════════════════╬════════════════╣\n\
//!      ║       ║                 ║                ║\n\
//!      ║      C║   Dennis Ritchie║            1972║\n\
//!      ╠═══════╬═════════════════╬════════════════╣\n\
//!      ║       ║                 ║                ║\n\
//!      ║     Go║         Rob Pike║            2009║\n\
//!      ╠═══════╬═════════════════╬════════════════╣\n\
//!      ║       ║                 ║                ║\n\
//!      ║   Rust║    Graydon Hoare║            2010║\n\
//!      ╚═══════╩═════════════════╩════════════════╝"
//! )
//! ```
//!
//! [`static_table`]: crate::static_table!
//! [`pool_table`]: crate::pool_table!

#![allow(clippy::uninlined_format_args)]
#![warn(
    rust_2018_idioms,
    rust_2018_compatibility,
    rust_2021_compatibility,
    missing_debug_implementations,
    unreachable_pub,
    missing_docs
)]
#![deny(unused_must_use)]
#![doc(
    html_logo_url = "https://raw.githubusercontent.com/zhiburt/tabled/86ac146e532ce9f7626608d7fd05072123603a2e/assets/tabled-gear.svg"
)]

use proc_macro_error::proc_macro_error;
use quote::quote;
use syn::parse_macro_input;

mod pool_table;
mod static_table;

/// Build a table.
///
/// ```
/// use static_table::static_table;
///
/// let table = static_table!([
///     [{"programming languages"; 3}],
///     ["name", "designed by", "first release"],
///     ["C", "Dennis Ritchie", "1972"],
///     ["Go", "Rob Pike", "2009"],
///     ["Rust", "Graydon Hoare", "2010"],
///     ["Hare", "Drew DeVault", "2022"],
/// ]);
///
/// assert_eq!(
///     table,
///     "+------+----------------+---------------+\n\
///      | programming languages                 |\n\
///      +------+----------------+---------------+\n\
///      | name | designed by    | first release |\n\
///      +------+----------------+---------------+\n\
///      | C    | Dennis Ritchie | 1972          |\n\
///      +------+----------------+---------------+\n\
///      | Go   | Rob Pike       | 2009          |\n\
///      +------+----------------+---------------+\n\
///      | Rust | Graydon Hoare  | 2010          |\n\
///      +------+----------------+---------------+\n\
///      | Hare | Drew DeVault   | 2022          |\n\
///      +------+----------------+---------------+"
/// );
///
/// ```
///
/// # Syntax
///
/// The input is expected to look like an array.
///
/// ```
/// # use static_table::static_table;
/// static_table!([["a", "b"], ["c", "d"]]);
/// ```
///
/// You can repeat an argument just like you'd do with general array of `vec!` macro.
///
/// ```
/// # use static_table::static_table;
/// static_table!([["123456789"; 2]; 5]);
/// ```
///
/// Optionally you can set a column span, using the following syntax.
///
/// ```
/// # use static_table::static_table;
/// static_table!([[{"a"; 2}], ["c", "d"]]);
/// ```
///
/// Optionally you can set a row span, using the following syntax.
///
/// ```
/// # use static_table::static_table;
/// static_table!([[{"a"}, "b"], [{ }, "d"]]);
/// ```
///
/// Optionally you can add settings to the table, using the following syntax.
///
/// ```
/// # use static_table::static_table;
/// static_table!([["a", "b"], ["c", "d"]], THEME = "ROUNDED");
/// ```
///
/// Supported settings are:
///
/// - THEME
/// - ALIGNMENT
/// - PADDING
/// - MARGIN
#[proc_macro]
#[proc_macro_error]
pub fn static_table(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    use crate::static_table::{build_table, TableStruct};

    let table = parse_macro_input!(input as TableStruct);
    let table = build_table(&table);
    match table {
        Ok(table) => proc_macro::TokenStream::from(quote! { #table }),
        Err(err) => proc_macro::TokenStream::from(err.into_compile_error()),
    }
}

/// Build a table.
///
/// ```
/// use static_table::pool_table;
///
/// let table = pool_table!([
///     ["programming languages"],
///     ["name", "designed by", "first release"],
///     ["C", "Dennis Ritchie", "1972"],
///     ["Go", "Rob Pike", "2009"],
///     ["Rust", "Graydon Hoare", "2010"],
///     ["Hare", "Drew DeVault", "2022"],
/// ]);
///
/// assert_eq!(
///     table,
///     "+------------------------------------+\n\
///      | programming languages              |\n\
///      +------+-------------+---------------+\n\
///      | name | designed by | first release |\n\
///      +------+-------------+-----+---------+\n\
///      | C    | Dennis Ritchie    | 1972    |\n\
///      +------+--+---------------++---------+\n\
///      | Go      | Rob Pike      | 2009     |\n\
///      +---------+---------------+-+--------+\n\
///      | Rust    | Graydon Hoare   | 2010   |\n\
///      +---------+-----------------+--------+\n\
///      | Hare    | Drew DeVault    | 2022   |\n\
///      +---------+-----------------+--------+"
/// );
///
/// ```
///
/// # Syntax
///
/// The input is expected to look like an array.
///
/// ```
/// # use static_table::pool_table;
/// pool_table!([["a", "b"], ["c", "d"]]);
/// ```
///
/// You can repeat an argument just like you'd do with general array of `vec!` macro.
///
/// ```
/// # use static_table::pool_table;
/// pool_table!([["123456789"; 2]; 5]);
/// ```
///
/// Optionally you can add settings to the table, using the following syntax.
///
/// ```
/// # use static_table::pool_table;
/// pool_table!([["a", "b"], ["c", "d"]], THEME = "ROUNDED");
/// ```
///
/// Supported settings are:
///
/// - THEME
/// - ALIGNMENT
/// - PADDING
/// - MARGIN
#[proc_macro]
#[proc_macro_error]
pub fn pool_table(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    use crate::pool_table::{build_table, TableStruct};

    let table = parse_macro_input!(input as TableStruct);
    let table = build_table(&table);
    match table {
        Ok(table) => proc_macro::TokenStream::from(quote! { #table }),
        Err(err) => proc_macro::TokenStream::from(err.into_compile_error()),
    }
}
