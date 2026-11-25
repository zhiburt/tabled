//! This module contains a [`Highlight`] primitive, which helps
//! changing a [`Border`] style of any segment on a [`Table`].
//!
//! [`Table`]: crate::Table

use std::collections::HashSet;

use crate::{
    grid::{
        ansi::ANSIBuf,
        config::{Border, ColoredConfig, Entity, Position, SpannedConfig},
        records::{ExactRecords, Records},
    },
    settings::{
        object::Object,
        style::{Border as ConstBorder, BorderColor},
        Color, TableOption,
    },
};

/// Highlight modifies a table style by changing a border of a target [`Table`] segment.
///
/// It basically highlights outer border of a given segment.
///
/// # Example
///
/// ```
/// use tabled::{
///     Table,
///     settings::{
///         Highlight, Border, Style,
///         object::{Segment, Object}
///     }
/// };
///
/// let data = [
///     ("ELF", "Extensible Linking Format", true),
///     ("DWARF", "", true),
///     ("PE", "Portable Executable", false),
/// ];
///
/// let table = Table::new(data.iter().enumerate())
///                .with(Style::markdown())
///                .with(Highlight::outline(Segment::all().not((0,0).and((1, 0)).and((0, 1)).and((0, 3))), '*'))
///                .to_string();
///
/// assert_eq!(
///     table,
///     concat!(
///         "                *****************************        \n",
///         "| usize | &str  * &str                      * bool  |\n",
///         "|-------*********---------------------------*********\n",
///         "| 0     * ELF   | Extensible Linking Format | true  *\n",
///         "*********                                           *\n",
///         "* 1     | DWARF |                           | true  *\n",
///         "*                                                   *\n",
///         "* 2     | PE    | Portable Executable       | false *\n",
///         "*****************************************************",
///     ),
/// );
/// ```
///
/// [`Table`]: crate::Table
#[derive(Debug)]
pub struct Highlight<O> {
    target: O,
    border: Option<Border<char>>,
    color: Option<Border<ANSIBuf>>,
    reverse: bool,
}

impl<O> Highlight<O> {
    /// Build a new instance of [`Highlight`] with '*' char being used as changer.
    ///
    /// BE AWARE: if target exceeds boundaries it may panic.
    pub const fn new(target: O) -> Self {
        Self::_new(target, None, None)
    }

    /// Build a new instance of [`Highlight`],
    /// highlighting by a character.
    ///
    /// BE AWARE: if target exceeds boundaries it may panic.
    pub const fn outline(target: O, c: char) -> Self {
        Self::_new(target, Some(Border::filled(c)), None)
    }

    /// Build a new instance of [`Highlight`],
    /// highlighting by a color and a given character for a border.
    ///
    /// BE AWARE: if target exceeds boundaries it may panic.
    pub fn colored(target: O, color: Color) -> Self {
        let color = Border::filled(&color).cloned().convert();
        Self::_new(target, None, Some(color))
    }

    /// Set a border for a [`Highlight`].
    pub fn border<T, B, L, R>(self, border: ConstBorder<T, B, L, R>) -> Self {
        let border = border.into_inner();
        Self {
            target: self.target,
            border: Some(border),
            color: self.color,
            reverse: self.reverse,
        }
    }

    /// Set a border color for a [`Highlight`].
    pub fn color(self, border: BorderColor) -> Self {
        let border = border.into_inner();
        let border = border.convert();

        Self {
            target: self.target,
            border: self.border,
            color: Some(border),
            reverse: self.reverse,
        }
    }

    /// Reverse the highlight direction so that borders are drawn from the surrounding
    /// unselected cells' side along the boundary shared with the target segment.
    ///
    /// Exterior edges of the target segment that are not adjacent to any unselected cell
    /// are left unchanged, and the distant outer perimeter of the table is not modified.
    ///
    /// # Example
    ///
    /// ```
    /// use tabled::{
    ///     Table,
    ///     settings::{
    ///         Border, Highlight, Span, Style,
    ///         object::Segment,
    ///         themes::BorderCorrection,
    ///     },
    /// };
    ///
    /// let data = [
    ///     ["0", "1", "2", "3"],
    ///     ["4", "5", "6", "7"],
    ///     ["8", "9", "x", ""],
    ///     ["c", "d", "", ""],
    /// ];
    ///
    /// let mut table = Table::nohead(data);
    /// table
    ///     .with(Style::modern())
    ///     .modify((2, 2), (Span::column(2), Span::row(2)))
    ///     .with(BorderCorrection::span())
    ///     .modify(Segment::new(2.., 2..), Border::filled(' '))
    ///     .with(
    ///         Highlight::new(Segment::new(2.., 2..))
    ///             .border(Border::inherit(Style::modern()))
    ///             .reverse(),
    ///     );
    ///
    /// assert_eq!(
    ///     table.to_string(),
    ///     concat!(
    ///         "┌───┬───┬───┬───┐\n",
    ///         "│ 0 │ 1 │ 2 │ 3 │\n",
    ///         "├───┼───┼───┼───┤\n",
    ///         "│ 4 │ 5 │ 6 │ 7 │\n",
    ///         "├───┼───┼───┴───┘\n",
    ///         "│ 8 │ 9 │ x      \n",
    ///         "├───┼───┤        \n",
    ///         "│ c │ d │        \n",
    ///         "└───┴───┘        ",
    ///     ),
    /// );
    /// ```
    pub const fn reverse(mut self) -> Self {
        self.reverse = true;
        self
    }

    /// Build a new instance of [`Highlight`]
    ///
    /// BE AWARE: if target exceeds boundaries it may panic.
    const fn _new(target: O, border: Option<Border<char>>, color: Option<Border<ANSIBuf>>) -> Self {
        Self {
            target,
            border,
            color,
            reverse: false,
        }
    }
}

impl<O, R, D> TableOption<R, ColoredConfig, D> for Highlight<O>
where
    O: Object<R>,
    R: Records + ExactRecords,
{
    fn change(self, records: &mut R, cfg: &mut ColoredConfig, _: &mut D) {
        let count_rows = records.count_rows();
        let count_cols = records.count_columns();

        if self.reverse {
            let selected = collect_cells(self.target.cells(records), count_rows, count_cols);
            if selected.is_empty() {
                return;
            }

            let shape = (count_rows, count_cols);
            match (self.border, self.color) {
                (None, Some(color)) => {
                    set_reverse_border_color(cfg, &selected, shape, &color);
                }
                (Some(border), None) => {
                    set_reverse_border(cfg, &selected, shape, border);
                }
                (Some(border), Some(color)) => {
                    set_reverse_border(cfg, &selected, shape, border);
                    set_reverse_border_color(cfg, &selected, shape, &color);
                }
                (None, None) => {
                    // noop
                }
            }

            return;
        }

        let cells = self.target.cells(records);
        let segments = split_segments(cells, count_rows, count_cols);

        match (self.border, self.color) {
            (None, Some(color)) => {
                for sector in segments {
                    set_border_color(cfg, &sector, &color);
                }
            }
            (Some(border), None) => {
                for sector in segments {
                    set_border(cfg, &sector, border);
                }
            }
            (Some(border), Some(color)) => {
                for sector in segments {
                    set_border(cfg, &sector, border);
                    set_border_color(cfg, &sector, &color);
                }
            }
            (None, None) => {
                // noop
            }
        }
    }

    fn hint_change(&self) -> Option<Entity> {
        None
    }
}

fn collect_cells(
    cells: impl Iterator<Item = Entity>,
    count_rows: usize,
    count_cols: usize,
) -> HashSet<Position> {
    let mut selected = HashSet::new();
    for entity in cells {
        for cell in entity.iter(count_rows, count_cols) {
            if cell.row < count_rows && cell.col < count_cols {
                let _ = selected.insert(cell);
            }
        }
    }
    selected
}

fn set_reverse_border(
    cfg: &mut SpannedConfig,
    selected: &HashSet<Position>,
    shape: (usize, usize),
    border: Border<char>,
) {
    if selected.is_empty() {
        return;
    }

    for &pos in selected {
        let mut b = build_reverse_cell_border(selected, shape, pos, &border);
        b.left_top_corner = refine_reverse_vertex_char(
            cfg,
            selected,
            shape,
            pos.row,
            pos.col,
            &border,
            b.left_top_corner,
        );
        b.right_top_corner = refine_reverse_vertex_char(
            cfg,
            selected,
            shape,
            pos.row,
            pos.col + 1,
            &border,
            b.right_top_corner,
        );
        b.left_bottom_corner = refine_reverse_vertex_char(
            cfg,
            selected,
            shape,
            pos.row + 1,
            pos.col,
            &border,
            b.left_bottom_corner,
        );
        b.right_bottom_corner = refine_reverse_vertex_char(
            cfg,
            selected,
            shape,
            pos.row + 1,
            pos.col + 1,
            &border,
            b.right_bottom_corner,
        );
        cfg.set_border(pos, b);
    }
}

fn set_reverse_border_color(
    cfg: &mut SpannedConfig,
    selected: &HashSet<Position>,
    shape: (usize, usize),
    border: &Border<ANSIBuf>,
) {
    if selected.is_empty() {
        return;
    }

    for &pos in selected {
        let b = build_reverse_cell_border(selected, shape, pos, border);
        cfg.set_border_color(pos, b);
    }
}

fn build_reverse_cell_border<T>(
    selected: &HashSet<Position>,
    shape: (usize, usize),
    p: Position,
    border: &Border<T>,
) -> Border<T>
where
    T: Default + Clone,
{
    let (count_rows, count_cols) = shape;
    let mut b = Border::default();

    if p.row > 0 && is_unselected(selected, p.row - 1, p.col, count_rows, count_cols) {
        b.top = border.bottom.clone();
    }

    if is_unselected(selected, p.row + 1, p.col, count_rows, count_cols) {
        b.bottom = border.top.clone();
    }

    if p.col > 0 && is_unselected(selected, p.row, p.col - 1, count_rows, count_cols) {
        b.left = border.right.clone();
    }

    if is_unselected(selected, p.row, p.col + 1, count_rows, count_cols) {
        b.right = border.left.clone();
    }

    b.left_top_corner = build_reverse_vertex(selected, shape, p.row, p.col, border);
    b.right_top_corner = build_reverse_vertex(selected, shape, p.row, p.col + 1, border);
    b.left_bottom_corner = build_reverse_vertex(selected, shape, p.row + 1, p.col, border);
    b.right_bottom_corner = build_reverse_vertex(selected, shape, p.row + 1, p.col + 1, border);

    b
}

fn vertex_unselected_quadrants(
    selected: &HashSet<Position>,
    (count_rows, count_cols): (usize, usize),
    row: usize,
    col: usize,
) -> (bool, bool, bool, bool) {
    let nw = row > 0 && col > 0 && is_unselected(selected, row - 1, col - 1, count_rows, count_cols);
    let ne = row > 0 && is_unselected(selected, row - 1, col, count_rows, count_cols);
    let sw = col > 0 && is_unselected(selected, row, col - 1, count_rows, count_cols);
    let se = is_unselected(selected, row, col, count_rows, count_cols);

    (nw, ne, sw, se)
}

fn build_reverse_vertex<T>(
    selected: &HashSet<Position>,
    shape: (usize, usize),
    row: usize,
    col: usize,
    border: &Border<T>,
) -> Option<T>
where
    T: Clone,
{
    match vertex_unselected_quadrants(selected, shape, row, col) {
        (false, false, false, false) | (true, true, true, true) => None,

        (true, false, false, false) => border.right_bottom_corner.clone(),
        (false, true, false, false) => border.left_bottom_corner.clone(),
        (false, false, true, false) => border.right_top_corner.clone(),
        (false, false, false, true) => border.left_top_corner.clone(),

        (true, true, false, false) => border.bottom.clone(),
        (false, false, true, true) => border.top.clone(),
        (true, false, true, false) => border.right.clone(),
        (false, true, false, true) => border.left.clone(),

        (true, true, true, false) => border.left_top_corner.clone(),
        (true, true, false, true) => border.right_top_corner.clone(),
        (true, false, true, true) => border.left_bottom_corner.clone(),
        (false, true, true, true) => border.right_bottom_corner.clone(),

        (true, false, false, true) => border
            .left_top_corner
            .clone()
            .or_else(|| border.right_bottom_corner.clone()),
        (false, true, true, false) => border
            .right_top_corner
            .clone()
            .or_else(|| border.left_bottom_corner.clone()),
    }
}

fn refine_reverse_vertex_char(
    cfg: &SpannedConfig,
    selected: &HashSet<Position>,
    shape: (usize, usize),
    row: usize,
    col: usize,
    border: &Border<char>,
    default: Option<char>,
) -> Option<char> {
    if default.is_none() {
        return None;
    }

    let table = cfg.get_borders();
    let matches_h =
        |c: Option<char>| c.is_some() && (c == table.horizontal || c == table.top || c == table.bottom);
    let matches_v =
        |c: Option<char>| c.is_some() && (c == table.vertical || c == table.left || c == table.right);

    match vertex_unselected_quadrants(selected, shape, row, col) {
        (true, true, false, false) => {
            if matches_h(border.bottom) && has_vertical_divider(cfg, shape, row - 1, col) {
                table.bottom_intersection.or(default)
            } else {
                default
            }
        }
        (false, false, true, true) => {
            if matches_h(border.top) && has_vertical_divider(cfg, shape, row, col) {
                table.top_intersection.or(default)
            } else {
                default
            }
        }
        (true, false, true, false) => {
            if matches_v(border.right) && has_horizontal_divider(cfg, shape, row, col - 1) {
                table.right_intersection.or(default)
            } else {
                default
            }
        }
        (false, true, false, true) => {
            if matches_v(border.left) && has_horizontal_divider(cfg, shape, row, col) {
                table.left_intersection.or(default)
            } else {
                default
            }
        }
        (true, true, true, false) => {
            if matches_h(border.bottom)
                && matches_v(border.right)
                && border.left_top_corner == table.top_left
            {
                let up = has_vertical_divider(cfg, shape, row - 1, col);
                let left = has_horizontal_divider(cfg, shape, row, col - 1);
                match (up, left) {
                    (true, true) => table.intersection.or(default),
                    (true, false) => table.left_intersection.or(default),
                    (false, true) => table.top_intersection.or(default),
                    (false, false) => default,
                }
            } else {
                default
            }
        }
        (true, true, false, true) => {
            if matches_h(border.bottom)
                && matches_v(border.left)
                && border.right_top_corner == table.top_right
            {
                let up = has_vertical_divider(cfg, shape, row - 1, col);
                let right = has_horizontal_divider(cfg, shape, row, col);
                match (up, right) {
                    (true, true) => table.intersection.or(default),
                    (true, false) => table.right_intersection.or(default),
                    (false, true) => table.top_intersection.or(default),
                    (false, false) => default,
                }
            } else {
                default
            }
        }
        (true, false, true, true) => {
            if matches_h(border.top)
                && matches_v(border.right)
                && border.left_bottom_corner == table.bottom_left
            {
                let down = has_vertical_divider(cfg, shape, row, col);
                let left = has_horizontal_divider(cfg, shape, row, col - 1);
                match (down, left) {
                    (true, true) => table.intersection.or(default),
                    (true, false) => table.left_intersection.or(default),
                    (false, true) => table.bottom_intersection.or(default),
                    (false, false) => default,
                }
            } else {
                default
            }
        }
        (false, true, true, true) => {
            if matches_h(border.top)
                && matches_v(border.left)
                && border.right_bottom_corner == table.bottom_right
            {
                let down = has_vertical_divider(cfg, shape, row, col);
                let right = has_horizontal_divider(cfg, shape, row, col);
                match (down, right) {
                    (true, true) => table.intersection.or(default),
                    (true, false) => table.right_intersection.or(default),
                    (false, true) => table.bottom_intersection.or(default),
                    (false, false) => default,
                }
            } else {
                default
            }
        }
        (true, false, false, true) => {
            if matches_h(border.top)
                && matches_h(border.bottom)
                && matches_v(border.left)
                && matches_v(border.right)
                && (border.left_top_corner == table.top_left
                    || border.right_bottom_corner == table.bottom_right)
            {
                table.intersection.or(default)
            } else {
                default
            }
        }
        (false, true, true, false) => {
            if matches_h(border.top)
                && matches_h(border.bottom)
                && matches_v(border.left)
                && matches_v(border.right)
                && (border.right_top_corner == table.top_right
                    || border.left_bottom_corner == table.bottom_left)
            {
                table.intersection.or(default)
            } else {
                default
            }
        }
        _ => default,
    }
}

fn is_unselected(
    selected: &HashSet<Position>,
    row: usize,
    col: usize,
    count_rows: usize,
    count_cols: usize,
) -> bool {
    row < count_rows && col < count_cols && !selected.contains(&Position::new(row, col))
}

fn has_vertical_divider(
    cfg: &SpannedConfig,
    shape: (usize, usize),
    row: usize,
    col: usize,
) -> bool {
    let pos = Position::new(row, col);
    if cfg.is_cell_covered_by_column_span(pos) || cfg.is_cell_covered_by_both_spans(pos) {
        return false;
    }

    matches!(cfg.get_border(pos, shape).left, Some(c) if c != ' ')
}

fn has_horizontal_divider(
    cfg: &SpannedConfig,
    shape: (usize, usize),
    row: usize,
    col: usize,
) -> bool {
    let pos = Position::new(row, col);
    if cfg.is_cell_covered_by_row_span(pos) || cfg.is_cell_covered_by_both_spans(pos) {
        return false;
    }

    matches!(cfg.get_border(pos, shape).top, Some(c) if c != ' ')
}

fn set_border_color(cfg: &mut SpannedConfig, sector: &HashSet<Position>, border: &Border<ANSIBuf>) {
    if sector.is_empty() {
        return;
    }
    let color = border.clone();
    for &p in sector {
        let border = build_cell_border(sector, p, &color);
        cfg.set_border_color(p, border);
    }
}

fn split_segments(
    cells: impl Iterator<Item = Entity>,
    count_rows: usize,
    count_cols: usize,
) -> Vec<HashSet<Position>> {
    let mut segments: Vec<HashSet<Position>> = Vec::new();
    for entity in cells {
        for cell in entity.iter(count_rows, count_cols) {
            let found_segment = segments
                .iter_mut()
                .find(|s| s.iter().any(|&c| is_cell_connected(cell, c)));

            match found_segment {
                Some(segment) => {
                    let _ = segment.insert(cell);
                }
                None => {
                    let mut segment = HashSet::new();
                    let _ = segment.insert(cell);
                    segments.push(segment);
                }
            }
        }
    }

    let mut squashed_segments: Vec<HashSet<Position>> = Vec::new();
    while !segments.is_empty() {
        let mut segment = segments.remove(0);

        let mut i = 0;
        while i < segments.len() {
            if is_segment_connected(&segment, &segments[i]) {
                segment.extend(&segments[i]);
                let _ = segments.remove(i);
            } else {
                i += 1;
            }
        }

        squashed_segments.push(segment);
    }

    squashed_segments
}

fn is_cell_connected(p1: Position, p2: Position) -> bool {
    if p1.col == p2.col {
        let up = p1.row == p2.row + 1;
        let down = p2.row > 0 && p1.row == p2.row - 1;

        if up || down {
            return true;
        }
    }

    if p1.row == p2.row {
        let left = p2.col > 0 && p1.col == p2.col - 1;
        let right = p1.col == p2.col + 1;

        if left || right {
            return true;
        }
    }

    false
}

fn is_segment_connected(segment1: &HashSet<Position>, segment2: &HashSet<Position>) -> bool {
    for &cell1 in segment1.iter() {
        for &cell2 in segment2.iter() {
            if is_cell_connected(cell1, cell2) {
                return true;
            }
        }
    }

    false
}

fn set_border(cfg: &mut SpannedConfig, sector: &HashSet<Position>, border: Border<char>) {
    if sector.is_empty() {
        return;
    }

    for &pos in sector {
        let border = build_cell_border(sector, pos, &border);
        cfg.set_border(pos, border);
    }
}

fn build_cell_border<T>(sector: &HashSet<Position>, p: Position, border: &Border<T>) -> Border<T>
where
    T: Default + Clone,
{
    let has_top_neighbor = has_top_neighbor(sector, p);
    let has_bottom_neighbor = has_bottom_neighbor(sector, p);
    let has_left_neighbor = has_left_neighbor(sector, p);
    let has_right_neighbor = has_right_neighbor(sector, p);
    let has_left_top_neighbor = has_left_top_neighbor(sector, p);
    let has_right_top_neighbor = has_right_top_neighbor(sector, p);
    let has_left_bottom_neighbor = has_left_bottom_neighbor(sector, p);
    let has_right_bottom_neighbor = has_right_bottom_neighbor(sector, p);

    let mut b = Border::default();

    if let Some(c) = border.top.clone() {
        if !has_top_neighbor {
            b.top = Some(c.clone());

            if has_right_neighbor && !has_right_top_neighbor {
                b.right_top_corner = Some(c);
            }
        }
    }

    if let Some(c) = border.bottom.clone() {
        if !has_bottom_neighbor {
            b.bottom = Some(c.clone());

            if has_right_neighbor && !has_right_bottom_neighbor {
                b.right_bottom_corner = Some(c);
            }
        }
    }

    if let Some(c) = border.left.clone() {
        if !has_left_neighbor {
            b.left = Some(c.clone());

            if has_bottom_neighbor && !has_left_bottom_neighbor {
                b.left_bottom_corner = Some(c);
            }
        }
    }

    if let Some(c) = border.right.clone() {
        if !has_right_neighbor {
            b.right = Some(c.clone());

            if has_bottom_neighbor && !has_right_bottom_neighbor {
                b.right_bottom_corner = Some(c);
            }
        }
    }

    if let Some(c) = border.left_top_corner.clone() {
        if !has_left_neighbor && !has_top_neighbor {
            b.left_top_corner = Some(c);
        }
    }

    if let Some(c) = border.left_bottom_corner.clone() {
        if !has_left_neighbor && !has_bottom_neighbor {
            b.left_bottom_corner = Some(c);
        }
    }

    if let Some(c) = border.right_top_corner.clone() {
        if !has_right_neighbor && !has_top_neighbor {
            b.right_top_corner = Some(c);
        }
    }

    if let Some(c) = border.right_bottom_corner.clone() {
        if !has_right_neighbor && !has_bottom_neighbor {
            b.right_bottom_corner = Some(c);
        }
    }

    {
        if !has_bottom_neighbor {
            if !has_left_neighbor && has_left_top_neighbor {
                if let Some(c) = border.right_top_corner.clone() {
                    b.left_top_corner = Some(c);
                }
            }

            if has_left_neighbor && has_left_bottom_neighbor {
                if let Some(c) = border.left_top_corner.clone() {
                    b.left_bottom_corner = Some(c);
                }
            }

            if !has_right_neighbor && has_right_top_neighbor {
                if let Some(c) = border.left_top_corner.clone() {
                    b.right_top_corner = Some(c);
                }
            }

            if has_right_neighbor && has_right_bottom_neighbor {
                if let Some(c) = border.right_top_corner.clone() {
                    b.right_bottom_corner = Some(c);
                }
            }
        }

        if !has_top_neighbor {
            if !has_left_neighbor && has_left_bottom_neighbor {
                if let Some(c) = border.right_bottom_corner.clone() {
                    b.left_bottom_corner = Some(c);
                }
            }

            if has_left_neighbor && has_left_top_neighbor {
                if let Some(c) = border.left_bottom_corner.clone() {
                    b.left_top_corner = Some(c);
                }
            }

            if !has_right_neighbor && has_right_bottom_neighbor {
                if let Some(c) = border.left_bottom_corner.clone() {
                    b.right_bottom_corner = Some(c);
                }
            }

            if has_right_neighbor && has_right_top_neighbor {
                if let Some(c) = border.right_bottom_corner.clone() {
                    b.right_top_corner = Some(c);
                }
            }
        }
    }

    b
}

fn has_top_neighbor(sector: &HashSet<Position>, p: Position) -> bool {
    p.row > 0 && sector.contains(&(p - (1, 0)))
}

fn has_bottom_neighbor(sector: &HashSet<Position>, p: Position) -> bool {
    sector.contains(&(p + (1, 0)))
}

fn has_left_neighbor(sector: &HashSet<Position>, p: Position) -> bool {
    p.col > 0 && sector.contains(&(p - (0, 1)))
}

fn has_right_neighbor(sector: &HashSet<Position>, p: Position) -> bool {
    sector.contains(&(p + (0, 1)))
}

fn has_left_top_neighbor(sector: &HashSet<Position>, p: Position) -> bool {
    p.row > 0 && p.col > 0 && sector.contains(&(p - (1, 1)))
}

fn has_right_top_neighbor(sector: &HashSet<Position>, p: Position) -> bool {
    p.row > 0 && sector.contains(&(p - (1, 0) + (0, 1)))
}

fn has_left_bottom_neighbor(sector: &HashSet<Position>, p: Position) -> bool {
    p.col > 0 && sector.contains(&(p + (1, 0) - (0, 1)))
}

fn has_right_bottom_neighbor(sector: &HashSet<Position>, p: Position) -> bool {
    sector.contains(&(p + (1, 1)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_connected() {
        assert!(is_cell_connected(Position::new(0, 0), Position::new(0, 1)));
        assert!(is_cell_connected(Position::new(0, 0), Position::new(1, 0)));
        assert!(!is_cell_connected(Position::new(0, 0), Position::new(1, 1)));

        assert!(is_cell_connected(Position::new(0, 1), Position::new(0, 0)));
        assert!(is_cell_connected(Position::new(1, 0), Position::new(0, 0)));
        assert!(!is_cell_connected(Position::new(1, 1), Position::new(0, 0)));

        assert!(is_cell_connected(Position::new(1, 1), Position::new(0, 1)));
        assert!(is_cell_connected(Position::new(1, 1), Position::new(1, 0)));
        assert!(is_cell_connected(Position::new(1, 1), Position::new(2, 1)));
        assert!(is_cell_connected(Position::new(1, 1), Position::new(1, 2)));
        assert!(!is_cell_connected(Position::new(1, 1), Position::new(1, 1)));
    }
}
