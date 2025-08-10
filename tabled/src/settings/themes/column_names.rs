use std::cmp;

use crate::{
    grid::{
        config::{AlignmentHorizontal, ColoredConfig, Entity, Offset, Position, Sides},
        dimension::{CompleteDimension, Dimension, Estimate},
        records::{
            vec_records::{Text, VecRecords},
            ExactRecords, PeekableRecords, Records, Resizable,
        },
        util::string::{get_char_width, get_line_width},
    },
    settings::{object::Rows, style::LineText, Alignment, Color, Padding, TableOption},
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

    /// Set an padding for the names.
    ///
    /// By default it's 0.
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
    pub fn padding<T>(self, padding: T) -> Self
    where
        T: Into<ListValue<Padding>>,
    {
        Self {
            names: self.names,
            delete_head: self.delete_head,
            line: self.line,
            colors: self.colors,
            alignments: self.alignments,
            paddings: padding.into(),
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

        let mut names = if self.delete_head {
            collect_head(records)
        } else {
            self.names
        };

        let size = records.count_columns();
        ensure_vector_size(&mut names, size);

        let alignment = convert_alignment_value(self.alignments.clone())
            .unwrap_or(ListValue::Static(AlignmentHorizontal::Left));
        let info = ColumnsInfo::new(names, self.colors, alignment, self.paddings, self.line);
        set_column_text(records, dims, cfg, info);
    }

    fn hint_change(&self) -> Option<Entity> {
        Some(Entity::Row(0))
    }
}

#[derive(Debug, Clone)]
struct ColumnsInfo {
    names: Vec<String>,
    colors: Option<ListValue<Color>>,
    alignments: ListValue<AlignmentHorizontal>,
    paddings: ListValue<Padding>,
    line: usize,
}

impl ColumnsInfo {
    fn new(
        names: Vec<String>,
        colors: Option<ListValue<Color>>,
        alignments: ListValue<AlignmentHorizontal>,
        paddings: ListValue<Padding>,
        line: usize,
    ) -> Self {
        Self {
            names,
            colors,
            alignments,
            paddings,
            line,
        }
    }
}

fn set_column_text(
    records: &mut VecRecords<Text<String>>,
    dims: &mut CompleteDimension,
    cfg: &mut ColoredConfig,
    info: ColumnsInfo,
) {
    dims.estimate(&*records, cfg);

    let count_columns = records.count_columns();

    let mut widths = Vec::with_capacity(count_columns);
    for (col, name) in info.names.iter().enumerate() {
        let pad = Sides::from(info.paddings.get_or_else(col, || Padding::zero()));
        let name_width = get_line_width(name) + pad.left.size + pad.right.size;
        let column_width = dims.get_width(col);

        let width = cmp::max(name_width, column_width);

        widths.push(width);
    }

    let mut global_offset = 0;
    for (column, (name, width)) in info.names.into_iter().zip(widths.iter()).enumerate() {
        let color = get_color(&info.colors, column);
        let alignment = info
            .alignments
            .get_or_else(column, || AlignmentHorizontal::Left);
        let padding = Sides::from(info.paddings.get_or_else(column, || Padding::zero()));
        let vertical_pos = (info.line, column).into();
        let left_vertical = get_vertical_width(cfg, vertical_pos, count_columns);
        let width_without_padding = *width - padding.left.size - padding.right.size;
        let text_indent = get_horizontal_indent(&name, alignment, width_without_padding);
        let offset = global_offset + left_vertical + padding.left.size + text_indent;

        // We set widths on each iteration because LineText will invalidate it....
        // TODO: Test LineText hint
        dims.set_widths(widths.clone());

        let linetext = create_line_text(&name, offset, color, Rows::one(info.line));
        linetext.change(records, cfg, dims);

        global_offset += width + left_vertical;
    }

    // We set widths second time because LineText does not have a hint - so it will invalidate it.
    dims.set_widths(widths);
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

fn get_vertical_width(cfg: &mut ColoredConfig, pos: Position, count_columns: usize) -> usize {
    cfg.get_vertical(pos, count_columns)
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

    fn get_or_else<F>(&self, i: usize, default: F) -> T
    where
        F: FnOnce() -> T,
        T: Copy,
    {
        self.get(i).unwrap_or_else(default)
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
