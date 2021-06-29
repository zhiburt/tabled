//! An easy to use library for pretty print tables of Rust `struct`s and `enum`s.
//!
//! # Get started
//!
//! The common and probably the best way to begin is to annotate your type with
//! `#[derive(Tabled)]`. You can also implement it on your own as well.
//!
//! ```rust
//!     use tabled::{Tabled, Table};
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
//!     let table = Table::new(languages).to_string();
//!
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
//!     assert_eq!(table, expected);
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
//!     use tabled::{Tabled, Table};
//!     let some_numbers = [1, 2, 3];
//!     let table = Table::new(&some_numbers);
//!     # let expected = "+-----+\n\
//!     #                 | i32 |\n\
//!     #                 +-----+\n\
//!     #                 |  1  |\n\
//!     #                 +-----+\n\
//!     #                 |  2  |\n\
//!     #                 +-----+\n\
//!     #                 |  3  |\n\
//!     #                 +-----+\n";
//!     # assert_eq!(table.to_string(), expected);
//! ```
//!
//! You also can combine structures by means of tuples.
//!
//! ```rust
//!     use tabled::{Tabled, Table, Style};
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
//!     let table = Table::new(data).with(Style::psql()).to_string();
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

use papergrid::{AlignmentHorizontal, Entity, Grid, Settings};
use std::fmt;

mod alignment;
mod disable;
mod formating;
mod indent;
mod object;
pub mod style;
mod width;

pub use crate::{
    alignment::*, disable::*, formating::*, indent::*, object::*, style::Style, width::*,
};
pub use papergrid;
pub use tabled_derive::Tabled;

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

impl<T> TableOption for &T
where
    T: TableOption + ?Sized,
{
    fn change(&self, grid: &mut Grid) {
        T::change(self, grid)
    }
}

/// CellOption is trait for configuring a `Cell` which represented by 'row' and 'column' indexes.
pub trait CellOption {
    /// Modification function of a `Cell`
    fn change_cell(&self, grid: &mut Grid, row: usize, column: usize);
}

/// Table structure provides an interface for building a table for types that implements [`Tabled`].
///
/// # Example
///
/// ## Basic usage
///
/// ```rust,no_run
///     use tabled::Table;
///     let data: Vec<&'static str> = Vec::new();
///     let table = Table::new(data);
///     println!("{}", table);
/// ```
///
/// ## A list of settings
///
/// It may take a list of arguments such as [`Style`](./enum.Style.html),
/// [`Alignment`](./struct.Alignment.html), [`ChangeRing`](./struct.ChangeRing.html)
///
/// ```rust,no_run
///     use tabled::{Table, Style, Alignment, Full, Modify};
///     let data = vec!["Hello", "2021"];
///     let table = Table::new(&data)
///                     .with(Style::psql())
///                     .with(Modify::new(Full).with(Alignment::left()));
///     println!("{}", table);
/// ```
///
/// [`Tabled`]: ./trait.Tabled.html
pub struct Table {
    grid: Grid,
}

impl Table {
    /// New creates a Table instance.
    pub fn new<T: Tabled>(iter: impl IntoIterator<Item = T>) -> Self {
        let grid = build_grid(iter);

        Self { grid }
    }

    /// With is a generic function which applies options to the table.
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

/// Modify structure provide a conviniet way for applying a set of [`CellOption`]s to the same object.
///
/// [`CellOption`]: trait.CellOption.html
pub struct Modify<O> {
    obj: O,
    modifiers: Vec<Box<dyn CellOption>>,
}

impl<O> Modify<O>
where
    O: Object,
{
    /// New creates a instance of Modify structure
    pub fn new(obj: O) -> Self {
        Self {
            obj,
            modifiers: Vec::new(),
        }
    }

    /// With a generic function which stores a [`CellOption`] to apply it later to an [`Object`]
    ///
    /// [`CellOption`]: trait.CellOption.html
    /// [`Object`]: trait.Object.html
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
