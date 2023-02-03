//! Builder module provides a [`Builder`] type which helps building
//! a [`Table`] dynamically.
//!
//! It also contains [`IndexBuilder`] which can help to build a table with index.
//!
//! # Examples
//!
//! Here's an example of [`IndexBuilder`] usage
//!
#![cfg_attr(feature = "derive", doc = "```")]
#![cfg_attr(not(feature = "derive"), doc = "```ignore")]
//! use tabled::{Table, Tabled, Style};
//!
//! #[derive(Tabled)]
//! struct Mission {
//!     name: &'static str,
//!     #[tabled(inline)]
//!     status: Status,
//! }
//!
//! #[derive(Tabled)]
//! enum Status {
//!     Complete,
//!     Started,
//!     Ready,
//!     Unknown,
//! }
//!
//! let data = [
//!     Mission { name: "Algebra", status: Status::Unknown },
//!     Mission { name: "Apolo", status: Status::Complete },
//! ];
//!
//! let mut builder = Table::builder(&data).index();
//! builder
//!     .set_index(0)
//!     .set_name(None)
//!     .transpose();
//!
//! let mut table = builder.build();
//! table.with(Style::modern());
//!
//! println!("{}", table);
//!
//! assert_eq!(
//!     table.to_string(),
//!     concat!(
//!         "┌──────────┬─────────┬───────┐\n",
//!         "│          │ Algebra │ Apolo │\n",
//!         "├──────────┼─────────┼───────┤\n",
//!         "│ Complete │         │ +     │\n",
//!         "├──────────┼─────────┼───────┤\n",
//!         "│ Started  │         │       │\n",
//!         "├──────────┼─────────┼───────┤\n",
//!         "│ Ready    │         │       │\n",
//!         "├──────────┼─────────┼───────┤\n",
//!         "│ Unknown  │ +       │       │\n",
//!         "└──────────┴─────────┴───────┘",
//!    ),
//! )
//! ```
//!
//! Example when we don't want to show empty data of enum where not all variants are used.
//!
#![cfg_attr(feature = "derive", doc = "```")]
#![cfg_attr(not(feature = "derive"), doc = "```ignore")]
//! use tabled::{Table, Tabled, Style};
//!
//! #[derive(Tabled)]
//! enum Status {
//!     #[tabled(inline)]
//!     Complete {
//!         started_timestamp: usize,
//!         finihsed_timestamp: usize,
//!     },
//!     #[tabled(inline)]
//!     Started {
//!         timestamp: usize,
//!     },
//!     Ready,
//!     Unknown,
//! }
//!
//! let data = [
//!     Status::Unknown,
//!     Status::Complete { started_timestamp: 123, finihsed_timestamp: 234 },
//! ];
//!
//! let mut builder = Table::builder(&data);
//! builder.clean();
//!
//! let table = builder.build()
//!     .with(Style::modern())
//!     .to_string();
//!
//! println!("{}", table);
//!
//! assert_eq!(
//!     table,
//!     concat!(
//!         "┌───────────────────┬────────────────────┬─────────┐\n",
//!         "│ started_timestamp │ finihsed_timestamp │ Unknown │\n",
//!         "├───────────────────┼────────────────────┼─────────┤\n",
//!         "│                   │                    │ +       │\n",
//!         "├───────────────────┼────────────────────┼─────────┤\n",
//!         "│ 123               │ 234                │         │\n",
//!         "└───────────────────┴────────────────────┴─────────┘",
//!    ),
//! )
//! ```

use std::{borrow::Cow, iter::FromIterator};

use crate::{records::Records, Table};

/// Builder creates a [`Table`] from dynamic data set.
///
/// It useful when the amount of columns or rows is not known statically.
///
/// ```rust
/// use tabled::builder::Builder;
///
/// let mut builder = Builder::default();
/// builder.set_columns(["index", "measure", "value"]);
/// builder.add_record(["0", "weight", "0.443"]);
///
/// let table = builder.build();
///
/// println!("{}", table);
/// ```
///
/// It may be useful to use [`FromIterator`] for building.
///
/// ```rust
/// use tabled::builder::Builder;
/// use std::iter::FromIterator;
///
/// let data = vec![
///     ["column1", "column2"],
///     ["data1", "data2"],
///     ["data3", "data4"],
/// ];
///
/// let table = Builder::from_iter(data).build();
///
/// println!("{}", table);
/// ```
#[derive(Debug, Default, Clone)]
pub struct Builder {
    /// A list of rows.
    records: Vec<Vec<Cow<'static, str>>>,
    /// A columns row.
    columns: Option<Vec<Cow<'static, str>>>,
    /// A number of columns.
    count_columns: usize,
    /// A flag that the rows are not consistent.
    is_consistent: bool,
    /// A content of cells which are created in case rows has different length.
    empty_cell_text: Option<Cow<'static, str>>,
}

impl Builder {
    /// Creates a [`Builder`] instance.
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets a [`Table`] header.
    ///
    /// ```rust
    /// use tabled::builder::Builder;
    ///
    /// let mut builder = Builder::default();
    /// builder
    ///     .set_header((0..3).map(|i| i.to_string()))
    ///     .add_record(["i", "surname", "lastname"]);
    /// ```
    pub fn set_header<H, T>(&mut self, columns: H) -> &mut Self
    where
        H: IntoIterator<Item = T>,
        T: Into<Cow<'static, str>>,
    {
        let list = create_row(columns, self.count_columns);

        self.update_size(list.len());
        self.columns = Some(list);

        self
    }

    /// Set a column size.
    ///
    /// If it make it lower then it was originally it is considered NOP.
    pub fn hint_column_size(&mut self, size: usize) {
        self.count_columns = size;
        self.is_consistent = true;
    }

    /// Sets off a [`Table`] header.
    ///
    /// If not set its a nop.
    ///
    /// ```rust
    /// use tabled::Table;
    ///
    /// let data = [("Hello", 1u8, false), ("World", 21u8, true)];
    ///
    /// let table = Table::builder(data).build().to_string();
    ///
    /// assert_eq!(
    ///     table,
    ///     "+-------+----+-------+\n\
    ///      | &str  | u8 | bool  |\n\
    ///      +-------+----+-------+\n\
    ///      | Hello | 1  | false |\n\
    ///      +-------+----+-------+\n\
    ///      | World | 21 | true  |\n\
    ///      +-------+----+-------+"
    /// );
    ///
    ///
    /// let mut builder = Table::builder(data);
    /// builder.remove_columns();
    /// let table = builder.build().to_string();
    ///
    /// assert_eq!(
    ///     table,
    ///     "+-------+----+-------+\n\
    ///      | Hello | 1  | false |\n\
    ///      +-------+----+-------+\n\
    ///      | World | 21 | true  |\n\
    ///      +-------+----+-------+"
    /// );
    ///
    /// ```
    pub fn remove_header(&mut self) -> &mut Self {
        self.columns = None;

        self
    }

    /// Adds a row to a [`Table`].
    ///
    /// If [`Self::set_columns`] is not set the first row will be considered a header.
    ///
    /// ```rust
    /// use tabled::builder::Builder;
    ///
    /// let mut builder = Builder::default();
    /// builder.add_record((0..3).map(|i| i.to_string()));
    /// builder.add_record(["i", "surname", "lastname"]);
    /// ```
    pub fn add_record<R, T>(&mut self, row: R) -> &mut Self
    where
        R: IntoIterator<Item = T>,
        T: Into<Cow<'static, str>>,
    {
        let list = create_row(row, self.count_columns);

        self.update_size(list.len());
        self.records.push(list);

        self
    }

    /// Sets a content of cells which are created in case rows has different length.
    ///
    ///
    /// ```rust
    /// use tabled::builder::Builder;
    ///
    /// let mut builder = Builder::default();
    /// builder.set_default_text("undefined");
    /// builder.set_columns((0..3).map(|i| i.to_string()));
    /// builder.add_record(["i"]);
    /// ```
    pub fn set_default_text<T>(&mut self, text: T) -> &mut Self
    where
        T: Into<Cow<'static, str>>,
    {
        self.empty_cell_text = Some(text.into());
        self
    }

    /// Build creates a [`Table`] instance.
    ///
    /// ```rust
    /// use tabled::builder::Builder;
    ///
    /// let mut builder = Builder::default();
    /// builder.set_columns(["i", "column1", "column2"]);
    /// builder.add_record(["0", "value1", "value2"]);
    /// ```
    pub fn build(mut self) -> Table {
        Table::from(self)
    }

    /// Add an index to the [`Table`].
    ///
    /// Default index is a range 0-N where N is amount of records.
    ///
    /// # Example
    ///
    /// ```
    /// use tabled::Table;
    ///
    /// let table = Table::builder(&["Hello", "World", "!"]).index().build();
    ///
    /// assert_eq!(
    ///     table.to_string(),
    ///     "+---+-------+\n\
    ///      |   | &str  |\n\
    ///      +---+-------+\n\
    ///      | 0 | Hello |\n\
    ///      +---+-------+\n\
    ///      | 1 | World |\n\
    ///      +---+-------+\n\
    ///      | 2 | !     |\n\
    ///      +---+-------+"
    /// )
    /// ```
    pub fn index(self) -> IndexBuilder {
        IndexBuilder::new(self)
    }

    /// Clean removes empty columns and rows.
    ///
    /// # Example
    ///
    /// ```
    /// use tabled::Table;
    ///
    /// let mut builder = Table::builder(&["Hello", "World", ""]);
    /// builder.clean();
    ///
    /// let table = builder.build();
    ///
    /// assert_eq!(
    ///     table.to_string(),
    ///     "+-------+\n\
    ///      | &str  |\n\
    ///      +-------+\n\
    ///      | Hello |\n\
    ///      +-------+\n\
    ///      | World |\n\
    ///      +-------+"
    /// )
    /// ```
    pub fn clean(&mut self) -> &mut Self {
        self.clean_columns();
        self.clean_rows();
        self
    }

    fn clean_columns(&mut self) {
        let mut i = 0;
        for col in 0..self.count_columns {
            let col = col - i;

            let mut is_empty = true;
            for row in 0..self.records.len() {
                if !self.records[row][col].is_empty() {
                    is_empty = false;
                    break;
                }
            }

            if is_empty {
                for row in 0..self.records.len() {
                    self.records[row].remove(col);
                }

                if let Some(columns) = self.columns.as_mut() {
                    if columns.len() > col {
                        columns.remove(col);
                    }
                }

                i += 1;
            }
        }

        self.count_columns -= i;
    }

    fn clean_rows(&mut self) {
        for row in (0..self.records.len()).rev() {
            let mut is_empty = true;
            for col in 0..self.count_columns {
                if !self.records[row][col].is_empty() {
                    is_empty = false;
                    break;
                }
            }

            if is_empty {
                self.records.remove(row);
            }

            if row == 0 {
                break;
            }
        }
    }

    fn update_size(&mut self, size: usize) {
        match size.cmp(&self.count_columns) {
            std::cmp::Ordering::Less => {
                if !self.records.is_empty() {
                    self.is_consistent = false;
                }
            }
            std::cmp::Ordering::Greater => {
                self.count_columns = size;

                if !self.records.is_empty() || self.columns.is_some() {
                    self.is_consistent = false;
                }
            }
            std::cmp::Ordering::Equal => (),
        }
    }

    fn get_size(&mut self) -> usize {
        let mut max = self.columns.as_ref().map_or(0, Vec::len);
        let max_records = self.records.iter().map(Vec::len).max().unwrap_or(0);
        max = std::cmp::max(max_records, max);

        max
    }

    fn fix_rows(&mut self) {
        let empty_cell = self.empty_cell_text.to_owned().unwrap_or_default();

        if let Some(header) = self.columns.as_mut() {
            if self.count_columns > header.len() {
                append_vec(header, empty_cell, self.count_columns - header.len());
            }
        }

        for row in &mut self.records {
            if self.count_columns > row.len() {
                append_vec(row, empty_cell, self.count_columns - row.len());
            }
        }
    }
}

impl Into<Vec<Vec<Cow<'static, str>>>> for Builder {
    fn into(self) -> Vec<Vec<Cow<'static, str>>> {
        if !self.is_consistent {
            self.fix_rows();
        }

        append_header(&mut self.records, self.columns);

        self.records
    }
}

impl<R, V> FromIterator<R> for Builder
where
    R: IntoIterator<Item = V>,
    V: Into<Cow<'static, str>>,
{
    fn from_iter<T: IntoIterator<Item = R>>(iter: T) -> Self {
        let mut builder = Self::default();
        for row in iter {
            builder.add_record(row);
        }

        builder
    }
}

impl<D> Extend<D> for Builder
where
    D: Into<Cow<'static, str>>,
{
    fn extend<T: IntoIterator<Item = D>>(&mut self, iter: T) {
        self.add_record(iter);
    }
}

impl From<Vec<Vec<String>>> for Builder {
    fn from(records: Vec<Vec<String>>) -> Self {
        let mut count_columns = 0;
        let mut is_consistent = true;

        let mut list = Vec::with_capacity(records.len());
        for row in records.into_iter() {
            if count_columns != 0 && count_columns != row.len() {
                is_consistent = false;
            }

            count_columns = std::cmp::max(count_columns, row.len());

            let row = row.into_iter().map(Cow::Owned).collect::<Vec<_>>();
            list.push(row);
        }

        Self {
            records: list,
            count_columns,
            columns: None,
            is_consistent,
            ..Default::default()
        }
    }
}

impl<'a> From<Vec<Vec<Cow<'static, str>>>> for Builder {
    fn from(records: Vec<Vec<Cow<'static, str>>>) -> Self {
        let count_columns = records.get(0).map_or(0, |row| row.len());

        Self {
            records,
            count_columns,
            columns: None,
            is_consistent: false,
            empty_cell_text: None,
        }
    }
}

/// [`IndexBuilder`] helps to add an index to the table.
///
/// Index is a column on the left of the table.
///
/// It also can be used to transpose the table.
///
/// # Example
///
/// ```
/// use tabled::builder::Builder;
///
/// let table = Builder::default()
///     .index()
///     .build();
/// ```
#[derive(Debug, Clone)]
pub struct IndexBuilder {
    /// Index is an index data.
    /// It's always set.
    index: Vec<Cow<'static, str>>,
    /// Name of an index
    name: Option<Cow<'static, str>>,
    /// A flag which checks if we need to actually use index.
    ///
    /// It might happen when it's only necessary to [Self::transpose] table.
    print_index: bool,
    /// A flag which checks if table was transposed.
    transposed: bool,
    /// Original builder instance.
    b: Builder,
}

impl IndexBuilder {
    /// Creates a new [`IndexBuilder`] instance.
    ///
    /// It creates a default index a range from 0 to N. (N - count rows)
    /// It also sets a default columns to the range 0 .. N (N - count columns).
    ///nfo<'a>
    /// # Example
    ///
    /// ```
    /// use tabled::builder::Builder;
    ///
    /// let mut builder = Builder::default();
    /// builder.set_columns(["i", "col-1", "col-2"]);
    /// builder.add_record(["0", "value-1", "value-2"]);
    ///
    /// let table = builder.index().build();
    ///
    /// assert_eq!(
    ///     table.to_string(),
    ///     "+---+---+---------+---------+\n\
    ///      |   | i | col-1   | col-2   |\n\
    ///      +---+---+---------+---------+\n\
    ///      | 0 | 0 | value-1 | value-2 |\n\
    ///      +---+---+---------+---------+"
    /// )
    /// ```
    fn new(mut b: Builder) -> Self {
        let index = build_range_index(b.records.len());

        if b.columns.is_none() {
            b.columns = Some(build_range_index(b.count_columns));
        }

        Self {
            index,
            name: None,
            print_index: true,
            transposed: false,
            b,
        }
    }

    /// No flag makes builder to not use an index.
    ///
    /// It may be useful when only [`Self::transpose`] need to be used.
    ///
    /// ```
    /// use tabled::builder::Builder;
    ///
    /// let mut builder = Builder::default();
    /// builder.set_columns(["i", "col-1", "col-2"]);
    /// builder.add_record(["0", "value-1", "value-2"]);
    /// builder.add_record(["2", "value-3", "value-4"]);
    ///
    /// let mut builder = builder.index();
    /// builder.hide();
    ///
    /// let table = builder.build();
    ///
    /// assert_eq!(
    ///     table.to_string(),
    ///     "+---+---------+---------+\n\
    ///      | i | col-1   | col-2   |\n\
    ///      +---+---------+---------+\n\
    ///      | 0 | value-1 | value-2 |\n\
    ///      +---+---------+---------+\n\
    ///      | 2 | value-3 | value-4 |\n\
    ///      +---+---------+---------+"
    /// )
    /// ```
    pub fn hide(&mut self) -> &mut Self {
        self.print_index = false;
        self
    }

    /// Set an index name.
    ///
    /// When [`None`] the name won't be used.
    pub fn name(&mut self, name: Option<Cow<'_, str>>) -> &mut Self {
        self.name = name;
        self
    }

    /// Sets a index to the chosen column.
    ///
    /// Also sets a name of the index to the column name.
    ///
    /// # Example
    ///
    /// ```
    /// use tabled::builder::Builder;
    ///
    /// let mut builder = Builder::default();
    /// builder.set_columns(["i", "column1", "column2"]);
    /// builder.add_record(["0", "value1", "value2"]);
    ///
    /// let mut builder = builder.index();
    /// builder.set_index(1);
    ///
    /// let table = builder.build();
    ///
    /// assert_eq!(
    ///     table.to_string(),
    ///     "+---------+---+---------+\n\
    ///      |         | i | column2 |\n\
    ///      +---------+---+---------+\n\
    ///      | column1 |   |         |\n\
    ///      +---------+---+---------+\n\
    ///      | value1  | 0 | value2  |\n\
    ///      +---------+---+---------+"
    /// )
    /// ```
    pub fn index(&mut self, column: usize) -> &mut Self {
        if self.b.columns.is_none() {
            return self;
        }

        if column >= self.b.count_columns {
            return self;
        }

        let name = self
            .b
            .columns
            .as_mut()
            .map(|v| remove_or_default(v, column))
            .unwrap_or_default();

        self.name = Some(name);

        self.index = get_column(&mut self.b.records, column);

        self.b.count_columns -= 1;

        self
    }

    /// Transpose index and columns.
    ///
    /// # Example
    ///
    /// ```
    /// use tabled::builder::Builder;
    ///
    /// let mut builder = Builder::default();
    /// builder.set_columns(["i", "column-1", "column-2", "column-3"]);
    /// builder.add_record(["0", "value-1", "value-2", "value-3"]);
    /// builder.add_record(["1", "value-4", "value-5", "value-6"]);
    /// builder.add_record(["2", "value-7", "value-8", "value-9"]);
    ///
    /// let mut builder = builder.index();
    /// builder.set_index(1).transpose();
    ///
    /// let table = builder.build();
    ///
    /// assert_eq!(
    ///     table.to_string(),
    ///     "+----------+---------+---------+---------+\n\
    ///      | column-1 | value-1 | value-4 | value-7 |\n\
    ///      +----------+---------+---------+---------+\n\
    ///      | i        | 0       | 1       | 2       |\n\
    ///      +----------+---------+---------+---------+\n\
    ///      | column-2 | value-2 | value-5 | value-8 |\n\
    ///      +----------+---------+---------+---------+\n\
    ///      | column-3 | value-3 | value-6 | value-9 |\n\
    ///      +----------+---------+---------+---------+"
    /// )
    /// ```
    pub fn transpose(&mut self) -> &mut Self {
        let columns = self.b.columns.take().unwrap_or_default();

        let index = std::mem::replace(&mut self.index, columns);
        self.b.columns = Some(index);

        let new_count_columns = self.b.records.len();
        make_rows_columns(&mut self.b.records, self.b.count_columns);
        self.b.count_columns = new_count_columns;

        self.transposed = !self.transposed;

        self
    }

    /// Builds a table.
    pub fn build(self) -> Table {
        Builder::from(self).build()
    }
}

impl<'a> From<IndexBuilder> for Builder {
    fn from(index_builder: IndexBuilder) -> Self {
        let mut b = build_index(index_builder);

        // fixme: we don't update builder size internally
        b.fix_rows();
        b.is_consistent = true;

        b
    }
}

fn make_rows_columns<T: Default>(v: &mut Vec<Vec<T>>, count_columns: usize) {
    let mut columns = Vec::with_capacity(count_columns);
    for _ in 0..count_columns {
        let column = get_column(v, 0);
        columns.push(column);
    }

    v.clear();

    for column in columns {
        v.push(column);
    }
}

fn build_index(mut b: IndexBuilder) -> Builder {
    if b.index.is_empty() {
        return b.b;
    }

    let records = &mut b.b.records;

    // it's guaranteed to be set
    let columns = b.b.columns.take().unwrap();

    records.insert(0, columns);

    // add index column
    if b.print_index {
        b.b.count_columns += 1;
        b.index.insert(0, Cow::default());
        insert_column(records, b.index, 0);
    }

    if let Some(name) = b.name {
        if b.transposed && b.print_index {
            records[0][0] = name;
        } else {
            records.insert(1, vec![name]);
        }
    }

    b.b
}

fn insert_column<T: Default>(v: &mut [Vec<T>], mut column: Vec<T>, col: usize) {
    for row in v.iter_mut() {
        let value = remove_or_default(&mut column, col);
        row.insert(col, value);
    }
}

fn get_column<T: Default>(v: &mut [Vec<T>], col: usize) -> Vec<T> {
    let mut column = Vec::with_capacity(v.len());
    for row in v.iter_mut() {
        let value = remove_or_default(row, col);
        column.push(value);
    }

    column
}

fn remove_or_default<T: Default>(v: &mut Vec<T>, i: usize) -> T {
    if v.len() > i {
        v.remove(i)
    } else {
        T::default()
    }
}

fn build_range_index(n: usize) -> Vec<Cow<'static, str>> {
    (0..n).map(|i| Cow::Owned(i.to_string())).collect()
}

fn create_row<'a, R, T>(row: R, size: usize) -> Vec<Cow<'a, str>>
where
    R: IntoIterator<Item = T>,
    T: Into<Cow<'a, str>>,
{
    let mut list = Vec::with_capacity(size);
    for text in row {
        list.push(text.into());
    }

    list
}

fn append_header<'a>(records: &mut Vec<Vec<Cow<'a, str>>>, columns: Option<Vec<Cow<'a, str>>>) {
    if let Some(columns) = columns {
        records.insert(0, columns);
    }
}

fn append_vec<'a, T: Clone>(v: &mut Vec<T>, value: T, n: usize) {
    v.extend((0..n).map(|_| value.clone()));
}
