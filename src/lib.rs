#![warn(
    missing_docs,
    rust_2018_idioms,
    missing_debug_implementations,
    unreachable_pub
)]
#![deny(unused_must_use)]

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
//!                 | name |  designed_by   | invented_year |\n\
//!                 +------+----------------+---------------+\n\
//!                 |  C   | Dennis Ritchie |     1972      |\n\
//!                 +------+----------------+---------------+\n\
//!                 | Rust | Graydon Hoare  |     2010      |\n\
//!                 +------+----------------+---------------+\n\
//!                 |  Go  |    Rob Pike    |     2009      |\n\
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
//! #                 |  1  |\n\
//! #                 +-----+\n\
//! #                 |  2  |\n\
//! #                 +-----+\n\
//! #                 |  3  |\n\
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
//! use tabled::{Tabled, Table, Style};
//!
//! #[derive(Tabled)]
//! enum Domain {
//!     Security,
//!     Embeded,
//!     Frontend,
//!     Unknown,
//! }
//!
//! #[derive(Tabled)]
//! struct Developer(#[tabled(rename = "name")] &'static str);
//!     
//! let data = vec![
//!     (Developer("Terri Kshlerin"), Domain::Embeded),
//!     (Developer("Catalina Dicki"), Domain::Security),
//!     (Developer("Jennie Schmeler"), Domain::Frontend),
//!     (Developer("Maxim Zhiburt"), Domain::Unknown),
//! ];
//!     
//! let table = Table::new(data).with(Style::psql()).to_string();
//!
//! assert_eq!(
//!     table,
//!     concat!(
//!         "      name       | Security | Embeded | Frontend | Unknown \n",
//!         "-----------------+----------+---------+----------+---------\n",
//!         " Terri Kshlerin  |          |    +    |          |         \n",
//!         " Catalina Dicki  |    +     |         |          |         \n",
//!         " Jennie Schmeler |          |         |    +     |         \n",
//!         "  Maxim Zhiburt  |          |         |          |    +    "
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
//! ## Settings
//!
//! You can find more examples of settings and attributes in
//! [README.md](https://github.com/zhiburt/tabled/blob/master/README.md)

#![doc(
    html_logo_url = "https://raw.githubusercontent.com/zhiburt/tabled/86ac146e532ce9f7626608d7fd05072123603a2e/assets/tabled-gear.svg"
)]

mod alignment;
mod concat;
mod disable;
mod extract;
mod formating;
mod margin;
mod modify;
mod padding;
mod panel;
mod rotate;
mod span;
mod table;

pub mod builder;
pub mod display;
pub mod formatting_settings;
pub mod highlight;
pub mod object;
pub mod style;
pub mod width;

use std::fmt;

#[cfg(feature = "derive")]
#[cfg_attr(docsrs, doc(cfg(feature = "color")))]
pub use tabled_derive::Tabled;

pub use papergrid;

pub use crate::{
    alignment::*, concat::*, disable::*, extract::*, formating::*, highlight::Highlight, margin::*,
    modify::*, padding::*, panel::*, rotate::*, span::*, style::Style, table::*, width::Width,
};

/// Tabled a trait responsible for providing a header fields and a row fields.
///
/// It's urgent that `header` len is equal to `fields` len.
///
/// ```text
/// Self::headers().len() == self.fields().len()
/// ```
pub trait Tabled {
    /// A length of fields and headers,
    /// which must be the same.
    const LENGTH: usize;

    /// Fields method must return a list of cells.
    ///
    /// The cells will be placed in the same row, preserving the order.
    fn fields(&self) -> Vec<String>;
    /// Headers must return a list of column names.
    fn headers() -> Vec<String>;
}

impl<T> Tabled for &T
where
    T: Tabled,
{
    const LENGTH: usize = T::LENGTH;

    fn fields(&self) -> Vec<String> {
        T::fields(self)
    }
    fn headers() -> Vec<String> {
        T::headers()
    }
}

macro_rules! tuple_table {
    ( $($name:ident)+ ) => {
        impl<$($name: Tabled),+> Tabled for ($($name,)+){
            const LENGTH: usize = $($name::LENGTH+)+ 0;

            fn fields(&self) -> Vec<String> {
                #![allow(non_snake_case)]
                let ($($name,)+) = self;
                let mut fields = Vec::new();
                $(fields.append(&mut $name.fields());)+
                fields
            }

            fn headers() -> Vec<String> {
                let mut fields = Vec::new();
                $(fields.append(&mut $name::headers());)+
                fields
            }
        }
    };
}

tuple_table! { A }
tuple_table! { A B }
tuple_table! { A B C }
tuple_table! { A B C D }
tuple_table! { A B C D E }
tuple_table! { A B C D E F }

macro_rules! default_table {
    ( $t:ty ) => {
        impl Tabled for $t {
            const LENGTH: usize = 1;

            fn fields(&self) -> Vec<String> {
                vec![format!("{}", self)]
            }
            fn headers() -> Vec<String> {
                vec![stringify!($t).to_string()]
            }
        }
    };
}

default_table!(&str);
default_table!(String);

default_table!(char);

default_table!(bool);

default_table!(isize);
default_table!(usize);

default_table!(u8);
default_table!(u16);
default_table!(u32);
default_table!(u64);
default_table!(u128);

default_table!(i8);
default_table!(i16);
default_table!(i32);
default_table!(i64);
default_table!(i128);

default_table!(f32);
default_table!(f64);

impl<T: fmt::Display, const N: usize> Tabled for [T; N] {
    const LENGTH: usize = N;

    fn fields(&self) -> Vec<String> {
        self.iter().map(ToString::to_string).collect()
    }

    fn headers() -> Vec<String> {
        (0..N).map(|i| format!("{}", i)).collect()
    }
}
