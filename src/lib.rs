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

//! This library provides an interface to pretty print vectors of structs
//!
//! # Get started
//!
//! The common and probably the best way to begin is to annotate your type with
//! `#[derive(Tabled)]`. You can also implement it on your own as well.
//!
//! There's an example. Precisely it can be printed and you
//! will see the content of `expected` variable as an output.
//!
//! ```rust
//! use tabled::{Tabled, table};
//!
//! #[derive(Tabled)]
//! struct Language {
//!     name: String,
//!     designed_by: String,
//!     invented_year: usize,
//! }
//!
//! let languages = vec![
//!     Language{
//!         name: "C".to_owned(),
//!         designed_by: "Dennis Ritchie".to_owned(),
//!         invented_year: 1972
//!     },
//!     Language{
//!         name: "Rust".to_owned(),
//!         designed_by: "Graydon Hoare".to_owned(),
//!         invented_year: 2010},
//! ];
//!
//! let table = table!(&languages);
//! let expected = "+------+----------------+---------------+\n\
//!                 | name |  designed_by   | invented_year |\n\
//!                 +------+----------------+---------------+\n\
//!                 |  C   | Dennis Ritchie |     1972      |\n\
//!                 +------+----------------+---------------+\n\
//!                 | Rust | Graydon Hoare  |     2010      |\n\
//!                 +------+----------------+---------------+\n";
//!
//! assert_eq!(expected, table);
//! ```
//!
//! It should have a clue in what why print the field
//! accordingly each field should implement `std::fmt::Display`
//! The example below is not compiled
//!
//! ```rust,compile_fail
//! # use tabled::Tabled;
//! #[derive(Tabled)]
//! struct SomeType {
//!     field1: SomeOtherType,
//! }
//!
//! struct SomeOtherType;
//! ```
//! This crate implement the trait for default types.
//! Therefore you can use this to print one column vectors
//!
//! ```rust
//! use tabled::{Tabled, table};
//!
//! let some_numbers = [1, 2, 3];
//! let table = table!(&some_numbers);
//! # let expected = "+-----+\n\
//! #                 | i32 |\n\
//! #                 +-----+\n\
//! #                 |  1  |\n\
//! #                 +-----+\n\
//! #                 |  2  |\n\
//! #                 +-----+\n\
//! #                 |  3  |\n\
//! #                 +-----+\n";
//! # assert_eq!(expected, table);
//! ```

use papergrid::{Alignment, Border, Entity, Grid, Settings};

pub use tabled_derive::Tabled;

pub trait Tabled {
    fn fields(&self) -> Vec<String>;
    fn headers() -> Vec<String>;
}

pub fn print_table<T: Tabled>(iter: impl IntoIterator<Item = T>, style: Style) -> String {
    let headers = T::headers();
    let obj: Vec<Vec<String>> = iter.into_iter().map(|t| t.fields()).collect();

    let mut grid = create_grid(obj.len() + 1, headers.len(), style);
    for (i, h) in headers.iter().enumerate() {
        grid.set(
            Entity::Cell(0, i),
            Settings::new()
                .text(h)
                .ident(1, 1, 0, 0)
                .alignment(Alignment::Center),
        );
    }

    for (i, fields) in obj.iter().enumerate() {
        for (j, field) in fields.iter().enumerate() {
            grid.set(
                Entity::Cell(i + 1, j),
                Settings::new()
                    .text(field)
                    .ident(1, 1, 0, 0)
                    .alignment(Alignment::Center),
            );
        }
    }

    grid.to_string()
}

fn create_grid(count_rows: usize, count_columns: usize, style: Style) -> Grid {
    let mut grid = Grid::new(count_rows, count_columns);

    for row in 0..count_rows {
        let border = grid.get_border_mut(row);
        style.make(border, row, count_rows);
    }

    grid
}

pub enum Style {
    Default,
    Psql,
    GithubMarkdown,
    Pseudo,
    PseudoClean,
    NoBorder,
}

impl Style {
    fn make(&self, border: &mut Border, row: usize, count_rows: usize) {
        match self {
            Style::Default => (),
            Style::NoBorder => Self::noborder_style(border),
            Style::GithubMarkdown => Self::github_markdown_style(border, row),
            Style::Pseudo => Self::pseudo_style(border, row, count_rows),
            Style::PseudoClean => Self::pseudo_clean_style(border, row, count_rows),
            Style::Psql => Self::psql_style(border, row),
        }
    }

    fn noborder_style(border: &mut Border) {
        border.empty().inner(Some(' '), None, None);
    }

    fn psql_style(border: &mut Border, row: usize) {
        if row == 0 {
            border
                .empty()
                .bottom('-', '+', None, None)
                .inner(Some('|'), None, None);
        } else {
            border.empty().inner(Some('|'), None, None);
        }
    }

    fn github_markdown_style(border: &mut Border, row: usize) {
        if row == 0 {
            border.empty().bottom('-', '+', Some('|'), Some('|')).inner(
                Some('|'),
                Some('|'),
                Some('|'),
            );
        } else {
            border.empty().inner(Some('|'), Some('|'), Some('|'));
        }
    }

    fn pseudo_style(border: &mut Border, row: usize, count_rows: usize) {
        if row == 0 {
            border
                .empty()
                .top('─', '┬', Some('┌'), Some('┐'))
                .bottom('─', '┼', Some('├'), Some('┤'))
                .inner(Some('│'), Some('│'), Some('│'));
        } else if row == count_rows - 1 {
            border.empty().bottom('─', '┴', Some('└'), Some('┘')).inner(
                Some('│'),
                Some('│'),
                Some('│'),
            );
        } else {
            border.empty().bottom('─', '┼', Some('├'), Some('┤')).inner(
                Some('│'),
                Some('│'),
                Some('│'),
            );
        }
    }

    fn pseudo_clean_style(border: &mut Border, row: usize, count_rows: usize) {
        if row == 0 {
            border
                .empty()
                .top('─', '┬', Some('┌'), Some('┐'))
                .bottom('─', '┼', Some('├'), Some('┤'))
                .inner(Some('│'), Some('│'), Some('│'));
        } else if row == count_rows - 1 {
            border.empty().bottom('─', '┴', Some('└'), Some('┘')).inner(
                Some('│'),
                Some('│'),
                Some('│'),
            );
        } else {
            border.empty().inner(Some('│'), Some('│'), Some('│'));
        }
    }
}

#[macro_export]
macro_rules! table {
    ( $data:expr ) => {
        tabled::print_table($data, tabled::Style::Default)
    };
    ( $data:expr, $style:expr ) => {
        tabled::print_table($data, $style)
    };
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
