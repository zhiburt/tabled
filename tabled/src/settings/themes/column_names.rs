use crate::{
    grid::{
        config::{AlignmentHorizontal, ColoredConfig},
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
/// use tabled::{Table, settings::themes::ColumnNames};
///
/// let data = vec![
///     vec!["Hello", "World"],
///     vec!["Hello", "World"],
/// ];
///
/// let mut table = Table::from_iter(data);
/// table.with(ColumnNames::new(["head1", "head2"]).set_offset(3).set_line(2));
///
/// assert_eq!(
///     table.to_string(),
///     "+--------+--------+\n\
///      | Hello  | World  |\n\
///      +--------+--------+\n\
///      | Hello  | World  |\n\
///      +---head1+---head2+"
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
    colors: Vec<Option<Color>>,
    offset: usize,
    line: usize,
    alignment: AlignmentHorizontal,
}

impl Default for ColumnNames {
    fn default() -> Self {
        Self {
            names: Default::default(),
            colors: Default::default(),
            offset: Default::default(),
            line: Default::default(),
            alignment: AlignmentHorizontal::Left,
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
            colors: Vec::new(),
            offset: 0,
            line: 0,
            alignment: AlignmentHorizontal::Left,
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
    /// table.with(ColumnNames::new(["head1", "head2"]).set_colors([Color::FG_RED]));
    ///
    /// assert_eq!(
    ///     table.to_string(),
    ///     "+\u{1b}[31mh\u{1b}[39m\u{1b}[31me\u{1b}[39m\u{1b}[31ma\u{1b}[39m\u{1b}[31md\u{1b}[39m\u{1b}[31m1\u{1b}[39m--+head2--+\n\
    ///      | Hello | World |\n\
    ///      +-------+-------+"
    /// );
    /// ```
    pub fn set_colors<I>(self, colors: I) -> Self
    where
        I: IntoIterator,
        I::Item: Into<Option<Color>>,
    {
        let colors = colors.into_iter().map(Into::into).collect::<Vec<_>>();
        Self {
            names: self.names,
            offset: self.offset,
            line: self.line,
            alignment: self.alignment,
            colors,
        }
    }

    /// Set a left offset after which the names will be used.
    ///
    /// By default there's no offset.
    ///
    /// # Example
    ///
    /// ```
    /// use std::iter::FromIterator;
    /// use tabled::{Table, settings::themes::ColumnNames};
    ///
    /// let mut table = Table::from_iter(vec![vec!["Hello", "World"]]);
    /// table.with(ColumnNames::new(["head1", "head2"]).set_offset(1));
    ///
    /// assert_eq!(
    ///     table.to_string(),
    ///     "+-head1-+-head2-+\n\
    ///      | Hello | World |\n\
    ///      +-------+-------+"
    /// );
    /// ```
    pub fn set_offset(self, i: usize) -> Self {
        Self {
            names: self.names,
            colors: self.colors,
            line: self.line,
            alignment: self.alignment,
            offset: i,
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
            colors: self.colors,
            offset: self.offset,
            alignment: self.alignment,
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
    pub fn set_alignment(self, alignment: AlignmentHorizontal) -> Self {
        Self {
            names: self.names,
            colors: self.colors,
            offset: self.offset,
            line: self.line,
            alignment,
        }
    }
}

impl TableOption<VecRecords<CellInfo<String>>, CompleteDimensionVecRecords<'static>, ColoredConfig>
    for ColumnNames
{
    fn change(
        self,
        records: &mut VecRecords<CellInfo<String>>,
        cfg: &mut ColoredConfig,
        dims: &mut CompleteDimensionVecRecords<'static>,
    ) {
        let names = match self.names {
            Some(names) => names,
            None => {
                if records.count_rows() == 0 || records.count_columns() == 0 {
                    return;
                }

                let names = (0..records.count_columns())
                    .map(|column| records.get_text((0, column)))
                    .map(ToString::to_string)
                    .collect::<Vec<_>>();

                records.remove_row(0);

                names
            }
        };

        let names = names.iter().map(|name| name.lines().next().unwrap_or(""));

        dims.estimate(&*records, cfg);

        let mut widths = (0..records.count_columns())
            .map(|column| dims.get_width(column))
            .collect::<Vec<_>>();

        let names = names.take(widths.len());

        let offset = self.offset;
        widths
            .iter_mut()
            .zip(names.clone())
            .for_each(|(width, text)| {
                let name_width = string_width(text) + offset;
                *width = std::cmp::max(name_width, *width);
            });

        let _ = dims.set_widths(widths.clone());

        let mut total_width = 0;
        for (i, (width, name)) in widths.iter().zip(names).enumerate() {
            let color = get_color(&self.colors, i);
            let offset = total_width + 1;
            let btext = get_border_text(
                name,
                offset,
                *width,
                self.alignment,
                self.offset,
                self.line,
                color,
            );
            btext.change(records, cfg, dims);

            total_width += width + 1;
        }
    }
}

fn get_border_text(
    text: &str,
    offset: usize,
    available: usize,
    alignment: AlignmentHorizontal,
    alignment_offset: usize,
    line: usize,
    color: Option<&Color>,
) -> BorderText<usize> {
    let name = text.to_string();
    let left_indent = get_indent(text, alignment, alignment_offset, available);
    let left_offset = Offset::Begin(offset + left_indent);
    let mut btext = BorderText::new(name).horizontal(line).offset(left_offset);
    if let Some(color) = color {
        btext = btext.color(color.clone());
    }

    btext
}

fn get_color(colors: &[Option<Color>], i: usize) -> Option<&Color> {
    colors.get(i).and_then(|color| match color {
        Some(color) => Some(color),
        None => None,
    })
}

fn get_indent(text: &str, align: AlignmentHorizontal, offset: usize, available: usize) -> usize {
    match align {
        AlignmentHorizontal::Left => offset,
        AlignmentHorizontal::Right => available - string_width(text) - offset,
        AlignmentHorizontal::Center => (available - string_width(text)) / 2,
    }
}
