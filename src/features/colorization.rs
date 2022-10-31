//! A colorization module.

use papergrid::{
    records::{Records, RecordsMut},
    width::CfgWidthFunction,
};

use crate::{color::Color, CellOption, TableOption};

/// Colorize a table via pattern.
#[derive(Debug)]
pub struct Colorization;

impl Colorization {
    /// Colorize a table to be like a chessboard.
    pub fn chess(first: Color, second: Color) -> ChessColorization {
        ChessColorization { first, second }
    }

    /// Colorize a table row by row.
    pub fn rows(first: Color, second: Color) -> RowsColorization {
        RowsColorization { first, second }
    }

    /// Colorize a table column by column.
    pub fn columns(first: Color, second: Color) -> ColumnsColorization {
        ColumnsColorization { first, second }
    }
}

/// A chessboard pattern.
#[derive(Debug)]
pub struct ChessColorization {
    first: Color,
    second: Color,
}

impl<R> TableOption<R> for ChessColorization
where
    R: Records + RecordsMut<String>,
{
    fn change(&mut self, table: &mut crate::Table<R>) {
        let ctrl = CfgWidthFunction::from_cfg(table.get_config());

        let mut is_first = true;
        for row in 0..table.count_rows() {
            for col in 0..table.count_columns() {
                let color = match is_first {
                    true => &self.first,
                    false => &self.second,
                };

                is_first = !is_first;

                let text = table.get_records().get_text((row, col));
                let text = color.colorize(text);
                table.get_records_mut().set((row, col), text, &ctrl)
            }
        }
    }
}

/// A row by row pattern.
#[derive(Debug)]
pub struct RowsColorization {
    first: Color,
    second: Color,
}

impl<R> TableOption<R> for RowsColorization
where
    R: Records + RecordsMut<String>,
{
    fn change(&mut self, table: &mut crate::Table<R>) {
        let ctrl = CfgWidthFunction::from_cfg(table.get_config());

        let mut is_first = true;
        for row in 0..table.count_rows() {
            let color = match is_first {
                true => &self.first,
                false => &self.second,
            };

            is_first = !is_first;

            for col in 0..table.count_columns() {
                let text = table.get_records().get_text((row, col));
                let text = color.colorize(text);
                table.get_records_mut().set((row, col), text, &ctrl)
            }
        }
    }
}

/// A column by column pattern.
#[derive(Debug)]
pub struct ColumnsColorization {
    first: Color,
    second: Color,
}

impl<R> TableOption<R> for ColumnsColorization
where
    R: Records + RecordsMut<String>,
{
    fn change(&mut self, table: &mut crate::Table<R>) {
        let ctrl = CfgWidthFunction::from_cfg(table.get_config());

        let mut is_first = true;
        for col in 0..table.count_columns() {
            let color = match is_first {
                true => &self.first,
                false => &self.second,
            };

            is_first = !is_first;
            for row in 0..table.count_rows() {
                let text = table.get_records().get_text((row, col));
                let text = color.colorize(text);
                table.get_records_mut().set((row, col), text, &ctrl)
            }
        }
    }
}

/// Embed given ansi to the cells
#[derive(Debug)]
pub struct Colorize(Color);

impl From<Color> for Colorize {
    fn from(color: Color) -> Self {
        Colorize(color)
    }
}

impl<R> CellOption<R> for Colorize
where
    R: Records + RecordsMut<String>,
{
    fn change_cell(&mut self, table: &mut crate::Table<R>, entity: papergrid::Entity) {
        let ctrl = CfgWidthFunction::from_cfg(table.get_config());
        for (row, col) in entity.iter(table.count_rows(), table.count_columns()) {
            let text = table.get_records().get_text((row, col));
            let text = self.0.colorize(text);
            table.get_records_mut().set((row, col), text, &ctrl)
        }
    }
}
