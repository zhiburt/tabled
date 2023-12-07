use crate::{grid::records::vec_records::CellInfo, Table};

use super::Builder;

/// [`IndexBuilder`] helps to add an index to the table.
///
/// Index is a column on the left of the table.
///
/// It also can be used to transpose the table.
///
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
/// builder.push_record(["i", "col-1", "col-2"]);
/// builder.push_record(["0", "value-1", "value-2"]);
///
/// let table = builder.index().build().to_string();
///
/// assert_eq!(
///     table,
///     "+---+---+---------+---------+\n\
///      |   | i | col-1   | col-2   |\n\
///      +---+---+---------+---------+\n\
///      | 0 | 0 | value-1 | value-2 |\n\
///      +---+---+---------+---------+"
/// )
/// ```
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
    index: Vec<CellInfo<String>>,
    /// Name of an index
    name: Option<CellInfo<String>>,
    /// A flag which checks if we need to actually use index.
    ///
    /// It might happen when it's only necessary to [`Self::transpose`] table.
    print_index: bool,
    /// A flag which checks if table was transposed.
    transposed: bool,
    /// Data originated in [`Builder`].
    data: Vec<Vec<CellInfo<String>>>,
    /// A size of columns
    count_columns: usize,
}

impl IndexBuilder {
    /// No flag makes builder to not use an index.
    ///
    /// It may be useful when only [`Self::transpose`] need to be used.
    ///
    /// ```
    /// use tabled::builder::Builder;
    ///
    /// let mut builder = Builder::default();
    /// builder.push_record(["i", "col-1", "col-2"]);
    /// builder.push_record(["0", "value-1", "value-2"]);
    /// builder.push_record(["2", "value-3", "value-4"]);
    ///
    /// let table = builder.index().hide().build().to_string();
    ///
    /// assert_eq!(
    ///     table,
    ///     "+---+---------+---------+\n\
    ///      | i | col-1   | col-2   |\n\
    ///      +---+---------+---------+\n\
    ///      | 0 | value-1 | value-2 |\n\
    ///      +---+---------+---------+\n\
    ///      | 2 | value-3 | value-4 |\n\
    ///      +---+---------+---------+"
    /// )
    /// ```
    pub fn hide(mut self) -> Self {
        self.print_index = false;
        self
    }

    /// Set an index name.
    ///
    /// When [`None`] the name won't be used.
    ///
    /// # Example
    ///
    /// ```
    /// use tabled::builder::Builder;
    ///
    /// let mut builder = Builder::default();
    /// builder.push_record(["i", "column1", "column2"]);
    /// builder.push_record(["0", "value1", "value2"]);
    ///
    /// let table = builder.index()
    ///     .column(1)
    ///     .name(Some(String::from("index")))
    ///     .build();
    ///
    /// assert_eq!(
    ///     table.to_string(),
    ///     "+--------+---+---------+\n\
    ///      |        | i | column2 |\n\
    ///      +--------+---+---------+\n\
    ///      | index  |   |         |\n\
    ///      +--------+---+---------+\n\
    ///      | value1 | 0 | value2  |\n\
    ///      +--------+---+---------+"
    /// )
    /// ```
    pub fn name(mut self, name: Option<String>) -> Self {
        self.name = name.map(CellInfo::new);
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
    /// builder.push_record(["i", "column1", "column2"]);
    /// builder.push_record(["0", "value1", "value2"]);
    ///
    /// let table = builder.index().column(1).build();
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
    pub fn column(mut self, column: usize) -> Self {
        if column >= self.count_columns {
            return self;
        }

        self.index = get_column(&mut self.data, column);

        let name = self.index.remove(0);
        self.name = Some(name);

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
    /// builder.push_record(["i", "column-1", "column-2", "column-3"]);
    /// builder.push_record(["0", "value-1", "value-2", "value-3"]);
    /// builder.push_record(["1", "value-4", "value-5", "value-6"]);
    /// builder.push_record(["2", "value-7", "value-8", "value-9"]);
    ///
    /// let table = builder.index().column(1).transpose().build();
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
    pub fn transpose(mut self) -> Self {
        if self.data.is_empty() {
            return self;
        }

        let mut columns = self.data.remove(0);
        std::mem::swap(&mut self.index, &mut columns);

        let count_columns = columns.len();
        make_rows_columns(&mut self.data, self.index.len());

        self.data.insert(0, columns);

        self.transposed = !self.transposed;
        self.count_columns = count_columns;

        self
    }

    /// Builds a table.
    pub fn build(self) -> Table {
        let builder: Builder = self.into();
        builder.build()
    }
}

impl From<Builder> for IndexBuilder {
    fn from(builder: Builder) -> Self {
        let count_columns = builder.count_columns();
        let data: Vec<Vec<_>> = builder.into();

        let mut index = Vec::new();
        if !data.is_empty() {
            // we exclude first row which contains a header
            let count_rows = data.len() - 1;
            index = build_range_index(count_rows);
        }

        Self {
            index,
            data,
            count_columns,
            name: None,
            print_index: true,
            transposed: false,
        }
    }
}

impl From<IndexBuilder> for Builder {
    fn from(b: IndexBuilder) -> Self {
        build_index(b)
    }
}

fn build_index(mut b: IndexBuilder) -> Builder {
    // we can skip the conversion if this builder has neither data rows nor header row
    if b.index.is_empty() && b.count_columns == 0 {
        return Builder::default();
    }

    // add index column
    if b.print_index {
        b.index.insert(0, CellInfo::default());
        insert_column(&mut b.data, b.index, 0);
    }

    if let Some(name) = b.name {
        if b.transposed && b.print_index {
            b.data[0][0] = name;
        } else {
            let count_columns = b.data[0].len();
            let mut name_row = vec![CellInfo::default(); count_columns];
            name_row[0] = name;

            b.data.insert(1, name_row);
        }
    }

    Builder::from_vec(b.data)
}

fn build_range_index(n: usize) -> Vec<CellInfo<String>> {
    (0..n).map(|i| i.to_string()).map(CellInfo::new).collect()
}

fn get_column<T>(v: &mut [Vec<T>], col: usize) -> Vec<T>
where
    T: Default,
{
    let mut column = Vec::with_capacity(v.len());
    for row in v.iter_mut() {
        let value = row.remove(col);
        column.push(value);
    }

    column
}

// todo: Seems like can be hugely simplified.
fn make_rows_columns<T>(v: &mut Vec<Vec<T>>, count_columns: usize)
where
    T: Default,
{
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

fn insert_column<T: Default>(v: &mut [Vec<T>], mut column: Vec<T>, col: usize) {
    for row in v.iter_mut() {
        let value = column.remove(col);
        row.insert(col, value);
    }
}
