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
/// builder.set_header(["index", "measure", "value"]);
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
    /// A columns row.
    columns: Option<Vec<CellInfo<String>>>,
    /// A number of columns.
    count_columns: usize,
    /// A flag that the rows are not consistent.
    is_consistent: bool,
    /// A content of cells which are created in case rows has different length.
    empty_cell_text: Option<String>,
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
    /// let mut builder = Builder::with_capacity(2);
    /// builder.push_record((0..3).map(|i| i.to_string()));
    /// builder.push_record(["i", "surname", "lastname"]);
    /// ```
    pub fn with_capacity(capacity: usize) -> Self {
        let mut b = Self::new();
        b.data = Vec::with_capacity(capacity);

        b
    }

    /// Sets a [`Table`] header.
    ///
    /// ```
    /// # use tabled::builder::Builder;
    /// let mut builder = Builder::default();
    /// builder.set_header((0..3).map(|i| i.to_string()));
    /// ```
    pub fn set_header<H, T>(&mut self, columns: H) -> &mut Self
    where
        H: IntoIterator<Item = T>,
        T: Into<String>,
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
    /// let mut builder = Table::builder(data);
    /// builder.remove_header();
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
        self.count_columns = self.get_size();

        self
    }

    /// Sets a content of cells which are created in case rows has different length.
    ///
    ///
    /// ```rust
    /// use tabled::builder::Builder;
    ///
    /// let mut builder = Builder::default();
    /// builder
    ///     .set_default_text("undefined")
    ///     .set_header((0..3).map(|i| i.to_string()))
    ///     .push_record(["i"]);
    /// ```
    pub fn set_default_text<T>(&mut self, text: T) -> &mut Self
    where
        T: Into<String>,
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
    /// builder.set_header(["i", "column1", "column2"]);
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
    pub fn push_record<R, T>(&mut self, row: R) -> &mut Self
    where
        R: IntoIterator<Item = T>,
        T: Into<String>,
    {
        let list = create_row(row, self.count_columns);

        self.update_size(list.len());
        self.data.push(list);

        self
    }

    /// Insert a row into a specific position.
    ///
    /// # Panics
    ///
    /// Panics if `index > count_rows`.
    pub fn insert_record<R>(&mut self, index: usize, record: R) -> bool
    where
        R: IntoIterator,
        R::Item: Into<String>,
    {
        let list = create_row(record, self.count_columns);

        self.update_size(list.len());
        self.data.insert(index, list);

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
    pub fn clean(&mut self) -> &mut Self {
        self.clean_columns();
        self.clean_rows();
        self
    }

    /// Set a column size.
    ///
    /// If it make it lower then it was originally it is considered NOP.
    pub fn hint_column_size(&mut self, size: usize) -> &mut Self {
        self.count_columns = size;
        self.is_consistent = true;
        self
    }

    /// Returns an amount of columns which would be present in a built table.
    pub fn count_columns(&self) -> usize {
        self.count_columns
    }

    /// Returns an amount of rows which would be present in a built table.
    pub fn count_rows(&self) -> usize {
        self.data.len()
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
            for row in 0..self.data.len() {
                let cell = &self.data[row][col];
                if !cell.as_ref().is_empty() {
                    is_empty = false;
                    break;
                }
            }

            if is_empty {
                for row in 0..self.data.len() {
                    let _ = self.data[row].remove(col);
                }

                if let Some(columns) = self.columns.as_mut() {
                    if columns.len() > col {
                        let _ = columns.remove(col);
                    }
                }

                i += 1;
            }
        }

        self.count_columns -= i;
    }

    fn clean_rows(&mut self) {
        for row in (0..self.data.len()).rev() {
            let mut is_empty = true;
            for col in 0..self.count_columns {
                let cell = &self.data[row][col];
                if !cell.as_ref().is_empty() {
                    is_empty = false;
                    break;
                }
            }

            if is_empty {
                let _ = self.data.remove(row);
            }

            if row == 0 {
                break;
            }
        }
    }

    fn update_size(&mut self, size: usize) {
        use std::cmp::Ordering;

        match size.cmp(&self.count_columns) {
            Ordering::Less => {
                if !self.data.is_empty() {
                    self.is_consistent = false;
                }
            }
            Ordering::Greater => {
                self.count_columns = size;

                if !self.data.is_empty() || self.columns.is_some() {
                    self.is_consistent = false;
                }
            }
            Ordering::Equal => (),
        }
    }

    fn get_size(&mut self) -> usize {
        let mut max = self.columns.as_ref().map_or(0, Vec::len);
        let max_records = self.data.iter().map(Vec::len).max().unwrap_or(0);
        max = std::cmp::max(max_records, max);

        max
    }

    fn fix_rows(&mut self) {
        let empty_cell = self.empty_cell_text.to_owned().unwrap_or_default();
        let empty = CellInfo::new(empty_cell);

        if let Some(header) = self.columns.as_mut() {
            if self.count_columns > header.len() {
                let count = self.count_columns - header.len();
                append_vec(header, empty.clone(), count);
            }
        }

        for row in &mut self.data {
            if self.count_columns > row.len() {
                let count = self.count_columns - row.len();
                append_vec(row, empty.clone(), count);
            }
        }
    }
}

impl From<Builder> for Vec<Vec<String>> {
    fn from(mut builder: Builder) -> Self {
        if !builder.is_consistent {
            builder.fix_rows();
        }

        if let Some(columns) = builder.columns {
            builder.data.insert(0, columns);
        }

        builder
            .data
            .into_iter()
            .map(|row| row.into_iter().map(|c| c.into_inner()).collect())
            .collect()
    }
}

impl From<Builder> for Vec<Vec<CellInfo<String>>> {
    fn from(mut builder: Builder) -> Self {
        if !builder.is_consistent {
            builder.fix_rows();
        }

        if let Some(columns) = builder.columns {
            builder.data.insert(0, columns);
        }

        builder.data
    }
}

impl<R, V> FromIterator<R> for Builder
where
    R: IntoIterator<Item = V>,
    V: Into<String>,
{
    fn from_iter<T: IntoIterator<Item = R>>(iter: T) -> Self {
        let mut builder = Self::default();
        for row in iter {
            let _ = builder.push_record(row);
        }

        builder
    }
}

impl<D> Extend<D> for Builder
where
    D: Into<String>,
{
    fn extend<T: IntoIterator<Item = D>>(&mut self, iter: T) {
        let _ = self.push_record(iter);
    }
}

impl From<Vec<Vec<String>>> for Builder {
    fn from(data: Vec<Vec<String>>) -> Self {
        let count_columns = data.get(0).map_or(0, |row| row.len());

        let data = data
            .into_iter()
            .map(|row| row.into_iter().map(CellInfo::new).collect())
            .collect();

        Self {
            data,
            count_columns,
            columns: None,
            is_consistent: false,
            empty_cell_text: None,
        }
    }
}

impl From<Vec<Vec<CellInfo<String>>>> for Builder {
    fn from(data: Vec<Vec<CellInfo<String>>>) -> Self {
        let count_columns = data.get(0).map_or(0, |row| row.len());

        Self {
            data,
            count_columns,
            columns: None,
            is_consistent: false,
            empty_cell_text: None,
        }
    }
}

fn create_row<R, T>(row: R, size: usize) -> Vec<CellInfo<String>>
where
    R: IntoIterator<Item = T>,
    T: Into<String>,
{
    let mut list = Vec::with_capacity(size);
    for text in row {
        let text = text.into();
        let info = CellInfo::new(text);
        list.push(info);
    }

    list
}

fn append_vec<T: Clone>(v: &mut Vec<T>, value: T, n: usize) {
    v.extend((0..n).map(|_| value.clone()));
}
