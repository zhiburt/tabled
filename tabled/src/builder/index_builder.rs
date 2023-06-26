use crate::Table;

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
/// builder.set_header(["i", "col-1", "col-2"]);
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
    index: Vec<String>,
    /// Name of an index
    name: Option<String>,
    /// A flag which checks if we need to actually use index.
    ///
    /// It might happen when it's only necessary to [`Self::transpose`] table.
    print_index: bool,
    /// A flag which checks if table was transposed.
    transposed: bool,
    /// Data originated in [`Builder`].
    data: Vec<Vec<String>>,
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
    /// builder.set_header(["i", "col-1", "col-2"]);
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
    /// builder.set_header(["i", "column1", "column2"]);
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
    /// builder.set_header(["i", "column1", "column2"]);
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
        if column >= matrix_count_columns(&self.data) {
            return self;
        }

        self.index = get_column(&mut self.data, column);

        let name = remove_or_default(&mut self.index, 0);
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
    /// builder.set_header(["i", "column-1", "column-2", "column-3"]);
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
        let columns = &mut self.data[0];
        std::mem::swap(&mut self.index, columns);

        let columns = self.data.remove(0);

        make_rows_columns(&mut self.data);

        self.data.insert(0, columns);

        self.transposed = !self.transposed;

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
        let has_header = builder.has_header();

        let mut data: Vec<Vec<_>> = builder.into();

        if !has_header {
            let count_columns = matrix_count_columns(&data);
            data.insert(0, build_range_index(count_columns));
        }

        // we exclude first row which contains a header
        let data_len = data.len().saturating_sub(1);
        let index = build_range_index(data_len);

        Self {
            index,
            name: None,
            print_index: true,
            transposed: false,
            data,
        }
    }
}

impl From<IndexBuilder> for Builder {
    fn from(b: IndexBuilder) -> Self {
        build_index(b)
    }
}

fn build_index(mut b: IndexBuilder) -> Builder {
    if b.index.is_empty() {
        return Builder::default();
    }

    // add index column
    if b.print_index {
        b.index.insert(0, String::default());

        insert_column(&mut b.data, b.index, 0);
    }

    if let Some(name) = b.name {
        if b.transposed && b.print_index {
            b.data[0][0] = name;
        } else {
            b.data.insert(1, vec![name]);
        }
    }

    Builder::from(b.data)
}

fn build_range_index(n: usize) -> Vec<String> {
    (0..n).map(|i| i.to_string()).collect()
}

fn remove_or_default<T: Default>(v: &mut Vec<T>, i: usize) -> T {
    if v.len() > i {
        v.remove(i)
    } else {
        T::default()
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

fn make_rows_columns<T: Default>(v: &mut Vec<Vec<T>>) {
    let count_columns = matrix_count_columns(v);

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
        let value = remove_or_default(&mut column, col);
        row.insert(col, value);
    }
}

fn matrix_count_columns<T>(v: &[Vec<T>]) -> usize {
    v.first().map_or(0, |row| row.len())
}
