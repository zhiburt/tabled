//! Builder module provides a [Builder] type which helps building
//! a [Table] dynamically.

use std::{fmt::Display, iter::FromIterator};

use papergrid::{AlignmentHorizontal, Entity, Grid, Settings};

use crate::{Style, Table};

/// Builder creates a [Table] from dynamic data set.
///
/// It usefull when the amount of columns or rows is not known statically.
///
/// ```rust
/// use tabled::builder::Builder;
/// let table = Builder::default()
///     .header(["index", "measure", "value"])
///     .add_row(["0", "weight", "0.443"])
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
    /// A header row.
    headers: Option<Vec<String>>,
    /// A list of rows.
    rows: Vec<Vec<String>>,
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
    ///     .header(0..3)
    ///     .add_row(["i", "surname", "lastname"]);
    /// ```
    pub fn header<H, T>(mut self, header: H) -> Self
    where
        H: IntoIterator<Item = T>,
        T: Display,
    {
        let header: Vec<String> = header.into_iter().map(|t| t.to_string()).collect();
        self.update_size(header.len());
        self.headers = Some(header);

        self
    }

    /// Adds a row to a [Table].
    ///
    /// If [Self::header] is not set the row will be considered a header.
    ///
    /// ```rust
    /// use tabled::builder::Builder;
    /// let builder = Builder::default()
    ///     .add_row(0..3)
    ///     .add_row(["i", "surname", "lastname"]);
    /// ```
    pub fn add_row<R, T>(mut self, row: R) -> Self
    where
        R: IntoIterator<Item = T>,
        T: Display,
    {
        let row: Vec<String> = row.into_iter().map(|t| t.to_string()).collect();
        self.update_size(row.len());
        self.rows.push(row);

        self
    }

    /// Sets a content of cells which are created in case rows has different length.
    ///
    ///
    /// ```rust
    /// use tabled::builder::Builder;
    /// let table = Builder::default()
    ///     .set_default_text("undefined")
    ///     .header(0..3)
    ///     .add_row(["i"])
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
    ///     .header(["i", "column1", "column2"])
    ///     .add_row(["0", "value1", "value2"])
    ///     .build();
    /// ```
    pub fn build(mut self) -> Table {
        if let Some(empty_cell_text) = self.empty_cell_text {
            if let Some(header) = self.headers.as_mut() {
                if self.size > header.len() {
                    append_vec(header, self.size - header.len(), empty_cell_text.clone());
                }
            }

            for row in self.rows.iter_mut() {
                if self.size > row.len() {
                    append_vec(row, self.size - row.len(), empty_cell_text.clone());
                }
            }
        }

        build_table(self.headers, self.rows, self.size)
    }

    fn update_size(&mut self, size: usize) {
        if size > self.size {
            self.size = size;
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
            builder = builder.add_row(row);
        }

        builder
    }
}

/// Building [Table] from ordinary data.
fn build_table(header: Option<Vec<String>>, rows: Vec<Vec<String>>, count_columns: usize) -> Table {
    let grid = build_grid(header, rows, count_columns);
    create_table_from_grid(grid)
}

/// Building [Grid] from ordinary data.
fn build_grid(header: Option<Vec<String>>, rows: Vec<Vec<String>>, count_columns: usize) -> Grid {
    let mut count_rows = rows.len();

    if header.is_some() {
        count_rows += 1;
    }

    let mut grid = Grid::new(count_rows, count_columns);

    let mut row = 0;
    if let Some(headers) = header {
        for (i, text) in headers.into_iter().enumerate() {
            grid.set(&Entity::Cell(0, i), Settings::new().text(text));
        }

        row = 1;
    }

    for fields in rows.into_iter() {
        // don't show off a empty data array
        if fields.is_empty() {
            continue;
        }

        for (column, field) in fields.into_iter().enumerate() {
            grid.set(&Entity::Cell(row, column), Settings::new().text(field));
        }

        row += 1;
    }

    grid
}

fn create_table_from_grid(mut grid: Grid) -> Table {
    // it's crusial to set a global setting rather than a setting for an each cell
    // as it will be hard to override that since how Grid::style method works
    grid.set(
        &Entity::Global,
        Settings::new()
            .indent(1, 1, 0, 0)
            .alignment(AlignmentHorizontal::Center),
    );

    let table = Table { grid };
    table.with(Style::ASCII)
}

fn append_vec(v: &mut Vec<String>, n: usize, value: String) {
    v.extend((0..n).map(|_| value.clone()));
}
