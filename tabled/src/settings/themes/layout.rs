//! Module contains [`Layout`] setting.

use papergrid::records::{ExactRecords, PeekableRecords, Records};

use crate::{
    grid::{
        config::{AlignmentHorizontal, AlignmentVertical},
        records::{RecordsMut, Resizable},
    },
    settings::{Alignment, Rotate, TableOption},
};

/// Layout can be used to move header to a specific corner.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Layout {
    orientation: HeadPosition,
    footer: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum HeadPosition {
    Top,
    Bottom,
    Left,
    Right,
}

impl Layout {
    /// Construct a new layout setting.
    pub fn new(stick: Alignment, footer: bool) -> Self {
        let orientation = convert_orientation(stick);

        Self {
            footer,
            orientation,
        }
    }
}

impl<R, C, D> TableOption<R, C, D> for Layout
where
    R: Records + Resizable + ExactRecords + PeekableRecords + RecordsMut<String>,
{
    fn change(self, records: &mut R, _: &mut C, _: &mut D) {
        move_head_if(records, self.orientation);

        if self.footer {
            copy_head(records, self.orientation);
        }
    }
}

const fn convert_orientation(position: Alignment) -> HeadPosition {
    match (position.as_horizontal(), position.as_vertical()) {
        (None, Some(AlignmentVertical::Top)) => HeadPosition::Top,
        (None, Some(AlignmentVertical::Bottom)) => HeadPosition::Bottom,
        (Some(AlignmentHorizontal::Left), None) => HeadPosition::Left,
        (Some(AlignmentHorizontal::Right), None) => HeadPosition::Right,
        (None, Some(AlignmentVertical::Center)) => HeadPosition::Top,
        (Some(AlignmentHorizontal::Center), None) => HeadPosition::Top,
        (None, None) | (Some(_), Some(_)) => HeadPosition::Top,
    }
}

fn copy_head<R>(records: &mut R, orientation: HeadPosition)
where
    R: Records + Resizable + ExactRecords + PeekableRecords + RecordsMut<String>,
{
    let head = collect_head_by(records, orientation);
    match orientation {
        HeadPosition::Top => cp_row(records, head, records.count_rows()),
        HeadPosition::Bottom => cp_row(records, head, 0),
        HeadPosition::Left => cp_column(records, head, records.count_columns()),
        HeadPosition::Right => cp_column(records, head, 0),
    }
}

fn collect_head_by<R>(records: &mut R, orientation: HeadPosition) -> Vec<String>
where
    R: Records + PeekableRecords + ExactRecords,
{
    match orientation {
        HeadPosition::Top => collect_head(records, 0),
        HeadPosition::Bottom => collect_head(records, records.count_rows() - 1),
        HeadPosition::Left => collect_head_vertical(records, 0),
        HeadPosition::Right => collect_head_vertical(records, records.count_columns() - 1),
    }
}

fn cp_row<R>(records: &mut R, row: Vec<String>, pos: usize)
where
    R: Records + Resizable + ExactRecords + PeekableRecords + RecordsMut<String>,
{
    records.insert_row(pos);

    for (col, text) in row.into_iter().enumerate() {
        records.set((pos, col), text);
    }
}

fn cp_column<R>(records: &mut R, column: Vec<String>, pos: usize)
where
    R: Records + Resizable + ExactRecords + PeekableRecords + RecordsMut<String>,
{
    records.insert_column(pos);

    for (row, text) in column.into_iter().enumerate() {
        records.set((row, pos), text);
    }
}

fn move_head_if<R>(records: &mut R, orientation: HeadPosition)
where
    R: Records + Resizable + ExactRecords + PeekableRecords + RecordsMut<String>,
{
    match orientation {
        HeadPosition::Top => {}
        HeadPosition::Bottom => {
            let head = collect_head(records, 0);
            push_row(records, head);
            records.remove_row(0);
        }
        HeadPosition::Left => {
            Rotate::Left.change(records, &mut (), &mut ());
            Rotate::Bottom.change(records, &mut (), &mut ());
        }
        HeadPosition::Right => {
            Rotate::Right.change(records, &mut (), &mut ());
        }
    }
}

fn collect_head<R>(records: &mut R, row: usize) -> Vec<String>
where
    R: Records + PeekableRecords,
{
    (0..records.count_columns())
        .map(|column| records.get_text((row, column)))
        .map(ToString::to_string)
        .collect()
}

fn collect_head_vertical<R>(records: &mut R, column: usize) -> Vec<String>
where
    R: Records + PeekableRecords + ExactRecords,
{
    (0..records.count_rows())
        .map(|row| records.get_text((row, column)))
        .map(ToString::to_string)
        .collect()
}

fn push_row<R>(records: &mut R, row: Vec<String>)
where
    R: Records + ExactRecords + Resizable + RecordsMut<String>,
{
    records.push_row();

    let last_row = records.count_rows() - 1;

    for (col, text) in row.into_iter().enumerate() {
        records.set((last_row, col), text);
    }
}
