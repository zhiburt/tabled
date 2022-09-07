//! This module contains a main table representation of this crate [`Table`].

use std::{fmt, iter::FromIterator};

use papergrid::{
    height::HeightEstimator,
    records::{
        cell_info::CellInfo,
        vec_records::{CellMut, VecRecords},
        Records, RecordsMut,
    },
    width::{CfgWidthFunction, WidthEstimator},
    Estimate, Grid, GridConfig,
};

use crate::{builder::Builder, object::Entity, Tabled};

/// A trait which is responsilbe for configuration of a [`Table`].
pub trait TableOption<R> {
    /// The function modifies a [`Grid`] object.
    fn change(&mut self, table: &mut Table<R>);
}

impl<T, R> TableOption<R> for &mut T
where
    T: TableOption<R> + ?Sized,
{
    fn change(&mut self, table: &mut Table<R>) {
        T::change(self, table);
    }
}

/// A trait for configuring a single cell.
/// Where cell represented by 'row' and 'column' indexes.
///
/// A cell can be targeted by [`Cell`].
///
/// [`Cell`]: crate::object::Cell
pub trait CellOption<R> {
    /// Modification function of a single cell.
    fn change_cell(&mut self, table: &mut Table<R>, entity: Entity);
}

/// The structure provides an interface for building a table for types that implements [`Tabled`].
///
/// To build a string representation of a table you must use a [`std::fmt::Display`].
/// Or simply call `.to_string()` method.
///
/// The default table [`Style`] is [`Style::ascii`],
/// with a 1 left and right [`Padding`].
///
/// ## Example
///
/// ### Basic usage
///
/// ```rust,no_run
/// use tabled::Table;
///
/// let table = Table::new(&["Year", "2021"]);
/// ```
///
/// ### With settings
///
/// ```rust,no_run
/// use tabled::{Table, Style, Alignment, object::Segment, Modify};
///
/// let data = vec!["Hello", "2021"];
/// let table = Table::new(&data)
///                 .with(Style::psql())
///                 .with(Modify::new(Segment::all()).with(Alignment::left()));
///
/// println!("{}", table);
/// ```
///
/// [`Padding`]: crate::Padding
/// [`Style`]: crate::Style
/// [`Style::ascii`]: crate::Style::ascii
#[derive(Debug, Clone)]
pub struct Table<R = VecRecords<CellInfo<'static>>> {
    records: R,
    cfg: GridConfig,
    widths: Option<Vec<usize>>,
}

impl Table<VecRecords<CellInfo<'static>>> {
    /// New creates a Table instance.
    pub fn new<I, T>(iter: I) -> Self
    where
        I: IntoIterator<Item = T>,
        T: Tabled,
    {
        let ctrl = CfgWidthFunction::new(4);

        let mut header = vec![CellInfo::default(); T::LENGTH];
        for (text, cell) in T::headers().into_iter().zip(header.iter_mut()) {
            CellMut::set(cell, text, &ctrl);
        }

        let mut records = vec![header];
        for row in iter.into_iter() {
            let mut list = vec![CellInfo::default(); T::LENGTH];
            for (text, cell) in row.fields().into_iter().zip(list.iter_mut()) {
                CellMut::set(cell, text.into_owned(), &ctrl);
            }

            records.push(list);
        }

        Builder::custom(VecRecords::from(records)).build()
    }
}

impl Table<()> {
    /// Creates a builder from a data set given.
    ///
    /// # Example
    ///
    ///
    #[cfg_attr(feature = "derive", doc = "```")]
    #[cfg_attr(not(feature = "derive"), doc = "```ignore")]
    /// use tabled::{Table, Tabled, object::Segment, ModifyObject, Alignment};
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
    /// let mut builder = Table::builder(data).index();
    /// builder.set_index(0);
    /// builder.transpose();
    ///
    /// let table = builder.build().with(Segment::new(1.., 1..).modify().with(Alignment::center())).to_string();
    ///
    /// assert_eq!(
    ///     table,
    ///     "+----------------+------+---------+------+\n\
    ///      | name           | Vlad | Dimitry | John |\n\
    ///      +----------------+------+---------+------+\n\
    ///      | device::PC     |      |    +    |  +   |\n\
    ///      +----------------+------+---------+------+\n\
    ///      | device::Mobile |  +   |         |      |\n\
    ///      +----------------+------+---------+------+"
    /// )
    /// ```
    pub fn builder<I, T>(iter: I) -> Builder<'static>
    where
        T: Tabled,
        I: IntoIterator<Item = T>,
    {
        let ctrl = CfgWidthFunction::new(4);
        let mut records = Vec::new();
        for row in iter {
            let mut list = vec![CellInfo::default(); T::LENGTH];
            for (text, cell) in row.fields().into_iter().zip(list.iter_mut()) {
                CellMut::set(cell, text.into_owned(), &ctrl);
            }

            records.push(list);
        }

        let mut b = Builder::from(records);
        b.hint_column_size(T::LENGTH);
        b.set_columns(T::headers());

        b
    }
}

impl<R> Table<R> {
    /// Get a reference to the table's cfg.
    pub fn get_config(&self) -> &GridConfig {
        &self.cfg
    }

    /// Get a reference to the table's cfg.
    pub fn get_config_mut(&mut self) -> &mut GridConfig {
        &mut self.cfg
    }

    /// Get a reference to the table's records.
    pub fn get_records(&self) -> &R {
        &self.records
    }

    /// Get a reference to the table's records.
    pub fn get_records_mut(&mut self) -> &mut R {
        &mut self.records
    }

    /// With is a generic function which applies options to the [`Table`].
    ///
    /// It applies settings immediately.
    pub fn with<O>(mut self, mut option: O) -> Self
    where
        O: TableOption<R>,
    {
        option.change(&mut self);
        self
    }

    pub(crate) fn cache_width(&mut self, widths: Vec<usize>) {
        self.widths = Some(widths);
    }

    pub(crate) fn destroy_width_cache(&mut self) {
        self.widths = None;
    }
}

impl<R> Table<R>
where
    R: Records,
{
    /// Returns a table shape (count rows, count columns).
    pub fn shape(&self) -> (usize, usize) {
        (
            self.get_records().count_rows(),
            self.get_records().count_columns(),
        )
    }

    /// Returns an amount of rows in the table.
    pub fn count_rows(&self) -> usize {
        self.get_records().count_rows()
    }

    /// Returns an amount of columns in the table.
    pub fn count_columns(&self) -> usize {
        self.get_records().count_columns()
    }

    /// Returns a table shape (count rows, count columns).
    pub fn is_empty(&self) -> bool {
        let (count_rows, count_cols) = self.shape();
        count_rows == 0 || count_cols == 0
    }
}

impl<R> Table<R>
where
    R: Records + RecordsMut<String>,
{
    pub(crate) fn update_records(&mut self) {
        let ctrl = CfgWidthFunction::from_cfg(self.get_config());

        for row in 0..self.get_records().count_rows() {
            for col in 0..self.get_records().count_columns() {
                let records = self.get_records_mut();
                records.update((row, col), &ctrl);
            }
        }
    }
}

impl<R> From<R> for Table<R>
where
    R: Records,
{
    fn from(records: R) -> Self {
        Self {
            records,
            cfg: GridConfig::default(),
            widths: None,
        }
    }
}

impl<'a, T> FromIterator<&'a T> for Table<VecRecords<CellInfo<'a>>>
where
    T: Tabled + 'a,
{
    fn from_iter<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = &'a T>,
    {
        let ctrl = CfgWidthFunction::new(4);

        let mut header = vec![CellInfo::default(); T::LENGTH];
        for (text, cell) in T::headers().into_iter().zip(header.iter_mut()) {
            CellMut::set(cell, text, &ctrl);
        }

        let mut records = vec![header];
        for row in iter.into_iter() {
            let mut list = vec![CellInfo::default(); T::LENGTH];
            for (text, cell) in row.fields().into_iter().zip(list.iter_mut()) {
                CellMut::set(cell, text.into_owned(), &ctrl);
            }

            records.push(list);
        }

        Builder::custom(VecRecords::from(records)).build()
    }
}

impl<R> fmt::Display for Table<R>
where
    R: Records,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut height = HeightEstimator::default();
        height.estimate(&self.records, &self.cfg);

        // check if we have cached widths values.
        let width = {
            match &self.widths {
                Some(widths) => WidthCtrl::Cached(widths),
                None => {
                    let mut w = WidthEstimator::default();
                    w.estimate(&self.records, &self.cfg);
                    WidthCtrl::Ctrl(w)
                }
            }
        };

        let grid = Grid::new(&self.records, &self.cfg, &width, &height);

        write!(f, "{}", grid)
    }
}

/// A trait for [`IntoIterator`] whose Item type is bound to [`Tabled`].
/// Any type implements [`IntoIterator`] can call this function directly
///
/// ```rust
/// use tabled::{TableIteratorExt, Style};
///
/// let strings: &[&str] = &["Hello", "World"];
///
/// let table = strings.table().with(Style::psql());
///
/// println!("{}", table);
/// ```
pub trait TableIteratorExt {
    /// A underline [`Records`],
    type Records;

    /// Returns a [`Table`] instance from a given type
    fn table(self) -> Table<Self::Records>;
}

impl<I, T> TableIteratorExt for I
where
    I: IntoIterator<Item = T>,
    T: Tabled,
{
    type Records = VecRecords<CellInfo<'static>>;

    fn table(self) -> Table<Self::Records> {
        Table::new(self)
    }
}

#[derive(Debug)]
enum WidthCtrl<'a> {
    Cached(&'a [usize]),
    Ctrl(WidthEstimator),
}

impl<R> Estimate<R> for WidthCtrl<'_>
where
    R: Records,
{
    fn estimate(&mut self, _: R, _: &GridConfig) {}

    fn get(&self, i: usize) -> Option<usize> {
        match self {
            WidthCtrl::Cached(list) => list.get(i).copied(),
            WidthCtrl::Ctrl(e) => Estimate::<R>::get(e, i),
        }
    }

    fn total(&self) -> usize {
        match self {
            WidthCtrl::Cached(list) => list.iter().sum(),
            WidthCtrl::Ctrl(e) => Estimate::<R>::total(e),
        }
    }
}
