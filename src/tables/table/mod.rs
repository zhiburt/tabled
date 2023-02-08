//! This module contains a main table representation of this crate [`Table`].

use std::{borrow::Cow, fmt, iter::FromIterator};

use crate::{
    builder::Builder,
    grid::config::AlignmentHorizontal,
    grid::{
        config::{Entity, Formatting, GridConfig, Indent, Padding},
        dimension::{Dimension, ExactDimension},
        grid_projection::GridProjection,
        Grid,
    },
    records::{ExactRecords, Records, VecRecords},
    settings::{style::Style, TableOption},
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
pub struct Table {
    records: VecRecords<Cow<'static, str>>,
    cfg: GridConfig,
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
        let mut header = vec![Cow::Borrowed(""); T::LENGTH];
        for (text, cell) in T::headers().into_iter().zip(header.iter_mut()) {
            *cell = text;
        }

        let mut records = vec![header];
        for row in iter.into_iter() {
            let mut list = vec![Cow::Borrowed(""); T::LENGTH];
            for (col, cell) in row.fields().into_iter().enumerate() {
                let cell = Cow::Owned(cell.into_owned());
                list[col] = cell;
            }

            records.push(list);
        }

        let records = VecRecords::new(records);

        Self {
            records,
            cfg: configure_grid(),
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
    pub fn builder<I, T>(iter: I) -> Builder
    where
        T: Tabled,
        I: IntoIterator<Item = T>,
    {
        let mut records = Vec::new();
        for row in iter {
            let mut list = vec![Cow::Borrowed(""); T::LENGTH];
            for (col, cell) in row.fields().into_iter().enumerate() {
                let cell = Cow::Owned(cell.into_owned());
                list[col] = cell;
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
    pub fn with<O: TableOption<VecRecords<Cow<'static, str>>, TableDimension<'static>>>(
        &mut self,
        option: O,
    ) -> &mut Self {
        self.dimension.clear_width();
        self.dimension.clear_height();

        let mut option = option;
        option.change(&mut self.records, &mut self.cfg, &mut self.dimension);

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
        dims.estimate(&self.records, &self.cfg);

        let gp = GridProjection::new(&self.cfg).count_rows(self.count_rows());
        let width = gp.total_height(&dims);

        let margin = self.cfg.get_margin();

        width + margin.left.size + margin.right.size
    }

    /// Returns total widths of a table, including margin and vertical lines.
    pub fn total_width(&self) -> usize {
        let mut dims = TableDimension::from_origin(&self.dimension);
        dims.estimate(&self.records, &self.cfg);

        let gp = GridProjection::new(&self.cfg).count_columns(self.count_columns());
        let width = gp.total_width(&dims);

        let margin = self.cfg.get_margin();

        width + margin.left.size + margin.right.size
    }
}

impl fmt::Display for Table {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let config = use_format_configuration(f, self);

        let mut dimension = self.dimension.clone();
        dimension.estimate(&self.records, &config);

        let grid = Grid::new(self.records.clone(), &config, &dimension);

        write!(f, "{}", grid)
    }
}

impl<T> FromIterator<T> for Table
where
    T: Tabled,
{
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        Self::new(iter)
    }
}

impl From<Builder> for Table {
    fn from(builder: Builder) -> Self {
        let data = builder.into();
        let records = VecRecords::new(data);

        Self {
            records,
            cfg: configure_grid(),
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

#[derive(Debug, Default, Clone)]
pub struct TableDimension<'a> {
    width: Option<Cow<'a, [usize]>>,
    height: Option<Cow<'a, [usize]>>,
}

impl TableDimension<'_> {
    pub fn is_complete(&self) -> bool {
        self.width.is_some() && self.height.is_some()
    }

    pub fn count_rows(&self) -> Option<usize> {
        self.height.as_ref().map(|list| list.len())
    }

    pub fn count_columns(&self) -> Option<usize> {
        self.width.as_ref().map(|list| list.len())
    }

    /// Set column widths.
    ///
    /// In general the method is only considered to be usefull to a [`TableOption`].
    ///
    /// BE CAREFUL WITH THIS METHOD as it supposed that the content is not bigger than the provided widths.
    pub fn set_widths(&mut self, columns: Vec<usize>) -> bool {
        self.width = Some(Cow::Owned(columns));

        true
    }

    /// Set rows heights.
    ///
    /// In general the method is only considered to be usefull to a [`TableOption`].
    ///
    /// BE CAREFUL WITH THIS METHOD as it supposed that the content is not bigger than the provided heights.
    pub fn set_heights(&mut self, rows: Vec<usize>) -> bool {
        self.height = Some(Cow::Owned(rows));

        true
    }

    pub fn clear_width(&mut self) {
        self.width = None;
    }

    pub fn clear_height(&mut self) {
        self.height = None;
    }

    fn from_origin<'a>(origin: &'a TableDimension<'_>) -> TableDimension<'a> {
        let width = match origin.width.as_deref() {
            Some(v) => Some(Cow::Borrowed(v)),
            None => None,
        };

        let height = match origin.height.as_deref() {
            Some(v) => Some(Cow::Borrowed(v)),
            None => None,
        };

        TableDimension { width, height }
    }
}

impl Dimension for TableDimension<'_> {
    fn estimate<R: Records>(&mut self, records: R, cfg: &GridConfig) {
        match (self.width.is_some(), self.height.is_some()) {
            (true, true) => {}
            (true, false) => {
                self.height = Some(Cow::Owned(ExactDimension::height(records, cfg)));
            }
            (false, true) => {
                self.width = Some(Cow::Owned(ExactDimension::width(records, cfg)));
            }
            (false, false) => {
                let mut dims = ExactDimension::default();
                dims.estimate(records, cfg);
                let (width, height) = dims.into();

                self.width = Some(Cow::Owned(width));
                self.height = Some(Cow::Owned(height));
            }
        }
    }

    fn get_width(&self, column: usize) -> usize {
        let width = self
            .width
            .as_ref()
            .expect("It must always be Some at this point");

        width[column]
    }

    fn get_height(&self, row: usize) -> usize {
        let height = self
            .height
            .as_ref()
            .expect("It must always be Some at this point");

        if row >= height.len() {
            // the if is made for a user wrong calls
            return 1;
        } else {
            height[row]
        }
    }
}

fn configure_grid() -> GridConfig {
    let mut cfg = GridConfig::default();
    cfg.set_tab_width(4);
    cfg.set_padding(
        Entity::Global,
        Padding::new(
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
        let mut cfg = table.cfg.clone();

        set_align_table(f, &mut cfg);
        set_width_table(f, &mut cfg, table);

        Cow::Owned(cfg)
    } else {
        Cow::Borrowed(&table.cfg)
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

        cfg.set_margin(margin)
    }
}

fn set_width_table2<R>(f: &fmt::Formatter<'_>, cfg: &mut GridConfig, table: &Table)
where
    for<'a> &'a R: Records,
{
}
