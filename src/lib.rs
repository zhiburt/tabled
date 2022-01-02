//! An easy to use library for pretty print tables of Rust `struct`s and `enum`s.
//!
//! The library is based on a [Tabled] trait which is used to actually build tables.
//! It also provides an variate of dynamic settings for customization of a [Table].
//!
//! [Table] can be build from vast majority of Rust's standart types.
//!
//! ## Usage
//!
//! If you want to build a table for your custom type.
//! A starting point is to a anotate your type with `#[derive(Tabled)]`.
//!
//! Then you can create `Table::new` to create a table;
//!
//! ```rust
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
//!                 +------+----------------+---------------+\n";
//!
//! assert_eq!(table, expected);
//! ```
//!
//! You can also create a table by using [TableIteratorExt].
//!
//! ```rust,no_run
//! # let languages = [""];
//! use tabled::TableIteratorExt;
//! let table = languages.table();
//! ```
//!
//! Not all types can derive [Tabled] trait though.
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
//! each field must implement [std::fmt::Display].
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
//! #                 +-----+\n";
//! # assert_eq!(table.to_string(), expected);
//! ```
//!
//! ### Combination of types via tuples
//!
//! Personally I consider this a feature which drives the library to shine.
//! You can combine any types that implements [Tabled] trait into one table.
//!
//! You can also see in this example a `#[header("name")]` usage which configures a header
//! of a table which will be printed.
//! You could change it dynamically as well.
//!
//! ```rust
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
//! struct Developer(#[header("name")] &'static str);
//!     
//! let data = vec![
//!     (Developer("Terri Kshlerin"), Domain::Embeded),
//!     (Developer("Catalina Dicki"), Domain::Security),
//!     (Developer("Jennie Schmeler"), Domain::Frontend),
//!     (Developer("Maxim Zhiburt"), Domain::Unknown),
//! ];
//!     
//! let table = Table::new(data).with(Style::PSQL).to_string();
//!
//! assert_eq!(
//!     table,
//!     concat!(
//!         "      name       | Security | Embeded | Frontend | Unknown \n",
//!         "-----------------+----------+---------+----------+---------\n",
//!         " Terri Kshlerin  |          |    +    |          |         \n",
//!         " Catalina Dicki  |    +     |         |          |         \n",
//!         " Jennie Schmeler |          |         |    +     |         \n",
//!         "  Maxim Zhiburt  |          |         |          |    +    \n"
//!     )
//! );
//! ```
//!
//! ## Settings
//!
//! You can find more examples of settings and attributes in
//! [README.md](https://github.com/zhiburt/tabled/blob/master/README.md)
//!

use builder::Builder;
use papergrid::Grid;
use std::{fmt, iter::FromIterator};

mod alignment;
mod concat;
mod disable;
mod formating;
mod highlight;
mod indent;
mod object;
mod panel;
mod rotate;
mod width;

pub mod builder;
pub mod display;
pub mod style;

pub use crate::{
    alignment::*, concat::*, disable::*, formating::*, highlight::*, indent::*, object::*,
    panel::*, rotate::*, style::Style, width::*,
};

pub use tabled_derive::Tabled;

// todo: change return type to impl Iterator<Cow<str

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

/// A trait which is responsilbe for configuration of a [Grid].
pub trait TableOption {
    /// The function modifies a [Grid] object.
    fn change(&mut self, grid: &mut Grid);
}

impl<T> TableOption for &mut T
where
    T: TableOption + ?Sized,
{
    fn change(&mut self, grid: &mut Grid) {
        T::change(self, grid)
    }
}

/// A trait for configuring a [Cell] a single cell.
/// Where cell represented by 'row' and 'column' indexes.
pub trait CellOption {
    /// Modification function of a [Cell]
    fn change_cell(&mut self, grid: &mut Grid, row: usize, column: usize);
}

/// Table structure provides an interface for building a table for types that implements [Tabled].
///
/// To build a string representation of a table you must use a [std::fmt::Display].
/// Or simply call `.to_string()` method.
///
/// ## Example
///
/// ### Basic usage
///
/// ```rust,no_run
/// use tabled::Table;
/// let table = Table::new(&["Year", "2021"]);
/// ```
///
/// ### With settings
///
/// ```rust,no_run
/// use tabled::{Table, Style, Alignment, Full, Modify};
/// let data = vec!["Hello", "2021"];
/// let table = Table::new(&data)
///                 .with(Style::PSQL)
///                 .with(Modify::new(Full).with(Alignment::left()));
/// println!("{}", table);
/// ```
pub struct Table {
    grid: Grid,
}

impl Table {
    /// New creates a Table instance.
    pub fn new<T: Tabled>(iter: impl IntoIterator<Item = T>) -> Self {
        let rows = iter.into_iter().map(|t| t.fields());
        Builder::from_iter(rows).header(T::headers()).build()
    }

    /// Returns a table shape (count rows, count columns).
    pub fn shape(&self) -> (usize, usize) {
        (self.grid.count_rows(), self.grid.count_columns())
    }

    /// With is a generic function which applies options to the [Table].
    ///
    /// It applies settings immediately.
    pub fn with<O>(mut self, mut option: O) -> Self
    where
        O: TableOption,
    {
        option.change(&mut self.grid);
        self
    }
}

/// A trait for [IntoIterator] whose Item type is bound to [Tabled].
/// Any type implements [IntoIterator] can call this function directly
///
/// ```rust
/// use tabled::{TableIteratorExt, Style};
/// let strings: &[&str] = &["Hello", "World"];
/// let table = strings.table().with(Style::PSQL);
/// println!("{}", table);
/// ```
pub trait TableIteratorExt {
    /// Returns a [Table] instance from a given type
    fn table(self) -> Table;
}

impl<T, U> TableIteratorExt for U
where
    T: Tabled,
    U: IntoIterator<Item = T>,
{
    fn table(self) -> Table {
        Table::new(self)
    }
}

impl fmt::Display for Table {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.grid)
    }
}

/// Modify structure provide an abstraction, to be able to apply
/// a set of [CellOption]s to the same object.
pub struct Modify<O> {
    obj: O,
    modifiers: Vec<Box<dyn CellOption>>,
}

impl<O> Modify<O>
where
    O: Object,
{
    /// Creates a new [Modify] without any options.
    pub fn new(obj: O) -> Self {
        Self {
            obj,
            modifiers: Vec::new(),
        }
    }

    /// With a generic function which stores a [CellOption].
    ///
    /// The function *doesn't* changes a [Grid]. [Grid] will be changed
    /// only after passing [Modify] object to [Table::with].
    pub fn with<F>(mut self, f: F) -> Self
    where
        F: CellOption + 'static,
    {
        let func = Box::new(f);
        self.modifiers.push(func);
        self
    }
}

impl<O> TableOption for Modify<O>
where
    O: Object,
{
    fn change(&mut self, grid: &mut Grid) {
        let cells = self.obj.cells(grid.count_rows(), grid.count_columns());
        for func in &mut self.modifiers {
            for &(row, column) in &cells {
                func.change_cell(grid, row, column)
            }
        }
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
        self.iter().map(|e| e.to_string()).collect()
    }

    fn headers() -> Vec<String> {
        (0..N).map(|i| format!("{}", i)).collect()
    }
}
