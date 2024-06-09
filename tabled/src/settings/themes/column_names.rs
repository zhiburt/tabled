use std::cmp;

use crate::{
    grid::{
        config::{AlignmentHorizontal, AlignmentVertical, ColoredConfig, Position},
        dimension::{CompleteDimensionVecRecords, Dimension, Estimate},
        records::{
            vec_records::{CellInfo, VecRecords},
            ExactRecords, PeekableRecords, Records, Resizable,
        },
        util::string::string_width,
    },
    settings::{
        object::{Column, Row},
        style::{LineText, Offset},
        Alignment, Color, TableOption,
    },
};

/// [`ColumnNames`] sets strings on horizontal lines for the columns.
///
/// Notice that using a [`Default`] would reuse a names from the first row.
///
/// # Examples
///
/// ```
/// use std::iter::FromIterator;
/// use tabled::{
///     Table,
///     settings::{themes::ColumnNames, Alignment},
/// };
///
/// let data = vec![
///     vec!["Hello", "World"],
///     vec!["Hello", "World"],
/// ];
///
/// let mut table = Table::from_iter(data);
/// table.with(
///     ColumnNames::new(["head1", "head2"])
///         .line(2)
///         .alignment(Alignment::right())
/// );
///
/// assert_eq!(
///     table.to_string(),
///     "+-------+-------+\n\
///      | Hello | World |\n\
///      +-------+-------+\n\
///      | Hello | World |\n\
///      +--head1+--head2+"
/// );
/// ```
///
/// [`Default`] usage.
///
/// ```
/// use std::iter::FromIterator;
/// use tabled::{Table, settings::themes::ColumnNames};
///
/// let data = vec![
///     vec!["Hello", "World"],
///     vec!["Hello", "World"],
/// ];
///
/// let mut table = Table::from_iter(data);
/// table.with(ColumnNames::default());
///
/// assert_eq!(
///     table.to_string(),
///     "+Hello--+World--+\n\
///      | Hello | World |\n\
///      +-------+-------+"
/// );
/// ```
#[derive(Debug, Clone)]
pub struct ColumnNames {
    names: Option<Vec<String>>,
    colors: Option<ListValue<Color>>,
    alignments: ListValue<Alignment>,
    line: usize,
}

impl Default for ColumnNames {
    fn default() -> Self {
        Self {
            names: Default::default(),
            colors: Default::default(),
            line: Default::default(),
            alignments: ListValue::Static(Alignment::left()),
        }
    }
}

impl ColumnNames {
    /// Creates a [`ColumnNames`] with a given names.
    ///
    /// Using a [`Default`] would reuse a names from the first row.
    ///
    /// # Example
    ///
    /// ```
    /// use std::iter::FromIterator;
    /// use tabled::{Table, settings::themes::ColumnNames};
    ///
    /// let mut table = Table::from_iter(vec![vec!["Hello", "World"]]);
    /// table.with(ColumnNames::new(["head1", "head2"]));
    ///
    /// assert_eq!(
    ///     table.to_string(),
    ///     "+head1--+head2--+\n\
    ///      | Hello | World |\n\
    ///      +-------+-------+"
    /// );
    /// ```
    pub fn new<I>(names: I) -> Self
    where
        I: IntoIterator,
        I::Item: Into<String>,
    {
        let names = names.into_iter().map(Into::into).collect::<Vec<_>>();
        Self {
            names: Some(names),
            ..Default::default()
        }
    }

    /// Set color for the column names.
    ///
    /// By default there's no colors.
    ///
    /// # Example
    ///
    /// ```
    /// use std::iter::FromIterator;
    /// use tabled::Table;
    /// use tabled::settings::{Color, themes::ColumnNames};
    ///
    /// let mut table = Table::from_iter(vec![vec!["Hello", "World"]]);
    /// table.with(ColumnNames::new(["head1", "head2"]).color(vec![Color::FG_RED]));
    ///
    /// assert_eq!(
    ///     table.to_string(),
    ///     "+\u{1b}[31mh\u{1b}[39m\u{1b}[31me\u{1b}[39m\u{1b}[31ma\u{1b}[39m\u{1b}[31md\u{1b}[39m\u{1b}[31m1\u{1b}[39m--+head2--+\n\
    ///      | Hello | World |\n\
    ///      +-------+-------+"
    /// );
    /// ```
    pub fn color<T>(self, color: T) -> Self
    where
        T: Into<ListValue<Color>>,
    {
        Self {
            names: self.names,
            line: self.line,
            alignments: self.alignments,
            colors: Some(color.into()),
        }
    }

    /// Set a horizontal line the names will be applied to.
    ///
    /// The default value is 0 (the top horizontal line).
    ///
    /// # Example
    ///
    /// ```
    /// use std::iter::FromIterator;
    /// use tabled::{Table, settings::themes::ColumnNames};
    ///
    /// let mut table = Table::from_iter(vec![vec!["Hello", "World"]]);
    /// table.with(ColumnNames::new(["head1", "head2"]).line(1));
    ///
    /// assert_eq!(
    ///     table.to_string(),
    ///     "+-------+-------+\n\
    ///      | Hello | World |\n\
    ///      +head1--+head2--+"
    /// );
    /// ```
    pub fn line(self, i: usize) -> Self {
        Self {
            names: self.names,
            line: i,
            alignments: self.alignments,
            colors: self.colors,
        }
    }

    /// Set an alignment for the names.
    ///
    /// By default it's left aligned.
    ///
    /// # Example
    ///
    /// ```
    /// use std::iter::FromIterator;
    /// use tabled::{
    ///     Table,
    ///     settings::{themes::ColumnNames, Alignment},
    /// };
    ///
    /// let mut table = Table::from_iter(vec![vec!["Hello", "World"]]);
    /// table.with(ColumnNames::new(["head1", "head2"]).alignment(Alignment::right()));
    ///
    /// assert_eq!(
    ///     table.to_string(),
    ///     "+--head1+--head2+\n\
    ///      | Hello | World |\n\
    ///      +-------+-------+"
    /// );
    /// ```
    pub fn alignment<T>(self, alignment: T) -> Self
    where
        T: Into<ListValue<Alignment>>,
    {
        Self {
            names: self.names,
            line: self.line,
            alignments: alignment.into(),
            colors: self.colors,
        }
    }
}

impl TableOption<VecRecords<CellInfo<String>>, ColoredConfig, CompleteDimensionVecRecords<'_>>
    for ColumnNames
{
    fn change(
        self,
        records: &mut VecRecords<CellInfo<String>>,
        cfg: &mut ColoredConfig,
        dims: &mut CompleteDimensionVecRecords<'_>,
    ) {
        let count_rows = records.count_rows();
        let count_columns = records.count_columns();

        if count_columns == 0 || count_rows == 0 || self.line > count_rows {
            return;
        }

        let alignment_horizontal = convert_alignment_value(self.alignments.clone());
        let alignment_vertical = convert_alignment_value(self.alignments.clone());

        if let Some(alignment) = alignment_horizontal {
            let names = get_column_names(records, self.names);
            let names = vec_set_size(names, records.count_columns());
            set_column_text(names, self.line, alignment, self.colors, records, dims, cfg);
            return;
        }

        if let Some(alignment) = alignment_vertical {
            let names = get_column_names(records, self.names);
            let names = vec_set_size(names, records.count_rows());
            set_row_text(names, self.line, alignment, self.colors, records, dims, cfg);
            return;
        }

        let names = get_column_names(records, self.names);
        let names = vec_set_size(names, records.count_columns());
        let alignment = ListValue::Static(AlignmentHorizontal::Left);
        set_column_text(names, self.line, alignment, self.colors, records, dims, cfg);
    }
}

fn set_column_text(
    names: Vec<String>,
    target_line: usize,
    alignments: ListValue<AlignmentHorizontal>,
    colors: Option<ListValue<Color>>,
    records: &mut VecRecords<CellInfo<String>>,
    dims: &mut CompleteDimensionVecRecords<'_>,
    cfg: &mut ColoredConfig,
) {
    dims.estimate(&*records, cfg);

    let count_columns = names.len();
    let widths = names
        .iter()
        .enumerate()
        .map(|(col, name)| (cmp::max(string_width(name), dims.get_width(col))))
        .collect::<Vec<_>>();

    dims.set_widths(widths.clone());

    let mut total_width = 0;
    for (column, (width, name)) in widths.into_iter().zip(names).enumerate() {
        let color = get_color(&colors, column);
        let alignment = alignments.get(column).unwrap_or(AlignmentHorizontal::Left);
        let left_vertical = get_vertical_width(cfg, (target_line, column), count_columns);
        let grid_offset =
            total_width + left_vertical + get_horizontal_indent(&name, alignment, width);
        let line = Row::from(target_line);

        let linetext = create_line_text(&name, grid_offset, color, line);
        linetext.change(records, cfg, dims);

        total_width += width + left_vertical;
    }
}

fn set_row_text(
    names: Vec<String>,
    target_line: usize,
    alignments: ListValue<AlignmentVertical>,
    colors: Option<ListValue<Color>>,
    records: &mut VecRecords<CellInfo<String>>,
    dims: &mut CompleteDimensionVecRecords<'_>,
    cfg: &mut ColoredConfig,
) {
    dims.estimate(&*records, cfg);

    let count_rows = names.len();
    let heights = names
        .iter()
        .enumerate()
        .map(|(row, name)| (cmp::max(string_width(name), dims.get_height(row))))
        .collect::<Vec<_>>();

    dims.set_heights(heights.clone());

    let mut total_height = 0;
    for (row, (row_height, name)) in heights.into_iter().zip(names).enumerate() {
        let color = get_color(&colors, row);
        let alignment = alignments.get(row).unwrap_or(AlignmentVertical::Top);
        let top_horizontal = get_horizontal_width(cfg, (row, target_line), count_rows);
        let cell_indent = get_vertical_indent(&name, alignment, row_height);
        let grid_offset = total_height + top_horizontal + cell_indent;
        let line = Column::from(target_line);

        let linetext = create_line_text(&name, grid_offset, color, line);
        linetext.change(records, cfg, dims);

        total_height += row_height + top_horizontal;
    }
}

fn get_column_names(
    records: &mut VecRecords<CellInfo<String>>,
    opt: Option<Vec<String>>,
) -> Vec<String> {
    match opt {
        Some(names) => names
            .into_iter()
            .map(|name| name.lines().next().unwrap_or("").to_string())
            .collect::<Vec<_>>(),
        None => collect_head(records),
    }
}

fn vec_set_size(mut data: Vec<String>, size: usize) -> Vec<String> {
    match data.len().cmp(&size) {
        cmp::Ordering::Equal => {}
        cmp::Ordering::Less => {
            let additional_size = size - data.len();
            data.extend(std::iter::repeat(String::new()).take(additional_size));
        }
        cmp::Ordering::Greater => {
            data.truncate(size);
        }
    }

    data
}

fn collect_head(records: &mut VecRecords<CellInfo<String>>) -> Vec<String> {
    if records.count_rows() == 0 || records.count_columns() == 0 {
        return Vec::new();
    }

    let names = (0..records.count_columns())
        .map(|column| records.get_line((0, column), 0))
        .map(ToString::to_string)
        .collect();

    records.remove_row(0);

    names
}

fn create_line_text<T>(text: &str, offset: usize, color: Option<&Color>, line: T) -> LineText<T> {
    let offset = Offset::Begin(offset);
    let mut btext = LineText::new(text, line).offset(offset);
    if let Some(color) = color {
        btext = btext.color(color.clone());
    }

    btext
}

fn get_color(colors: &Option<ListValue<Color>>, i: usize) -> Option<&Color> {
    match colors {
        Some(ListValue::List(list)) => list.get(i),
        Some(ListValue::Static(color)) => Some(color),
        None => None,
    }
}

fn get_horizontal_indent(text: &str, align: AlignmentHorizontal, available: usize) -> usize {
    match align {
        AlignmentHorizontal::Left => 0,
        AlignmentHorizontal::Right => available - string_width(text),
        AlignmentHorizontal::Center => (available - string_width(text)) / 2,
    }
}

fn get_vertical_indent(text: &str, align: AlignmentVertical, available: usize) -> usize {
    match align {
        AlignmentVertical::Top => 0,
        AlignmentVertical::Bottom => available - string_width(text),
        AlignmentVertical::Center => (available - string_width(text)) / 2,
    }
}

fn get_vertical_width(cfg: &mut ColoredConfig, pos: Position, count_columns: usize) -> usize {
    cfg.get_vertical(pos, count_columns)
        .and_then(unicode_width::UnicodeWidthChar::width)
        .unwrap_or(0)
}

fn get_horizontal_width(cfg: &mut ColoredConfig, pos: Position, count_rows: usize) -> usize {
    cfg.get_horizontal(pos, count_rows)
        .and_then(unicode_width::UnicodeWidthChar::width)
        .unwrap_or(0)
}

fn convert_alignment_value<T>(value: ListValue<Alignment>) -> Option<ListValue<T>>
where
    Option<T>: From<Alignment>,
{
    match value {
        ListValue::List(list) => {
            let new = list
                .iter()
                .flat_map(|value| Option::from(*value))
                .collect::<Vec<_>>();
            if new.len() == list.len() {
                Some(ListValue::List(new))
            } else {
                None
            }
        }
        ListValue::Static(value) => Option::from(value).map(ListValue::Static),
    }
}

#[derive(Debug, Clone)]
pub enum ListValue<T> {
    List(Vec<T>),
    Static(T),
}

impl<T> ListValue<T> {
    fn get(&self, i: usize) -> Option<T>
    where
        T: Copy,
    {
        match self {
            ListValue::List(list) => list.get(i).copied(),
            ListValue::Static(alignment) => Some(*alignment),
        }
    }
}

impl<T> From<T> for ListValue<T> {
    fn from(value: T) -> Self {
        Self::Static(value)
    }
}

impl<T> From<Vec<T>> for ListValue<T> {
    fn from(value: Vec<T>) -> Self {
        Self::List(value)
    }
}

impl<T> Default for ListValue<T>
where
    T: Default,
{
    fn default() -> Self {
        Self::Static(T::default())
    }
}
