use std::iter::FromIterator;

use crate::{grid::records::vec_records::CellInfo, Table};

use super::IndexBuilder;

/// Builder creates a [`Table`] from dynamic data set.
///
/// It useful when the amount of columns or rows is not known statically.
///
/// ```rust
/// use tabled::builder::Builder;
///
/// let mut builder = Builder::default();
/// builder.push_record(["index", "measure", "value"]);
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
    data: Vec<Vec<CellInfo<String>>>,
    /// A number of columns.
    count_columns: usize,
    /// A content of cells which are created in case rows has different length.
    empty_text: CellInfo<String>,
}

impl Builder {
    /// Creates a [`Builder`] instance.
    ///
    /// ```
    /// use tabled::builder::Builder;
    ///
    /// let builder = Builder::new();
    /// ```
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates a [`Builder`] instance with a given row capacity.
    ///
    /// ```
    /// use tabled::builder::Builder;
    ///
    /// let mut builder = Builder::with_capacity(2, 3);
    /// builder.push_record((0..3).map(|i| i.to_string()));
    /// builder.push_record(["i", "surname", "lastname"]);
    /// ```
    pub fn with_capacity(count_records: usize, count_columns: usize) -> Self {
        let mut builder = Self::new();
        builder.data = Vec::with_capacity(count_records);
        builder.count_columns = count_columns;

        builder
    }

    /// Creates a [`Builder`] instance.
    ///
    /// # Safety
    ///
    /// It's marked unsafe to emphasize that you shall make sure that all rows bound to have the same length.
    ///
    /// ```
    /// use tabled::builder::Builder;
    ///
    /// let data = vec![];
    /// let builder = Builder::from_vec(data);
    /// ```
    pub fn from_vec(data: Vec<Vec<CellInfo<String>>>) -> Self {
        let count_columns = if data.is_empty() { 0 } else { data[0].len() };

        Self {
            data,
            count_columns,
            empty_text: CellInfo::default(),
        }
    }

    /// Sets a content of cells which are created in case rows has different length.
    ///
    ///
    /// ```rust
    /// use tabled::builder::Builder;
    ///
    /// let mut builder = Builder::default();
    /// builder.set_empty("undefined");
    /// builder.push_record((0..3).map(|i| i.to_string()));
    /// builder.push_record(["i"]);
    /// ```
    pub fn set_empty<T>(&mut self, text: T)
    where
        T: Into<String>,
    {
        self.empty_text = CellInfo::new(text.into());
    }

    /// Build creates a [`Table`] instance.
    ///
    /// ```rust
    /// use tabled::builder::Builder;
    ///
    /// let mut builder = Builder::default();
    /// builder.push_record(["i", "column1", "column2"]);
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
    /// ```
    /// use tabled::builder::Builder;
    ///
    /// let mut builder = Builder::default();
    /// builder.push_record((0..3).map(|i| i.to_string()));
    /// builder.push_record(["i", "surname", "lastname"]);
    /// ```
    pub fn push_record<R>(&mut self, record: R)
    where
        R: IntoIterator,
        R::Item: Into<String>,
    {
        let list = create_row(record, self.count_columns, &self.empty_text);
        let list_length = list.len();

        if !is_size_eq(self.count_columns, list_length) {
            let size = list_length - self.count_columns;
            resize_rows(&mut self.data, size, &self.empty_text)
        }

        self.count_columns = list_length;
        self.data.push(list);
    }

    /// Insert a row into a specific position.
    ///
    /// # Panics
    ///
    /// Panics if `index > count_rows`.
    pub fn insert_record<R>(&mut self, index: usize, record: R)
    where
        R: IntoIterator,
        R::Item: Into<String>,
    {
        let list = create_row(record, self.count_columns, &self.empty_text);
        let list_length = list.len();

        if !is_size_eq(self.count_columns, list_length) {
            let size = list_length - self.count_columns;
            resize_rows(&mut self.data, size, &self.empty_text)
        }

        self.count_columns = list_length;
        self.data.insert(index, list);
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
        self.count_columns -= remove_empty_columns(&mut self.data, self.count_columns);
        remove_empty_rows(&mut self.data, self.count_columns);
    }

    /// Removes a row with a specific position.
    ///
    /// Index expected to be in range.
    /// `Builder::count_records() < x >= 0`
    ///
    /// # Panics
    ///
    /// Panics if `row_index > count_rows`.
    pub fn remove_record(&mut self, index: usize) {
        let _ = self.data.remove(index);
    }

    /// Removes a column with a specific position.
    ///
    /// Index expected to be in range.
    /// `Builder::count_columns() < x >= 0`
    ///
    /// # Panics
    ///
    /// Panics if `index > count_columns`.
    pub fn remove_column(&mut self, index: usize) {
        for row in &mut self.data {
            let _ = row.remove(index);
        }

        self.count_columns -= 1;
    }

    /// Push a column.
    pub fn push_column<I>(&mut self, column: I)
    where
        I: IntoIterator,
        I::Item: Into<String>,
    {
        let mut iter = column.into_iter();

        for row in self.data.iter_mut() {
            let text = iter
                .next()
                .map(Into::into)
                .map(CellInfo::new)
                .unwrap_or(self.empty_text.clone());

            row.push(text);
        }

        for text in iter {
            let text = CellInfo::new(text.into());

            let mut row = Vec::with_capacity(self.count_columns + 1);
            for _ in 0..self.count_columns {
                row.push(self.empty_text.clone());
            }

            row.push(text);
            self.data.push(row);
        }

        self.count_columns += 1;
    }

    /// Insert a column with a specific position.
    ///
    /// In case a column is bigger then the total amount of rows it will be truncated.
    ///
    /// # Panics
    ///
    /// Panics if `index > count_columns`.
    pub fn insert_column<I>(&mut self, index: usize, column: I)
    where
        I: IntoIterator,
        I::Item: Into<String>,
    {
        let mut iter = column.into_iter();

        for row in self.data.iter_mut() {
            let text = iter
                .next()
                .map(Into::into)
                .map(CellInfo::new)
                .unwrap_or(self.empty_text.clone());

            row.insert(index, text);
        }

        for text in iter {
            let text = CellInfo::new(text.into());

            let mut row = Vec::with_capacity(self.count_columns + 1);
            for _ in 0..index {
                row.push(self.empty_text.clone());
            }

            row.push(text);

            for _ in index..self.count_columns {
                row.push(self.empty_text.clone());
            }
        }

        self.count_columns += 1;
    }

    /// Remove all records.
    pub fn clear(&mut self) {
        self.data.clear();
        self.count_columns = 0;
    }

    /// Returns an amount of columns which would be present in a built table.
    pub fn count_columns(&self) -> usize {
        self.count_columns
    }

    /// Returns an amount of rows which would be present in a built table.
    ///
    /// Notice that it does not include header if present;
    /// It returns only amount of records.
    pub fn count_records(&self) -> usize {
        self.data.len()
    }
}

impl From<Builder> for Vec<Vec<String>> {
    fn from(builder: Builder) -> Self {
        builder
            .data
            .into_iter()
            .map(|row| row.into_iter().map(CellInfo::into_inner).collect())
            .collect()
    }
}

impl From<Builder> for Vec<Vec<CellInfo<String>>> {
    fn from(builder: Builder) -> Self {
        builder.data
    }
}

impl<R> FromIterator<R> for Builder
where
    R: IntoIterator,
    R::Item: Into<String>,
{
    fn from_iter<T: IntoIterator<Item = R>>(iter: T) -> Self {
        let mut builder = Self::new();
        for row in iter {
            builder.push_record(row);
        }

        builder
    }
}

impl<D> Extend<D> for Builder
where
    D: Into<String>,
{
    fn extend<T: IntoIterator<Item = D>>(&mut self, iter: T) {
        self.push_record(iter);
    }
}

impl From<Vec<Vec<String>>> for Builder {
    fn from(data: Vec<Vec<String>>) -> Self {
        let mut data = data
            .into_iter()
            .map(|row| row.into_iter().map(CellInfo::new).collect())
            .collect();

        let count_columns = equalize_row_length(&mut data);

        Self {
            data,
            count_columns,
            empty_text: CellInfo::default(),
        }
    }
}

impl From<Vec<Vec<CellInfo<String>>>> for Builder {
    fn from(mut data: Vec<Vec<CellInfo<String>>>) -> Self {
        let count_columns = equalize_row_length(&mut data);

        Self {
            data,
            count_columns,
            empty_text: CellInfo::default(),
        }
    }
}

fn create_row<R>(row: R, size: usize, default: &CellInfo<String>) -> Vec<CellInfo<String>>
where
    R: IntoIterator,
    R::Item: Into<String>,
{
    let mut list = Vec::with_capacity(size);
    for text in row {
        let text = text.into();
        let text = CellInfo::new(text);
        list.push(text);
    }

    if list.len() < size {
        for _ in 0..size - list.len() {
            let text = default.clone();
            list.push(text);
        }
    }

    list
}

fn remove_empty_columns(data: &mut [Vec<CellInfo<String>>], count_columns: usize) -> usize {
    let mut deleted = 0;
    for col in 0..count_columns {
        let col = col - deleted;

        let mut is_empty_column = true;
        for row in data.iter() {
            let text = &row[col];
            if !text.as_ref().is_empty() {
                is_empty_column = false;
                break;
            }
        }

        if is_empty_column {
            for row in data.iter_mut() {
                let _ = row.remove(col);
            }

            deleted += 1;
        }
    }

    deleted
}

fn remove_empty_rows(data: &mut Vec<Vec<CellInfo<String>>>, count_columns: usize) {
    let mut deleted = 0;

    for row in 0..data.len() {
        let row = row - deleted;

        let mut is_empty_row = true;
        for col in 0..count_columns {
            let cell = &data[row][col];
            if !cell.as_ref().is_empty() {
                is_empty_row = false;
                break;
            }
        }

        if is_empty_row {
            let _ = data.remove(row);
            deleted += 1;
        }
    }
}

fn resize_rows(data: &mut Vec<Vec<CellInfo<String>>>, size: usize, empty_text: &CellInfo<String>) {
    for row in data {
        append_vec(row, empty_text.clone(), size);
    }
}

fn append_vec<T>(v: &mut Vec<T>, value: T, n: usize)
where
    T: Clone,
{
    for _ in 0..n {
        v.push(value.clone());
    }
}

fn is_size_eq(expected: usize, new: usize) -> bool {
    use std::cmp::Ordering;

    match new.cmp(&expected) {
        Ordering::Less => {
            unreachable!("must be impossible due to the assumptions/checks we do");
        }
        Ordering::Greater => false,
        Ordering::Equal => true,
    }
}

fn equalize_row_length(data: &mut Vec<Vec<CellInfo<String>>>) -> usize {
    if data.is_empty() {
        return 0;
    }

    let first_row_length = data[0].len();
    let init = (first_row_length, true);
    let (count_columns, is_consistent) = data.iter().fold(init, |mut acc, cur| {
        let length = cur.len();
        acc.1 = acc.1 && acc.0 == length;
        acc.0 = std::cmp::max(acc.0, length);
        acc
    });

    if !is_consistent {
        let empty_text = CellInfo::default();
        for row in data {
            let size = count_columns - row.len();
            append_vec(row, empty_text.clone(), size);
        }
    }

    count_columns
}
