// Copyright (c) 2021 Maxim Zhiburt
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.

//! An easy to use library for pretty print tables of Rust `struct`s and `enum`s.
//!
//! # Get started
//!
//! The common and probably the best way to begin is to annotate your type with
//! `#[derive(Tabled)]`. You can also implement it on your own as well.
//!
//! ```rust
//!     use tabled::{Tabled, table};
//!
//!     #[derive(Tabled)]
//!     struct Language {
//!         name: &'static str,
//!         designed_by: &'static str,
//!         invented_year: usize,
//!     }
//!
//!     let languages = vec![
//!         Language{
//!             name: "C",
//!             designed_by: "Dennis Ritchie",
//!             invented_year: 1972
//!         },
//!         Language{
//!             name: "Rust",
//!             designed_by: "Graydon Hoare",
//!             invented_year: 2010
//!         },
//!         Language{
//!             name: "Go",
//!             designed_by: "Rob Pike",
//!             invented_year: 2009
//!         },
//!     ];
//!
//!     let table = table!(&languages);
//!     let expected = "+------+----------------+---------------+\n\
//!                     | name |  designed_by   | invented_year |\n\
//!                     +------+----------------+---------------+\n\
//!                     |  C   | Dennis Ritchie |     1972      |\n\
//!                     +------+----------------+---------------+\n\
//!                     | Rust | Graydon Hoare  |     2010      |\n\
//!                     +------+----------------+---------------+\n\
//!                     |  Go  |    Rob Pike    |     2009      |\n\
//!                     +------+----------------+---------------+\n";
//!
//!     assert_eq!(expected, table);
//! ```
//!
//! We must to know what we print in the field
//! accordingly each field should implement `std::fmt::Display`
//! The example below is not compiled
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
//! Most of the default types implements the trait out of the box.
//!
//! ```rust
//!     use tabled::{Tabled, table};
//!     let some_numbers = [1, 2, 3];
//!     let table = table!(&some_numbers);
//!     # let expected = "+-----+\n\
//!     #                 | i32 |\n\
//!     #                 +-----+\n\
//!     #                 |  1  |\n\
//!     #                 +-----+\n\
//!     #                 |  2  |\n\
//!     #                 +-----+\n\
//!     #                 |  3  |\n\
//!     #                 +-----+\n";
//!     # assert_eq!(expected, table);
//! ```
//!
//! You also can combine structures by means of tuples.
//!
//! ```rust
//!     use tabled::{Tabled, table, Style};
//!
//!     #[derive(Tabled)]
//!     enum Domain {
//!         Security,
//!         Embeded,
//!         Frontend,
//!         Unknown,
//!     }
//!
//!     #[derive(Tabled)]
//!     struct Developer(#[header("name")] &'static str);
//!     
//!     let data = vec![
//!         (Developer("Terri Kshlerin"), Domain::Embeded),
//!         (Developer("Catalina Dicki"), Domain::Security),
//!         (Developer("Jennie Schmeler"), Domain::Frontend),
//!         (Developer("Maxim Zhiburt"), Domain::Unknown),
//!     ];
//!     
//!     let table = table!(data, Style::Psql);
//!
//!     assert_eq!(
//!         table,
//!         concat!(
//!             "      name       | Security | Embeded | Frontend | Unknown \n",
//!             "-----------------+----------+---------+----------+---------\n",
//!             " Terri Kshlerin  |          |    +    |          |         \n",
//!             " Catalina Dicki  |    +     |         |          |         \n",
//!             " Jennie Schmeler |          |         |    +     |         \n",
//!             "  Maxim Zhiburt  |          |         |          |    +    \n"
//!         )
//!     );
//! ```
//!

mod alignment;
mod formating;
mod object;
mod style;
mod disable;

pub use crate::{alignment::*, formating::*, object::*, style::Style, disable::*};
pub use papergrid::Alignment;
pub use tabled_derive::Tabled;

use papergrid::{Entity, Grid, Settings};

/// Tabled a trait responsible for providing a header filds and a row fields.
///
/// It's urgent that `header` len is equal to `fields` len.
///
/// ```text
///     Self::headers().len() == self.fields().len()
/// ```
pub trait Tabled {
    /// Fields must return a list of cell in a row
    fn fields(&self) -> Vec<String>;
    /// Headers return a list of names for columns
    fn headers() -> Vec<String>;
}

impl<T> Tabled for &T
where
    T: Tabled,
{
    fn fields(&self) -> Vec<String> {
        T::fields(self)
    }
    fn headers() -> Vec<String> {
        T::headers()
    }
}

/// A trait for configuring a `Grid`.
///
/// Mainly was created to be able to have a variadic set of parameters in a [the `table` macros](./macros.table.html)
pub trait TableOption {
    /// Modification function of a `Grid`
    fn change(&self, grid: &mut Grid);
}

impl TableOption for () {
    fn change(&self, _: &mut Grid) {}
}

impl<E> TableOption for Vec<E>
where
    E: TableOption,
{
    fn change(&self, grid: &mut Grid) {
        for e in self {
            e.change(grid);
        }
    }
}

/// Table macro returns a built table as a string.
/// It may take a list of arguments such as [`Style`](./enum.Style.html),
/// [`HorizontalAlignment`](./struct.HorizontalAlignment.html), [`ChangeRing`](./struct.ChangeRing.html)
///
/// # Example
///
/// ## Basic usage
///
/// ```rust,no_run
///     use tabled::table;
///     let data: Vec<&'static str> = Vec::new();
///     let table = table!(data);
///     println!("{}", table);
/// ```
///
/// ## A list of settings
///
/// ```rust,no_run
///     use tabled::{table, Style, HorizontalAlignment, Alignment, Full};
///     let data = vec!["Hello", "2021"];
///     let table = table!(
///        &data,
///        Style::Psql,
///        HorizontalAlignment(Full, Alignment::Left)
///     );
///     println!("{}", table);
/// ```
///
#[macro_export]
macro_rules! table {
    ( $data:expr ) => {
        tabled::table!($data, tabled::Style::Default)
    };
    ( $data:expr, $($opt:expr),+ $(,)? ) => {{
        use tabled::TableOption;
        let mut grid = tabled::build_grid($data);
        $(
            $opt.change(&mut grid);
        )+

        grid.to_string()
    }};
}

/// Build_grid function build a [`Grid`](../papergrid/struct.Grid.html) from a data.
/// A [`table` macros](./macro.table.html) should be prefered over this function.
pub fn build_grid<T: Tabled>(iter: impl IntoIterator<Item = T>) -> Grid {
    let headers = T::headers();
    let obj: Vec<Vec<String>> = iter.into_iter().map(|t| t.fields()).collect();

    let mut grid = Grid::new(obj.len() + 1, headers.len());

    // it's crusial to set a global setting rather than a setting for an each cell
    // as it will be hard to override that since how Grid::style method works
    grid.set(
        Entity::Global,
        Settings::new()
            .ident(1, 1, 0, 0)
            .alignment(Alignment::Center),
    );

    for (i, h) in headers.iter().enumerate() {
        grid.set(Entity::Cell(0, i), Settings::new().text(h));
    }

    let mut row = 1;
    for fields in &obj {
        for (column, field) in fields.iter().enumerate() {
            grid.set(Entity::Cell(row, column), Settings::new().text(field));
        }

        // don't show off a empty data array
        // currently it's possible when `#[header(hidden)]` attribute used for a enum
        if !fields.is_empty() {
            row += 1;
        }
    }

    grid
}

macro_rules! tuple_table {
    ( $($name:ident)+ ) => {
        impl<$($name: Tabled),+> Tabled for ($($name,)+){
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
tuple_table! { A B C D}
tuple_table! { A B C D E}
tuple_table! { A B C D E F}

macro_rules! default_table {
    ( $t:ty ) => {
        impl Tabled for $t {
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
