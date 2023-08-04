use std::cmp;

use crate::{
    grid::{
        config::{AlignmentHorizontal, ColoredConfig, Position},
        dimension::{CompleteDimensionVecRecords, Dimension, Estimate},
        records::{
            vec_records::{CellInfo, VecRecords},
            ExactRecords, PeekableRecords, Records, Resizable,
        },
        util::string::string_width,
    },
    settings::{
        style::{BorderText, Offset},
        Color, TableOption,
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
/// use tabled::{Table, settings::themes::ColumnNames, grid::config::AlignmentHorizontal};
///
/// let data = vec![
///     vec!["Hello", "World"],
///     vec!["Hello", "World"],
/// ];
///
/// let mut table = Table::from_iter(data);
/// table.with(ColumnNames::new(["head1", "head2"]).set_line(2).set_alignment(AlignmentHorizontal::Right));
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
    alignments: ListValue<AlignmentHorizontal>,
    line: usize,
}

impl Default for ColumnNames {
    fn default() -> Self {
        Self {
            names: Default::default(),
            colors: Default::default(),
            line: Default::default(),
            alignments: ListValue::from(AlignmentHorizontal::Left),
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
    /// table.with(ColumnNames::new(["head1", "head2"]).set_color(vec![Color::FG_RED]));
    ///
    /// assert_eq!(
    ///     table.to_string(),
    ///     "+\u{1b}[31mh\u{1b}[39m\u{1b}[31me\u{1b}[39m\u{1b}[31ma\u{1b}[39m\u{1b}[31md\u{1b}[39m\u{1b}[31m1\u{1b}[39m--+head2--+\n\
    ///      | Hello | World |\n\
    ///      +-------+-------+"
    /// );
    /// ```
    pub fn set_color<T>(self, color: T) -> Self
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
    /// table.with(ColumnNames::new(["head1", "head2"]).set_line(1));
    ///
    /// assert_eq!(
    ///     table.to_string(),
    ///     "+-------+-------+\n\
    ///      | Hello | World |\n\
    ///      +head1--+head2--+"
    /// );
    /// ```
    pub fn set_line(self, i: usize) -> Self {
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
    ///     settings::themes::ColumnNames,
    ///     grid::config::AlignmentHorizontal,
    /// };
    ///
    /// let mut table = Table::from_iter(vec![vec!["Hello", "World"]]);
    /// table.with(ColumnNames::new(["head1", "head2"]).set_alignment(AlignmentHorizontal::Right));
    ///
    /// assert_eq!(
    ///     table.to_string(),
    ///     "+--head1+--head2+\n\
    ///      | Hello | World |\n\
    ///      +-------+-------+"
    /// );
    /// ```
    pub fn set_alignment<T>(self, alignment: T) -> Self
    where
        T: Into<ListValue<AlignmentHorizontal>>,
    {
        Self {
            names: self.names,
            line: self.line,
            alignments: alignment.into(),
            colors: self.colors,
        }
    }
}

impl TableOption<VecRecords<CellInfo<String>>, CompleteDimensionVecRecords<'_>, ColoredConfig>
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

        let names = match self.names {
            Some(names) => names,
            None => collect_head(records),
        };

        let names = names
            .iter()
            .take(count_columns)
            .map(|name| name.lines().next().unwrap_or(""))
            .collect::<Vec<_>>();

        dims.estimate(&*records, cfg);

        let widths = (0..count_columns)
            .map(|column| {
                let text = names.get(column).unwrap_or(&"");
                let width = dims.get_width(column);

                (width, text)
            })
            .map(|(width, text)| cmp::max(string_width(text), width))
            .collect::<Vec<_>>();

        dims.set_widths(widths.clone());

        let mut total_width = 0;
        for (i, (width, name)) in widths.into_iter().zip(names).enumerate() {
            let color = get_color(&self.colors, i);
            let alignment = get_alignment(&self.alignments, i);
            let left_vertical = get_vertical_width(cfg, (self.line, i), count_columns);
            let grid_offset = total_width + left_vertical;
            let btext = get_border_text(name, grid_offset, width, alignment, self.line, color);
            btext.change(records, cfg, dims);

            total_width += width + left_vertical;
        }
    }
}

fn collect_head(records: &mut VecRecords<CellInfo<String>>) -> Vec<String> {
    if records.count_rows() == 0 || records.count_columns() == 0 {
        return Vec::new();
    }

    let names = (0..records.count_columns())
        .map(|column| records.get_text((0, column)))
        .map(ToString::to_string)
        .collect();

    records.remove_row(0);

    names
}

fn get_border_text(
    text: &str,
    offset: usize,
    available: usize,
    align: AlignmentHorizontal,
    line: usize,
    color: Option<&Color>,
) -> BorderText<usize> {
    let left_indent = get_indent(text, align, available);
    let offset = Offset::Begin(offset + left_indent);
    let mut btext = BorderText::new(text).horizontal(line).offset(offset);
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

fn get_alignment(alignments: &ListValue<AlignmentHorizontal>, i: usize) -> AlignmentHorizontal {
    match alignments {
        ListValue::List(list) => list.get(i).copied().unwrap_or(AlignmentHorizontal::Left),
        ListValue::Static(alignment) => *alignment,
    }
}

fn get_indent(text: &str, align: AlignmentHorizontal, available: usize) -> usize {
    match align {
        AlignmentHorizontal::Left => 0,
        AlignmentHorizontal::Right => available - string_width(text),
        AlignmentHorizontal::Center => (available - string_width(text)) / 2,
    }
}

fn get_vertical_width(cfg: &mut ColoredConfig, pos: Position, count_columns: usize) -> usize {
    cfg.get_vertical(pos, count_columns)
        .and_then(unicode_width::UnicodeWidthChar::width)
        .unwrap_or(0)
}

#[derive(Debug, Clone)]
pub enum ListValue<T> {
    List(Vec<T>),
    Static(T),
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
