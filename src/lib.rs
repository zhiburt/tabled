//! An easy to use library for pretty print tables of Rust `struct`s and `enum`s.
//!
//! The library is based on a [Tabled] trait which is used to actually build tables.
//! It also provides an variate of dynamic settings for customization of a [Table].
//!
//! [Table] can be build from vast majority of Rust's standart types.
//!
//! ## Examples
//!
//! If you wan't to build a table for your data.
//! Most likely a starting point is to anotate your type with `#[derive(Tabled)]`.
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

use papergrid::{AlignmentHorizontal, Entity, Grid, Settings};
use std::fmt;

mod alignment;
mod disable;
mod formating;
mod indent;
mod object;
mod panel;
pub mod style;
mod width;

pub use crate::{
    alignment::*, disable::*, formating::*, indent::*, object::*, panel::*, style::Style, width::*,
};
pub use papergrid;
pub use tabled_derive::Tabled;

/// Tabled a trait responsible for providing a header fields and a row fields.
///
/// It's urgent that `header` len is equal to `fields` len.
///
/// ```text
/// Self::headers().len() == self.fields().len()
/// ```
pub trait Tabled {
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
    fn change(&self, grid: &mut Grid);
}

impl<T> TableOption for &T
where
    T: TableOption + ?Sized,
{
    fn change(&self, grid: &mut Grid) {
        T::change(self, grid)
    }
}

/// A trait for configuring a [Cell] a single cell.
/// Where cell represented by 'row' and 'column' indexes.
pub trait CellOption {
    /// Modification function of a [Cell]
    fn change_cell(&self, grid: &mut Grid, row: usize, column: usize);
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
/// let table = Table::new(["Year", "2021"]);
/// ```
///
/// ### With settings
///
/// ```rust,no_run
/// use tabled::{Table, Style, Alignment, Full, Modify};
/// let data = vec!["Hello", "2021"];
/// let table = Table::new(&data)
///                 .with(Style::psql())
///                 .with(Modify::new(Full).with(Alignment::left()));
/// println!("{}", table);
/// ```
pub struct Table {
    grid: Grid,
}

impl Table {
    /// New creates a Table instance.
    pub fn new<T: Tabled>(iter: impl IntoIterator<Item = T>) -> Self {
        let grid = build_grid(iter);

        Self { grid }
    }

    /// With is a generic function which applies options to the [Table].
    ///
    /// It applies settings immediately.
    pub fn with<O>(mut self, option: O) -> Self
    where
        O: TableOption,
    {
        option.change(&mut self.grid);
        self
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
    fn change(&self, grid: &mut Grid) {
        let cells = self.obj.cells(grid.count_rows(), grid.count_columns());
        for func in &self.modifiers {
            for &(row, column) in &cells {
                func.change_cell(grid, row, column)
            }
        }
    }
}

/// Building [Grid] from a data.
/// You must prefer [Table] over this function.
fn build_grid<T: Tabled>(iter: impl IntoIterator<Item = T>) -> Grid {
    let headers = T::headers();
    let obj: Vec<Vec<String>> = iter.into_iter().map(|t| t.fields()).collect();

    let mut grid = Grid::new(obj.len() + 1, headers.len());

    // it's crusial to set a global setting rather than a setting for an each cell
    // as it will be hard to override that since how Grid::style method works
    grid.set(
        Entity::Global,
        Settings::new()
            .indent(1, 1, 0, 0)
            .alignment(AlignmentHorizontal::Center),
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
tuple_table! { A B C D }
tuple_table! { A B C D E }
tuple_table! { A B C D E F }

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
    fn fields(&self) -> Vec<String> {
        (&self).iter().map(|e| e.to_string()).collect()
    }

    fn headers() -> Vec<String> {
        (0..N).map(|i| format!("{}", i)).collect()
    }
}
