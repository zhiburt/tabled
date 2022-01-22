//! Papergrid is a library for generating text-based tables for display
//!
//! # Example
//! ```rust
//!     use papergrid::{Grid, Entity, Settings, DEFAULT_CELL_STYLE};
//!     let mut grid = Grid::new(2, 2);
//!     grid.set_cell_borders(DEFAULT_CELL_STYLE.clone());
//!
//!     grid.set(&Entity::Cell(0, 0), Settings::new().text("0-0"));
//!     grid.set(&Entity::Cell(0, 1), Settings::new().text("0-1"));
//!     grid.set(&Entity::Cell(1, 0), Settings::new().text("1-0"));
//!     grid.set(&Entity::Cell(1, 1), Settings::new().text("1-1"));
//!
//!     let expected = concat!(
//!         "+---+---+\n",
//!         "|0-0|0-1|\n",
//!         "+---+---+\n",
//!         "|1-0|1-1|\n",
//!         "+---+---+\n",
//!     );
//!
//!     assert_eq!(expected, grid.to_string());
//! ```

use std::{
    cmp::max,
    collections::{BTreeSet, HashMap},
    fmt::{self, Display},
    ops::{Bound, RangeBounds},
};

pub const DEFAULT_CELL_STYLE: Border = Border {
    top: Some('-'),
    bottom: Some('-'),
    left: Some('|'),
    right: Some('|'),
    right_top_corner: Some('+'),
    left_bottom_corner: Some('+'),
    left_top_corner: Some('+'),
    right_bottom_corner: Some('+'),
};

const DEFAULT_SPLIT_BORDER_CHAR: char = ' ';

const DEFAULT_SPLIT_INTERSECTION_CHAR: char = ' ';

/// Grid provides a set of methods for building a text-based table
pub struct Grid {
    size: (usize, usize),
    styles: HashMap<Entity, Style>,
    cells: Vec<Vec<String>>,
    borders: Borders,
}

impl Grid {
    /// The new method creates a grid instance with default styles.
    ///
    /// The size of the grid can not be changed after the instance is created.
    ///
    /// # Example
    ///
    /// ```rust
    ///     use papergrid::{Grid, Entity, Settings, DEFAULT_CELL_STYLE};
    ///     let mut grid = Grid::new(2, 2);
    ///     grid.set_cell_borders(DEFAULT_CELL_STYLE.clone());
    ///     let str = grid.to_string();
    ///     assert_eq!(
    ///          str,
    ///          "+++\n\
    ///           |||\n\
    ///           +++\n\
    ///           |||\n\
    ///           +++\n"
    ///     )
    /// ```
    pub fn new(rows: usize, columns: usize) -> Self {
        let mut styles = HashMap::new();
        styles.insert(Entity::Global, Style::default());

        Grid {
            size: (rows, columns),
            cells: vec![vec![String::new(); columns]; rows],
            styles,
            borders: Borders::new(rows, columns),
        }
    }

    /// Set method is responsible for modification of cell/row/column.
    ///
    /// The method panics if incorrect cell/row/column index is given.
    ///
    /// # Example
    ///
    /// ```rust
    ///     use papergrid::{Grid, Entity, Settings, DEFAULT_CELL_STYLE};
    ///     let mut grid = Grid::new(2, 2);
    ///     grid.set_cell_borders(DEFAULT_CELL_STYLE.clone());
    ///     grid.set(&Entity::Row(0), Settings::new().text("row 1"));
    ///     grid.set(&Entity::Row(1), Settings::new().text("row 2"));
    ///     let str = grid.to_string();
    ///     assert_eq!(
    ///          str,
    ///          "+-----+-----+\n\
    ///           |row 1|row 1|\n\
    ///           +-----+-----+\n\
    ///           |row 2|row 2|\n\
    ///           +-----+-----+\n"
    ///     )
    /// ```
    ///
    pub fn set(&mut self, entity: &Entity, settings: Settings) {
        if let Some(text) = settings.text {
            self.set_text(entity, text);
        }

        if let Some(indent) = settings.indent {
            self.style_mut(entity).indent = indent;
        }

        if let Some(alignment_h) = settings.alignment_h {
            self.style_mut(entity).alignment_h = alignment_h;
        }

        if let Some(alignment_v) = settings.alignment_v {
            self.style_mut(entity).alignment_v = alignment_v;
        }

        if let Some(span) = settings.span {
            self.style_mut(entity).span = span;
        }

        if let Some(border) = settings.border {
            let frame = self.frame_from_entity(entity);
            if settings.border_split_check {
                self.add_split_lines_for_border(&frame, &border);
            }

            self.set_border(&frame, border);
        }
    }

    pub fn add_horizontal_split(&mut self, row: usize) {
        self.insert_horizontal_split(
            row,
            SplitLine::new(
                vec![DEFAULT_SPLIT_BORDER_CHAR; self.count_columns()],
                vec![DEFAULT_SPLIT_INTERSECTION_CHAR; self.borders.need_horizontal_intersections()],
            ),
        );
    }

    pub fn add_vertical_split(&mut self, column: usize) {
        self.insert_vertical_split(
            column,
            SplitLine::new(
                vec![DEFAULT_SPLIT_BORDER_CHAR; self.count_rows()],
                vec![DEFAULT_SPLIT_INTERSECTION_CHAR; self.borders.need_vertical_intersections()],
            ),
        );
    }

    fn insert_horizontal_split(&mut self, row: usize, line: SplitLine) {
        self.borders
            .set_horizontal(row, line.borders, &line.intersections)
            .unwrap();
    }

    fn insert_vertical_split(&mut self, column: usize, line: SplitLine) {
        self.borders
            .set_vertical(column, line.borders, &line.intersections)
            .unwrap();
    }

    fn is_vertical_present(&mut self, column: usize) -> bool {
        self.borders.is_there_vertical(column)
    }

    fn is_horizontal_present(&mut self, row: usize) -> bool {
        self.borders.is_there_horizontal(row)
    }

    pub fn add_grid_split(&mut self) {
        for row in 0..self.count_rows() + 1 {
            self.add_horizontal_split(row);
        }

        for column in 0..self.count_columns() + 1 {
            self.add_vertical_split(column);
        }
    }

    pub fn clear_split_grid(&mut self) {
        self.borders.clear()
    }

    fn set_border(&mut self, frame: &EntityFrame, border: Border) {
        if let Some(top) = border.top {
            for column in frame.left_column..frame.right_column {
                self.borders
                    .set_row_symbol((frame.top_row, column), top)
                    .unwrap();

                // in case it continues line we change intersection symbol
                if frame.right_column - frame.left_column > 1 {
                    self.borders
                        .set_intersection((frame.top_row, column), top)
                        .unwrap();
                }
            }
        }

        if let Some(bottom) = border.bottom {
            for column in frame.left_column..frame.right_column {
                self.borders
                    .set_row_symbol((frame.bottom_row, column), bottom)
                    .unwrap();

                // in case it continues line we change intersection symbol
                if frame.right_column - frame.left_column > 1 {
                    self.borders
                        .set_intersection((frame.bottom_row, column), bottom)
                        .unwrap();
                }
            }
        }

        if let Some(left) = border.left {
            for row in frame.top_row..frame.bottom_row {
                self.borders
                    .set_column_symbol((row, frame.left_column), left)
                    .unwrap();

                // in case it continues line we change intersection symbol
                if frame.bottom_row - frame.top_row > 1 {
                    self.borders
                        .set_intersection((row, frame.left_column), left)
                        .unwrap();
                }
            }
        }

        if let Some(right) = border.right {
            for row in frame.top_row..frame.bottom_row {
                self.borders
                    .set_column_symbol((row, frame.right_column), right)
                    .unwrap();

                // in case it continues line we change intersection symbol
                if frame.bottom_row - frame.top_row > 1 {
                    self.borders
                        .set_intersection((row, frame.right_column), right)
                        .unwrap();
                }
            }
        }

        if let Some(top_left_corner) = border.left_top_corner {
            self.borders
                .set_intersection(frame.top_left_corner(), top_left_corner)
                .unwrap();
        }

        if let Some(top_right_corner) = border.right_top_corner {
            self.borders
                .set_intersection(frame.top_right_corner(), top_right_corner)
                .unwrap();
        }

        if let Some(bottom_left_corner) = border.left_bottom_corner {
            self.borders
                .set_intersection(frame.bottom_left_corner(), bottom_left_corner)
                .unwrap();
        }

        if let Some(bottom_right_corner) = border.right_bottom_corner {
            self.borders
                .set_intersection(frame.bottom_right_corner(), bottom_right_corner)
                .unwrap();
        }
    }

    /// get_cell_settings returns a settings of a cell
    pub fn get_settings(&self, row: usize, column: usize) -> Settings {
        let style = self.style(&Entity::Cell(row, column));
        let content = &self.cells[row][column];
        let border = self.borders.get_border(row, column).unwrap();

        Settings::default()
            .text(content)
            .alignment(style.alignment_h)
            .vertical_alignment(style.alignment_v)
            .span(style.span)
            .indent(
                style.indent.left,
                style.indent.right,
                style.indent.top,
                style.indent.bottom,
            )
            .border(border)
    }

    pub fn get_border(&mut self, row: usize, column: usize) -> Border {
        self.borders.get_border(row, column).unwrap()
    }

    pub fn style(&self, entity: &Entity) -> &Style {
        let lookup_table = match entity {
            Entity::Global => vec![Entity::Global],
            Entity::Column(column) => vec![Entity::Column(*column), Entity::Global],
            Entity::Row(row) => vec![Entity::Row(*row), Entity::Global],
            Entity::Cell(row, column) => vec![
                Entity::Cell(*row, *column),
                Entity::Column(*column),
                Entity::Row(*row),
                Entity::Global,
            ],
        };

        for entity in lookup_table {
            if let Some(style) = self.styles.get(&entity) {
                return style;
            }
        }

        unreachable!("there's a Entity::Global setting guaranted in the map")
    }

    fn style_mut(&mut self, entity: &Entity) -> &mut Style {
        if self.styles.contains_key(entity) {
            return self.styles.get_mut(entity).unwrap();
        }

        let style = self.style(entity).clone();
        self.styles.insert(entity.clone(), style);

        let style = self.styles.get_mut(entity).unwrap();
        style
    }

    /// get_cell_content returns content without any style changes
    pub fn get_cell_content(&self, row: usize, column: usize) -> &str {
        self.cells[row][column].as_str()
    }

    /// Count_rows returns an amount of rows on the grid
    pub fn count_rows(&self) -> usize {
        self.size.0
    }

    /// Count_rows returns an amount of columns on the grid
    pub fn count_columns(&self) -> usize {
        self.size.1
    }

    pub fn set_text<S: Into<String>>(&mut self, entity: &Entity, text: S) {
        let text = text.into();
        match *entity {
            Entity::Cell(row, column) => {
                self.cells[row][column] = text;
            }
            Entity::Column(column) => {
                for row in 0..self.count_rows() {
                    self.cells[row][column] = text.clone();
                }
            }
            Entity::Row(row) => {
                for column in 0..self.count_columns() {
                    self.cells[row][column] = text.clone();
                }
            }
            Entity::Global => {
                for row in 0..self.count_rows() {
                    for column in 0..self.count_columns() {
                        self.cells[row][column] = text.clone();
                    }
                }
            }
        }
    }

    pub fn set_cell_borders(&mut self, border: Border) {
        self.add_grid_split();
        for row in 0..self.count_rows() {
            for column in 0..self.count_columns() {
                self.set(
                    &Entity::Cell(row, column),
                    Settings::new().border(border.clone()),
                );
            }
        }
    }

    pub fn extract<R, C>(&self, rows: R, columns: C) -> Self
    where
        R: RangeBounds<usize>,
        C: RangeBounds<usize>,
    {
        let (start_row, end_row) =
            bounds_to_usize(rows.start_bound(), rows.end_bound(), self.count_rows());
        let (start_column, end_column) = bounds_to_usize(
            columns.start_bound(),
            columns.end_bound(),
            self.count_columns(),
        );

        let new_count_rows = end_row - start_row;
        let new_count_columns = end_column - start_column;
        let mut new_grid = Grid::new(new_count_rows, new_count_columns);

        for (new_row, row) in (start_row..end_row).enumerate() {
            for (new_column, column) in (start_column..end_column).enumerate() {
                let settings = self.get_settings(row, column);
                new_grid.set(
                    &Entity::Cell(new_row, new_column),
                    settings.border_restriction(false),
                );
            }
        }

        new_grid
    }

    fn add_split_lines_for_border(&mut self, frame: &EntityFrame, border: &Border) {
        if border.left.is_some() && !self.is_vertical_present(frame.left_column) {
            self.add_vertical_split(frame.left_column)
        }

        if border.right.is_some() && !self.is_vertical_present(frame.right_column) {
            self.add_vertical_split(frame.right_column)
        }

        if border.top.is_some() && !self.is_horizontal_present(frame.top_row) {
            self.add_horizontal_split(frame.top_row)
        }

        if border.bottom.is_some() && !self.is_horizontal_present(frame.bottom_row) {
            self.add_horizontal_split(frame.bottom_row)
        }
    }

    fn collect_cells(&self, count_rows: usize, count_columns: usize) -> Vec<Vec<Vec<&str>>> {
        let mut rows = Vec::with_capacity(count_rows);
        (0..count_rows).for_each(|row_index| {
            let mut row = Vec::with_capacity(count_columns);
            (0..count_columns).for_each(|column_index| {
                let content = &self.cells[row_index][column_index];
                // fixme: I guess it can be done in a different place?
                let cell: Vec<_> = content.lines().collect();
                row.push(cell);
            });

            rows.push(row);
        });

        rows
    }

    fn collect_styles(&self, count_rows: usize, count_columns: usize) -> Vec<Vec<Style>> {
        let mut rows = Vec::with_capacity(count_rows);
        (0..count_rows).for_each(|row_index| {
            let mut row = Vec::with_capacity(count_columns);
            (0..count_columns).for_each(|column_index| {
                let style = self.style(&Entity::Cell(row_index, column_index));
                row.push(style.clone());
            });

            rows.push(row);
        });

        rows
    }

    fn frame_from_entity(&self, entity: &Entity) -> EntityFrame {
        entity_frame(entity, self.count_rows(), self.count_columns())
    }

    fn get_split_line(&self, index: usize) -> Vec<BorderLine> {
        self.borders.get_row(index).unwrap()
    }

    fn get_inner_split_line(&self, index: usize) -> Vec<BorderLine> {
        self.borders.get_inner_row(index).unwrap()
    }
}

#[derive(Debug, Clone, Default)]
pub struct SplitLine {
    borders: Vec<char>,
    intersections: Vec<char>,
}

impl SplitLine {
    pub fn new(borders: Vec<char>, intersections: Vec<char>) -> Self {
        Self {
            borders,
            intersections,
        }
    }

    pub fn border(mut self, c: char) -> Self {
        self.borders.push(c);
        self
    }

    pub fn intersection(mut self, c: char) -> Self {
        self.intersections.push(c);
        self
    }
}

#[derive(Debug, Clone, Default)]
pub struct Border {
    pub top: Option<char>,
    pub bottom: Option<char>,
    pub left: Option<char>,
    pub right: Option<char>,
    pub left_top_corner: Option<char>,
    pub right_top_corner: Option<char>,
    pub left_bottom_corner: Option<char>,
    pub right_bottom_corner: Option<char>,
}

impl Border {
    /// full returns a border all walls
    #[allow(clippy::too_many_arguments)]
    pub fn full(
        top: char,
        bottom: char,
        left: char,
        right: char,
        top_left: char,
        top_right: char,
        bottom_left: char,
        bottom_right: char,
    ) -> Self {
        Self {
            top: Some(top),
            bottom: Some(bottom),
            right: Some(right),
            right_top_corner: Some(top_right),
            right_bottom_corner: Some(bottom_right),
            left: Some(left),
            left_bottom_corner: Some(bottom_left),
            left_top_corner: Some(top_left),
        }
    }

    pub fn top(mut self, c: char) -> Self {
        self.top = Some(c);
        self
    }

    pub fn bottom(mut self, c: char) -> Self {
        self.bottom = Some(c);
        self
    }

    pub fn left(mut self, c: char) -> Self {
        self.left = Some(c);
        self
    }

    pub fn right(mut self, c: char) -> Self {
        self.right = Some(c);
        self
    }

    pub fn top_left_corner(mut self, c: char) -> Self {
        self.left_top_corner = Some(c);
        self
    }

    pub fn top_right_corner(mut self, c: char) -> Self {
        self.right_top_corner = Some(c);
        self
    }

    pub fn bottom_left_corner(mut self, c: char) -> Self {
        self.left_bottom_corner = Some(c);
        self
    }

    pub fn bottom_right_corner(mut self, c: char) -> Self {
        self.right_bottom_corner = Some(c);
        self
    }
}

#[derive(Debug, Default)]
struct BorderLine {
    main: Option<char>,
    connector1: Option<char>,
    connector2: Option<char>,
}

/// Entity a structure which represent a set of cells.
#[derive(PartialEq, Eq, Debug, Hash, Clone)]
pub enum Entity {
    /// All cells on the grid.
    Global,
    /// All cells in a column on the grid.
    Column(usize),
    /// All cells in a row on the grid.
    Row(usize),
    /// A particular cell (row, column) on the grid.
    Cell(usize, usize),
}

#[derive(PartialEq, Eq, Debug, Hash, Clone)]
struct EntityFrame {
    left_column: usize,
    right_column: usize,
    top_row: usize,
    bottom_row: usize,
}

impl EntityFrame {
    fn new(left_column: usize, right_column: usize, top_row: usize, bottom_row: usize) -> Self {
        Self {
            left_column,
            right_column,
            top_row,
            bottom_row,
        }
    }

    fn top_left_corner(&self) -> GridPosition {
        (self.top_row, self.left_column)
    }

    fn top_right_corner(&self) -> GridPosition {
        (self.top_row, self.right_column)
    }

    fn bottom_left_corner(&self) -> GridPosition {
        (self.bottom_row, self.left_column)
    }

    fn bottom_right_corner(&self) -> GridPosition {
        (self.bottom_row, self.right_column)
    }
}

#[derive(Debug, Clone)]
pub struct Style {
    indent: Indent,
    alignment_h: AlignmentHorizontal,
    alignment_v: AlignmentVertical,
    span: usize,
}

impl Default for Style {
    fn default() -> Self {
        Self {
            alignment_h: AlignmentHorizontal::Left,
            alignment_v: AlignmentVertical::Top,
            indent: Indent {
                bottom: 0,
                left: 0,
                right: 0,
                top: 0,
            },
            span: 1,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Indent {
    pub top: usize,
    pub bottom: usize,
    pub left: usize,
    pub right: usize,
}

/// AlignmentHorizontal represents an horizontal aligment of a cell content.
#[derive(Debug, Clone, Copy)]
pub enum AlignmentHorizontal {
    Center,
    Left,
    Right,
}

impl AlignmentHorizontal {
    fn align(&self, f: &mut std::fmt::Formatter<'_>, text: &str, width: usize) -> fmt::Result {
        // it's important step
        // we are ignoring trailing spaces which allows us to do alignment with more space
        // example: tests::grid_2x2_alignment_test
        let text = text.trim();
        let text_width = string_width(text);
        let diff = width - text_width;
        match self {
            AlignmentHorizontal::Left => {
                write!(f, "{text}{: <1$}", "", diff, text = text)
            }
            AlignmentHorizontal::Right => {
                write!(f, "{: <1$}{text}", "", diff, text = text)
            }
            AlignmentHorizontal::Center => {
                let left = diff / 2;
                let right = diff - left;
                write!(
                    f,
                    "{: <left$}{text}{: <right$}",
                    "",
                    "",
                    left = left,
                    right = right,
                    text = text
                )
            }
        }
    }
}

/// AlignmentVertical represents an vertical aligment of a cell content.
#[derive(Debug, Clone, Copy)]
pub enum AlignmentVertical {
    Center,
    Top,
    Bottom,
}

impl AlignmentVertical {
    fn top_ident(&self, height: usize, real_height: usize) -> usize {
        match self {
            AlignmentVertical::Top => 0,
            AlignmentVertical::Bottom => height - real_height,
            AlignmentVertical::Center => (height - real_height) / 2,
        }
    }
}

/// Settings represent setting of a particular cell
#[derive(Debug, Clone, Default)]
pub struct Settings {
    text: Option<String>,
    indent: Option<Indent>,
    alignment_h: Option<AlignmentHorizontal>,
    alignment_v: Option<AlignmentVertical>,
    span: Option<usize>,
    border: Option<Border>,
    border_split_check: bool,
}

impl Settings {
    /// New method constructs an instance of settings
    pub fn new() -> Self {
        Self::default()
    }

    /// Text method sets content for a cell
    pub fn text<S: Into<String>>(mut self, text: S) -> Self {
        self.text = Some(text.into());
        self
    }

    /// Indent method sets indent for a cell
    pub fn indent(mut self, left: usize, right: usize, top: usize, bottom: usize) -> Self {
        self.indent = Some(Indent {
            top,
            bottom,
            left,
            right,
        });
        self
    }

    /// Alignment method sets horizontal alignment for a cell
    pub fn alignment(mut self, alignment: AlignmentHorizontal) -> Self {
        self.alignment_h = Some(alignment);
        self
    }

    /// Alignment method sets horizontal alignment for a cell
    pub fn vertical_alignment(mut self, alignment: AlignmentVertical) -> Self {
        self.alignment_v = Some(alignment);
        self
    }

    /// Set the settings's span.
    pub fn span(mut self, span: usize) -> Self {
        self.span = Some(span);
        self
    }

    /// Set the settings's border.
    ///
    /// The border setting is in a restrictive manner, by default.
    /// So if there was no split line but border relies on it
    /// a error will be issued.
    ///
    /// To fix it you can construct split lines before calling this function.
    /// Or you can pass a `false` argument into [Self::border_restriction]
    /// so if absent lines will be created.
    pub fn border(mut self, border: Border) -> Self {
        self.border = Some(border);
        self
    }

    /// Set a split lines check.
    pub fn border_restriction(mut self, strict: bool) -> Self {
        self.border_split_check = !strict;
        self
    }
}

impl std::fmt::Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let count_rows = self.count_rows();
        let count_columns = self.count_columns();

        // It may happen when all cells removed via `remove_row`, `remove_column` methods
        if count_rows == 0 || count_columns == 0 {
            return Ok(());
        }

        let mut cells = self.collect_cells(count_rows, count_columns);
        let mut styles = self.collect_styles(count_rows, count_columns);

        let row_heights = rows_height(&cells, &styles, count_rows, count_columns);
        let widths = columns_width(&mut cells, &mut styles, count_rows, count_columns);

        for row in 0..count_rows {
            let top_border = self.get_split_line(row);
            let inner_border = self.get_inner_split_line(row);
            let bottom_border = self.get_split_line(row + 1);

            build_row(
                f,
                &cells[row],
                &styles[row],
                &widths[row],
                row_heights[row],
                row == 0,
                (&top_border, &inner_border, &bottom_border),
            )?;
        }

        Ok(())
    }
}

fn build_row(
    f: &mut std::fmt::Formatter<'_>,
    cell_contents: &[Vec<&str>],
    cell_styles: &[Style],
    cell_widths: &[usize],
    height: usize,
    is_first_row: bool,
    (top_borders, inner_borders, bottom_borders): (&[BorderLine], &[BorderLine], &[BorderLine]),
) -> fmt::Result {
    if is_first_row {
        build_split_line(f, cell_widths, top_borders)?;
    }

    build_row_internals(
        f,
        cell_contents,
        cell_styles,
        cell_widths,
        height,
        inner_borders,
    )?;

    build_split_line(f, cell_widths, bottom_borders)?;
    Ok(())
}

fn build_row_internals(
    f: &mut std::fmt::Formatter<'_>,
    row: &[Vec<&str>],
    row_styles: &[Style],
    widths: &[usize],
    height: usize,
    border: &[BorderLine],
) -> fmt::Result {
    for line_index in 0..height {
        build_line(f, row.len(), border, |f, column| {
            build_row_internal_line(
                f,
                line_index,
                &row[column],
                &row_styles[column],
                widths[column],
                height,
            )
        })?;
    }

    Ok(())
}

fn build_row_internal_line(
    f: &mut std::fmt::Formatter<'_>,
    line_index: usize,
    cell: &[&str],
    style: &Style,
    width: usize,
    height: usize,
) -> fmt::Result {
    let top_indent = top_indent(cell, style, height);
    if top_indent > line_index {
        return empty_line(f, width);
    }

    let cell_line_index = line_index - top_indent;
    let cell_has_this_line = cell.len() > cell_line_index;
    // happen when other cells have bigger height
    if !cell_has_this_line {
        return empty_line(f, width);
    }

    let line_text = cell[cell_line_index];
    line(
        f,
        line_text,
        width,
        style.indent.left,
        style.indent.right,
        style.alignment_h,
    )
}

fn top_indent(cell: &[&str], style: &Style, height: usize) -> usize {
    let height = height - style.indent.top;
    let content_height = cell_height(cell, style) - style.indent.top - style.indent.bottom;
    let indent = style.alignment_v.top_ident(height, content_height);
    indent + style.indent.top
}

fn empty_line(f: &mut std::fmt::Formatter<'_>, n: usize) -> fmt::Result {
    write!(f, "{:1$}", "", n)
}

fn repeat_char(f: &mut std::fmt::Formatter<'_>, c: char, n: usize) -> fmt::Result {
    if n > 0 {
        write!(f, "{:1$}", c, n)
    } else {
        Ok(())
    }
}

fn line(
    f: &mut std::fmt::Formatter<'_>,
    text: &str,
    width: usize,
    left_indent: usize,
    right_indent: usize,
    alignment: AlignmentHorizontal,
) -> fmt::Result {
    repeat_char(f, ' ', left_indent)?;
    alignment.align(f, text, width - left_indent - right_indent)?;
    repeat_char(f, ' ', right_indent)?;
    Ok(())
}

fn build_line<F: Fn(&mut std::fmt::Formatter<'_>, usize) -> fmt::Result>(
    f: &mut std::fmt::Formatter<'_>,
    length: usize,
    borders: &[BorderLine],
    writer: F,
) -> fmt::Result {
    for (i, border) in borders.iter().enumerate().take(length) {
        write_option(f, border.connector1)?;
        writer(f, i)?;
        if i + 1 == length {
            write_option(f, border.connector2)?;
        }
    }

    writeln!(f)?;

    Ok(())
}

fn build_split_line(
    f: &mut std::fmt::Formatter<'_>,
    widths: &[usize],
    borders: &[BorderLine],
) -> fmt::Result {
    let theres_no_border = borders.iter().all(|l| l.main.is_none());
    if theres_no_border || borders.is_empty() {
        return Ok(());
    }

    build_line(f, widths.len(), borders, |f, i| {
        write_option(f, borders[i].main.map(|m| m.to_string().repeat(widths[i])))
    })
}

fn write_option<D: Display>(f: &mut std::fmt::Formatter<'_>, text: Option<D>) -> fmt::Result {
    match text {
        Some(text) => write!(f, "{}", text),
        None => Ok(()),
    }
}

#[cfg(not(feature = "color"))]
pub fn string_width(text: &str) -> usize {
    real_string_width(text)
}

#[cfg(feature = "color")]
pub fn string_width(text: &str) -> usize {
    let b = strip_ansi_escapes::strip(text.as_bytes()).unwrap();
    let s = std::str::from_utf8(&b).unwrap();
    real_string_width(s)
}

fn real_string_width(text: &str) -> usize {
    text.lines()
        .map(unicode_width::UnicodeWidthStr::width)
        .max()
        .unwrap_or(0)
}

#[cfg(feature = "color")]
pub fn strip_ansi_escapes(text: &str) -> String {
    let b = strip_ansi_escapes::strip(text.as_bytes()).unwrap();
    String::from_utf8_lossy(&b).to_string()
}

fn columns_width(
    cells: &mut [Vec<Vec<&str>>],
    styles: &mut [Vec<Style>],
    count_rows: usize,
    count_columns: usize,
) -> Vec<Vec<usize>> {
    let mut widths = vec![vec![0; count_columns]; count_rows];
    (0..count_rows).for_each(|row| {
        (0..count_columns).for_each(|column| {
            let cell = &cells[row][column];
            let style = &styles[row][column];
            if is_cell_visible(&styles[row], column) {
                widths[row][column] = cell_width(cell, style);
            } else {
                widths[row][column] = 0;
                styles[row][column].span = 0;
            }
        });
    });

    // it's crusial to preserve order in iterations
    // so we use BTreeSet
    let mut spans = BTreeSet::new();
    styles.iter().for_each(|row_styles| {
        row_styles.iter().for_each(|style| {
            spans.insert(style.span);
        })
    });
    spans.into_iter().filter(|&span| span > 0).for_each(|span| {
        adjust_width(&mut widths, styles, count_rows, count_columns, span);
    });

    // remove not visible cells to print everything correctly
    (0..count_rows).for_each(|row| {
        let mut n_removed = 0;
        (0..count_columns).for_each(|column| {
            let column = column - n_removed;
            if styles[row][column].span == 0 {
                widths[row].remove(column);
                cells[row].remove(column);
                styles[row].remove(column);
                n_removed += 1;
            }
        });
    });

    widths
}

fn adjust_width(
    widths: &mut [Vec<usize>],
    styles: &[Vec<Style>],
    count_rows: usize,
    count_columns: usize,
    span: usize,
) {
    for column in 0..count_columns {
        let start = column;
        let end = column + span;
        if end > count_columns {
            break;
        }

        adjust_range_width(widths, styles, count_rows, start, end);
    }
}

fn adjust_range_width(
    widths: &mut [Vec<usize>],
    styles: &[Vec<Style>],
    count_rows: usize,
    start_column: usize,
    end_column: usize,
) {
    if count_rows == 0 {
        return;
    }
    let span = end_column - start_column;

    let (row_with_max_width, max_width) = (0..count_rows)
        .filter(|&row| is_row_consistent(&styles[row][start_column..end_column]))
        .filter(|&row| {
            let row_spans = (start_column..end_column)
                .filter(|&column| is_cell_visible(&styles[row], column))
                .map(|col| styles[row][col].span)
                .sum::<usize>();

            row_spans <= span
        })
        .map(|row| {
            let width = row_width(&styles[row], &widths[row], start_column, end_column);
            (row, width)
        })
        .max_by_key(|&(_, width)| width)
        .unwrap_or_default();

    // might happen when we filtered every cell
    if max_width == 0 {
        return;
    }

    (0..count_rows)
        .filter(|&row| row != row_with_max_width)
        .filter(|&row| is_row_consistent(&styles[row][start_column..end_column]))
        .filter(|&row| {
            let row_spans = (start_column..end_column)
                .filter(|&column| is_cell_visible(&styles[row], column))
                .map(|col| styles[row][col].span)
                .sum::<usize>();

            row_spans <= span
        })
        .for_each(|row| {
            inc_width_to_cells(
                &mut widths[row],
                &styles[row],
                start_column,
                end_column,
                max_width,
            );
        });
}

fn is_row_consistent(styles: &[Style]) -> bool {
    if styles.is_empty() {
        return true;
    }

    if styles[0].span == 0 {
        return false;
    }

    styles
        .iter()
        .zip(styles.len()..)
        .all(|(style, max_span)| style.span <= max_span)
}

fn is_cell_visible(row_styles: &[Style], column: usize) -> bool {
    row_styles[column].span != 0
        && !row_styles[..column]
            .iter()
            .zip(column..)
            .any(|(style, span)| style.span > span)
}

fn row_width(styles: &[Style], widths: &[usize], column_start: usize, column_end: usize) -> usize {
    let width = (column_start..column_end)
        .filter(|&i| is_cell_visible(styles, i))
        .map(|i| widths[i])
        .sum::<usize>();

    if width == 0 {
        return 0;
    }

    // fixme: Is this style dependent?
    let border_count = (column_start..column_end)
        .filter(|&i| is_cell_visible(styles, i))
        .count()
        - 1;

    width + border_count
}

fn inc_width_to_cells(
    widths: &mut [usize],
    styles: &[Style],
    start_range: usize,
    end_range: usize,
    width: usize,
) {
    let a = row_width(styles, widths, start_range, end_range);
    let diff = width - a;

    (0..diff)
        .zip(
            (start_range..end_range)
                .filter(|&i| is_cell_visible(styles, i))
                .cycle(),
        )
        .for_each(|(_, i)| widths[i] += 1);
}

fn cell_width(cell: &[&str], style: &Style) -> usize {
    let content_width = cell.iter().map(|l| string_width(l)).max().unwrap_or(0);
    content_width + style.indent.left + style.indent.right
}

fn rows_height(
    cells: &[Vec<Vec<&str>>],
    styles: &[Vec<Style>],
    count_rows: usize,
    count_columns: usize,
) -> Vec<usize> {
    // default height is 1 as we consider empty string has height 1
    //
    // it's crusial since if the default height will be equal to 0
    // cell line will be not present on the grid like this
    //
    //  default 0      default 1
    //    +++            +++
    //    +++            |||
    //    +++            +++
    //                   |||
    //                   +++
    let mut row_heights = vec![1; count_rows];
    (0..count_rows).for_each(|row_index| {
        (0..count_columns).for_each(|column_index| {
            let cell = &cells[row_index][column_index];
            let style = &styles[row_index][column_index];
            row_heights[row_index] = max(row_heights[row_index], cell_height(cell, style));
        });
    });

    row_heights
}

fn cell_height(cell: &[&str], style: &Style) -> usize {
    let content_height = cell.len();
    content_height + style.indent.top + style.indent.bottom
}

#[derive(Debug)]
struct Borders {
    vertical: HashMap<CellIndex, Line>,
    horizontal: HashMap<CellIndex, Line>,
    intersections: HashMap<GridPosition, char>,
    count_columns: usize,
    count_rows: usize,
}

type CellIndex = usize;

type GridPosition = (CellIndex, CellIndex);

// self.len() == count of cells
type Line = Vec<char>;

impl Borders {
    fn new(count_rows: usize, count_columns: usize) -> Self {
        Self {
            vertical: HashMap::new(),
            horizontal: HashMap::new(),
            intersections: HashMap::new(),
            count_columns,
            count_rows,
        }
    }

    fn get_row(&self, row: usize) -> Result<Vec<BorderLine>, BorderError> {
        if row > self.count_rows {
            return Err(BorderError::WrongRowIndex);
        }

        if !self.horizontal.contains_key(&row) {
            return Ok(Vec::new());
        }

        let mut line = Vec::with_capacity(self.count_columns);
        for column in 0..self.count_columns {
            let border = BorderLine {
                main: Some(self.get_horizontal_char(row, column).unwrap()),
                connector1: None,
                connector2: None,
            };

            line.push(border);
        }

        for (column, border) in line.iter_mut().enumerate() {
            border.connector1 = self.get_intersection_char((row, column));
            border.connector2 = self.get_intersection_char((row, column + 1));
        }

        Ok(line)
    }

    fn get_inner_row(&self, row: usize) -> Result<Vec<BorderLine>, BorderError> {
        if row > self.count_rows {
            return Err(BorderError::WrongRowIndex);
        }

        let mut line: Vec<BorderLine> = Vec::new();
        let mut last_index = None;
        for column in 0..self.count_columns + 1 {
            let border = BorderLine {
                connector1: self.get_vertical_char(row, column),
                ..Default::default()
            };

            if border.connector1.is_some() {
                if let Some(last) = last_index {
                    let mut last: &mut BorderLine = &mut line[last];
                    last.connector2 = border.connector1;
                }
            }
            last_index = Some(line.len());

            line.push(border);
        }

        Ok(line)
    }

    // we can take only a border of a cell
    // which is a pitty,
    // would be cool if we could take a border of any Entity
    fn get_border(&self, row: usize, column: usize) -> Option<Border> {
        if row > self.count_rows || column > self.count_columns {
            return None;
        }

        let frame = entity_frame(
            &Entity::Cell(row, column),
            self.count_rows,
            self.count_columns,
        );

        let border = Border {
            top: self.get_horizontal_char(frame.top_row, column),
            bottom: self.get_horizontal_char(frame.bottom_row, column),
            left: self.get_vertical_char(row, frame.left_column),
            right: self.get_vertical_char(row, frame.right_column),
            left_top_corner: self.get_intersection_char(frame.top_left_corner()),
            left_bottom_corner: self.get_intersection_char(frame.bottom_left_corner()),
            right_top_corner: self.get_intersection_char(frame.top_right_corner()),
            right_bottom_corner: self.get_intersection_char(frame.bottom_right_corner()),
        };

        Some(border)
    }

    fn get_horizontal_char(&self, row: usize, column: usize) -> Option<char> {
        self.horizontal.get(&row).map(|line| {
            assert_eq!(line.len(), self.count_columns);
            line[column]
        })
    }

    fn get_vertical_char(&self, row: usize, column: usize) -> Option<char> {
        self.vertical.get(&column).map(|line| {
            assert_eq!(line.len(), self.count_rows);
            line[row]
        })
    }

    fn get_intersection_char(&self, (row, column): GridPosition) -> Option<char> {
        self.intersections.get(&(row, column)).copied()
    }

    fn set_horizontal(
        &mut self,
        row: usize,
        line: Vec<char>,
        intersections: &[char],
    ) -> Result<(), BorderError> {
        if row > self.count_rows {
            return Err(BorderError::WrongRowIndex);
        }

        if line.len() != self.count_columns {
            return Err(BorderError::NotEnoughLineSymbols);
        }

        let need_intersections = self.need_horizontal_intersections();
        if intersections.len() != need_intersections {
            return Err(BorderError::NotEnoughIntersections);
        }

        self.horizontal.insert(row, line);

        for (&vertical_line_index, &symbol) in self.vertical.keys().zip(intersections) {
            self.intersections
                .insert((row, vertical_line_index), symbol);
        }

        Ok(())
    }

    fn need_horizontal_intersections(&self) -> usize {
        self.vertical.len() + 1
    }

    fn need_vertical_intersections(&self) -> usize {
        self.horizontal.len() + 1
    }

    fn clear(&mut self) {
        self.horizontal.clear();
        self.vertical.clear();
        self.intersections.clear();
    }

    fn is_there_vertical(&self, column: usize) -> bool {
        self.vertical.contains_key(&column)
    }

    fn is_there_horizontal(&self, row: usize) -> bool {
        self.horizontal.contains_key(&row)
    }

    fn set_vertical(
        &mut self,
        column: usize,
        line: Vec<char>,
        intersections: &[char],
    ) -> Result<(), BorderError> {
        if column > self.count_columns {
            return Err(BorderError::WrongRowIndex);
        }

        if line.len() != self.count_rows {
            return Err(BorderError::NotEnoughLineSymbols);
        }

        let need_intersections = self.need_vertical_intersections();
        if intersections.len() != need_intersections {
            return Err(BorderError::NotEnoughIntersections);
        }

        self.vertical.insert(column, line);

        for (&row_index, &symbol) in self.horizontal.keys().zip(intersections) {
            self.intersections.insert((row_index, column), symbol);
        }

        Ok(())
    }

    fn set_intersection(&mut self, pos: GridPosition, c: char) -> Result<(), BorderError> {
        let (row, column) = pos;

        if row > self.count_rows + 1 || !self.horizontal.contains_key(&row) {
            return Err(BorderError::WrongRowIndex);
        }
        if column > self.count_columns + 1 || !self.vertical.contains_key(&column) {
            return Err(BorderError::WrongColumnIndex);
        }

        match self.intersections.get_mut(&pos) {
            Some(old) => {
                *old = c;
                Ok(())
            }
            None => Err(BorderError::WrongIntersectionIndex),
        }
    }

    fn set_row_symbol(&mut self, (row, column): GridPosition, c: char) -> Result<(), BorderError> {
        if row > self.count_rows || !self.horizontal.contains_key(&row) {
            return Err(BorderError::WrongRowIndex);
        }
        if column > self.count_columns {
            return Err(BorderError::WrongColumnIndex);
        }

        let chars = self.horizontal.get_mut(&row).unwrap();
        if column > chars.len() {
            return Err(BorderError::WrongColumnIndex);
        }

        *chars.get_mut(column).unwrap() = c;

        Ok(())
    }

    fn set_column_symbol(
        &mut self,
        (row, column): GridPosition,
        c: char,
    ) -> Result<(), BorderError> {
        if row > self.count_rows {
            return Err(BorderError::WrongRowIndex);
        }
        if column > self.count_columns || !self.vertical.contains_key(&column) {
            return Err(BorderError::WrongColumnIndex);
        }

        let chars = self.vertical.get_mut(&column).unwrap();
        if row > chars.len() {
            return Err(BorderError::WrongColumnIndex);
        }

        *chars.get_mut(row).unwrap() = c;

        Ok(())
    }
}

#[derive(Debug, Clone)]
enum BorderError {
    WrongIntersectionIndex,
    WrongRowIndex,
    WrongColumnIndex,
    NotEnoughLineSymbols,
    NotEnoughIntersections,
}

fn entity_frame(entity: &Entity, count_rows: usize, count_columns: usize) -> EntityFrame {
    match entity {
        Entity::Global => EntityFrame::new(0, count_columns, 0, count_rows),
        &Entity::Column(c) => EntityFrame::new(c, c + 1, 0, count_rows),
        &Entity::Row(r) => EntityFrame::new(0, count_columns, r, r + 1),
        &Entity::Cell(r, c) => EntityFrame::new(c, c + 1, r, r + 1),
    }
}

fn bounds_to_usize(left: Bound<&usize>, right: Bound<&usize>, length: usize) -> (usize, usize) {
    match (left, right) {
        (Bound::Included(x), Bound::Included(y)) => (*x, y + 1),
        (Bound::Included(x), Bound::Excluded(y)) => (*x, *y),
        (Bound::Included(x), Bound::Unbounded) => (*x, length),
        (Bound::Unbounded, Bound::Unbounded) => (0, length),
        (Bound::Unbounded, Bound::Included(y)) => (0, y + 1),
        (Bound::Unbounded, Bound::Excluded(y)) => (0, *y),
        (Bound::Excluded(_), Bound::Unbounded)
        | (Bound::Excluded(_), Bound::Included(_))
        | (Bound::Excluded(_), Bound::Excluded(_)) => {
            unreachable!("A start bound can't be excluded")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn string_width_emojie_test() {
        // ...emojis such as “joy”, which normally take up two columns when printed in a terminal
        // https://github.com/mgeisler/textwrap/pull/276
        assert_eq!(string_width("🎩"), 2);
        assert_eq!(string_width("Rust 💕"), 7);
        assert_eq!(string_width("Go 👍\nC 😎"), 5);
    }

    #[test]
    fn horizontal_aligment_test() {
        use std::fmt;

        struct F<'a>(&'a str, AlignmentHorizontal, usize);

        impl fmt::Display for F<'_> {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                self.1.align(f, self.0, self.2)
            }
        }

        assert_eq!(F("AAA", AlignmentHorizontal::Right, 4).to_string(), " AAA");
        assert_eq!(F("AAA", AlignmentHorizontal::Left, 4).to_string(), "AAA ");
        assert_eq!(F("AAA", AlignmentHorizontal::Center, 4).to_string(), "AAA ");
        assert_eq!(F("🎩", AlignmentHorizontal::Center, 4).to_string(), " 🎩 ");
        assert_eq!(F("🎩", AlignmentHorizontal::Center, 3).to_string(), "🎩 ");
        #[cfg(feature = "color")]
        {
            use colored::Colorize;
            let text = "Colored Text".red().to_string();
            assert_eq!(
                F(&text, AlignmentHorizontal::Center, 15).to_string(),
                format!(" {}  ", text)
            );
        }
    }

    #[test]
    fn vertical_aligment_test() {
        assert_eq!(AlignmentVertical::Bottom.top_ident(1, 1), 0);
        assert_eq!(AlignmentVertical::Top.top_ident(1, 1), 0);
        assert_eq!(AlignmentVertical::Center.top_ident(1, 1), 0);
        assert_eq!(AlignmentVertical::Bottom.top_ident(3, 1), 2);
        assert_eq!(AlignmentVertical::Top.top_ident(3, 1), 0);
        assert_eq!(AlignmentVertical::Center.top_ident(3, 1), 1);
        assert_eq!(AlignmentVertical::Center.top_ident(4, 1), 1);
    }

    #[cfg(feature = "color")]
    #[test]
    fn colored_string_width_test() {
        use colored::Colorize;
        assert_eq!(string_width(&"hello world".red().to_string()), 11);
        assert_eq!(string_width(&"hello\nworld".blue().to_string()), 5);
        assert_eq!(string_width("\u{1b}[34m0\u{1b}[0m"), 1);
        assert_eq!(string_width(&"0".red().to_string()), 1);
    }
}
