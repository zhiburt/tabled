//! This module contains [`MinWidth`] structure, used to increase width of a [`Table`]s or a cell on a [`Table`].

use std::marker::PhantomData;

use papergrid::{
    records::{Records, RecordsMut},
    width::CfgWidthFunction,
    Entity,
};

use crate::{
    measurment::Measurment,
    peaker::{Peaker, PriorityNone},
    CellOption, Table, TableOption, Width,
};

use super::get_table_widths_with_total;

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
/// use tabled::{object::Segment, Width, Modify, Style, Table};
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
/// use tabled::{Width, Table};
///
/// let table = Table::new(&["Hello World!"]).with(Width::increase(5));
/// ```
///
/// [`Padding`]: crate::Padding
#[derive(Debug)]
pub struct MinWidth<W = usize, P = PriorityNone> {
    width: W,
    fill: char,
    _priority: PhantomData<P>,
}

impl<W> MinWidth<W>
where
    W: Measurment<Width>,
{
    /// Creates a new instance of [`MinWidth`].
    pub fn new(width: W) -> Self {
        Self {
            width,
            fill: ' ',
            _priority: PhantomData::default(),
        }
    }
}

impl<W, P> MinWidth<W, P> {
    /// Set's a fill character which will be used to fill the space
    /// when increasing the length of the string to the set boundary.
    ///
    /// Used only if chaning cells.
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
    /// [`PriorityMax`]: crate::peaker::PriorityMax
    /// [`PriorityMin`]: crate::peaker::PriorityMin
    pub fn priority<PP: Peaker>(self) -> MinWidth<W, PP> {
        MinWidth {
            fill: self.fill,
            width: self.width,
            _priority: PhantomData::default(),
        }
    }
}

impl<W, R> CellOption<R> for MinWidth<W>
where
    W: Measurment<Width>,
    R: Records + RecordsMut<String>,
{
    fn change_cell(&mut self, table: &mut Table<R>, entity: Entity) {
        let width_ctrl = CfgWidthFunction::from_cfg(table.get_config());
        let width = self.width.measure(table.get_records(), table.get_config());

        let (count_rows, count_cols) = table.shape();
        for pos in entity.iter(count_rows, count_cols) {
            let records = table.get_records();
            let cell_width = records.get_width(pos, &width_ctrl);
            if cell_width >= width {
                continue;
            }

            let content = records.get_text(pos);
            let content = increase_width(content, width, self.fill);
            let records = table.get_records_mut();
            records.set(pos, content, &width_ctrl);
        }

        table.destroy_width_cache();
    }
}

impl<W, P, R> TableOption<R> for MinWidth<W, P>
where
    W: Measurment<Width>,
    P: Peaker,
    R: Records + RecordsMut<String>,
{
    fn change(&mut self, table: &mut Table<R>) {
        if table.is_empty() {
            return;
        }

        let width = self.width.measure(table.get_records(), table.get_config());
        let (widths, total_width) =
            get_table_widths_with_total(table.get_records(), table.get_config());
        if total_width >= width {
            return;
        }

        increase_total_width(table, widths, total_width, width, P::create());
    }
}

#[cfg(not(feature = "color"))]
fn increase_width(s: &str, width: usize, fill_with: char) -> String {
    use papergrid::util::string_width;

    s.lines()
        .map(|line| {
            let length = string_width(line);
            if width > length {
                let remain = width - length;
                let mut new_line = String::with_capacity(width);
                new_line.push_str(line);
                new_line.extend(std::iter::repeat(fill_with).take(remain));
                std::borrow::Cow::Owned(new_line)
            } else {
                std::borrow::Cow::Borrowed(line)
            }
        })
        .collect::<Vec<_>>()
        .join("\n")
}

#[cfg(feature = "color")]
fn increase_width(s: &str, width: usize, fill_with: char) -> String {
    use papergrid::util::string_width;

    ansi_str::AnsiStr::ansi_split(s, "\n")
        .map(|line| {
            let length = string_width(&line);
            if length < width {
                let mut line = line.into_owned();
                let remain = width - length;
                line.extend(std::iter::repeat(fill_with).take(remain));
                std::borrow::Cow::Owned(line)
            } else {
                line
            }
        })
        .collect::<Vec<_>>()
        .join("\n")
}

fn increase_total_width<P, R>(
    table: &mut Table<R>,
    widths: Vec<usize>,
    total_width: usize,
    expected_width: usize,
    priority: P,
) where
    P: Peaker,
    R: Records + RecordsMut<String>,
{
    let increase_list = get_increase_list(widths, expected_width, total_width, priority);
    table.cache_width(increase_list);
}

fn get_increase_list<F>(
    mut widths: Vec<usize>,
    total_width: usize,
    mut width: usize,
    mut peaker: F,
) -> Vec<usize>
where
    F: Peaker,
{
    while width != total_width {
        let col = match peaker.peak(&[], &widths) {
            Some(col) => col,
            None => break,
        };

        widths[col] += 1;
        width += 1;
    }

    widths
}
