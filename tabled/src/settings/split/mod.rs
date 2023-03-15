//! This module contains a [`Split`] setting which is used to
//! format the cells of a [`Table`] by a provided index, direction, behavior, and display preference.
//!
//! [`Table`]: crate::Table

use papergrid::config::Position;

use crate::records::{ExactRecords, Records, Resizable};

use super::TableOption;

#[derive(Debug, Clone, Copy)]
enum Direction {
    Column,
    Row,
}

#[derive(Debug, Clone, Copy)]
enum Behavior {
    Concat,
    Zip,
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Display {
    Clean,
    Retain,
}

/// Returns a new [`Table`] formatted with several optional parameters.
///
/// The required index parameter determines how many columns/rows a table will be redistributed into.
///
/// - index [`usize`]
/// - direction [`Direction`]
/// - behavior [`Behavior`] DEFAULT [`Behavior::Zip`]
/// - display [`Display`] DEFAULT [`Display::Clean`]
///
/// # Example
///
/// ```rust,no_run
/// use std::iter::FromIterator;
/// use tabled::{
///     settings::split::Split,
///     Table,
/// };
///
/// let mut table = Table::from_iter(['a'..='z']);
/// let table = table.with(Split::column(4)).to_string();
///
/// assert_eq!(table, "+---+---+---+---+\n\
///                    | a | b | c | d |\n\
///                    +---+---+---+---+\n\
///                    | e | f | g | h |\n\
///                    +---+---+---+---+\n\
///                    | i | j | k | l |\n\
///                    +---+---+---+---+\n\
///                    | m | n | o | p |\n\
///                    +---+---+---+---+\n\
///                    | q | r | s | t |\n\
///                    +---+---+---+---+\n\
///                    | u | v | w | x |\n\
///                    +---+---+---+---+\n\
///                    | y | z |   |   |\n\
///                    +---+---+---+---+")
/// ```
///
/// [`Table`]: crate::Table
#[derive(Debug, Clone, Copy)]
pub struct Split {
    direction: Direction,
    behavior: Behavior,
    display: Display,
    index: usize,
}

impl Split {
    /// Returns a new [`Table`] split on the column at the provided index.
    ///
    /// The column found at that index becomes the new right-most column in the returned table.
    /// Columns found beyond the index are redistributed into the table based on other defined
    /// parameters.
    ///
    /// ```rust,no_run
    /// # use tabled::settings::split::Split;
    /// Split::column(4);
    /// ```
    ///
    /// [`Table`]: crate::Table
    pub fn column(index: usize) -> Self {
        Split {
            direction: Direction::Column,
            behavior: Behavior::Zip,
            display: Display::Clean,
            index,
        }
    }

    /// Returns a new [`Table`] split on the row at the provided index.
    ///
    /// The row found at that index becomes the new bottom row in the returned table.
    /// Rows found beyond the index are redistributed into the table based on other defined
    /// parameters.
    ///
    /// ```rust,no_run
    /// # use tabled::settings::split::Split;
    /// Split::row(4);
    /// ```
    ///
    /// [`Table`]: crate::Table
    pub fn row(index: usize) -> Self {
        Split {
            direction: Direction::Row,
            behavior: Behavior::Zip,
            display: Display::Clean,
            index,
        }
    }

    /// Returns a split [`Table`] with the redistributed cells pushed to the back of the new shape.
    ///
    /// ```text
    ///                                                 +---+---+
    ///                                                 | a | b |
    ///                                                 +---+---+
    /// +---+---+---+---+---+                           | f | g |
    /// | a | b | c | d | e | Split::column(2).concat() +---+---+
    /// +---+---+---+---+---+           =>              | c | d |
    /// | f | g | h | i | j |                           +---+---+
    /// +---+---+---+---+---+                           | h | i |
    ///                                                 +---+---+
    ///                                                 | e |   |
    ///                                                 +---+---+
    ///                                                 | j |   |
    ///                                                 +---+---+
    /// ```
    ///
    /// [`Table`]: crate::Table
    pub fn concat(self) -> Self {
        Self {
            behavior: Behavior::Concat,
            ..self
        }
    }

    /// Returns a split [`Table`] with the redistributed cells inserted behind
    /// the first correlating column/row one after another.
    ///
    /// ```text
    ///                                              +---+---+
    ///                                              | a | b |
    ///                                              +---+---+
    /// +---+---+---+---+---+                        | c | d |
    /// | a | b | c | d | e | Split::column(2).zip() +---+---+
    /// +---+---+---+---+---+           =>           | e |   |
    /// | f | g | h | i | j |                        +---+---+
    /// +---+---+---+---+---+                        | f | g |
    ///                                              +---+---+
    ///                                              | h | i |
    ///                                              +---+---+
    ///                                              | j |   |
    ///                                              +---+---+
    /// ```
    ///
    /// [`Table`]: crate::Table
    pub fn zip(self) -> Self {
        Self {
            behavior: Behavior::Zip,
            ..self
        }
    }

    /// Returns a split [`Table`] with the empty columns/rows filtered out.
    ///
    /// ```text
    ///                                                
    ///                                                
    ///                                                +---+---+---+
    /// +---+---+---+---+---+                          | a | b | c |
    /// | a | b | c | d | e | Split::column(3).clean() +---+---+---+
    /// +---+---+---+---+---+           =>             | d | e |   |
    /// | f | g | h |   |   |                          +---+---+---+
    /// +---+---+---+---+---+                          | f | g | h |
    ///               ^   ^                            +---+---+---+
    ///               these cells are filtered
    ///               from the resulting table
    /// ```
    ///
    /// ## Notes
    ///
    /// This is apart of the default configuration for Split.
    ///
    /// See [`retain`] for an alternative display option.
    ///
    /// [`Table`]: crate::Table
    pub fn clean(self) -> Self {
        Self {
            display: Display::Clean,
            ..self
        }
    }

    /// Returns a split [`Table`] with all cells retained.
    ///
    /// ```text
    ///                                                 +---+---+---+
    ///                                                 | a | b | c |
    ///                                                 +---+---+---+
    /// +---+---+---+---+---+                           | d | e |   |
    /// | a | b | c | d | e | Split::column(3).retain() +---+---+---+
    /// +---+---+---+---+---+           =>              | f | g | h |
    /// | f | g | h |   |   |                           +---+---+---+
    /// +---+---+---+---+---+             |-----------> |   |   |   |
    ///               ^   ^               |             +---+---+---+
    ///               |___|_____cells are kept!
    /// ```
    ///
    /// ## Notes
    ///
    /// See [`clean`] for an alternative display option.
    ///
    /// [`Table`]: crate::Table
    pub fn retain(self) -> Self {
        Self {
            display: Display::Retain,
            ..self
        }
    }
}

impl<R, D, Cfg> TableOption<R, D, Cfg> for Split
where
    R: Records + ExactRecords + Resizable,
{
    fn change(&mut self, records: &mut R, _: &mut Cfg, _: &mut D) {
        // prepare many variables
        let Split {
            direction,
            behavior,
            display,
            index: section_length,
        } = *self;
        let columns = records.count_columns();
        let rows = records.count_rows();
        if columns == 0 || rows == 0 {
            return;
        }
        let (primary_length, secondary_length) = match direction {
            Direction::Column => (columns, rows),
            Direction::Row => (rows, columns),
        };
        let sections_per_direction = ceil_div(primary_length, section_length);
        let mut filtered_sections = 0;

        // determine section over direction or vice versa based on behavior
        let (outer_range, inner_range) = match behavior {
            Behavior::Concat => (1..sections_per_direction, 0..secondary_length),
            Behavior::Zip => (0..secondary_length, 1..sections_per_direction),
        };
        for outer_index in outer_range {
            for inner_index in inner_range.clone() {
                let (section_index, from_secondary_index, to_secondary_index) = match behavior {
                    Behavior::Concat => (
                        outer_index,
                        inner_index,
                        inner_index + outer_index * secondary_length - filtered_sections,
                    ),
                    Behavior::Zip => (
                        inner_index,
                        usize::checked_sub(outer_index * sections_per_direction, filtered_sections)
                            .unwrap_or(0),
                        outer_index * sections_per_direction + inner_index - filtered_sections,
                    ),
                };

                // mutate table
                match (direction, behavior) {
                    (Direction::Column, Behavior::Concat) => records.push_row(),
                    (Direction::Column, Behavior::Zip) => records.insert_row(to_secondary_index),
                    (Direction::Row, Behavior::Concat) => records.push_column(),
                    (Direction::Row, Behavior::Zip) => records.insert_column(to_secondary_index),
                }

                // copy cells to new location
                let mut section_is_empty = true;
                for to_primary_index in 0..section_length {
                    let from_primary_index = to_primary_index + section_index * section_length;

                    if from_primary_index < primary_length {
                        let from_position =
                            format_position(direction, from_primary_index, from_secondary_index);
                        if records.get_cell(from_position).as_ref() != "" {
                            section_is_empty = false;
                        }
                        records.swap(
                            from_position,
                            format_position(direction, to_primary_index, to_secondary_index),
                        );
                    }
                }

                // remove empty rows/columns if cell retention is disabled
                if section_is_empty && display == Display::Clean {
                    match direction {
                        Direction::Column => records.remove_row(to_secondary_index),
                        Direction::Row => records.remove_column(to_secondary_index),
                    }
                    filtered_sections += 1;
                }
            }
        }

        // reduce table to selected index
        for segment in (section_length..primary_length).rev() {
            match direction {
                Direction::Column => records.remove_column(segment),
                Direction::Row => records.remove_row(segment),
            }
        }
    }
}

fn format_position(direction: Direction, primary_index: usize, secondary_index: usize) -> Position {
    match direction {
        Direction::Column => (secondary_index, primary_index),
        Direction::Row => (primary_index, secondary_index),
    }
}

fn ceil_div(x: usize, y: usize) -> usize {
    debug_assert!(x != 0);
    1 + ((x - 1) / y)
}
