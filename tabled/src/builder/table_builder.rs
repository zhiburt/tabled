// todo: I'd rather switch back to -> Self from -> &mut Self
//
// but these &self methods are definetely a smell...

use std::{borrow::Cow, iter::FromIterator};

use crate::Table;

use super::IndexBuilder;

/// Builder creates a [`Table`] from dynamic data set.
///
/// It useful when the amount of columns or rows is not known statically.
///
/// ```rust
/// use tabled::builder::Builder;
///
/// let mut builder = Builder::default().set_header(["index", "measure", "value"]);
/// builder.push_record(["0", "weight", "0.443"]);
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

    /// Creates a [`Builder`] instance.
    pub fn with_capacity(capacity: usize) -> Self {
        let mut b = Self::new();
        b.records = Vec::with_capacity(capacity);

        b
    }

    /// Sets a [`Table`] header.
    ///
    /// ```rust
    /// # use tabled::builder::Builder;
    /// let mut builder = Builder::default().set_header((0..3).map(|i| i.to_string()));
    /// ```
    pub fn set_header<H, T>(mut self, columns: H) -> Self
    where
        H: IntoIterator<Item = T>,
        T: Into<Cow<'static, str>>,
    {
        let list = create_row(columns, self.count_columns);

        self.update_size(list.len());
        self.columns = Some(list);

        self
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
    /// let mut builder = Table::builder(data).remove_header();
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
    pub fn remove_header(mut self) -> Self {
        self.columns = None;
        self.count_columns = self.get_size();

        self
    }

    /// Sets a content of cells which are created in case rows has different length.
    ///
    ///
    /// ```rust
    /// use tabled::builder::Builder;
    ///
    /// let mut builder = Builder::default()
    ///     .set_default_text("undefined")
    ///     .set_header((0..3).map(|i| i.to_string()));
    /// builder.push_record(["i"]);
    /// ```
    pub fn set_default_text<T>(mut self, text: T) -> Self
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
    /// let mut builder = Builder::default().set_header(["i", "column1", "column2"]);
    /// builder.push_record(["0", "value1", "value2"]);
    /// ```
    pub fn build(self) -> Table {
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
        IndexBuilder::from(self)
    }

    /// Adds a row to a [`Table`].
    ///
    /// ```rust
    /// use tabled::builder::Builder;
    ///
    /// let mut builder = Builder::default();
    /// builder.push_record((0..3).map(|i| i.to_string()));
    /// builder.push_record(["i", "surname", "lastname"]);
    /// ```
    pub fn push_record<R, T>(&mut self, row: R)
    where
        R: IntoIterator<Item = T>,
        T: Into<Cow<'static, str>>,
    {
        let list = create_row(row, self.count_columns);

        self.update_size(list.len());
        self.records.push(list);
    }

    /// Insert a row into a specific position.
    ///
    /// # Panics
    ///
    /// Panics if `index > count_rows`.
    pub fn insert_record<R>(&mut self, index: usize, record: R) -> bool
    where
        R: IntoIterator,
        R::Item: Into<Cow<'static, str>>,
    {
        let list = create_row(record, self.count_columns);

        self.update_size(list.len());
        self.records.insert(index, list);

        true
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
    pub fn clean(&mut self) {
        self.clean_columns();
        self.clean_rows();
    }

    /// Set a column size.
    ///
    /// If it make it lower then it was originally it is considered NOP.
    pub fn hint_column_size(&mut self, size: usize) {
        self.count_columns = size;
        self.is_consistent = true;
    }

    /// Returns an amount of columns which would be present in a built table.
    pub fn count_columns(&self) -> usize {
        self.count_columns
    }

    /// Returns an amount of rows which would be present in a built table.
    pub fn count_rows(&self) -> usize {
        self.records.len()
    }

    /// Checks whether a builder contains a header set.
    pub fn has_header(&self) -> bool {
        self.columns.is_some()
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
                let count = self.count_columns - header.len();
                append_vec(header, empty_cell.clone(), count);
            }
        }

        for row in &mut self.records {
            if self.count_columns > row.len() {
                let count = self.count_columns - row.len();
                append_vec(row, empty_cell.clone(), count);
            }
        }
    }
}

impl From<Builder> for Vec<Vec<Cow<'static, str>>> {
    fn from(mut builder: Builder) -> Self {
        if !builder.is_consistent {
            builder.fix_rows();
        }

        append_header(&mut builder.records, builder.columns);

        builder.records
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
            builder.push_record(row);
        }

        builder
    }
}

impl<D> Extend<D> for Builder
where
    D: Into<Cow<'static, str>>,
{
    fn extend<T: IntoIterator<Item = D>>(&mut self, iter: T) {
        self.push_record(iter);
    }
}

impl From<Vec<Vec<Cow<'static, str>>>> for Builder {
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

fn append_vec<T: Clone>(v: &mut Vec<T>, value: T, n: usize) {
    v.extend((0..n).map(|_| value.clone()));
}
