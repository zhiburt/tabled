//! This module contains a main table representation [`Table`].

use core::ops::DerefMut;
use std::{borrow::Cow, fmt, iter::FromIterator};

use crate::{
    builder::Builder,
    grid::{
        colors::NoColors,
        config::{
            AlignmentHorizontal, ColorMap, ColoredConfig, CompactConfig, Entity, Indent, Sides,
            SpannedConfig,
        },
        dimension::{CompleteDimension, Dimension, Estimate, PeekableGridDimension},
        records::{
            vec_records::{Text, VecRecords},
            ExactRecords, Records,
        },
        PeekableGrid,
    },
    settings::{object::Object, CellOption, Style, TableOption},
    Tabled,
};

/// The structure provides an interface for building a table for types that implements [`Tabled`].
///
/// To build a string representation of a table you must use a [`std::fmt::Display`].
/// Or simply call `.to_string()` method.
///
/// The default table [`Style`] is [`Style::ascii`],
/// with a single left and right [`Padding`].
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
/// table
///     .with(Style::psql())
///     .with(Alignment::left());
/// ```
///
/// ### With a [`Tabled`] trait.
///
/// ```rust,no_run
/// use tabled::{Table, Tabled};
///
/// #[derive(Tabled)]
/// struct Character {
///     good: f32,
///     bad: f32,
///     encouraging: f32,
///     destructive: f32,
/// }
///
/// #[derive(Tabled)]
/// struct Person<'a>(
///     #[tabled(rename = "name")] &'a str,
///     #[tabled(inline)] Character,
/// );
///
/// let data = vec![
///     Person("007", Character { good: 0.8, bad: 0.2, encouraging: 0.8, destructive: 0.1}),
///     Person("001", Character { good: 0.2, bad: 0.5, encouraging: 0.2, destructive: 0.1}),
///     Person("006", Character { good: 0.4, bad: 0.1, encouraging: 0.5, destructive: 0.8}),
/// ];
///
/// let table = Table::new(&data);
/// ```
///
/// [`Padding`]: crate::settings::Padding
/// [`Style`]: crate::settings::Style
/// [`Style::ascii`]: crate::settings::Style::ascii
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Table {
    records: VecRecords<Text<String>>,
    config: ColoredConfig,
    dimension: CompleteDimension,
}

impl Table {
    /// Creates a Table instance, from a list of [`Tabled`] values.
    ///
    /// If you use a reference iterator you'd better use [`FromIterator`] instead.
    /// As it has a different lifetime constraints and make less copies therefore.
    ///
    /// # Examples
    ///
    /// ```
    /// use tabled::{Table, Tabled, assert::assert_table};
    ///
    /// #[derive(Tabled)]
    /// struct Relationship {
    ///     love: bool
    /// }
    ///
    /// let list = vec![
    ///     Relationship { love: true },
    ///     Relationship { love: false },
    /// ];
    ///
    /// let table = Table::new(list);
    ///
    /// assert_table!(
    ///     table,
    ///     "+-------+"
    ///     "| love  |"
    ///     "+-------+"
    ///     "| true  |"
    ///     "+-------+"
    ///     "| false |"
    ///     "+-------+"
    /// );
    /// ```
    ///
    /// ## Don't hesitate to use iterators.
    ///
    /// ```
    /// use tabled::{Table, Tabled, assert::assert_table};
    ///
    /// #[derive(Tabled)]
    /// struct Relationship {
    ///     person: String,
    ///     love: bool
    /// }
    ///
    /// let list = vec![
    ///     Relationship { person: String::from("Clara"), love: true },
    ///     Relationship { person: String::from("Greg"), love: false },
    /// ];
    ///
    /// // Maybe don't love but don't hate :)
    /// let iter = list.into_iter()
    ///     .map(|mut rel| {
    ///         if !rel.love {
    ///             rel.love = true;
    ///         }
    ///
    ///         rel
    ///     });
    ///
    /// let table = Table::new(iter);
    ///
    /// assert_table!(
    ///     table,
    ///     "+--------+------+"
    ///     "| person | love |"
    ///     "+--------+------+"
    ///     "| Clara  | true |"
    ///     "+--------+------+"
    ///     "| Greg   | true |"
    ///     "+--------+------+"
    /// );
    /// ```
    ///
    /// ## Notice that you can pass tuples.
    ///
    /// ```
    /// use tabled::{Table, Tabled, assert::assert_table};
    ///
    /// #[derive(Tabled)]
    /// struct Relationship {
    ///     love: bool
    /// }
    ///
    /// let list = vec![
    ///     ("Kate", Relationship { love: true }),
    ///     ("", Relationship { love: false }),
    /// ];
    ///
    /// let table = Table::new(list);
    ///
    /// assert_table!(
    ///     table,
    ///     "+------+-------+"
    ///     "| &str | love  |"
    ///     "+------+-------+"
    ///     "| Kate | true  |"
    ///     "+------+-------+"
    ///     "|      | false |"
    ///     "+------+-------+"
    /// );
    /// ```
    ///
    /// ## As a different way to create a [`Table`], you can use [`Table::from_iter`].  
    ///
    /// ```
    /// use std::iter::FromIterator;
    /// use tabled::{Table, assert::assert_table};
    ///
    /// let list = vec![
    ///     vec!["Kate", "+", "+", "+", "-"],
    ///     vec!["", "-", "-", "-", "-"],
    /// ];
    ///
    /// let table = Table::from_iter(list);
    ///
    /// assert_table!(
    ///     table,
    ///     "+------+---+---+---+---+"
    ///     "| Kate | + | + | + | - |"
    ///     "+------+---+---+---+---+"
    ///     "|      | - | - | - | - |"
    ///     "+------+---+---+---+---+"
    /// );
    /// ```
    pub fn new<I, T>(iter: I) -> Self
    where
        I: IntoIterator<Item = T>,
        T: Tabled,
    {
        let mut header = Vec::with_capacity(T::LENGTH);
        for text in T::headers() {
            let text = text.into_owned();
            let cell = Text::new(text);
            header.push(cell);
        }

        let mut records = vec![header];
        for row in iter.into_iter() {
            let mut list = Vec::with_capacity(T::LENGTH);
            for text in row.fields().into_iter() {
                let text = text.into_owned();
                let cell = Text::new(text);

                list.push(cell);
            }

            records.push(list);
        }

        let records = VecRecords::new(records);
        let config = ColoredConfig::new(configure_grid());
        let dimension = CompleteDimension::default();

        Self {
            records,
            config,
            dimension,
        }
    }

    /// Creates a Table instance, from a list of [`Tabled`] values.
    ///
    /// It's an optimized version of [`Table::new`].
    ///
    /// ```
    /// use tabled::{Table, Tabled, assert::assert_table};
    ///
    /// #[derive(Tabled)]
    /// struct Relationship {
    ///     person: String,
    ///     love: bool
    /// }
    ///
    /// let list = vec![
    ///     Relationship { person: String::from("Clara"), love: true },
    ///     Relationship { person: String::from("Greg"), love: false },
    /// ];
    ///
    /// let table = Table::with_capacity(&list, list.len());
    ///
    /// assert_table!(
    ///     table,
    ///     "+--------+-------+"
    ///     "| person | love  |"
    ///     "+--------+-------+"
    ///     "| Clara  | true  |"
    ///     "+--------+-------+"
    ///     "| Greg   | false |"
    ///     "+--------+-------+"
    /// );
    /// ```
    pub fn with_capacity<I, T>(iter: I, count_rows: usize) -> Self
    where
        I: IntoIterator<Item = T>,
        T: Tabled,
    {
        let mut header = Vec::with_capacity(T::LENGTH);
        for text in T::headers() {
            let text = text.into_owned();
            let cell = Text::new(text);
            header.push(cell);
        }

        let mut records = Vec::with_capacity(count_rows + 1);
        records.push(header);

        for row in iter.into_iter() {
            let mut list = Vec::with_capacity(T::LENGTH);
            for text in row.fields().into_iter() {
                let text = text.into_owned();
                let cell = Text::new(text);

                list.push(cell);
            }

            records.push(list);
        }

        let records = VecRecords::new(records);
        let config = ColoredConfig::new(configure_grid());
        let dimension = CompleteDimension::default();

        Self {
            records,
            config,
            dimension,
        }
    }

    /// Creates a Table instance, from a list of [`Tabled`] values.
    ///
    /// Compared to [`Table::new`] it does not use a "header" (first line).
    ///
    /// If you use a reference iterator you'd better use [`FromIterator`] instead.
    /// As it has a different lifetime constraints and make less copies therefore.
    ///
    /// # Examples
    ///
    /// ```
    /// use tabled::{Table, Tabled, assert::assert_table};
    ///
    /// #[derive(Tabled)]
    /// struct Relationship {
    ///     love: bool
    /// }
    ///
    /// let list = vec![
    ///     ("Kate", Relationship { love: true }),
    ///     ("", Relationship { love: false }),
    /// ];
    ///
    /// let table = Table::nohead(list);
    ///
    /// assert_table!(
    ///     table,
    ///     "+------+-------+"
    ///     "| Kate | true  |"
    ///     "+------+-------+"
    ///     "|      | false |"
    ///     "+------+-------+"
    /// );
    /// ```
    pub fn nohead<I, T>(iter: I) -> Self
    where
        I: IntoIterator<Item = T>,
        T: Tabled,
    {
        let mut records = vec![];
        for row in iter.into_iter() {
            let mut list = Vec::with_capacity(T::LENGTH);
            for text in row.fields().into_iter() {
                let text = text.into_owned();
                let cell = Text::new(text);

                list.push(cell);
            }

            records.push(list);
        }

        let records = VecRecords::new(records);
        let config = ColoredConfig::new(configure_grid());
        let dimension = CompleteDimension::default();

        Self {
            records,
            config,
            dimension,
        }
    }

    /// Creates a Key-Value [`Table`] instance, from a list of [`Tabled`] values.
    ///
    /// # Examples
    ///
    /// ```
    /// use tabled::{Table, Tabled, assert::assert_table};
    ///
    /// #[derive(Tabled)]
    /// #[tabled(rename_all = "PascalCase")]
    /// struct Swim {
    ///     event: String,
    ///     time: String,
    ///     #[tabled(rename = "Pool Length")]
    ///     pool: u8,
    /// }
    ///
    /// const POOL_25: u8 = 25;
    /// const POOL_50: u8 = 50;
    ///
    /// let list = vec![
    ///     Swim { event: String::from("Men 100 Freestyle"), time: String::from("47.77"), pool: POOL_25 },
    ///     Swim { event: String::from("Men 400 Freestyle"), time: String::from("03:59.16"), pool: POOL_25 },
    ///     Swim { event: String::from("Men 800 Freestyle"), time: String::from("08:06.70"), pool: POOL_25 },
    ///     Swim { event: String::from("Men 4x100 Medley Relay"), time: String::from("03:27.28"), pool: POOL_50 },
    /// ];
    ///
    /// let table = Table::kv(list);
    ///
    /// assert_table!(
    ///     table,
    ///     "+-------------+------------------------+"
    ///     "| Event       | Men 100 Freestyle      |"
    ///     "+-------------+------------------------+"
    ///     "| Time        | 47.77                  |"
    ///     "+-------------+------------------------+"
    ///     "| Pool Length | 25                     |"
    ///     "+-------------+------------------------+"
    ///     "| Event       | Men 400 Freestyle      |"
    ///     "+-------------+------------------------+"
    ///     "| Time        | 03:59.16               |"
    ///     "+-------------+------------------------+"
    ///     "| Pool Length | 25                     |"
    ///     "+-------------+------------------------+"
    ///     "| Event       | Men 800 Freestyle      |"
    ///     "+-------------+------------------------+"
    ///     "| Time        | 08:06.70               |"
    ///     "+-------------+------------------------+"
    ///     "| Pool Length | 25                     |"
    ///     "+-------------+------------------------+"
    ///     "| Event       | Men 4x100 Medley Relay |"
    ///     "+-------------+------------------------+"
    ///     "| Time        | 03:27.28               |"
    ///     "+-------------+------------------------+"
    ///     "| Pool Length | 50                     |"
    ///     "+-------------+------------------------+"
    /// );
    /// ```
    ///
    /// Next you'll find a more complex example with a subtle style.
    ///
    /// ```
    /// use tabled::{Table, Tabled, settings::Style};
    /// use tabled::settings::{style::HorizontalLine, Theme};
    /// use tabled::assert::assert_table;
    ///
    /// #[derive(Tabled)]
    /// #[tabled(rename_all = "PascalCase")]
    /// struct Swim {
    ///     event: String,
    ///     time: String,
    ///     #[tabled(rename = "Pool Length")]
    ///     pool: u8,
    /// }
    ///
    /// const POOL_25: u8 = 25;
    /// const POOL_50: u8 = 50;
    ///
    /// let list = vec![
    ///     Swim { event: String::from("Men 100 Freestyle"), time: String::from("47.77"), pool: POOL_25 },
    ///     Swim { event: String::from("Men 400 Freestyle"), time: String::from("03:59.16"), pool: POOL_25 },
    ///     Swim { event: String::from("Men 800 Freestyle"), time: String::from("08:06.70"), pool: POOL_25 },
    ///     Swim { event: String::from("Men 4x100 Medley Relay"), time: String::from("03:27.28"), pool: POOL_50 },
    /// ];
    ///
    /// let mut table = Table::kv(list);
    ///
    /// let mut style = Theme::from_style(Style::rounded().remove_horizontals());
    /// for entry in 1 .. table.count_rows() / Swim::LENGTH {
    ///     style.insert_horizontal_line(entry * Swim::LENGTH, HorizontalLine::inherit(Style::modern()));
    /// }
    ///
    /// table.with(style);
    ///
    /// assert_table!(
    ///     table,
    ///     "╭─────────────┬────────────────────────╮"
    ///     "│ Event       │ Men 100 Freestyle      │"
    ///     "│ Time        │ 47.77                  │"
    ///     "│ Pool Length │ 25                     │"
    ///     "├─────────────┼────────────────────────┤"
    ///     "│ Event       │ Men 400 Freestyle      │"
    ///     "│ Time        │ 03:59.16               │"
    ///     "│ Pool Length │ 25                     │"
    ///     "├─────────────┼────────────────────────┤"
    ///     "│ Event       │ Men 800 Freestyle      │"
    ///     "│ Time        │ 08:06.70               │"
    ///     "│ Pool Length │ 25                     │"
    ///     "├─────────────┼────────────────────────┤"
    ///     "│ Event       │ Men 4x100 Medley Relay │"
    ///     "│ Time        │ 03:27.28               │"
    ///     "│ Pool Length │ 50                     │"
    ///     "╰─────────────┴────────────────────────╯"
    /// );
    /// ```
    pub fn kv<I, T>(iter: I) -> Self
    where
        I: IntoIterator<Item = T>,
        T: Tabled,
    {
        let headers = T::headers();

        let mut records = Vec::new();
        for row in iter.into_iter() {
            for (text, name) in row.fields().into_iter().zip(headers.iter()) {
                let key = Text::new(name.clone().into_owned());
                let value = Text::new(text.into_owned());

                records.push(vec![key, value]);
            }
        }

        let records = VecRecords::new(records);
        let config = ColoredConfig::new(configure_grid());
        let dimension = CompleteDimension::default();

        Self {
            records,
            config,
            dimension,
        }
    }

    /// Creates a builder from a data set given.
    ///
    /// # Example
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
    ///     .modify(Segment::new(1.., 1..), Alignment::center())
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
        let mut builder = Builder::with_capacity(1, T::LENGTH);
        builder.push_record(T::headers());

        for row in iter {
            builder.push_record(row.fields().into_iter());
        }

        builder
    }

    /// It's a generic function which applies options to the [`Table`].
    ///
    /// It applies settings immediately.
    ///
    /// ```
    /// use tabled::{Table, settings::Style};
    /// use tabled::assert::assert_table;
    ///
    /// let data = vec![
    ///     ("number", "name"),
    ///     ("285-324-7322", "Rosalia"),
    ///     ("564.549.6468", "Mary"),
    /// ];
    ///
    /// let mut table = Table::new(data);
    /// table.with(Style::markdown());
    ///
    /// assert_table!(
    ///     table,
    ///     "| &str         | &str    |"
    ///     "|--------------|---------|"
    ///     "| number       | name    |"
    ///     "| 285-324-7322 | Rosalia |"
    ///     "| 564.549.6468 | Mary    |"
    /// );
    /// ```
    pub fn with<O>(&mut self, option: O) -> &mut Self
    where
        O: TableOption<VecRecords<Text<String>>, ColoredConfig, CompleteDimension>,
    {
        let reastimation_hint = option.hint_change();

        option.change(&mut self.records, &mut self.config, &mut self.dimension);
        self.dimension.reastimate(reastimation_hint);

        self
    }

    /// It's a generic function which applies options to particular cells on the [`Table`].
    /// Target cells using [`Object`]s such as [`Cell`], [`Rows`], [`Location`] and more.
    ///
    /// It applies settings immediately.
    ///
    /// ```
    /// use tabled::{Table, settings::{object::Columns, Alignment}};
    /// use tabled::assert::assert_table;
    ///
    /// let data = vec![
    ///     ("number", "name"),
    ///     ("285-324-7322", "Rosalia"),
    ///     ("564.549.6468", "Mary"),
    /// ];
    ///
    /// let mut table = Table::new(data);
    /// table.modify(Columns::first(), Alignment::right());
    /// table.modify(Columns::one(1), Alignment::center());
    ///
    /// assert_table!(
    ///     table,
    ///     "+--------------+---------+"
    ///     "|         &str |  &str   |"
    ///     "+--------------+---------+"
    ///     "|       number |  name   |"
    ///     "+--------------+---------+"
    ///     "| 285-324-7322 | Rosalia |"
    ///     "+--------------+---------+"
    ///     "| 564.549.6468 |  Mary   |"
    ///     "+--------------+---------+"
    /// );
    /// ```
    ///
    /// [`Cell`]: crate::settings::object::Cell
    /// [`Rows`]: crate::settings::object::Rows
    /// [`Location`]: crate::settings::location::Locator
    pub fn modify<T, O>(&mut self, target: T, option: O) -> &mut Self
    where
        T: Object<VecRecords<Text<String>>>,
        O: CellOption<VecRecords<Text<String>>, ColoredConfig> + Clone,
    {
        for entity in target.cells(&self.records) {
            let opt = option.clone();
            opt.change(&mut self.records, &mut self.config, entity);
        }

        let hint = option.hint_change();
        self.dimension.reastimate(hint);

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
        let mut dims = self.dimension.clone();
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
        let mut dims = self.dimension.clone();
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
    pub fn get_records(&self) -> &VecRecords<Text<String>> {
        &self.records
    }

    /// Returns a used records.
    pub fn get_records_mut(&mut self) -> &mut VecRecords<Text<String>> {
        &mut self.records
    }

    /// Returns a dimension.
    pub fn get_dimension(&self) -> &CompleteDimension {
        &self.dimension
    }

    /// Returns a dimension.
    pub fn get_dimension_mut(&mut self) -> &mut CompleteDimension {
        &mut self.dimension
    }
}

impl Default for Table {
    fn default() -> Self {
        Self {
            records: VecRecords::default(),
            config: ColoredConfig::new(configure_grid()),
            dimension: CompleteDimension::default(),
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
            let mut dims = PeekableGridDimension::default();
            dims.estimate(&self.records, &config);

            print_grid(f, &self.records, &config, &dims, colors)
        }
    }
}

impl<T> FromIterator<T> for Table
where
    T: IntoIterator,
    T::Item: Into<String>,
{
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        Builder::from_iter(iter.into_iter().map(|i| i.into_iter().map(|s| s.into()))).build()
    }
}

impl From<Builder> for Table {
    fn from(builder: Builder) -> Self {
        let data = builder.into();
        let records = VecRecords::new(data);
        let config = ColoredConfig::new(configure_grid());
        let dimension = CompleteDimension::default();

        Self {
            records,
            config,
            dimension,
        }
    }
}

impl From<Table> for Builder {
    fn from(val: Table) -> Self {
        let data = val.records.into();
        Builder::from_vec(data)
    }
}

impl<R, D> TableOption<R, ColoredConfig, D> for CompactConfig {
    fn change(self, _: &mut R, cfg: &mut ColoredConfig, _: &mut D) {
        *cfg.deref_mut() = self.into();
    }
}

impl<R, D> TableOption<R, ColoredConfig, D> for ColoredConfig {
    fn change(self, _: &mut R, cfg: &mut ColoredConfig, _: &mut D) {
        *cfg = self;
    }
}

impl<R, D> TableOption<R, ColoredConfig, D> for SpannedConfig {
    fn change(self, _: &mut R, cfg: &mut ColoredConfig, _: &mut D) {
        *cfg.deref_mut() = self;
    }
}

impl<R, D> TableOption<R, ColoredConfig, D> for &SpannedConfig {
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
    cfg.set_line_alignment(Entity::Global, false);
    cfg.set_trim_horizontal(Entity::Global, false);
    cfg.set_trim_vertical(Entity::Global, false);
    cfg.set_borders(Style::ascii().get_borders());

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
    records: &VecRecords<Text<String>>,
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
