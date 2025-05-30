//! This module contains [`MinWidth`] structure, used to increase width of a [`Table`]s or a cell on a [`Table`].
//!
//! [`Table`]: crate::Table

use std::iter::repeat_n;

use papergrid::dimension::Estimate;

use crate::{
    grid::{
        config::{ColoredConfig, Entity, Position},
        dimension::CompleteDimension,
        records::{
            vec_records::Cell, ExactRecords, IntoRecords, PeekableRecords, Records, RecordsMut,
        },
        util::string::{get_line_width, get_lines},
    },
    settings::{
        measurement::Measurement,
        peaker::{Peaker, PriorityNone},
        CellOption, TableOption, Width,
    },
};

use super::util::get_table_total_width;

/// [`MinWidth`] changes a content in case if it's length is lower then the boundary.
///
/// It can be applied to a whole table.
///
/// It does nothing in case if the content's length is bigger then the boundary.
///
/// Be aware that further changes of the table may cause the width being not set.
/// For example applying [`Padding`] after applying [`MinWidth`] will make the former have no affect.
/// (You should use [`Padding`] first).
///
/// Be aware that it doesn't consider padding.
/// So if you want to set a exact width you might need to use [`Padding`] to set it to 0.
///
/// ## Examples
///
/// Cell change
///
/// ```
/// use tabled::{Table, settings::{object::Segment, Width, Style, Modify}};
///
/// let data = ["Hello", "World", "!"];
///
/// let table = Table::new(&data)
///     .with(Style::markdown())
///     .with(Modify::new(Segment::all()).with(Width::increase(10)));
/// ```
/// Table change
///
/// ```
/// use tabled::{Table, settings::Width};
///
/// let table = Table::new(&["Hello World!"]).with(Width::increase(5));
/// ```
///
/// [`Padding`]: crate::settings::Padding
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct MinWidth<W = usize, P = PriorityNone> {
    width: W,
    fill: char,
    priority: P,
}

impl<W> MinWidth<W>
where
    W: Measurement<Width>,
{
    /// Creates a new instance of [`MinWidth`].
    pub const fn new(width: W) -> Self {
        Self {
            width,
            fill: ' ',
            priority: PriorityNone::new(),
        }
    }
}

impl<W, P> MinWidth<W, P> {
    /// Set's a fill character which will be used to fill the space
    /// when increasing the length of the string to the set boundary.
    ///
    /// Used only if changing cells.
    pub fn fill_with(mut self, c: char) -> Self {
        self.fill = c;
        self
    }

    /// Priority defines the logic by which a increase of width will be applied when is done for the whole table.
    ///
    /// - [`PriorityNone`] which inc the columns one after another.
    /// - [`PriorityMax`] inc the biggest columns first.
    /// - [`PriorityMin`] inc the lowest columns first.
    ///
    /// [`PriorityMax`]: crate::settings::peaker::PriorityMax
    /// [`PriorityMin`]: crate::settings::peaker::PriorityMin
    pub fn priority<PP: Peaker>(self, peacker: PP) -> MinWidth<W, PP> {
        MinWidth {
            fill: self.fill,
            width: self.width,
            priority: peacker,
        }
    }
}

impl<W, R, P> CellOption<R, ColoredConfig> for MinWidth<W, P>
where
    W: Measurement<Width>,
    R: Records + ExactRecords + PeekableRecords + RecordsMut<String>,
    for<'a> &'a R: Records,
    for<'a> <<&'a R as Records>::Iter as IntoRecords>::Cell: AsRef<str>,
{
    fn change(self, records: &mut R, cfg: &mut ColoredConfig, entity: Entity) {
        let width = self.width.measure(&*records, cfg);

        let count_rows = records.count_rows();
        let count_columns = records.count_columns();
        let max_pos = Position::new(count_rows, count_columns);

        for pos in entity.iter(count_rows, count_columns) {
            if !max_pos.has_coverage(pos) {
                continue;
            }

            let cell_width = records.get_width(pos);
            if cell_width >= width {
                continue;
            }

            let cell = records.get_text(pos);
            let content = increase_width(cell, width, self.fill);
            records.set(pos, content);
        }
    }
}

impl<W, P, R> TableOption<R, ColoredConfig, CompleteDimension> for MinWidth<W, P>
where
    W: Measurement<Width>,
    P: Peaker,
    R: Records + ExactRecords + PeekableRecords,
    for<'a> &'a R: Records,
    for<'a> <<&'a R as Records>::Iter as IntoRecords>::Cell: Cell + AsRef<str>,
{
    fn change(self, records: &mut R, cfg: &mut ColoredConfig, dims: &mut CompleteDimension) {
        if records.count_rows() == 0 || records.count_columns() == 0 {
            return;
        }

        let minwidth = self.width.measure(&*records, cfg);

        dims.estimate(&*records, cfg);
        let widths = dims.get_widths().expect("must be present");

        let total_width = get_table_total_width(widths, cfg);
        if total_width >= minwidth {
            return;
        }

        let widths = get_increase_list(widths, minwidth, total_width, self.priority);
        dims.set_widths(widths);
    }

    fn hint_change(&self) -> Option<Entity> {
        // NOTE:
        // We set proper widths,
        // While keeping height unchanged,
        // So we can safely assume nothing needs reestimation.
        None
    }
}

// todo:  Rename MinWidth?

fn get_increase_list<F>(
    widths: &[usize],
    need: usize,
    mut current: usize,
    mut peaker: F,
) -> Vec<usize>
where
    F: Peaker,
{
    let mut widths = widths.to_vec();

    while need != current {
        let col = match peaker.peak(&[], &widths) {
            Some(col) => col,
            None => break,
        };

        widths[col] += 1;
        current += 1;
    }

    widths
}

fn increase_width(s: &str, width: usize, fill_with: char) -> String {
    let mut buf = String::new();
    for (i, line) in get_lines(s).enumerate() {
        if i > 0 {
            buf.push('\n');
        }

        buf.push_str(&line);

        let length = get_line_width(&line);
        if length < width {
            let remain = width - length;
            buf.extend(repeat_n(fill_with, remain));
        }
    }

    buf
}
