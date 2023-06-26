//! This module contains a main table representation [`Table`].

use core::ops::DerefMut;
use std::{borrow::Cow, fmt, iter::FromIterator};

use crate::{
    builder::Builder,
    grid::{
        colors::NoColors,
        config::{
            AlignmentHorizontal, ColorMap, ColoredConfig, CompactConfig, Entity, Formatting,
            Indent, Sides, SpannedConfig,
        },
        dimension::{CompleteDimensionVecRecords, Dimension, Estimate, PeekableDimension},
        records::{
            vec_records::{CellInfo, VecRecords},
            ExactRecords, Records,
        },
        PeekableGrid,
    },
    settings::{Style, TableOption},
    Tabled,
};

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
/// use tabled::{Table, settings::{Style, Alignment}};
///
/// let data = vec!["Hello", "2021"];
/// let mut table = Table::new(&data);
/// table.with(Style::psql()).with(Alignment::left());
///
/// println!("{}", table);
/// ```
///
/// [`Padding`]: crate::settings::Padding
/// [`Style`]: crate::settings::Style
/// [`Style::ascii`]: crate::settings::Style::ascii
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Table {
    records: VecRecords<CellInfo<String>>,
    config: ColoredConfig,
    dimension: CompleteDimensionVecRecords<'static>,
}

impl Table {
    /// New creates a Table instance.
    ///
    /// If you use a reference iterator you'd better use [`FromIterator`] instead.
    /// As it has a different lifetime constraints and make less copies therefore.
    pub fn new<I, T>(iter: I) -> Self
    where
        I: IntoIterator<Item = T>,
        T: Tabled,
    {
        let mut header = Vec::with_capacity(T::LENGTH);
        for text in T::headers() {
            let text = text.into_owned();
            let cell = CellInfo::new(text);
            header.push(cell);
        }

        let mut records = vec![header];
        for row in iter.into_iter() {
            let mut list = Vec::with_capacity(T::LENGTH);
            for text in row.fields().into_iter() {
                let text = text.into_owned();
                let cell = CellInfo::new(text);

                list.push(cell);
            }

            records.push(list);
        }

        let records = VecRecords::new(records);

        Self {
            records,
            config: ColoredConfig::new(configure_grid()),
            dimension: CompleteDimensionVecRecords::default(),
        }
    }

    /// Creates a builder from a data set given.
    ///
    /// # Example
    ///
    ///
    #[cfg_attr(feature = "derive", doc = "```")]
    #[cfg_attr(not(feature = "derive"), doc = "```ignore")]
    /// use tabled::{
    ///     Table, Tabled,
    ///     settings::{object::Segment, Modify, Alignment}
    /// };
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
    /// let mut table = Table::builder(data)
    ///     .index()
    ///     .column(0)
    ///     .transpose()
    ///     .build()
    ///     .with(Modify::new(Segment::new(1.., 1..)).with(Alignment::center()))
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
    pub fn builder<I, T>(iter: I) -> Builder
    where
        T: Tabled,
        I: IntoIterator<Item = T>,
    {
        let mut records = Vec::new();
        for row in iter {
            let mut list = Vec::with_capacity(T::LENGTH);
            for text in row.fields().into_iter() {
                list.push(text.into_owned());
            }

            records.push(list);
        }

        let mut b = Builder::from(records);
        let _ = b.set_header(T::headers()).hint_column_size(T::LENGTH);

        b
    }

    /// With is a generic function which applies options to the [`Table`].
    ///
    /// It applies settings immediately.
    pub fn with<O>(&mut self, option: O) -> &mut Self
    where
        O: TableOption<
            VecRecords<CellInfo<String>>,
            CompleteDimensionVecRecords<'static>,
            ColoredConfig,
        >,
    {
        self.dimension.clear_width();
        self.dimension.clear_height();

        option.change(&mut self.records, &mut self.config, &mut self.dimension);

        self
    }

    /// Returns a table shape (count rows, count columns).
    pub fn shape(&self) -> (usize, usize) {
        (self.count_rows(), self.count_columns())
    }

    /// Returns an amount of rows in the table.
    pub fn count_rows(&self) -> usize {
        self.records.count_rows()
    }

    /// Returns an amount of columns in the table.
    pub fn count_columns(&self) -> usize {
        self.records.count_columns()
    }

    /// Returns a table shape (count rows, count columns).
    pub fn is_empty(&self) -> bool {
        let (count_rows, count_cols) = self.shape();
        count_rows == 0 || count_cols == 0
    }

    /// Returns total widths of a table, including margin and horizontal lines.
    pub fn total_height(&self) -> usize {
        let mut dims = CompleteDimensionVecRecords::from_origin(&self.dimension);
        dims.estimate(&self.records, self.config.as_ref());

        let total = (0..self.count_rows())
            .map(|row| dims.get_height(row))
            .sum::<usize>();
        let counth = self.config.count_horizontal(self.count_rows());

        let margin = self.config.get_margin();

        total + counth + margin.top.size + margin.bottom.size
    }

    /// Returns total widths of a table, including margin and vertical lines.
    pub fn total_width(&self) -> usize {
        let mut dims = CompleteDimensionVecRecords::from_origin(&self.dimension);
        dims.estimate(&self.records, self.config.as_ref());

        let total = (0..self.count_columns())
            .map(|col| dims.get_width(col))
            .sum::<usize>();
        let countv = self.config.count_vertical(self.count_columns());

        let margin = self.config.get_margin();

        total + countv + margin.left.size + margin.right.size
    }

    /// Returns a table config.
    pub fn get_config(&self) -> &ColoredConfig {
        &self.config
    }

    /// Returns a table config.
    pub fn get_config_mut(&mut self) -> &mut ColoredConfig {
        &mut self.config
    }

    /// Returns a used records.
    pub fn get_records(&self) -> &VecRecords<CellInfo<String>> {
        &self.records
    }

    /// Returns a used records.
    pub fn get_records_mut(&mut self) -> &mut VecRecords<CellInfo<String>> {
        &mut self.records
    }
}

impl Default for Table {
    fn default() -> Self {
        Self {
            records: VecRecords::default(),
            config: ColoredConfig::new(configure_grid()),
            dimension: CompleteDimensionVecRecords::default(),
        }
    }
}

impl fmt::Display for Table {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.is_empty() {
            return Ok(());
        }

        let config = use_format_configuration(f, self);
        let colors = self.config.get_colors();

        if !self.dimension.is_empty() {
            let mut dims = self.dimension.clone();
            dims.estimate(&self.records, config.as_ref());

            print_grid(f, &self.records, &config, &dims, colors)
        } else {
            let mut dims = PeekableDimension::default();
            dims.estimate(&self.records, &config);

            print_grid(f, &self.records, &config, &dims, colors)
        }
    }
}

impl<T, V> FromIterator<T> for Table
where
    T: IntoIterator<Item = V>,
    V: Into<String>,
{
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        Builder::from_iter(iter.into_iter().map(|i| i.into_iter().map(|s| s.into()))).build()
    }
}

impl From<Builder> for Table {
    fn from(builder: Builder) -> Self {
        let data: Vec<Vec<CellInfo<String>>> = builder.into();
        let records = VecRecords::new(data);

        Self {
            records,
            config: ColoredConfig::new(configure_grid()),
            dimension: CompleteDimensionVecRecords::default(),
        }
    }
}

impl From<Table> for Builder {
    fn from(val: Table) -> Self {
        let count_columns = val.count_columns();
        let data: Vec<Vec<CellInfo<String>>> = val.records.into();
        let mut builder = Builder::from(data);
        let _ = builder.hint_column_size(count_columns);
        builder
    }
}

impl<R, D> TableOption<R, D, ColoredConfig> for CompactConfig {
    fn change(self, _: &mut R, cfg: &mut ColoredConfig, _: &mut D) {
        *cfg.deref_mut() = self.into();
    }
}

impl<R, D> TableOption<R, D, ColoredConfig> for ColoredConfig {
    fn change(self, _: &mut R, cfg: &mut ColoredConfig, _: &mut D) {
        *cfg = self;
    }
}

impl<R, D> TableOption<R, D, ColoredConfig> for SpannedConfig {
    fn change(self, _: &mut R, cfg: &mut ColoredConfig, _: &mut D) {
        *cfg.deref_mut() = self;
    }
}

impl<R, D> TableOption<R, D, ColoredConfig> for &SpannedConfig {
    fn change(self, _: &mut R, cfg: &mut ColoredConfig, _: &mut D) {
        *cfg.deref_mut() = self.clone();
    }
}

fn convert_fmt_alignment(alignment: fmt::Alignment) -> AlignmentHorizontal {
    match alignment {
        fmt::Alignment::Left => AlignmentHorizontal::Left,
        fmt::Alignment::Right => AlignmentHorizontal::Right,
        fmt::Alignment::Center => AlignmentHorizontal::Center,
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

fn configure_grid() -> SpannedConfig {
    let mut cfg = SpannedConfig::default();
    cfg.set_padding(
        Entity::Global,
        Sides::new(
            Indent::spaced(1),
            Indent::spaced(1),
            Indent::default(),
            Indent::default(),
        ),
    );
    cfg.set_alignment_horizontal(Entity::Global, AlignmentHorizontal::Left);
    cfg.set_formatting(Entity::Global, Formatting::new(false, false, false));
    cfg.set_borders(*Style::ascii().get_borders());

    cfg
}

fn use_format_configuration<'a>(
    f: &mut fmt::Formatter<'_>,
    table: &'a Table,
) -> Cow<'a, SpannedConfig> {
    if f.align().is_some() || f.width().is_some() {
        let mut cfg = table.config.as_ref().clone();

        set_align_table(f, &mut cfg);
        set_width_table(f, &mut cfg, table);

        Cow::Owned(cfg)
    } else {
        Cow::Borrowed(table.config.as_ref())
    }
}

fn set_align_table(f: &fmt::Formatter<'_>, cfg: &mut SpannedConfig) {
    if let Some(alignment) = f.align() {
        let alignment = convert_fmt_alignment(alignment);
        cfg.set_alignment_horizontal(Entity::Global, alignment);
    }
}

fn set_width_table(f: &fmt::Formatter<'_>, cfg: &mut SpannedConfig, table: &Table) {
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

        let mut margin = cfg.get_margin();
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

        cfg.set_margin(margin);
    }
}

fn print_grid<F: fmt::Write, D: Dimension>(
    f: &mut F,
    records: &VecRecords<CellInfo<String>>,
    cfg: &SpannedConfig,
    dims: D,
    colors: &ColorMap,
) -> fmt::Result {
    if !colors.is_empty() {
        PeekableGrid::new(records, cfg, &dims, colors).build(f)
    } else {
        PeekableGrid::new(records, cfg, &dims, NoColors).build(f)
    }
}
