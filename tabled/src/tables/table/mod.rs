//! This module contains a main table representation [`Table`].

mod dimension;

use core::ops::{Deref, DerefMut};
use std::{borrow::Cow, collections::HashMap, fmt, iter::FromIterator};

use crate::{
    builder::Builder,
    grid::config::AlignmentHorizontal,
    grid::{
        color::AnsiColor,
        config::{Entity, Indent, Position, Sides},
        dimension::{Dimension, Estimate},
        spanned::{
            config::{Formatting, GridConfig},
            Grid,
        },
    },
    records::{ExactRecords, Records, VecRecords},
    settings::{style::Style, TableOption},
    Tabled,
};

pub use dimension::TableDimension;

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
/// [`Padding`]: crate::settings::padding::Padding
/// [`Style`]: crate::settings::style::Style
/// [`Style::ascii`]: crate::settings::style::Style::ascii
#[derive(Debug, Clone)]
pub struct Table {
    records: VecRecords<String>,
    config: ColoredConfig,
    dimension: TableDimension<'static>,
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
            header.push(text.into_owned());
        }

        let mut records = vec![header];
        for row in iter.into_iter() {
            let mut list = Vec::with_capacity(T::LENGTH);
            for text in row.fields().into_iter() {
                list.push(text.into_owned());
            }

            records.push(list);
        }

        let records = VecRecords::new(records);

        Self {
            records,
            config: ColoredConfig::new(configure_grid(), HashMap::default()),
            dimension: TableDimension::default(),
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

        let mut b = Builder::from(records).set_header(T::headers());
        b.hint_column_size(T::LENGTH);

        b
    }

    /// With is a generic function which applies options to the [`Table`].
    ///
    /// It applies settings immediately.
    pub fn with<O: TableOption<VecRecords<String>, TableDimension<'static>, ColoredConfig>>(
        &mut self,
        option: O,
    ) -> &mut Self {
        self.dimension.clear_width();
        self.dimension.clear_height();

        let mut option = option;
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
        let mut dims = TableDimension::from_origin(&self.dimension);
        dims.estimate(&self.records, &self.config);

        let total = (0..self.count_rows())
            .map(|row| dims.get_height(row))
            .sum::<usize>();
        let counth = self.config.count_horizontal(self.count_rows());

        let margin = self.config.get_margin();

        total + counth + margin.top.indent.size + margin.bottom.indent.size
    }

    /// Returns total widths of a table, including margin and vertical lines.
    pub fn total_width(&self) -> usize {
        let mut dims = TableDimension::from_origin(&self.dimension);
        dims.estimate(&self.records, &self.config);

        let total = (0..self.count_columns())
            .map(|col| dims.get_width(col))
            .sum::<usize>();
        let countv = self.config.count_vertical(self.count_columns());

        let margin = self.config.get_margin();

        total + countv + margin.left.indent.size + margin.right.indent.size
    }

    /// Returns a table config.
    pub fn get_config(&self) -> &ColoredConfig {
        &self.config
    }
}

impl fmt::Display for Table {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.is_empty() {
            return Ok(());
        }

        let config = use_format_configuration(f, self);

        let mut dimension = self.dimension.clone();
        dimension.estimate(&self.records, &config);

        if !self.config.colors.is_empty() {
            Grid::new(&self.records, &dimension, config.as_ref())
                .with_colors(&self.config.colors)
                .build(f)
        } else {
            Grid::new(&self.records, &dimension, config.as_ref()).build(f)
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
        let data = builder.into();
        let records = VecRecords::new(data);

        Self {
            records,
            config: ColoredConfig::new(configure_grid(), HashMap::default()),
            dimension: TableDimension::default(),
        }
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

fn configure_grid() -> GridConfig {
    let mut cfg = GridConfig::default();
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
) -> Cow<'a, GridConfig> {
    if f.align().is_some() || f.width().is_some() {
        let mut cfg = table.config.config.clone();

        set_align_table(f, &mut cfg);
        set_width_table(f, &mut cfg, table);

        Cow::Owned(cfg)
    } else {
        Cow::Borrowed(&table.config.config)
    }
}

fn set_align_table(f: &fmt::Formatter<'_>, cfg: &mut GridConfig) {
    if let Some(alignment) = f.align() {
        let alignment = convert_fmt_alignment(alignment);
        cfg.set_alignment_horizontal(Entity::Global, alignment);
    }
}

fn set_width_table(f: &fmt::Formatter<'_>, cfg: &mut GridConfig, table: &Table) {
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

        let mut margin = cfg.get_margin_mut();
        margin.left.indent.size += left;
        margin.right.indent.size += right;

        if (margin.left.indent.size > 0 && margin.left.indent.fill == char::default())
            || fill != char::default()
        {
            margin.left.indent.fill = fill;
        }

        if (margin.right.indent.size > 0 && margin.right.indent.fill == char::default())
            || fill != char::default()
        {
            margin.right.indent.fill = fill;
        }
    }
}

/// A [`Table`] configuration.
#[derive(Debug, Clone)]
pub struct ColoredConfig {
    config: GridConfig,
    colors: HashMap<Position, AnsiColor<'static>>,
}

impl ColoredConfig {
    /// Create a new colored config.
    pub fn new(config: GridConfig, colors: HashMap<Position, AnsiColor<'static>>) -> Self {
        Self { config, colors }
    }

    /// Set a color for a given cell.
    ///
    /// The outcome is the same as if you'd use [`Format`] and added a color but it'd work only with `color` feature on.
    /// While this method works in all contexts.
    pub fn set_color(&mut self, pos: Position, color: AnsiColor<'static>) -> &mut Self {
        let _ = self.colors.insert(pos, color);
        self
    }
}

impl Deref for ColoredConfig {
    type Target = GridConfig;

    fn deref(&self) -> &Self::Target {
        &self.config
    }
}

impl DerefMut for ColoredConfig {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.config
    }
}

impl From<GridConfig> for ColoredConfig {
    fn from(value: GridConfig) -> Self {
        Self::new(value, Default::default())
    }
}
