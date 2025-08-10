use std::cmp;

use crate::{
    grid::{
        config::{AlignmentVertical, ColoredConfig, Entity, Offset, Position, Sides},
        dimension::{CompleteDimension, Dimension, Estimate},
        records::{
            vec_records::{Text, VecRecords},
            ExactRecords, Records,
        },
        util::string::{get_char_width, get_line_width},
    },
    settings::{object::Columns, style::LineText, Alignment, Color, Padding, TableOption},
};

/// [`RowNames`] sets strings on vertical lines for the rows.
///
/// # Examples
///
/// ```
/// use tabled::{
///     Table,
///     settings::{themes::RowNames, Alignment},
///     assert::assert_table,
/// };
///
/// let data = vec![
///     ["Hello", "World"],
///     ["Hello", "World"],
/// ];
///
/// let mut table = Table::new(data);
/// table.with(RowNames::new(["", "head1", "head2"]));
///
/// assert_table!(
///     table,
///     "+-------+-------+"
///     "| 0     | 1     |"
///     "+-------+-------+"
///     "h Hello | World |"
///     "e       |       |"
///     "a       |       |"
///     "d       |       |"
///     "1       |       |"
///     "+-------+-------+"
///     "h Hello | World |"
///     "e       |       |"
///     "a       |       |"
///     "d       |       |"
///     "2       |       |"
///     "+-------+-------+"
/// );
/// ```
#[derive(Debug, Clone)]
pub struct RowNames {
    names: Vec<String>,
    colors: Option<ListValue<Color>>,
    alignments: ListValue<Alignment>,
    paddings: ListValue<Padding>,
    line: usize,
}

impl RowNames {
    /// Creates a [`RowNames`] with a given names.
    ///
    /// # Example
    ///
    /// ```
    /// use std::iter::FromIterator;
    /// use tabled::{Table, settings::themes::RowNames, assert::assert_table};
    ///
    /// let data = vec![vec!["Hello", "World"]];
    ///
    /// let mut table = Table::from_iter(data);
    /// table.with(RowNames::new(["head1", "head2"]));
    ///
    /// assert_table!(
    ///     table,
    ///     "+-------+-------+"
    ///     "h Hello | World |"
    ///     "e       |       |"
    ///     "a       |       |"
    ///     "d       |       |"
    ///     "1       |       |"
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
            paddings: ListValue::Static(Padding::zero()),
            alignments: ListValue::Static(Alignment::left()),
            colors: Default::default(),
            line: 0,
        }
    }

    /// Set color for the names.
    ///
    /// By default there's no colors.
    ///
    /// # Example
    ///
    /// ```
    /// use std::iter::FromIterator;
    /// use tabled::Table;
    /// use tabled::settings::{Color, themes::RowNames};
    /// use tabled::assert::assert_table;
    ///
    /// let data = vec![vec!["Hello", "World"]];
    /// let mut table = Table::from_iter(data);
    /// table.with(RowNames::new(["head1", "head2"]).color(vec![Color::FG_RED]));
    ///
    /// assert_table!(
    ///     table,
    ///     "+-------+-------+"
    ///     "\u{1b}[31mh\u{1b}[39m Hello | World |"
    ///     "\u{1b}[31me\u{1b}[39m       |       |"
    ///     "\u{1b}[31ma\u{1b}[39m       |       |"
    ///     "\u{1b}[31md\u{1b}[39m       |       |"
    ///     "\u{1b}[31m1\u{1b}[39m       |       |"
    ///     "+-------+-------+"
    /// );
    /// ```
    pub fn color<T>(self, color: T) -> Self
    where
        T: Into<ListValue<Color>>,
    {
        Self {
            names: self.names,
            line: self.line,
            paddings: self.paddings,
            alignments: self.alignments,
            colors: Some(color.into()),
        }
    }

    /// Set a vertical line the names will be applied to.
    ///
    /// The default value is 0 (the left most vertical line).
    ///
    /// # Example
    ///
    /// ```
    /// use std::iter::FromIterator;
    /// use tabled::{Table, settings::themes::RowNames, assert::assert_table};
    ///
    /// let mut table = Table::from_iter(vec![vec!["Hello", "World"]]);
    /// table.with(RowNames::new(["head1", "head2"]).line(1));
    ///
    /// assert_table!(
    ///     table,
    ///     "+-------+-------+"
    ///     "| Hello h World |"
    ///     "|       e       |"
    ///     "|       a       |"
    ///     "|       d       |"
    ///     "|       1       |"
    ///     "+-------+-------+"
    /// );
    /// ```
    pub fn line(self, i: usize) -> Self {
        Self {
            names: self.names,
            paddings: self.paddings,
            alignments: self.alignments,
            colors: self.colors,
            line: i,
        }
    }

    /// Set an alignment for the names.
    ///
    /// By default it's top aligned.
    ///
    /// # Example
    ///
    /// ```
    /// use std::iter::FromIterator;
    /// use tabled::{
    ///     Table,
    ///     settings::{themes::RowNames, Alignment},
    ///     assert::assert_table,
    /// };
    ///
    /// let mut table = Table::from_iter(vec![vec!["Hello", "World\nWorld\nWorld\nWorld\nWorld\nWorld\nWorld\n"]]);
    /// table.with(RowNames::new(["head1", "head2"]).alignment(Alignment::bottom()));
    ///
    /// assert_table!(
    ///     table,
    ///     "+-------+-------+"
    ///     "| Hello | World |"
    ///     "|       | World |"
    ///     "|       | World |"
    ///     "h       | World |"
    ///     "e       | World |"
    ///     "a       | World |"
    ///     "d       | World |"
    ///     "1       |       |"
    ///     "+-------+-------+"
    /// );
    /// ```
    pub fn alignment<T>(self, alignment: T) -> Self
    where
        T: Into<ListValue<Alignment>>,
    {
        Self {
            names: self.names,
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
    ///     settings::{themes::RowNames, Padding},
    ///     assert::assert_table,
    /// };
    ///
    /// let data = vec![vec!["Hello", "World"]];
    /// let mut table = Table::from_iter(data);
    /// table.with(RowNames::new(["head1", "head2"]).padding(Padding::new(0, 0, 2, 4)));
    ///
    /// assert_table!(
    ///     table,
    ///     "+-------+-------+"
    ///     "| Hello | World |"
    ///     "|       |       |"
    ///     "h       |       |"
    ///     "e       |       |"
    ///     "a       |       |"
    ///     "d       |       |"
    ///     "1       |       |"
    ///     "|       |       |"
    ///     "|       |       |"
    ///     "|       |       |"
    ///     "|       |       |"
    ///     "+-------+-------+"
    /// );
    /// ```
    pub fn padding<T>(self, padding: T) -> Self
    where
        T: Into<ListValue<Padding>>,
    {
        Self {
            names: self.names,
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

impl TableOption<VecRecords<Text<String>>, ColoredConfig, CompleteDimension> for RowNames {
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

        let mut names = self.names;
        let size = records.count_rows();
        ensure_vector_size(&mut names, size);

        let alignment = convert_alignment_value(self.alignments)
            .unwrap_or(ListValue::Static(AlignmentVertical::Top));
        let info = RowInfo::new(names, self.colors, alignment, self.paddings, self.line);
        set_row_text(records, dims, cfg, info);
    }

    fn hint_change(&self) -> Option<Entity> {
        Some(Entity::Column(0))
    }
}

#[derive(Debug, Clone)]
struct RowInfo {
    names: Vec<String>,
    colors: Option<ListValue<Color>>,
    alignments: ListValue<AlignmentVertical>,
    paddings: ListValue<Padding>,
    line: usize,
}

impl RowInfo {
    fn new(
        names: Vec<String>,
        colors: Option<ListValue<Color>>,
        alignments: ListValue<AlignmentVertical>,
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

fn set_row_text(
    records: &mut VecRecords<Text<String>>,
    dims: &mut CompleteDimension,
    cfg: &mut ColoredConfig,
    info: RowInfo,
) {
    dims.estimate(&*records, cfg);

    let count_rows = info.names.len();

    let mut heights = Vec::with_capacity(count_rows);
    for (row, name) in info.names.iter().enumerate() {
        let pad = Sides::from(info.paddings.get_or_else(row, || Padding::zero()));
        let name_height = get_line_width(name) + pad.top.size + pad.bottom.size;
        let row_height = dims.get_height(row);

        let height = cmp::max(name_height, row_height);

        heights.push(height);
    }

    let mut global_offset = 0;
    for (row, (name, row_height)) in info.names.into_iter().zip(heights.iter()).enumerate() {
        let color = get_color(&info.colors, row);
        let alignment = info.alignments.get_or_else(row, || AlignmentVertical::Top);
        let padding = Sides::from(info.paddings.get_or_else(row, || Padding::zero()));
        let horizontal_pos = (row, info.line).into();
        let top_horizontal = get_horizontal_width(cfg, horizontal_pos, count_rows);
        let height = row_height - padding.top.size - padding.bottom.size;
        let cell_indent = get_vertical_indent(&name, alignment, height);
        let offset = global_offset + top_horizontal + cell_indent + padding.top.size;

        dims.set_heights(heights.clone());

        let linetext = create_line_text(&name, offset, color, Columns::one(info.line));
        linetext.change(records, cfg, dims);

        global_offset += row_height + top_horizontal;
    }

    dims.set_heights(heights);
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

fn get_vertical_indent(text: &str, align: AlignmentVertical, available: usize) -> usize {
    match align {
        AlignmentVertical::Top => 0,
        AlignmentVertical::Bottom => available - get_line_width(text),
        AlignmentVertical::Center => (available - get_line_width(text)) / 2,
    }
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
