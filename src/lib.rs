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
//! # fn main() {
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
//! let table = table(&languages);
//! let expected = "+------+----------------+---------------+\n\
//!                 | name |  designed_by   | invented_year |\n\
//!                 +------+----------------+---------------+\n\
//!                 |  C   | Dennis Ritchie |     1972      |\n\
//!                 +------+----------------+---------------+\n\
//!                 | Rust | Graydon Hoare  |     2010      |\n\
//!                 +------+----------------+---------------+\n";
//!
//! assert_eq!(expected, table);
//! # }
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
//! # fn main() {
//! let some_numbers = [1, 2, 3];
//! let table = table(&some_numbers);
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
//! # }
//! ```
use papergrid;
pub use tabled_derive::Tabled;

pub trait Tabled {
    fn fields(&self) -> Vec<String>;
    fn headers() -> Vec<String>;
}

pub fn table<T: Tabled>(iter: impl IntoIterator<Item = T>) -> String {
    let headers = T::headers();
    let obj: Vec<Vec<String>> = iter.into_iter().map(|t| t.fields()).collect();

    let mut grid = papergrid::Grid::new(obj.len() + 1, headers.len());
    for (i, h) in headers.iter().enumerate() {
        grid.cell(0, i).set_content(h).set_horizontal_ident(1);
    }

    for (i, fields) in obj.iter().enumerate() {
        for (j, field) in fields.iter().enumerate() {
            grid.cell(i + 1, j)
                .set_content(field)
                .set_horizontal_ident(1);
        }
    }

    grid.to_string()
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
