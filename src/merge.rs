//! The module contains a set of methods to merge cells together via [`Span`]s.
//!
//! [`Span`]: crate::Span

use crate::{papergrid::records::Records, Table, TableOption};

/// Merge to combine duplicates together, using [`Span`].
///
/// [`Span`]: crate::Span
#[derive(Debug)]
pub struct Merge;

impl Merge {
    /// Vertical merge.
    pub fn vertical() -> MergeDuplicatesVertical {
        MergeDuplicatesVertical
    }

    /// Horizontal merge.
    pub fn horizontal() -> MergeDuplicatesHorizontal {
        MergeDuplicatesHorizontal
    }
}

/// A modificator for [`Table`] which looks up for duplicates in columns and
/// in case of duplicate merges the cells together using [`Span`].
///
/// [`Table`]: crate::Table
/// [`Span`]: crate::Span
#[derive(Debug)]
pub struct MergeDuplicatesVertical;

impl<R> TableOption<R> for MergeDuplicatesVertical
where
    R: Records,
{
    fn change(&mut self, table: &mut Table<R>) {
        if table.is_empty() {
            return;
        }

        for column in 0..table.get_records().count_columns() {
            let mut repeat_length = 0;
            let mut repeat_value = String::new();
            let mut repeat_is_set = false;
            let mut last_is_row_span = false;
            for row in (0..table.get_records().count_rows()).rev() {
                if last_is_row_span {
                    last_is_row_span = false;
                    continue;
                }

                // we need to mitigate messing existing spans
                let is_cell_visible = table
                    .get_config()
                    .is_cell_visible((row, column), table.shape());

                let is_row_span_cell = table
                    .get_config()
                    .get_column_span((row, column), table.shape())
                    .is_some();

                if !repeat_is_set {
                    if !is_cell_visible {
                        continue;
                    }

                    if is_row_span_cell {
                        continue;
                    }

                    repeat_length = 1;
                    repeat_value = table.get_records().get_text((row, column)).to_owned();
                    repeat_is_set = true;
                    continue;
                }

                if is_row_span_cell {
                    repeat_is_set = false;
                    last_is_row_span = true;
                    continue;
                }

                if !is_cell_visible {
                    repeat_is_set = false;
                    continue;
                }

                let text = table.get_records().get_text((row, column));
                let is_duplicate = text == repeat_value;

                if is_duplicate {
                    repeat_length += 1;
                    continue;
                }

                if repeat_length > 1 {
                    table
                        .get_config_mut()
                        .set_row_span((row + 1, column), repeat_length);
                }

                repeat_length = 1;
                repeat_value = table.get_records().get_text((row, column)).to_owned();
            }

            if repeat_length > 1 {
                table
                    .get_config_mut()
                    .set_row_span((0, column), repeat_length);
            }
        }
    }
}

/// A modificator for [`Table`] which looks up for duplicates in rows and
/// in case of duplicate merges the cells together using [`Span`].
///
/// [`Table`]: crate::Table
/// [`Span`]: crate::Span
#[derive(Debug)]
pub struct MergeDuplicatesHorizontal;

impl<R> TableOption<R> for MergeDuplicatesHorizontal
where
    R: Records,
{
    fn change(&mut self, table: &mut Table<R>) {
        if table.is_empty() {
            return;
        }

        for row in 0..table.get_records().count_rows() {
            let mut repeat_length = 0;
            let mut repeat_value = String::new();
            let mut repeat_is_set = false;
            let mut last_is_col_span = false;
            for column in (0..table.get_records().count_columns()).rev() {
                if last_is_col_span {
                    last_is_col_span = false;
                    continue;
                }

                // we need to mitigate messing existing spans
                let is_cell_visible = table
                    .get_config()
                    .is_cell_visible((row, column), table.shape());

                let is_col_span_cell = table
                    .get_config()
                    .get_row_span((row, column), table.shape())
                    .is_some();

                if !repeat_is_set {
                    if !is_cell_visible {
                        continue;
                    }

                    if is_col_span_cell {
                        continue;
                    }

                    repeat_length = 1;
                    repeat_value = table.get_records().get_text((row, column)).to_owned();
                    repeat_is_set = true;
                    continue;
                }

                if is_col_span_cell {
                    repeat_is_set = false;
                    last_is_col_span = true;
                    continue;
                }

                if !is_cell_visible {
                    repeat_is_set = false;
                    continue;
                }

                let text = table.get_records().get_text((row, column));
                let is_duplicate = text == repeat_value;

                if is_duplicate {
                    repeat_length += 1;
                    continue;
                }

                if repeat_length > 1 {
                    table
                        .get_config_mut()
                        .set_column_span((row, column + 1), repeat_length);
                }

                repeat_length = 1;
                repeat_value = table.get_records().get_text((row, column)).to_owned();
            }

            if repeat_length > 1 {
                table
                    .get_config_mut()
                    .set_column_span((row, 0), repeat_length);
            }
        }
    }
}
