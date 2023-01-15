//! This module contains a main table representation of this crate [`Table`].

use std::{borrow::Cow, fmt, iter::FromIterator};

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

use crate::{
    builder::Builder, height::get_table_total_height, object::Entity, width::get_table_total_width,
    Tabled,
};

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
/// use tabled::{Table, Style, Alignment};
///
/// let data = vec!["Hello", "2021"];
/// let mut table = Table::new(&data);
/// table.with(Style::psql()).with(Alignment::left());
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
    has_header: bool,
    widths: Option<Vec<usize>>,
    heights: Option<Vec<usize>>,
}

impl Table<VecRecords<CellInfo<'static>>> {
    /// New creates a Table instance.
    ///
    /// If you use a reference iterator you'd better use [`FromIterator`] instead.
    /// As it has a different lifetime constraints and make less copies therefore.
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

        let mut b = Builder::custom(VecRecords::from(records));
        b.with_header();
        b.build()
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
    /// let table = builder.build()
    ///     .with(Segment::new(1.., 1..).modify().with(Alignment::center()))
    ///     .to_string();
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
    pub fn with<O>(&mut self, mut option: O) -> &mut Self
    where
        O: TableOption<R>,
    {
        option.change(self);
        self
    }

    /// A verification that first row is actually a header.
    ///
    /// It's `true` when [`Table::new`] and [`Table::builder`] is used.
    /// In many other cases it's `false`.
    pub fn has_header(&self) -> bool {
        self.has_header
    }

    pub(crate) fn cache_width(&mut self, widths: Vec<usize>) {
        self.widths = Some(widths);
    }

    pub(crate) fn destroy_width_cache(&mut self) {
        self.widths = None;
    }

    pub(crate) fn cache_height(&mut self, widths: Vec<usize>) {
        self.heights = Some(widths);
    }

    pub(crate) fn destroy_height_cache(&mut self) {
        self.heights = None;
    }

    pub(crate) fn set_header_flag(&mut self, has_header: bool) {
        self.has_header = has_header;
    }
}

impl<R> Table<R>
where
    R: Records,
{
    /// Returns a table shape (count rows, count columns).
    pub fn shape(&self) -> (usize, usize) {
        let records = self.get_records();
        (records.count_rows(), records.count_columns())
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

    /// Returns total widths of a table, including margin and vertical lines.
    pub fn total_width(&self) -> usize {
        let ctrl = self.get_width_ctrl();
        get_table_total_width(&self.records, &self.cfg, &ctrl)
    }

    /// Returns total widths of a table, including margin and horizontal lines.
    pub fn total_height(&self) -> usize {
        let ctrl = self.get_height_ctrl();
        get_table_total_height(&self.records, &self.cfg, &ctrl)
    }

    fn get_width_ctrl(&self) -> CachedEstimator<'_, WidthEstimator> {
        match &self.widths {
            Some(widths) => CachedEstimator::Cached(widths),
            None => {
                let mut w = WidthEstimator::default();
                w.estimate(&self.records, &self.cfg);
                CachedEstimator::Ctrl(w)
            }
        }
    }

    fn get_height_ctrl(&self) -> CachedEstimator<'_, HeightEstimator> {
        match &self.heights {
            Some(heights) => CachedEstimator::Cached(heights),
            None => {
                let mut w = HeightEstimator::default();
                w.estimate(&self.records, &self.cfg);
                CachedEstimator::Ctrl(w)
            }
        }
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

impl<R> fmt::Display for Table<R>
where
    R: Records,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut cfg = Cow::Borrowed(&self.cfg);
        set_align_table(f, &mut cfg);
        set_width_table(f, &mut cfg, self);

        let width = self.get_width_ctrl();
        let height = self.get_height_ctrl();

        let grid = Grid::new(&self.records, &cfg, &width, &height);

        write!(f, "{grid}")
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
            has_header: false,
            widths: None,
            heights: None,
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
                CellMut::set(cell, text, &ctrl);
            }

            records.push(list);
        }

        let mut b = Builder::custom(VecRecords::from(records));
        b.with_header();
        b.build()
    }
}

#[derive(Debug)]
enum CachedEstimator<'a, E> {
    Cached(&'a [usize]),
    Ctrl(E),
}

impl<R, E> Estimate<R> for CachedEstimator<'_, E>
where
    R: Records,
    E: Estimate<R>,
{
    fn estimate(&mut self, _: R, _: &GridConfig) {}

    fn get(&self, i: usize) -> Option<usize> {
        match self {
            Self::Cached(list) => list.get(i).copied(),
            Self::Ctrl(e) => Estimate::<R>::get(e, i),
        }
    }

    fn total(&self) -> usize {
        match self {
            Self::Cached(list) => list.iter().sum(),
            Self::Ctrl(e) => Estimate::<R>::total(e),
        }
    }
}

fn set_align_table(f: &fmt::Formatter<'_>, cfg: &mut Cow<'_, GridConfig>) {
    if let Some(alignment) = f.align() {
        let alignment = convert_fmt_alignment(alignment);

        match cfg {
            Cow::Borrowed(c) => {
                let mut new = c.clone();
                new.set_alignment_horizontal(Entity::Global, alignment);
                *cfg = Cow::Owned(new);
            }
            Cow::Owned(cfg) => {
                cfg.set_alignment_horizontal(Entity::Global, alignment);
            }
        }
    }
}

fn set_width_table<R>(f: &fmt::Formatter<'_>, cfg: &mut Cow<'_, GridConfig>, table: &Table<R>)
where
    R: Records,
{
    if let Some(width) = f.width() {
        let total_width = table.total_width();
        if total_width >= width {
            return;
        }

        let mut fill = f.fill();
        if fill == char::default() {
            fill = ' ';
        }

        let available = width - total_width;
        let alignment = f.align().unwrap_or(fmt::Alignment::Left);
        let (left, right) = table_padding(alignment, available);

        let mut margin = *cfg.get_margin();
        margin.left.size += left;
        margin.right.size += right;

        if (margin.left.size > 0 && margin.left.fill == char::default()) || fill != char::default()
        {
            margin.left.fill = fill;
        }

        if (margin.right.size > 0 && margin.right.fill == char::default())
            || fill != char::default()
        {
            margin.right.fill = fill;
        }

        match cfg {
            Cow::Borrowed(c) => {
                let mut new = c.clone();
                new.set_margin(margin);
                *cfg = Cow::Owned(new);
            }
            Cow::Owned(cfg) => cfg.set_margin(margin),
        }
    }
}

fn convert_fmt_alignment(alignment: fmt::Alignment) -> papergrid::AlignmentHorizontal {
    match alignment {
        fmt::Alignment::Left => papergrid::AlignmentHorizontal::Left,
        fmt::Alignment::Right => papergrid::AlignmentHorizontal::Right,
        fmt::Alignment::Center => papergrid::AlignmentHorizontal::Center,
    }
}

fn table_padding(alignment: fmt::Alignment, available: usize) -> (usize, usize) {
    match alignment {
        fmt::Alignment::Left => (available, 0),
        fmt::Alignment::Right => (0, available),
        fmt::Alignment::Center => {
            let left = available / 2;
            let right = available - left;
            (left, right)
        }
    }
}
