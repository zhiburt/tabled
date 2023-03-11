//! Builder module provides a [`Builder`] type which helps building
//! a [`Table`] dynamically.
//!
//! It also contains [`IndexBuilder`] which can help to build a table with index.
//!
//! # Examples
//!
//! Here's an example of [`IndexBuilder`] usage
//!
#![cfg_attr(feature = "derive", doc = "```")]
#![cfg_attr(not(feature = "derive"), doc = "```ignore")]
//! use tabled::{Table, Tabled, settings::Style};
//!
//! #[derive(Tabled)]
//! struct Mission {
//!     name: &'static str,
//!     #[tabled(inline)]
//!     status: Status,
//! }
//!
//! #[derive(Tabled)]
//! enum Status {
//!     Complete,
//!     Started,
//!     Ready,
//!     Unknown,
//! }
//!
//! let data = [
//!     Mission { name: "Algebra", status: Status::Unknown },
//!     Mission { name: "Apolo", status: Status::Complete },
//! ];
//!
//! let mut builder = Table::builder(&data)
//!     .index()
//!     .column(0)
//!     .name(None)
//!     .transpose();
//!
//! let mut table = builder.build();
//! table.with(Style::modern());
//!
//! println!("{}", table);
//!
//! assert_eq!(
//!     table.to_string(),
//!     concat!(
//!         "┌──────────┬─────────┬───────┐\n",
//!         "│          │ Algebra │ Apolo │\n",
//!         "├──────────┼─────────┼───────┤\n",
//!         "│ Complete │         │ +     │\n",
//!         "├──────────┼─────────┼───────┤\n",
//!         "│ Started  │         │       │\n",
//!         "├──────────┼─────────┼───────┤\n",
//!         "│ Ready    │         │       │\n",
//!         "├──────────┼─────────┼───────┤\n",
//!         "│ Unknown  │ +       │       │\n",
//!         "└──────────┴─────────┴───────┘",
//!    ),
//! )
//! ```
//!
//! Example when we don't want to show empty data of enum where not all variants are used.
//!
#![cfg_attr(feature = "derive", doc = "```")]
#![cfg_attr(not(feature = "derive"), doc = "```ignore")]
//! use tabled::{Table, Tabled, settings::Style};
//!
//! #[derive(Tabled)]
//! enum Status {
//!     #[tabled(inline)]
//!     Complete {
//!         started_timestamp: usize,
//!         finihsed_timestamp: usize,
//!     },
//!     #[tabled(inline)]
//!     Started {
//!         timestamp: usize,
//!     },
//!     Ready,
//!     Unknown,
//! }
//!
//! let data = [
//!     Status::Unknown,
//!     Status::Complete { started_timestamp: 123, finihsed_timestamp: 234 },
//! ];
//!
//! let mut builder = Table::builder(&data);
//! builder.clean();
//!
//! let table = builder.build()
//!     .with(Style::modern())
//!     .to_string();
//!
//! println!("{}", table);
//!
//! assert_eq!(
//!     table,
//!     concat!(
//!         "┌───────────────────┬────────────────────┬─────────┐\n",
//!         "│ started_timestamp │ finihsed_timestamp │ Unknown │\n",
//!         "├───────────────────┼────────────────────┼─────────┤\n",
//!         "│                   │                    │ +       │\n",
//!         "├───────────────────┼────────────────────┼─────────┤\n",
//!         "│ 123               │ 234                │         │\n",
//!         "└───────────────────┴────────────────────┴─────────┘",
//!    ),
//! )
//! ```
//!
//! [`Table`]: crate::Table

mod index_builder;
mod table_builder;

pub use index_builder::IndexBuilder;
pub use table_builder::Builder;
