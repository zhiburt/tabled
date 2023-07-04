//! An easy to use library for pretty print tables of Rust `struct`s and `enum`s.
//!
//! The library supports different approaches of table building.
//! You can use [`Tabled`] trait if the data type is known.
//! Or you can use [`Builder`] to construct the table from scratch.
//!
//! ## Usage
//!
//! If you want to build a table for your custom type.
//! A starting point is to a anotate your type with `#[derive(Tabled)]`.
//!
//! Then to provide your collection to [`Table::new`] and you will be set to render table.
//!
#![cfg_attr(all(feature = "derive", feature = "std"), doc = "```")]
#![cfg_attr(not(all(feature = "derive", feature = "std")), doc = "```ignore")]
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
//! Because `tabled` must know what we're up to print as a field, so
//! each (almoust) field must implement [`std::fmt::Display`].
//!
//! ### Default implementations
//!
//! [`Table`] can be build from vast majority of Rust's standard types.
//! This allows you to run the following code.
//!
#![cfg_attr(feature = "std", doc = "```")]
#![cfg_attr(not(feature = "std"), doc = "```ignore")]
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
//! ### Dynamic table
//!
//! When you data scheme is not known at compile time.
//! You most likely will not able to relay on [`Tabled`] trait.
//!
//! So one option would be is to use [`Builder`].
//!
#![cfg_attr(feature = "std", doc = "```")]
#![cfg_attr(not(feature = "std"), doc = "```ignore")]
//! use std::iter;
//!
//! use tabled::{
//!     builder::Builder,
//!     settings::{Modify, object::Rows, Alignment, Style}
//! };
//!
//! let (x, y) = (3, 10);
//!
//! let mut builder = Builder::default();
//!
//! let header = iter::once(String::from("i"))
//!     .chain((0..y)
//!     .map(|i| i.to_string()));
//! builder.set_header(header);
//!
//! for i in 0..x {
//!     let row = iter::once(i)
//!         .chain((0..y).map(|j| i * j))
//!         .map(|i| i.to_string());
//!     builder.push_record(row);
//! }
//!
//! let table = builder.build()
//!     .with(Style::rounded())
//!     .with(Modify::new(Rows::new(1..)).with(Alignment::left()))
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
//! ### Build table using [`row!`] and [`col!`] macros.
//!
#![cfg_attr(all(feature = "macros", feature = "std"), doc = "```")]
#![cfg_attr(not(all(feature = "macros", feature = "std")), doc = "```ignore")]
//! use tabled::{row, col};
//!
//! let table = row![
//!     col!["Hello", "World", "!"],
//!     col!["Hello"; 3],
//!     col!["World"; 3],
//! ];
//!
//! assert_eq!(
//!     table.to_string(),
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
//! ### Settings
//!
//! You can use many settings which is found in [`tabled::settings`] module.
//!
//! # Advanced
//!
//! ## Alloc
//!
//! [`Table`] keeps data buffered, which sometimes not ideal choise.
//! For such reason there is [`IterTable`] and [`CompactTable`].
//!
//! ### Less allocations
//!
//! [`IterTable`] stands on a middle ground between [`Table`] and [`CompactTable`].
//!
//! It does allocate memory but in a much smaller chunks that a [`Table`] does.
//! The benefit is that it can be used interchangebly with [`Table`].
//!
#![cfg_attr(feature = "std", doc = "```")]
#![cfg_attr(not(feature = "std"), doc = "```ignore")]
//! use tabled::tables::IterTable;
//!
//! let iterator = (0..3).map(|row| (0..4).map(move |col| format!("{}-{}", row, col)));
//!
//! let table = IterTable::new(iterator).to_string();
//!
//! assert_eq!(
//!     table,
//!     "+-----+-----+-----+-----+\n\
//!      | 0-0 | 0-1 | 0-2 | 0-3 |\n\
//!      +-----+-----+-----+-----+\n\
//!      | 1-0 | 1-1 | 1-2 | 1-3 |\n\
//!      +-----+-----+-----+-----+\n\
//!      | 2-0 | 2-1 | 2-2 | 2-3 |\n\
//!      +-----+-----+-----+-----+",
//! );
//! ```
//!
//! ## Alloc free (`#nostd`)
//!
//! [`CompactTable`] can be configured ('1) to not make any allocations.
//! But the price is that the set of settings which can be applied to it is limited.  
//!
//! It also can be printed directly to [`fmt::Write`] to not have any intermidiaries.
//!
//! '1. It does not make any allocations in case you provide it with `width` and `count_rows`.
//!
//! ```
//! use tabled::{settings::Style, tables::CompactTable};
//! use core::fmt::{Write, Result};
//!
//! struct StubWriter;
//!
//! impl Write for StubWriter {
//!     fn write_str(&mut self, _: &str) -> Result {
//!         Ok(())
//!     }
//! }
//!
//! let data = [
//!     ["FreeBSD", "1993", "William and Lynne Jolitz", "?"],
//!     ["OpenBSD", "1995", "Theo de Raadt", ""],
//!     ["HardenedBSD", "2014", "Oliver Pinter and Shawn Webb", ""],
//! ];
//!
//! let table = CompactTable::from(data).with(Style::psql());
//!
//! table.fmt(StubWriter);
//! ```
//!
//! ## More information
//!
//! You can find more examples of settings and attributes in
//! [README.md](https://github.com/zhiburt/tabled/blob/master/README.md)
//!
//! [`Builder`]: crate::builder::Builder
//! [`IterTable`]: crate::tables::IterTable
//! [`CompactTable`]: crate::tables::CompactTable
//! [`fmt::Write`]: core::fmt::Write
//! [`row!`]: crate::row
//! [`col!`]: crate::col

#![cfg_attr(not(any(feature = "std", test)), no_std)]
#![cfg_attr(docsrs, feature(doc_cfg, doc_auto_cfg))]
#![doc(
    html_logo_url = "https://raw.githubusercontent.com/zhiburt/tabled/86ac146e532ce9f7626608d7fd05072123603a2e/assets/tabled-gear.svg"
)]
#![deny(unused_must_use)]
#![warn(
    missing_docs,
    rust_2018_idioms,
    rust_2018_compatibility,
    missing_debug_implementations,
    unreachable_pub,
    future_incompatible,
    single_use_lifetimes,
    trivial_casts,
    trivial_numeric_casts,
    unused_extern_crates,
    unused_import_braces,
    unused_qualifications,
    unused_results,
    unused_variables,
    variant_size_differences
)]
#![allow(clippy::uninlined_format_args)]

#[cfg(feature = "std")]
mod tabled;

#[cfg(feature = "std")]
#[cfg_attr(docsrs, doc(cfg(feature = "std")))]
pub mod builder;
pub mod settings;
pub mod tables;

#[cfg(feature = "macros")]
#[cfg_attr(docsrs, doc(cfg(feature = "macros")))]
pub mod macros;

pub mod grid;

#[cfg(feature = "std")]
#[cfg_attr(docsrs, doc(cfg(feature = "std")))]
pub use crate::{tabled::Tabled, tables::Table};

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
/// You can mark fields as hidden in which case they fill be ignored and not be present on a sheet.
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
/// using `#[tabled(display_with("some_function", self))]`
///
/// ```rust,no_run
/// use tabled::Tabled;
///
/// #[derive(Tabled)]
/// pub struct MyRecord {
///     pub id: i64,
///     #[tabled(display_with("Self::display_valid", self))]
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
#[cfg(feature = "derive")]
#[cfg_attr(docsrs, doc(cfg(feature = "derive")))]
pub use tabled_derive::Tabled;
