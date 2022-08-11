//! This module contains a main table representation of this crate [`Table`].

use std::{fmt, iter::FromIterator};

use papergrid::{
    height::HeightEstimator,
    records::{records_info::RecordsInfo, Cell, Records, RecordsMut, Text},
    width::{CfgWidthFunction, WidthEstimator},
    Estimate, Grid, GridConfig,
};

#[cfg(feature = "color")]
use papergrid::Color;

use crate::{builder::Builder, object::Entity, Tabled};

// todo: rename TableOption/CellOption

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
pub struct Table<R = RecordsInfo<'static>> {
    records: R,
    cfg: GridConfig,
    widths: Option<Vec<usize>>,
}

impl<'a> Table<RecordsInfo<'a>> {
    /// New creates a Table instance.
    pub fn new<I, T>(iter: I) -> Self
    where
        I: IntoIterator<Item = T> + 'a,
        T: Tabled,
    {
        Self::builder(iter).build()
    }

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
    pub fn builder<I, T>(iter: I) -> Builder
    where
        T: Tabled,
        I: IntoIterator<Item = T>,
    {
        let mut b = Builder::new();
        b.hint_column_size(T::LENGTH);
        b.set_columns(T::headers());

        for c in iter {
            let fields = c.fields();
            b.add_record(fields);
        }

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
}

impl<R> Table<R>
where
    for<'a> &'a R: Records,
{
    /// Returns a table shape (count rows, count columns).
    pub fn shape(&self) -> (usize, usize) {
        let (count_rows, count_cols) = self.get_records().size();
        (count_rows, count_cols)
    }

    /// Returns a table shape (count rows, count columns).
    pub fn is_empty(&self) -> bool {
        let (count_rows, count_cols) = self.shape();
        count_rows == 0 || count_cols == 0
    }
}

impl<R> Table<R> {
    pub(crate) fn new_raw(records: R, cfg: GridConfig) -> Self {
        Self {
            records,
            cfg,
            widths: None,
        }
    }
}

impl<R> Table<R>
where
    R: RecordsMut,
    for<'a> &'a R: Records,
{
    pub(crate) fn update_records(&mut self) {
        let ctrl = CfgWidthFunction::from_cfg(self.get_config());

        let (count_rows, count_cols) = self.get_records().size();
        for row in 0..count_rows {
            for col in 0..count_cols {
                let records = self.get_records_mut();
                records.update((row, col), &ctrl);
            }
        }
    }

    pub(crate) fn cache_width(&mut self, widths: Vec<usize>) {
        self.widths = Some(widths);
    }

    pub(crate) fn destroy_width_cache(&mut self) {
        self.widths = None;
    }
}

#[cfg(feature = "color")]
trait RecordsCell: Cell + Color {}

#[cfg(feature = "color")]
impl<C> RecordsCell for C where C: Cell + Color {}

#[cfg(not(feature = "color"))]
trait RecordsCell: Cell {}

#[cfg(not(feature = "color"))]
impl<C> RecordsCell for C where C: Cell {}

impl<R> fmt::Display for Table<R>
where
    for<'a> &'a R: Records,
    for<'a> <&'a R as Records>::Cell: RecordsCell,
    for<'a> <<&'a R as Records>::Cell as Cell>::Text: Text + Default,
    for<'a> <<&'a R as Records>::Cell as Cell>::Lines: Iterator,
    for<'a> <<<&'a R as Records>::Cell as Cell>::Lines as Iterator>::Item: Text + Default,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // check if we have cached widths values.
        let width = match &self.widths {
            Some(widths) => WidthEstimator::from(widths.clone()),
            None => {
                let mut w = WidthEstimator::default();
                w.estimate(&self.records, &self.cfg);
                w
            }
        };

        let mut height = HeightEstimator::default();
        height.estimate(&self.records, &self.cfg);

        let grid = Grid::new(&self.records, &self.cfg, width, height);

        write!(f, "{}", grid)
    }
}

impl<'a, D> FromIterator<D> for Table<RecordsInfo<'a>>
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
pub trait TableIteratorExt<'a> {
    /// Returns a [`Table`] instance from a given type
    fn table(self) -> Table<RecordsInfo<'a>>;
}

impl<'a, T, U> TableIteratorExt<'a> for U
where
    T: Tabled,
    U: IntoIterator<Item = T> + 'a,
{
    fn table(self) -> Table<RecordsInfo<'a>> {
        Table::new(self)
    }
}
