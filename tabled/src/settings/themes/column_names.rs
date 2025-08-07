use std::cmp;

use crate::{
    grid::{
        config::{AlignmentHorizontal, AlignmentVertical, ColoredConfig, Entity, Offset, Position},
        dimension::{CompleteDimension, Dimension, Estimate},
        records::{
            vec_records::{Text, VecRecords},
            ExactRecords, PeekableRecords, Records, Resizable,
        },
        util::string::{get_char_width, get_line_width},
    },
    settings::{
        object::{Column, Row},
        style::LineText,
        Alignment, Color, Padding, TableOption,
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
/// table.with(ColumnNames::head());
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
    // todo: In case of vertical column names we shall delete column right????
    delete_head: bool,
    names: Vec<String>,
    colors: Option<ListValue<Color>>,
    alignments: ListValue<Alignment>,
    paddings: ListValue<Padding>,
    line: usize,
}

impl ColumnNames {
    /// Creates a [`ColumnNames`]
    /// which will be removing the head row and putting it right on the given border.
    ///
    /// # Example
    ///
    /// ```
    /// use std::iter::FromIterator;
    /// use tabled::{Table, settings::themes::ColumnNames, assert::assert_table};
    ///
    /// let data = vec![
    ///     vec!["head1", "head2"],
    ///     vec!["Hello", "World"],
    /// ];
    ///
    /// let mut table = Table::from_iter(data);
    /// table.with(ColumnNames::head());
    ///
    /// assert_table!(
    ///     table,
    ///     "+head1--+head2--+"
    ///     "| Hello | World |"
    ///     "+-------+-------+"
    /// );
    /// ```
    pub fn head() -> Self {
        Self {
            delete_head: true,
            names: Default::default(),
            colors: Default::default(),
            line: Default::default(),
            paddings: ListValue::Static(Padding::zero()),
            alignments: ListValue::Static(Alignment::left()),
        }
    }

    /// Creates a [`ColumnNames`] with a given names.
    ///
    /// Using a [`Default`] would reuse a names from the first row.
    ///
    /// # Example
    ///
    /// ```
    /// use std::iter::FromIterator;
    /// use tabled::{Table, settings::themes::ColumnNames, assert::assert_table};
    ///
    /// let data = vec![vec!["Hello", "World"]];
    /// let mut table = Table::from_iter(data);
    /// table.with(ColumnNames::new(["head1", "head2"]));
    ///
    /// assert_table!(
    ///     table,
    ///     "+head1--+head2--+"
    ///     "| Hello | World |"
    ///     "+-------+-------+"
    /// );
    /// ```
    pub fn new<I>(names: I) -> Self
    where
        I: IntoIterator,
        I::Item: AsRef<str>,
    {
        let names = collect_first_lines(names);

        Self {
            names,
            delete_head: false,
            paddings: ListValue::Static(Padding::zero()),
            alignments: ListValue::Static(Alignment::left()),
            colors: Default::default(),
            line: 0,
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
    /// use tabled::assert::assert_table;
    ///
    /// let data = vec![vec!["Hello", "World"]];
    /// let mut table = Table::from_iter(data);
    /// table.with(ColumnNames::new(["head1", "head2"]).color(vec![Color::FG_RED]));
    ///
    /// assert_table!(
    ///     table,
    ///     "+\u{1b}[31mh\u{1b}[39m\u{1b}[31me\u{1b}[39m\u{1b}[31ma\u{1b}[39m\u{1b}[31md\u{1b}[39m\u{1b}[31m1\u{1b}[39m--+head2--+"
    ///     "| Hello | World |"
    ///     "+-------+-------+"
    /// );
    /// ```
    pub fn color<T>(self, color: T) -> Self
    where
        T: Into<ListValue<Color>>,
    {
        Self {
            names: self.names,
            delete_head: self.delete_head,
            line: self.line,
            paddings: self.paddings,
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
            delete_head: self.delete_head,
            paddings: self.paddings,
            alignments: self.alignments,
            colors: self.colors,
            line: i,
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
            delete_head: self.delete_head,
            paddings: self.paddings,
            line: self.line,
            colors: self.colors,
            alignments: alignment.into(),
        }
    }
}

fn collect_first_lines<I>(names: I) -> Vec<String>
where
    I: IntoIterator,
    I::Item: AsRef<str>,
{
    names
        .into_iter()
        .map(|name| name.as_ref().lines().next().unwrap_or("").to_string())
        .collect::<Vec<_>>()
}

// TODO: Split into ColumnNames and RowNames

impl TableOption<VecRecords<Text<String>>, ColoredConfig, CompleteDimension> for ColumnNames {
    fn change(
        self,
        records: &mut VecRecords<Text<String>>,
        cfg: &mut ColoredConfig,
        dims: &mut CompleteDimension,
    ) {
        let count_rows = records.count_rows();
        let count_columns = records.count_columns();

        if count_columns == 0 || count_rows == 0 || self.line > count_rows {
            return;
        }

        let alignment_horizontal = convert_alignment_value(self.alignments.clone());
        let alignment_vertical = convert_alignment_value(self.alignments);
        let is_vertical = alignment_vertical.is_some();

        let mut names = self.names;
        if self.delete_head {
            names = collect_head(records);
        }

        let size = if is_vertical {
            records.count_rows()
        } else {
            records.count_columns()
        };

        ensure_vector_size(&mut names, size);

        if let Some(alignment) = alignment_vertical {
            set_row_text(names, self.line, alignment, self.colors, records, dims, cfg);
            return;
        }

        let alignment =
            alignment_horizontal.unwrap_or(ListValue::Static(AlignmentHorizontal::Left));
        set_column_text(names, self.line, alignment, self.colors, records, dims, cfg);
    }

    fn hint_change(&self) -> Option<Entity> {
        let alignment_vertical =
            convert_alignment_value::<AlignmentVertical>(self.alignments.clone());
        if alignment_vertical.is_some() {
            Some(Entity::Column(0))
        } else {
            Some(Entity::Row(0))
        }
    }
}

fn set_column_text(
    names: Vec<String>,
    target_line: usize,
    alignments: ListValue<AlignmentHorizontal>,
    colors: Option<ListValue<Color>>,
    records: &mut VecRecords<Text<String>>,
    dims: &mut CompleteDimension,
    cfg: &mut ColoredConfig,
) {
    dims.estimate(&*records, cfg);

    let count_columns = names.len();
    let widths = names
        .iter()
        .enumerate()
        .map(|(col, name)| (cmp::max(get_line_width(name), dims.get_width(col))))
        .collect::<Vec<_>>();

    dims.set_widths(widths.clone());

    let mut total_width = 0;
    for (column, (width, name)) in widths.into_iter().zip(names).enumerate() {
        let color = get_color(&colors, column);
        let alignment = alignments.get(column).unwrap_or(AlignmentHorizontal::Left);
        let left_vertical = get_vertical_width(cfg, (target_line, column).into(), count_columns);
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
    records: &mut VecRecords<Text<String>>,
    dims: &mut CompleteDimension,
    cfg: &mut ColoredConfig,
) {
    dims.estimate(&*records, cfg);

    let count_rows = names.len();
    let heights = names
        .iter()
        .enumerate()
        .map(|(row, name)| (cmp::max(get_line_width(name), dims.get_height(row))))
        .collect::<Vec<_>>();

    dims.set_heights(heights.clone());

    let mut total_height = 0;
    for (row, (row_height, name)) in heights.into_iter().zip(names).enumerate() {
        let color = get_color(&colors, row);
        let alignment = alignments.get(row).unwrap_or(AlignmentVertical::Top);
        let top_horizontal = get_horizontal_width(cfg, (row, target_line).into(), count_rows);
        let cell_indent = get_vertical_indent(&name, alignment, row_height);
        let grid_offset = total_height + top_horizontal + cell_indent;
        let line = Column::from(target_line);

        let linetext = create_line_text(&name, grid_offset, color, line);
        linetext.change(records, cfg, dims);

        total_height += row_height + top_horizontal;
    }
}

fn ensure_vector_size(data: &mut Vec<String>, size: usize) {
    match data.len().cmp(&size) {
        cmp::Ordering::Equal => {}
        cmp::Ordering::Less => {
            let additional_size = size - data.len();
            data.extend(std::iter::repeat_n(String::new(), additional_size));
        }
        cmp::Ordering::Greater => {
            data.truncate(size);
        }
    }
}

fn collect_head(records: &mut VecRecords<Text<String>>) -> Vec<String> {
    if records.count_rows() == 0 || records.count_columns() == 0 {
        return Vec::new();
    }

    let names = (0..records.count_columns())
        .map(|column| records.get_line((0, column).into(), 0))
        .map(ToString::to_string)
        .collect();

    records.remove_row(0);

    names
}

fn create_line_text<T>(text: &str, offset: usize, color: Option<&Color>, line: T) -> LineText<T> {
    let offset = Offset::Start(offset);
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
        AlignmentHorizontal::Right => available - get_line_width(text),
        AlignmentHorizontal::Center => (available - get_line_width(text)) / 2,
    }
}

fn get_vertical_indent(text: &str, align: AlignmentVertical, available: usize) -> usize {
    match align {
        AlignmentVertical::Top => 0,
        AlignmentVertical::Bottom => available - get_line_width(text),
        AlignmentVertical::Center => (available - get_line_width(text)) / 2,
    }
}

fn get_vertical_width(cfg: &mut ColoredConfig, pos: Position, count_columns: usize) -> usize {
    cfg.get_vertical(pos, count_columns)
        .map(get_char_width)
        .unwrap_or(0)
}

fn get_horizontal_width(cfg: &mut ColoredConfig, pos: Position, count_rows: usize) -> usize {
    cfg.get_horizontal(pos, count_rows)
        .map(get_char_width)
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
