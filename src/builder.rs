//! Builder module provides a [Builder] type which helps building
//! a [Table] dynamically.
//!
//! It also contains [IndexBuilder] which can help to build a table with index.
//!
//! # Example
//!
//! Here's an example of [IndexBuilder] usage
//!
//! ```
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
//! let table = Table::builder(&data)
//!     .index()
//!     .set_index(0)
//!     .set_name(None)
//!     .transpose()
//!     .build()
//!     .with(Style::modern());
//!
//! println!("{}", table);
//!
//! assert_eq!(
//!     table.to_string(),
//!     concat!(
//!         "┌──────────┬─────────┬───────┐\n",
//!         "│          │ Algebra │ Apolo │\n",
//!         "├──────────┼─────────┼───────┤\n",
//!         "│ Complete │         │   +   │\n",
//!         "├──────────┼─────────┼───────┤\n",
//!         "│ Started  │         │       │\n",
//!         "├──────────┼─────────┼───────┤\n",
//!         "│  Ready   │         │       │\n",
//!         "├──────────┼─────────┼───────┤\n",
//!         "│ Unknown  │    +    │       │\n",
//!         "└──────────┴─────────┴───────┘\n",
//!    ),
//! )
//! ```

use std::{fmt::Display, iter::FromIterator};

use papergrid::{AlignmentHorizontal, Entity, Formatting, Grid, Indent, Settings};

use crate::{Style, Table};

/// Builder creates a [Table] from dynamic data set.
///
/// It usefull when the amount of columns or rows is not known statically.
///
/// ```rust
/// use tabled::builder::Builder;
/// let table = Builder::default()
///     .set_columns(["index", "measure", "value"])
///     .add_record(["0", "weight", "0.443"])
///     .build();
///
/// println!("{}", table);
/// ```
///
/// It may be usefull to use [FromIterator] for building.
///
/// ```rust
/// use tabled::builder::Builder;
/// use std::iter::FromIterator;
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
    records: Vec<Vec<String>>,
    /// A columns row.
    columns: Option<Vec<String>>,
    /// A number of columns.
    size: usize,
    /// A content of cells which are created in case rows has different length.
    empty_cell_text: Option<String>,
}

impl Builder {
    /// Creates a [Builder] instance.
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets a [Table] header.
    ///
    /// If not set a first row will be considered a header.
    ///
    /// ```rust
    /// use tabled::builder::Builder;
    /// let builder = Builder::default()
    ///     .set_columns(0..3)
    ///     .add_record(["i", "surname", "lastname"]);
    /// ```
    pub fn set_columns<H, T>(mut self, columns: H) -> Self
    where
        H: IntoIterator<Item = T>,
        T: Display,
    {
        let columns: Vec<String> = columns.into_iter().map(|t| t.to_string()).collect();
        self.update_size(columns.len());
        self.columns = Some(columns);

        self
    }

    /// Adds a row to a [Table].
    ///
    /// If [Self::set_columns] is not set the first row will be considered a header.
    ///
    /// ```rust
    /// use tabled::builder::Builder;
    /// let builder = Builder::default()
    ///     .add_record(0..3)
    ///     .add_record(["i", "surname", "lastname"]);
    /// ```
    pub fn add_record<R, T>(mut self, record: R) -> Self
    where
        R: IntoIterator<Item = T>,
        T: Display,
    {
        let row: Vec<String> = record.into_iter().map(|t| t.to_string()).collect();
        self.update_size(row.len());
        self.records.push(row);

        self
    }

    /// Sets a content of cells which are created in case rows has different length.
    ///
    ///
    /// ```rust
    /// use tabled::builder::Builder;
    /// let table = Builder::default()
    ///     .set_default_text("undefined")
    ///     .set_columns(0..3)
    ///     .add_record(["i"])
    ///     .build();
    /// ```
    pub fn set_default_text<T: Into<String>>(mut self, text: T) -> Self {
        self.empty_cell_text = Some(text.into());
        self
    }

    /// Build creates a [Table] instance.
    ///
    /// ```rust
    /// use tabled::builder::Builder;
    /// let table = Builder::default()
    ///     .set_columns(["i", "column1", "column2"])
    ///     .add_record(["0", "value1", "value2"])
    ///     .build();
    /// ```
    pub fn build(mut self) -> Table {
        self.fix_rows();
        build_table(self.columns, self.records, self.size)
    }

    /// Add an index to the [Table].
    ///
    /// Default index is a range 0-N where N is amount of records.
    ///
    /// # Example
    ///
    /// ```
    /// use tabled::Table;
    /// let table = Table::builder(&["Hello", "World", "!"])
    ///     .index()
    ///     .build();
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
    ///      | 2 |   !   |\n\
    ///      +---+-------+\n"
    /// )
    /// ```
    pub fn index(self) -> IndexBuilder {
        IndexBuilder::new(self)
    }

    fn update_size(&mut self, size: usize) {
        if size > self.size {
            self.size = size;
        }
    }

    fn fix_rows(&mut self) {
        let empty_cell_text = self.empty_cell_text.clone().unwrap_or_default();

        if let Some(header) = self.columns.as_mut() {
            if self.size > header.len() {
                append_vec(header, self.size - header.len(), empty_cell_text.clone());
            }
        }

        for row in self.records.iter_mut() {
            if self.size > row.len() {
                append_vec(row, self.size - row.len(), empty_cell_text.clone());
            }
        }
    }
}

impl<R, V> FromIterator<R> for Builder
where
    R: IntoIterator<Item = V>,
    V: Display,
{
    fn from_iter<T: IntoIterator<Item = R>>(iter: T) -> Self {
        let mut builder = Self::default();
        for row in iter {
            builder = builder.add_record(row);
        }

        builder
    }
}

impl<D> Extend<D> for Builder
where
    D: Display,
{
    fn extend<T: IntoIterator<Item = D>>(&mut self, iter: T) {
        let row: Vec<String> = iter.into_iter().map(|t| t.to_string()).collect();
        self.update_size(row.len());
        self.records.push(row);
    }
}

impl From<Vec<Vec<String>>> for Builder {
    fn from(records: Vec<Vec<String>>) -> Self {
        let max_row_length = records.iter().map(|row| row.len()).max().unwrap_or(0);
        Self {
            records,
            size: max_row_length,
            ..Default::default()
        }
    }
}

/// Building [Table] from ordinary data.
fn build_table(
    columns: Option<Vec<String>>,
    records: Vec<Vec<String>>,
    count_columns: usize,
) -> Table {
    let grid = build_grid(records, columns, count_columns);
    create_table_from_grid(grid)
}

/// Building [Grid] from ordinary data.
fn build_grid(
    records: Vec<Vec<String>>,
    columns: Option<Vec<String>>,
    count_columns: usize,
) -> Grid {
    let mut count_rows = records.len();

    if columns.is_some() {
        count_rows += 1;
    }

    let mut grid = Grid::new(count_rows, count_columns);

    let mut row = 0;
    if let Some(headers) = columns {
        for (i, text) in headers.into_iter().enumerate() {
            grid.set(Entity::Cell(0, i), Settings::new().text(text));
        }

        row = 1;
    }

    for fields in records.into_iter() {
        // don't show off a empty data array
        if fields.is_empty() {
            continue;
        }

        for (column, field) in fields.into_iter().enumerate() {
            grid.set(Entity::Cell(row, column), Settings::new().text(field));
        }

        row += 1;
    }

    grid
}

fn create_table_from_grid(grid: Grid) -> Table {
    let mut table = Table { grid };

    // it's crusial to set a global setting rather than a setting for an each cell
    // as it will be hard to override that since how Grid::style method works
    table.grid.set(Entity::Global, default_cell_style());

    table.with(Style::ascii())
}

fn default_cell_style() -> Settings {
    Settings::new()
        .padding(
            Indent::spaced(1),
            Indent::spaced(1),
            Indent::spaced(0),
            Indent::spaced(0),
        )
        .alignment(AlignmentHorizontal::Center)
        .formatting(Formatting {
            tab_width: 4,
            horizontal_trim: true,
            allow_lines_alignement: false,
            vertical_trim: false,
        })
}

fn append_vec(v: &mut Vec<String>, n: usize, value: String) {
    v.extend((0..n).map(|_| value.clone()));
}

/// [IndexBuilder] helps to add an index to the table.
///
/// Index is a column on the left of the table.
///
/// It also can be used to transpose the table.
///
/// # Example
///
/// ```
/// use tabled::builder::Builder;
/// let table = Builder::default()
///     .index()
///     .build();
/// ```
#[derive(Debug, Clone)]
pub struct IndexBuilder {
    /// Index is an index data.
    /// It's always set.
    index: Vec<String>,
    /// Name of an index
    name: Option<String>,
    /// A flag which checks if we need to actualy use index.
    ///
    /// It might happen when it's only nessary to [Self::transpose] table.
    print_index: bool,
    /// A flag which checks if table was transposed.
    transposed: bool,
    /// Original builder instance.
    b: Builder,
}

impl IndexBuilder {
    /// Creates a new [Self] instance.
    ///
    /// It creates a default index a range from 0 to N. (N - count rows)
    /// It also sets a default columns to the range 0 .. N (N - count columns).
    ///
    /// # Example
    ///
    /// ```
    /// use tabled::builder::Builder;
    /// let table = Builder::default()
    ///     .set_columns(["i", "col-1", "col-2"])
    ///     .add_record(["0", "value-1", "value-2"])
    ///     .index()
    ///     .build();
    ///
    /// assert_eq!(
    ///     table.to_string(),
    ///     "+---+---+---------+---------+\n\
    ///      |   | i |  col-1  |  col-2  |\n\
    ///      +---+---+---------+---------+\n\
    ///      | 0 | 0 | value-1 | value-2 |\n\
    ///      +---+---+---------+---------+\n"
    /// )
    /// ```
    fn new(mut b: Builder) -> Self {
        let index = build_range_index(b.records.len());

        if b.columns.is_none() {
            b.columns = Some(build_range_index(b.size))
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
    /// It may be usefull when only [Self::transpose] need to be used.
    pub fn no_index(mut self) -> Self {
        self.print_index = false;
        self
    }

    /// Set an index name.
    ///
    /// When [None] the name won't be used.
    pub fn set_name(mut self, name: Option<String>) -> Self {
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
    /// let table = Builder::default()
    ///     .set_columns(["i", "column1", "column2"])
    ///     .add_record(["0", "value1", "value2"])
    ///     .index()
    ///     .set_index(1)
    ///     .build();
    ///
    /// assert_eq!(
    ///     table.to_string(),
    ///     "+---------+---+---------+\n\
    ///      |         | i | column2 |\n\
    ///      +---------+---+---------+\n\
    ///      | column1 |   |         |\n\
    ///      +---------+---+---------+\n\
    ///      | value1  | 0 | value2  |\n\
    ///      +---------+---+---------+\n"
    /// )
    /// ```
    pub fn set_index(mut self, column: usize) -> Self {
        if self.b.columns.is_none() {
            return self;
        }

        if column >= self.b.size {
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

        self.b.size -= 1;

        self
    }

    /// Transpose index and columns.
    ///
    /// # Example
    ///
    /// ```
    /// use tabled::builder::Builder;
    /// let table = Builder::default()
    ///     .set_columns(["i", "column-1", "column-2", "column-3"])
    ///     .add_record(["0", "value-1", "value-2", "value-3"])
    ///     .add_record(["1", "value-4", "value-5", "value-6"])
    ///     .add_record(["2", "value-7", "value-8", "value-9"])
    ///     .index()
    ///     .set_index(1)
    ///     .transpose()
    ///     .build();
    ///
    /// assert_eq!(
    ///     table.to_string(),
    ///     "+----------+---------+---------+---------+\n\
    ///      | column-1 | value-1 | value-4 | value-7 |\n\
    ///      +----------+---------+---------+---------+\n\
    ///      |    i     |    0    |    1    |    2    |\n\
    ///      +----------+---------+---------+---------+\n\
    ///      | column-2 | value-2 | value-5 | value-8 |\n\
    ///      +----------+---------+---------+---------+\n\
    ///      | column-3 | value-3 | value-6 | value-9 |\n\
    ///      +----------+---------+---------+---------+\n"
    /// )
    /// ```
    pub fn transpose(mut self) -> Self {
        let columns = self.b.columns.take().unwrap();

        self.b.columns = Some(self.index);
        self.index = columns;

        let new_count_columns = self.b.records.len();
        make_rows_columns(&mut self.b.records, self.b.size);
        self.b.size = new_count_columns;

        self.transposed = !self.transposed;

        self
    }

    /// Builds a table.
    pub fn build(self) -> Table {
        build_index(self).build()
    }
}

fn make_rows_columns(v: &mut Vec<Vec<String>>, count_columns: usize) {
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

    b.b.size += 1;

    let records = &mut b.b.records;

    let columns = b.b.columns.take().unwrap();
    records.insert(0, columns);

    // add index column
    if b.print_index {
        b.index.insert(0, String::new());
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

fn get_column(v: &mut [Vec<String>], col: usize) -> Vec<String> {
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

fn build_range_index(n: usize) -> Vec<String> {
    (0..n).map(|i| i.to_string()).collect()
}
