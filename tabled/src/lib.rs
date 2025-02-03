//! An easy to use library for pretty print tables of Rust `struct`s and `enum`s.
//!
//! There's two approaches to construct a table.
//!
//! 1. When the type of data is known.
//! 2. When it's unknown.
//!
//! Here you can work with both.\
//! For first approach you shall find [`derive::Tabled`] macros being very helpfull.\
//! For a later one you shall take a look at [`Builder`].
//!
//! There are a number of [`settings`] you can use\
//! to change table appearance, layout and data itself.
//!
//! Beside a default [`Table`] type there are more,\
//! more specific table which works best when there are some constraints.
//!
#![cfg_attr(all(feature = "derive", feature = "std"), doc = "```")]
#![cfg_attr(not(all(feature = "derive", feature = "std")), doc = "```ignore")]
//! use tabled::{Tabled, Table};
//! use tabled::settings::{Style, Alignment, object::Columns};
//! use testing_table::assert_table;
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
//! let mut table = Table::new(languages);
//! table.with(Style::modern());
//! table.modify(Columns::first(), Alignment::right());
//!
//! assert_table!(
//!     table,
//!     "┌──────┬────────────────┬───────────────┐"
//!     "│ name │ designed_by    │ invented_year │"
//!     "├──────┼────────────────┼───────────────┤"
//!     "│    C │ Dennis Ritchie │ 1972          │"
//!     "├──────┼────────────────┼───────────────┤"
//!     "│ Rust │ Graydon Hoare  │ 2010          │"
//!     "├──────┼────────────────┼───────────────┤"
//!     "│   Go │ Rob Pike       │ 2009          │"
//!     "└──────┴────────────────┴───────────────┘"
//! );
//! ```
//!
//! ## Building table step by step
//!
//! When you data scheme is not known at compile time.\
//! You most likely will not able to relay on [`Table`].\
//! One option would be is to use [`Builder`].
//!
#![cfg_attr(feature = "std", doc = "```")]
#![cfg_attr(not(feature = "std"), doc = "```ignore")]
//! use std::iter::once;
//! use tabled::{builder::Builder, settings::Style};
//! use testing_table::assert_table;
//!
//! const X: usize = 3;
//! const Y: usize = 5;
//!
//! let mut builder = Builder::default();
//!
//! for i in 0..X {
//!     let row = (0..Y).map(|j| (i * j).to_string());
//!     builder.push_record(row);
//! }
//!
//! builder.insert_record(0, (0..Y).map(|i| i.to_string()));
//! builder.insert_column(0, once(String::new()).chain((0..X).map(|i| i.to_string())));
//!
//! let mut table = builder.build();
//! table.with(Style::rounded());
//!
//! assert_table!(
//!     table,
//!     "╭───┬───┬───┬───┬───┬───╮"
//!     "│   │ 0 │ 1 │ 2 │ 3 │ 4 │"
//!     "├───┼───┼───┼───┼───┼───┤"
//!     "│ 0 │ 0 │ 0 │ 0 │ 0 │ 0 │"
//!     "│ 1 │ 0 │ 1 │ 2 │ 3 │ 4 │"
//!     "│ 2 │ 0 │ 2 │ 4 │ 6 │ 8 │"
//!     "╰───┴───┴───┴───┴───┴───╯"
//! );
//! ```
//!
//! ## Settings
//!
//! You can find lots of settings in [`tabled::settings`].
//!
//! ## Hints
//!
//! [`Table`] can be build from vast majority of Rust's standard types.\
//! This allows you to run the following code.
//!
#![cfg_attr(feature = "std", doc = "```")]
#![cfg_attr(not(feature = "std"), doc = "```ignore")]
//! use tabled::Table;
//! use testing_table::assert_table;
//!
//! let table = Table::new(&[1, 2, 3]);
//!
//! assert_table!(
//!     table,
//!     "+-----+"
//!     "| i32 |"
//!     "+-----+"
//!     "| 1   |"
//!     "+-----+"
//!     "| 2   |"
//!     "+-----+"
//!     "| 3   |"
//!     "+-----+"
//! );
//! ```
//!
//! You can compine types, and settings together using a tupples.\
//! And achive magical results.
//!
#![cfg_attr(feature = "std", doc = "```")]
#![cfg_attr(not(feature = "std"), doc = "```ignore")]
//! use tabled::Table;
//! use tabled::settings::{style::{Style, HorizontalLine}, Alignment, Padding};
//! use testing_table::assert_table;
//!
//! let data = &[(1, 2, "Hello"), (1, 3, "World")];
//!
//! let mut table = Table::new(data);
//! table.with(
//!     Style::modern()
//!         .remove_horizontal()
//!         .horizontals([(1, HorizontalLine::inherit(Style::modern()))])
//! );
//! table.with((Alignment::right(), Padding::new(2, 0, 2, 1)));
//!
//! assert_table!(
//!     table,
//!     "┌─────┬─────┬───────┐"
//!     "│     │     │       │"
//!     "│     │     │       │"
//!     "│  i32│  i32│   &str│"
//!     "│     │     │       │"
//!     "├─────┼─────┼───────┤"
//!     "│     │     │       │"
//!     "│     │     │       │"
//!     "│    1│    2│  Hello│"
//!     "│     │     │       │"
//!     "│     │     │       │"
//!     "│     │     │       │"
//!     "│    1│    3│  World│"
//!     "│     │     │       │"
//!     "└─────┴─────┴───────┘"
//! );
//! ```
//!
//! Be ware you don't obligated to `collect` your data before building.
//!
#![cfg_attr(all(feature = "derive", feature = "std"), doc = "```")]
#![cfg_attr(not(all(feature = "derive", feature = "std")), doc = "```ignore")]
//! use tabled::{Tabled, Table};
//! use testing_table::assert_table;
//! use std::iter::once;
//!
//! #[derive(Tabled)]
//! struct Data(
//!     #[tabled(rename = "word")]
//!     String,
//!     #[tabled(rename = "id")]
//!     usize,
//! );
//!
//! let data = once(Data(String::from("Hello"), 0))
//!     .chain(once(Data(String::from("World"), 1)))
//!     .chain(once(Data(String::from("!!!"), 2)));
//!
//! let mut table = Table::new(data);
//!
//! assert_table!(
//!     table,
//!     "+-------+----+"
//!     "| word  | id |"
//!     "+-------+----+"
//!     "| Hello | 0  |"
//!     "+-------+----+"
//!     "| World | 1  |"
//!     "+-------+----+"
//!     "| !!!   | 2  |"
//!     "+-------+----+"
//! );
//! ```
//!
//! Build table using [`row!`] and [`col!`] macros.
//!
#![cfg_attr(all(feature = "macros", feature = "std"), doc = "```")]
#![cfg_attr(not(all(feature = "macros", feature = "std")), doc = "```ignore")]
//! use tabled::{row, col};
//! use testing_table::assert_table;
//!
//! let table = row![
//!     col!["Hello", "World", "!"],
//!     col!["Hello"; 3],
//!     col!["World"; 3],
//! ];
//!
//! assert_table!(
//!     table,
//!     "+-----------+-----------+-----------+"
//!     "| +-------+ | +-------+ | +-------+ |"
//!     "| | Hello | | | Hello | | | World | |"
//!     "| +-------+ | +-------+ | +-------+ |"
//!     "| | World | | | Hello | | | World | |"
//!     "| +-------+ | +-------+ | +-------+ |"
//!     "| | !     | | | Hello | | | World | |"
//!     "| +-------+ | +-------+ | +-------+ |"
//!     "+-----------+-----------+-----------+"
//! );
//! ```
//!
//! # `no_std`
//!
//! Only [`CompactTable`] can be used in `no_std` context.
//!
//! # Features
//!
//! - `std`     - Used by default. If not its considered `no_std` with a limited set of functionality.
//! - `derive`  - Used by default. A support for `Tabled` derive macro.
//! - `ansi`    - A support for ANSI sequences.
//! - `macros`  - A support for `row!`, `col!` macro.
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

#[cfg(feature = "macros")]
mod macros;
#[cfg(feature = "std")]
mod tabled;
mod util;

#[cfg(feature = "std")]
#[cfg_attr(docsrs, doc(cfg(feature = "std")))]
pub mod builder;
#[cfg(feature = "derive")]
#[cfg_attr(docsrs, doc(cfg(feature = "derive")))]
pub mod derive;
pub mod grid;
pub mod iter;
pub mod settings;
pub mod tables;

#[cfg(feature = "std")]
#[cfg_attr(docsrs, doc(cfg(feature = "std")))]
pub use crate::{tabled::Tabled, tables::Table};

#[cfg(feature = "derive")]
#[cfg_attr(docsrs, doc(cfg(feature = "derive")))]
pub use derive::Tabled;
