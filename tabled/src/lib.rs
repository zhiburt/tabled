//! An easy to use library for pretty print tables of Rust `struct`s and `enum`s.
//!
//! The library supports different approaches of table building.
//! You can use [`Tabled`] trait if the data type is known.
//! Or you can use [`Builder`] to construct the table from scratch.
//!
//! ## Derive
//!
//! If you want to build a table for your custom type.
//! A starting point is to a annotate your type with `#[derive(Tabled)]`.
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
//!     Language{ name: "C", designed_by: "Dennis Ritchie", invented_year: 1972 },
//!     Language{ name: "Rust", designed_by: "Graydon Hoare", invented_year: 2010 },
//!     Language{ name: "Go", designed_by: "Rob Pike", invented_year: 2009 },
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
//! BEWARE not all types can derive [`Tabled`] trait.
//! The example below can't be compiled.
//!
//! Because `tabled` must know what we're up to print as a field, so
//! each field must implement [`std::fmt::Display`].
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
//! You can tweak it by derive options.
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
//! ### Builder
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
//! let header = iter::once(String::from("i")).chain((0..y).map(|i| i.to_string()));
//! builder.push_record(header);
//!
//! for i in 0..x {
//!     let row = iter::once(i).chain((0..y).map(|j| i * j)).map(|i| i.to_string());
//!     builder.push_record(row);
//! }
//!
//! let table = builder.build()
//!     .with(Style::rounded())
//!     .modify(Rows::new(1..), Alignment::left())
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
//! # Features
//!
//! - `std`     - Used by default. If not its considered `no_std` with a limited set of functionality.
//! - `derive`  - Used by default. A support for `Tabled` derive macro.
//! - `ansi`    - A support for ANSI sequences.
//! - `macros`  - A support for `row!`, `col!` macro.
//!
//! # Advanced
//!
//! ## Table types
//!
//! [`Table`] keeps data buffered, which sometimes not ideal choice.
//! For such reason there is [`IterTable`] and [`CompactTable`].
//!
//! ### [`IterTable`]
//!
//! [`IterTable`] stands on a middle ground between [`Table`] and [`CompactTable`].
//!
//! It does allocate memory but in a much smaller chunks that a [`Table`] does.
//! The benefit is that it can be used interchangeably with [`Table`].
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
//! ### [`CompactTable`]
//!
//! Alloc free can be configured ('1) to not make any allocations.
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
//! ## `no_std`
//!
//! [`CompactTable`] can be used in `no_std` context.
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
//! [`tabled::settings`]: crate::settings

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

mod util;

#[cfg(feature = "derive")]
mod derive;
#[cfg(feature = "macros")]
mod macros;
#[cfg(feature = "std")]
mod tabled;

#[cfg(feature = "std")]
#[cfg_attr(docsrs, doc(cfg(feature = "std")))]
pub mod builder;
pub mod grid;
pub mod settings;
pub mod tables;

#[cfg(feature = "std")]
#[cfg_attr(docsrs, doc(cfg(feature = "std")))]
pub use crate::{tabled::Tabled, tables::Table};

#[cfg(feature = "derive")]
#[cfg_attr(docsrs, doc(cfg(feature = "derive")))]
pub use derive::Tabled;
