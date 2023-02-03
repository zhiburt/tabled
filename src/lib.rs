//! An easy to use library for pretty print tables of Rust `struct`s and `enum`s.
//!
//! The library is based on a [`Tabled`] trait which is used to actually build tables.
//! It also provides an variate of dynamic settings for customization of a [`Table`].
//!
//! [`Table`] can be build from vast majority of Rust's standard types.
//!
//! ## Usage
//!
//! If you want to build a table for your custom type.
//! A starting point is to a anotate your type with `#[derive(Tabled)]`.
//!
//! Then one of ways to create a table is to call [`Table::new`] to create a table.
//!
#![cfg_attr(feature = "derive", doc = "```")]
#![cfg_attr(not(feature = "derive"), doc = "```ignore")]
//! use tabled::{Tabled, Table};
//!
//! #[derive(Tabled)]
//! struct Language {
//!     name: &'static str,
//!     designed_by: &'static str,
//!     invented_year: usize,
//! }
//!
//! let languages = vec![
//!     Language{
//!         name: "C",
//!         designed_by: "Dennis Ritchie",
//!         invented_year: 1972
//!     },
//!     Language{
//!         name: "Rust",
//!         designed_by: "Graydon Hoare",
//!         invented_year: 2010
//!     },
//!     Language{
//!         name: "Go",
//!         designed_by: "Rob Pike",
//!         invented_year: 2009
//!     },
//! ];
//!
//! let table = Table::new(languages).to_string();
//!
//! let expected = "+------+----------------+---------------+\n\
//!                 | name | designed_by    | invented_year |\n\
//!                 +------+----------------+---------------+\n\
//!                 | C    | Dennis Ritchie | 1972          |\n\
//!                 +------+----------------+---------------+\n\
//!                 | Rust | Graydon Hoare  | 2010          |\n\
//!                 +------+----------------+---------------+\n\
//!                 | Go   | Rob Pike       | 2009          |\n\
//!                 +------+----------------+---------------+";
//!
//! assert_eq!(table, expected);
//! ```
//!
//! You can also create a table by using [`TableIteratorExt`].
//!
//! ```rust,no_run
//! # let languages = [""];
//! use tabled::TableIteratorExt;
//! let table = languages.table();
//! ```
//!
//! Not all types can derive [`Tabled`] trait though.
//! The example below can't be compiled.
//!
//! ```rust,compile_fail
//!   # use tabled::Tabled;
//!     #[derive(Tabled)]
//!     struct SomeType {
//!         field1: SomeOtherType,
//!     }
//!
//!     struct SomeOtherType;
//! ```
//!
//! We must know what we're up to print as a field. Because of this
//! each field must implement [`std::fmt::Display`].
//!
//! ### Default implementations
//!
//! As I've already mentioned most of the default types implements the trait out of the box.
//!
//! This allows you to run the following code.
//!
//! ```rust
//! use tabled::{Tabled, Table};
//! let table = Table::new(&[1, 2, 3]);
//! # let expected = "+-----+\n\
//! #                 | i32 |\n\
//! #                 +-----+\n\
//! #                 | 1   |\n\
//! #                 +-----+\n\
//! #                 | 2   |\n\
//! #                 +-----+\n\
//! #                 | 3   |\n\
//! #                 +-----+";
//! # assert_eq!(table.to_string(), expected);
//! ```
//!
//! ### Combination of types via tuples
//!
//! Personally I consider this a feature which drives the library to shine.
//! You can combine any types that implements [`Tabled`] trait into one table.
//!
//! You can also see in this example a `#[header("name")]` usage which configures a header
//! of a table which will be printed.
//! You could change it dynamically as well.
//!
#![cfg_attr(feature = "derive", doc = "```")]
#![cfg_attr(not(feature = "derive"), doc = "```ignore")]
//! use tabled::{Tabled, Table, Style, Alignment, ModifyObject, object::{Rows, Columns, Object}};
//!
//! #[derive(Tabled)]
//! enum Domain {
//!     Security,
//!     Embedded,
//!     Frontend,
//!     Unknown,
//! }
//!
//! #[derive(Tabled)]
//! struct Developer(#[tabled(rename = "name")] &'static str);
//!     
//! let data = vec![
//!     (Developer("Terri Kshlerin"), Domain::Embedded),
//!     (Developer("Catalina Dicki"), Domain::Security),
//!     (Developer("Jennie Schmeler"), Domain::Frontend),
//!     (Developer("Maxim Zhiburt"), Domain::Unknown),
//! ];
//!     
//! let table = Table::new(data)
//!     .with(Style::psql())
//!     .with(Rows::new(1..).not(Columns::first()).modify().with(Alignment::center()))
//!     .to_string();
//!
//! assert_eq!(
//!     table,
//!     concat!(
//!         " name            | Security | Embedded | Frontend | Unknown \n",
//!         "-----------------+----------+----------+----------+---------\n",
//!         " Terri Kshlerin  |          |    +     |          |         \n",
//!         " Catalina Dicki  |    +     |          |          |         \n",
//!         " Jennie Schmeler |          |          |    +     |         \n",
//!         " Maxim Zhiburt   |          |          |          |    +    "
//!     )
//! );
//! ```
//!
//! ### Dynamic table
//!
//! When you data sheme is not known at compile time.
//! You mostlikely will not able to use [`Tabled`] trait.
//! But you could build table from scratch.
//!
//! ```
//! use tabled::{builder::Builder, ModifyObject, object::Rows, Alignment, Style};
//!
//! let mut builder = Builder::default();
//!
//! for i in 0..3 {
//!     let mut row = vec![];
//!     row.push(i.to_string());
//!     for j in 0..10 {
//!         row.push((i*j).to_string());
//!     }
//!
//!     builder.add_record(row);
//! }
//!
//! builder.set_columns(std::iter::once(String::from("i")).chain((0..10).map(|i| i.to_string())));
//!
//! let table = builder.build()
//!     .with(Style::rounded())
//!     .with(Rows::new(1..).modify().with(Alignment::left()))
//!     .to_string();
//!
//! assert_eq!(
//!     table,
//!     concat!(
//!         "╭───┬───┬───┬───┬───┬───┬────┬────┬────┬────┬────╮\n",
//!         "│ i │ 0 │ 1 │ 2 │ 3 │ 4 │ 5  │ 6  │ 7  │ 8  │ 9  │\n",
//!         "├───┼───┼───┼───┼───┼───┼────┼────┼────┼────┼────┤\n",
//!         "│ 0 │ 0 │ 0 │ 0 │ 0 │ 0 │ 0  │ 0  │ 0  │ 0  │ 0  │\n",
//!         "│ 1 │ 0 │ 1 │ 2 │ 3 │ 4 │ 5  │ 6  │ 7  │ 8  │ 9  │\n",
//!         "│ 2 │ 0 │ 2 │ 4 │ 6 │ 8 │ 10 │ 12 │ 14 │ 16 │ 18 │\n",
//!         "╰───┴───┴───┴───┴───┴───┴────┴────┴────┴────┴────╯",
//!     )
//! );
//! ```
//!
//! ### Build table using [`row`] and [`col`].
//!
#![cfg_attr(feature = "macros", doc = "```")]
#![cfg_attr(not(feature = "macros"), doc = "```ignore")]
//! use tabled::{row, col};
//!
//! let table = row![
//!     col!["Hello", "World", "!"],
//!     col!["Hello"; 3],
//!     col!["World"; 3],
//! ].to_string();
//!
//! assert_eq!(
//!     table,
//!     concat!(
//!         "+-----------+-----------+-----------+\n",
//!         "| +-------+ | +-------+ | +-------+ |\n",
//!         "| | Hello | | | Hello | | | World | |\n",
//!         "| +-------+ | +-------+ | +-------+ |\n",
//!         "| | World | | | Hello | | | World | |\n",
//!         "| +-------+ | +-------+ | +-------+ |\n",
//!         "| | !     | | | Hello | | | World | |\n",
//!         "| +-------+ | +-------+ | +-------+ |\n",
//!         "+-----------+-----------+-----------+",
//!     )
//! );
//! ```
//!
//! ## Settings
//!
//! You can find more examples of settings and attributes in
//! [README.md](https://github.com/zhiburt/tabled/blob/master/README.md)

#![cfg_attr(docsrs, feature(doc_cfg, doc_auto_cfg))]
#![doc(
    html_logo_url = "https://raw.githubusercontent.com/zhiburt/tabled/86ac146e532ce9f7626608d7fd05072123603a2e/assets/tabled-gear.svg"
)]
#![warn(
    missing_docs,
    rust_2018_idioms,
    missing_debug_implementations,
    unreachable_pub
)]
#![deny(unused_must_use)]

mod features;
mod modify;
mod records;
mod table;
mod table_iterator_ext;
mod tabled;

pub mod builder;
pub mod display;
pub mod object;

#[cfg(feature = "macros")]
#[cfg_attr(docsrs, doc(cfg(feature = "macros")))]
pub mod macros;

pub use crate::{
    features::style::{self, Border, BorderText, Style},
    modify::{CellSettingsList, Modify, ModifyList, ModifyObject},
    table::{CellOption, Table, TableOption},
    table_iterator_ext::TableIteratorExt,
    tabled::Tabled,
};

#[cfg_attr(docsrs, doc(cfg(feature = "color")))]
pub use crate::features::color;

pub mod grid {
    pub use papergrid::color;
    pub use papergrid::colors;
    pub use papergrid::config;
    pub use papergrid::dimension;
    pub use papergrid::grid_projection;
    pub use papergrid::util;
    pub use papergrid::Grid;
}

/// A derive to implement a [`Tabled`] trait.
///
/// The macros available only when `derive` feature in turned on (and it is by default).
///
/// To be able to use the derive each field must implement `std::fmt::Display`.
/// The following example will cause a error because of that.
///
/// ```rust,compile_fail
/// use tabled::Tabled;
/// #[derive(Tabled)]
/// struct SomeType {
///     field1: SomeOtherType,
/// }
///
/// struct SomeOtherType;
/// ```
///
/// Bellow you'll find available options for it.
///
/// ### Override a column name
///
/// You can use a `#[tabled(rename = "")]` attribute to override a column name.
///
/// ```rust,no_run
/// use tabled::Tabled;
///
/// #[derive(Tabled)]
/// struct Person {
///     #[tabled(rename = "Name")]
///     first_name: &'static str,
///     #[tabled(rename = "Surname")]
///     last_name: &'static str,
/// }
/// ```
///
/// ### Hide a column
///
/// You can mark filds as hidden in which case they fill be ignored and not be present on a sheet.
///
/// A similar affect could be achieved by the means of a `Disable` setting.
///
/// ```rust,no_run
/// use tabled::Tabled;
///
/// #[derive(Tabled)]
/// struct Person {
///    id: u8,
///    #[tabled(skip)]
///    number: &'static str,
///    name: &'static str,
/// }
/// ```
///
/// ### Set column order
///
/// You can change the order in which they will be displayed in table.
///
/// ```rust,no_run
/// use tabled::Tabled;
///
/// #[derive(Tabled)]
/// struct Person {
///    id: u8,
///    #[tabled(order = 0)]
///    number: &'static str,
///    #[tabled(order = 1)]
///    name: &'static str,
/// }
/// ```
///
/// ### Format fields
///
/// As was said already, using `#[derive(Tabled)]` is possible only when all fields implement a `Display` trait.
/// However, this may be often not the case for example when a field uses the `Option` type. There's 2 common ways how to solve this:
///
/// - Implement `Tabled` trait manually for a type.
/// - Wrap `Option` to something like `DisplayedOption<T>(Option<T>)` and implement a Display trait for it.
///
/// Alternatively, you can use the `#[tabled(display_with = "func")]` attribute for the field to specify a display function.
///
/// ```rust,no_run
/// use tabled::Tabled;
///
/// #[derive(Tabled)]
/// pub struct MyRecord {
///     pub id: i64,
///     #[tabled(display_with = "display_option")]
///     pub valid: Option<bool>
/// }
///
/// fn display_option(o: &Option<bool>) -> String {
///     match o {
///         Some(s) => format!("is valid thing = {}", s),
///         None => format!("is not valid"),
///     }
/// }
/// ```
///
/// It's also possible to change function argument to be `&self`,
/// using `#[tabled(display_with("some_function", args))]`
///
/// ```rust,no_run
/// use tabled::Tabled;
///
/// #[derive(Tabled)]
/// pub struct MyRecord {
///     pub id: i64,
///     #[tabled(display_with("Self::display_valid", args))]
///     pub valid: Option<bool>
/// }
///
/// impl MyRecord {
///     fn display_valid(&self) -> String {
///         match self.valid {
///             Some(s) => format!("is valid thing = {}", s),
///             None => format!("is not valid"),
///         }
///     }
/// }
/// ```
///
/// ### Format headers
///
/// Beside `#[tabled(rename = "")]` you can change a format of a column name using
/// `#[tabled(rename_all = "UPPERCASE")]`.
///
/// ```rust,no_run
/// use tabled::Tabled;
///
/// #[derive(Tabled)]
/// #[tabled(rename_all = "CamelCase")]
/// struct Person {
///     id: u8,
///     number: &'static str,
///     name: &'static str,
///     #[tabled(rename_all = "snake_case")]
///     middle_name: &'static str,
/// }
/// ```
///
/// ### Inline
///
/// It's possible to inline internal data if it implements the `Tabled` trait using `#[tabled(inline)]`.
/// You can also set a prefix which will be used for all inlined elements by `#[tabled(inline("prefix>>"))]`.
///
/// ```rust,no_run
/// use tabled::Tabled;
///
/// #[derive(Tabled)]
/// struct Person {
///     id: u8,
///     name: &'static str,
///     #[tabled(inline)]
///     ed: Education,
/// }
///
/// #[derive(Tabled)]
/// struct Education {
///     uni: &'static str,
///     graduated: bool,
/// }
/// ```
///
/// And it works for enums as well.
///
/// ```rust,no_run
/// use tabled::Tabled;
///
/// #[derive(Tabled)]
/// enum Vehicle {
///     #[tabled(inline("Auto::"))]
///     Auto {
///         model: &'static str,
///         engine: &'static str,
///     },
///     #[tabled(inline)]
///     Bikecycle(
///         &'static str,
///         #[tabled(inline)] Bike,
///     ),
/// }
///
/// #[derive(Tabled)]
/// struct Bike {
///     brand: &'static str,
///     price: f32,
/// }
/// ```
// @todo: Move the comment to tabled_derive
#[cfg(feature = "derive")]
#[cfg_attr(docsrs, doc(cfg(feature = "derive")))]
pub use tabled_derive::Tabled;
