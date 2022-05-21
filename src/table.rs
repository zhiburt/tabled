//! This module contains a main table representation of this crate [Table].
//!
//! There's 1 more table representation which is [ExpandedDisplay].
//!
//! [ExpandedDisplay]: crate::display::ExpandedDisplay

use std::{fmt, iter::FromIterator};

use papergrid::Grid;

use crate::{builder::Builder, object::Object, Tabled};

/// A trait which is responsilbe for configuration of a [Table].
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

/// A trait for configuring a single cell.
/// Where cell represented by 'row' and 'column' indexes.
///
/// A cell can be targeted by [Cell].
///
/// [Cell]: crate::object::Cell
pub trait CellOption {
    /// Modification function of a single cell.
    fn change_cell(&mut self, grid: &mut Grid, row: usize, column: usize);
}

/// Table structure provides an interface for building a table for types that implements [Tabled].
///
/// To build a string representation of a table you must use a [std::fmt::Display].
/// Or simply call `.to_string()` method.
///
/// The default table [Style] is [Style::ascii],
/// with a 1 left and right [Padding].
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
/// use tabled::{Table, Style, Alignment, object::Segment, Modify};
/// let data = vec!["Hello", "2021"];
/// let table = Table::new(&data)
///                 .with(Style::psql())
///                 .with(Modify::new(Segment::all()).with(Alignment::left()));
/// println!("{}", table);
/// ```
///
/// [Padding]: crate::Padding
/// [Style]: crate::Style
/// [Style::ascii]: crate::Style::ascii
#[derive(Clone)]
pub struct Table {
    pub(crate) grid: Grid,
}

impl Table {
    /// New creates a Table instance.
    pub fn new<T: Tabled>(iter: impl IntoIterator<Item = T>) -> Self {
        Self::from_iter(iter)
    }

    /// Creates a builder from a data set given.
    ///
    /// # Example
    ///
    /// ```
    /// use tabled::{Table, Tabled};
    ///
    /// #[derive(Tabled)]
    /// struct User {
    ///     name: &'static str,
    ///     #[tabled(inline("device::"))]
    ///     device: Device,
    /// }
    ///
    /// #[derive(Tabled)]
    /// enum Device {
    ///     PC,
    ///     Mobile
    /// }
    ///
    /// let data = vec![
    ///     User { name: "Vlad", device: Device::Mobile },
    ///     User { name: "Dimitry", device: Device::PC },
    ///     User { name: "John", device: Device::PC },
    /// ];
    ///
    /// let table = Table::builder(data)
    ///     .index()
    ///     .set_index(0)
    ///     .transpose()
    ///     .build()
    ///     .to_string();
    ///
    /// assert_eq!(
    ///     table,
    ///     "+----------------+------+---------+------+\n\
    ///      |      name      | Vlad | Dimitry | John |\n\
    ///      +----------------+------+---------+------+\n\
    ///      |   device::PC   |      |    +    |  +   |\n\
    ///      +----------------+------+---------+------+\n\
    ///      | device::Mobile |  +   |         |      |\n\
    ///      +----------------+------+---------+------+\n"
    /// )
    /// ```
    pub fn builder<I, T>(iter: I) -> Builder
    where
        T: Tabled,
        I: IntoIterator<Item = T>,
    {
        let rows = iter.into_iter().map(|t| t.fields());
        Builder::from_iter(rows).set_columns(T::headers())
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

impl fmt::Display for Table {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.grid)
    }
}

impl<D> FromIterator<D> for Table
where
    D: Tabled,
{
    fn from_iter<T>(iter: T) -> Self
    where
        T: IntoIterator<Item = D>,
    {
        Self::builder(iter).build()
    }
}

/// Modify structure provide an abstraction, to be able to apply
/// a set of [CellOption]s to the same object.
///
/// Be aware that the settings are applied all to a cell at a time.
/// So sometimes you may need to make a several calls of [Modify] in order to achieve the desired affect.
pub struct Modify<O> {
    obj: O,
}

impl<O> Modify<O>
where
    O: Object,
{
    /// Creates a new [Modify] without any options.
    pub fn new(obj: O) -> Self {
        Self { obj }
    }

    /// It's a generic function which stores a [CellOption].
    ///
    /// IMPORTANT:
    ///     The function *doesn't* changes a [Table].
    ///     [Table] will be changed only after passing [Modify] object to [Table::with].
    pub fn with<F>(self, s: F) -> ModifyList<O, F>
    where
        F: CellOption,
    {
        ModifyList {
            obj: self.obj,
            modifiers: s,
        }
    }
}

/// ModifyList is a container of [CellOption]s which are applied to a set [Object].
pub struct ModifyList<O, S> {
    obj: O,
    modifiers: S,
}

impl<O, S> ModifyList<O, S>
where
    O: Object,
    S: CellOption,
{
    /// With a generic function which stores a [CellOption].
    ///
    /// IMPORTANT:
    ///     The function *doesn't* changes a [Table].
    ///     [Table] will be changed only after passing [Modify] object to [Table::with].
    pub fn with<F>(self, s: F) -> ModifyList<O, CellSettingsList<S, F>>
    where
        F: CellOption,
    {
        ModifyList {
            obj: self.obj,
            modifiers: CellSettingsList {
                s1: self.modifiers,
                s2: s,
            },
        }
    }
}

impl<O, S> TableOption for ModifyList<O, S>
where
    O: Object,
    S: CellOption,
{
    fn change(&mut self, grid: &mut Grid) {
        let cells = self.obj.cells(grid.count_rows(), grid.count_columns());
        for (row, column) in cells {
            self.modifiers.change_cell(grid, row, column)
        }
    }
}

/// CellSettingsList is a container of [CellOption]s.
pub struct CellSettingsList<S1, S2> {
    s1: S1,
    s2: S2,
}

impl<S1, S2> CellOption for CellSettingsList<S1, S2>
where
    S1: CellOption,
    S2: CellOption,
{
    fn change_cell(&mut self, grid: &mut Grid, row: usize, column: usize) {
        self.s1.change_cell(grid, row, column);
        self.s2.change_cell(grid, row, column);
    }
}

/// A trait for [IntoIterator] whose Item type is bound to [Tabled].
/// Any type implements [IntoIterator] can call this function directly
///
/// ```rust
/// use tabled::{TableIteratorExt, Style};
/// let strings: &[&str] = &["Hello", "World"];
/// let table = strings.table().with(Style::psql());
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
