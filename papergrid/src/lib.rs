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

const DEFAULT_INDENT_FILL_CHAR: char = ' ';

/// Grid provides a set of methods for building a text-based table
pub struct Grid {
    size: (usize, usize),
    cells: Vec<Vec<String>>,
    styles: HashMap<Entity, Style>,
    borders: Borders,
    override_split_lines: HashMap<usize, String>,
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
            override_split_lines: HashMap::new(),
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

        if let Some(padding) = settings.padding {
            self.style_mut(entity).padding = padding;
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
            if settings.border_split_check {
                self.add_split_lines(entity.clone(), &border);
            }

            self.set_border(entity.clone(), &border);
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

    pub fn clear_overide_split_lines(&mut self) {
        self.override_split_lines.clear();
    }

    fn set_border(&mut self, entity: Entity, border: &Border) {
        match entity {
            Entity::Global => {
                for column in 0..self.count_columns() {
                    for row in 0..self.count_rows() {
                        self.set_border_for_cell(row, column, border);
                    }
                }
            }
            Entity::Column(column) => {
                for row in 0..self.count_rows() {
                    self.set_border_for_cell(row, column, border);
                }
            }
            Entity::Row(row) => {
                for column in 0..self.count_columns() {
                    self.set_border_for_cell(row, column, border);
                }
            }
            Entity::Cell(row, column) => {
                self.set_border_for_cell(row, column, border);
            }
        }
    }

    fn set_border_for_cell(&mut self, row: usize, column: usize, border: &Border) {
        let cell = CellBorderIndex::new(row, column);

        if let Some(left) = border.left {
            self.borders.set_column_symbol(cell.left(), left).unwrap();
        }

        if let Some(right) = border.right {
            self.borders.set_column_symbol(cell.right(), right).unwrap();
        }

        if let Some(top) = border.top {
            self.borders.set_row_symbol(cell.top(), top).unwrap();
        }

        if let Some(bottom) = border.bottom {
            self.borders.set_row_symbol(cell.bottom(), bottom).unwrap();
        }

        if let Some(left_top) = border.left_top_corner {
            self.borders
                .set_intersection(cell.top_left(), left_top)
                .unwrap();
        }

        if let Some(right_top) = border.right_top_corner {
            self.borders
                .set_intersection(cell.top_right(), right_top)
                .unwrap();
        }

        if let Some(left_bottom) = border.left_bottom_corner {
            self.borders
                .set_intersection(cell.bottom_left(), left_bottom)
                .unwrap();
        }

        if let Some(right_bottom) = border.right_bottom_corner {
            self.borders
                .set_intersection(cell.bottom_right(), right_bottom)
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
            .padding(
                style.padding.left,
                style.padding.right,
                style.padding.top,
                style.padding.bottom,
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

    /// Returns a new [Grid] that reflects a segment of the referenced [Grid]
    ///
    /// The segment is defined by [RangeBounds<usize>] for Rows and Columns
    ///
    /// # Example
    ///
    /// ```text,no_run
    /// grid
    /// +---+---+---+
    /// |0-0|0-1|0-2|
    /// +---+---+---+
    /// |1-0|1-1|1-2|
    /// +---+---+---+
    /// |2-0|2-1|2-2|
    /// +---+---+---+
    /// let rows = ..;
    /// let columns = ..1;
    /// grid.extract(rows, columns)
    /// +---+
    /// |0-0|
    /// +---+
    /// |1-0|
    /// +---+
    /// |2-0|
    /// +---+
    /// ```
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

    /// Returns a total width of table, including split lines.
    pub fn total_width(&self) -> usize {
        // can be simplified? by just getting a split line chars().count()

        let count_rows = self.count_rows();
        let count_columns = self.count_columns();
        let mut cells = self.collect_cells(count_rows, count_columns);
        let mut styles = self.collect_styles(count_rows, count_columns);
        let split_borders = (0..count_rows)
            .map(|row| self.get_inner_split_line(row))
            .collect::<Vec<_>>();

        let widths = columns_width(
            &mut cells,
            &mut styles,
            &split_borders,
            count_rows,
            count_columns,
        );

        let content_width = widths
            .into_iter()
            .next()
            .map(|row| row.into_iter().sum::<usize>())
            .unwrap_or(0);

        let count_borders = split_borders
            .into_iter()
            .next()
            .map(|row| {
                let left_border = row
                    .get(0)
                    .map_or(0, |b| if b.connector1.is_some() { 1 } else { 0 });
                let other_borders = row
                    .into_iter()
                    .enumerate()
                    .filter(|&(col, _)| is_cell_visible(&styles[0], col))
                    .filter(|(_, b)| b.connector2.is_some())
                    .count();

                left_border + other_borders
            })
            .unwrap_or(0);

        content_width + count_borders
    }

    pub fn override_split_line(&mut self, row: usize, line: impl Into<String>) {
        self.override_split_lines.insert(row, line.into());
    }

    pub fn row_width(&self, row: usize) -> usize {
        let row_widths = (0..self.count_columns())
            .map(|col| {
                let style = self.style(&Entity::Cell(row, col));
                let content = self.get_cell_content(row, col);
                string_width(content) + style.padding.left.size + style.padding.right.size
            })
            .collect::<Vec<_>>();

        let row_styles = (0..self.count_columns())
            .map(|col| self.style(&Entity::Cell(row, col)).clone())
            .collect::<Vec<_>>();

        let row_borders = self.get_inner_split_line(row);

        row_width(
            &row_styles,
            &row_widths,
            &row_borders,
            0,
            self.count_columns(),
        )
    }

    fn add_split_lines(&mut self, entity: Entity, border: &Border) {
        match entity {
            Entity::Global => {
                for column in 0..self.count_columns() {
                    for row in 0..self.count_rows() {
                        self.add_split_lines_for_cell(row, column, border);
                    }
                }
            }
            Entity::Column(column) => {
                for row in 0..self.count_rows() {
                    self.add_split_lines_for_cell(row, column, border);
                }
            }
            Entity::Row(row) => {
                for column in 0..self.count_columns() {
                    self.add_split_lines_for_cell(row, column, border);
                }
            }
            Entity::Cell(row, column) => {
                self.add_split_lines_for_cell(row, column, border);
            }
        }
    }

    fn add_split_lines_for_cell(&mut self, row: usize, column: usize, border: &Border) {
        let left_affected = border.left.is_some()
            || border.left_bottom_corner.is_some()
            || border.left_top_corner.is_some();
        if left_affected && !self.is_vertical_present(column) {
            self.add_vertical_split(column);
        }
        let right_affected = border.right.is_some()
            || border.right_bottom_corner.is_some()
            || border.right_top_corner.is_some();
        if right_affected && !self.is_vertical_present(column + 1) {
            self.add_vertical_split(column + 1);
        }
        let top_affected = border.top.is_some()
            || border.right_top_corner.is_some()
            || border.left_top_corner.is_some();
        if top_affected && !self.is_horizontal_present(row) {
            self.add_horizontal_split(row)
        }
        let bottom_affected = border.bottom.is_some()
            || border.right_bottom_corner.is_some()
            || border.left_bottom_corner.is_some();
        if bottom_affected && !self.is_horizontal_present(row + 1) {
            self.add_horizontal_split(row + 1)
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

#[derive(Debug, Clone, Default, Eq, PartialEq)]
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
    /// This function constructs a cell borders with all sides set.
    #[allow(clippy::too_many_arguments)]
    pub fn new(
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

    /// This function constructs a cell borders with all sides's char set to a given character.
    /// It behaives like [Border::new] with the same character set to each side.
    pub fn filled(c: char) -> Self {
        Self::new(c, c, c, c, c, c, c, c)
    }

    /// Set a top border character.
    pub fn top(mut self, c: char) -> Self {
        self.top = Some(c);
        self
    }

    /// Set a bottom border character.
    pub fn bottom(mut self, c: char) -> Self {
        self.bottom = Some(c);
        self
    }

    /// Set a left border character.
    pub fn left(mut self, c: char) -> Self {
        self.left = Some(c);
        self
    }

    /// Set a right border character.
    pub fn right(mut self, c: char) -> Self {
        self.right = Some(c);
        self
    }

    /// Set a top left intersection character.
    pub fn top_left_corner(mut self, c: char) -> Self {
        self.left_top_corner = Some(c);
        self
    }

    /// Set a top right intersection character.
    pub fn top_right_corner(mut self, c: char) -> Self {
        self.right_top_corner = Some(c);
        self
    }

    /// Set a bottom left intersection character.
    pub fn bottom_left_corner(mut self, c: char) -> Self {
        self.left_bottom_corner = Some(c);
        self
    }

    /// Set a bottom right intersection character.
    pub fn bottom_right_corner(mut self, c: char) -> Self {
        self.right_bottom_corner = Some(c);
        self
    }
}

#[derive(Debug, Default, Clone)]
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

#[derive(Debug, Clone)]
pub struct Style {
    pub padding: Padding,
    pub alignment_h: AlignmentHorizontal,
    pub alignment_v: AlignmentVertical,
    pub span: usize,
}

impl Default for Style {
    fn default() -> Self {
        Self {
            alignment_h: AlignmentHorizontal::Left,
            alignment_v: AlignmentVertical::Top,
            padding: Padding::default(),
            span: 1,
        }
    }
}

#[derive(Default, Debug, Clone, Copy)]
pub struct Padding {
    pub top: Indent,
    pub bottom: Indent,
    pub left: Indent,
    pub right: Indent,
}

#[derive(Debug, Clone, Copy)]
pub struct Indent {
    pub fill: char,
    pub size: usize,
}

impl Default for Indent {
    fn default() -> Self {
        Self {
            fill: DEFAULT_INDENT_FILL_CHAR,
            size: 0,
        }
    }
}

impl Indent {
    pub fn new(size: usize, fill: char) -> Self {
        Self { size, fill }
    }

    pub fn spaced(size: usize) -> Self {
        Self {
            size,
            fill: DEFAULT_INDENT_FILL_CHAR,
        }
    }
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
    padding: Option<Padding>,
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

    /// padding method sets padding for a cell
    pub fn padding(mut self, left: Indent, right: Indent, top: Indent, bottom: Indent) -> Self {
        self.padding = Some(Padding {
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

        let split_borders = (0..count_rows)
            .map(|row| self.get_inner_split_line(row))
            .collect::<Vec<_>>();

        let row_heights = rows_height(&cells, &styles, count_rows, count_columns);
        let widths = columns_width(
            &mut cells,
            &mut styles,
            &split_borders,
            count_rows,
            count_columns,
        );

        let normal_widths = normalized_width(&widths, &styles, count_rows, count_columns);

        for row in 0..count_rows {
            let inner_border = self.get_inner_split_line(row);
            let top_border = if row == 0 {
                Some((
                    self.get_split_line(row),
                    self.override_split_lines.get(&row),
                ))
            } else {
                None
            };
            let bottom_border = Some((
                self.get_split_line(row + 1),
                self.override_split_lines.get(&(row + 1)),
            ));

            build_row(
                f,
                &cells[row],
                &styles[row],
                &widths[row],
                &normal_widths,
                row_heights[row],
                inner_border,
                top_border,
                bottom_border,
            )?;
        }

        Ok(())
    }
}

#[allow(clippy::too_many_arguments)]
fn build_row(
    f: &mut std::fmt::Formatter<'_>,
    cell_contents: &[Vec<&str>],
    cell_styles: &[Style],
    cell_widths: &[usize],
    normal_widths: &[usize],
    height: usize,
    inner_border: Vec<BorderLine>,
    top_border: Option<(Vec<BorderLine>, Option<&String>)>,
    bottom_border: Option<(Vec<BorderLine>, Option<&String>)>,
) -> fmt::Result {
    if let Some((top_border, override_border)) = top_border {
        build_split_line(f, normal_widths, &top_border, override_border)?;
    }

    build_row_internals(
        f,
        cell_contents,
        cell_styles,
        cell_widths,
        height,
        &inner_border,
    )?;

    if let Some((bottom_border, override_border)) = bottom_border {
        build_split_line(f, normal_widths, &bottom_border, override_border)?;
    }

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
        build_line(f, border, row_styles, row.len(), |f, column| {
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
        return repeat_char(f, style.padding.top.fill, width);
    }

    let cell_line_index = line_index - top_indent;
    let cell_has_this_line = cell.len() > cell_line_index;
    // happen when other cells have bigger height
    if !cell_has_this_line {
        return repeat_char(f, style.padding.bottom.fill, width);
    }

    let line_text = cell[cell_line_index];
    line(
        f,
        line_text,
        width,
        style.padding.left,
        style.padding.right,
        style.alignment_h,
    )
}

fn top_indent(cell: &[&str], style: &Style, height: usize) -> usize {
    let height = height - style.padding.top.size;
    let content_height =
        cell_height(cell, style) - style.padding.top.size - style.padding.bottom.size;
    let indent = style.alignment_v.top_ident(height, content_height);
    indent + style.padding.top.size
}

fn repeat_char(f: &mut std::fmt::Formatter<'_>, c: char, n: usize) -> fmt::Result {
    if n > 0 {
        for _ in 0..n {
            write!(f, "{}", c)?;
        }
    }
    Ok(())
}

fn line(
    f: &mut std::fmt::Formatter<'_>,
    text: &str,
    width: usize,
    left_indent: Indent,
    right_indent: Indent,
    alignment: AlignmentHorizontal,
) -> fmt::Result {
    repeat_char(f, left_indent.fill, left_indent.size)?;
    alignment.align(f, text, width - left_indent.size - right_indent.size)?;
    repeat_char(f, right_indent.fill, right_indent.size)?;
    Ok(())
}

fn build_line<F: FnMut(&mut std::fmt::Formatter<'_>, usize) -> fmt::Result>(
    f: &mut std::fmt::Formatter<'_>,
    borders: &[BorderLine],
    row_styles: &[Style],
    length: usize,
    mut writer: F,
) -> fmt::Result {
    for (i, border) in borders.iter().enumerate() {
        if is_cell_visible(row_styles, i) {
            write_option(f, border.connector1)?;
            writer(f, i)?;
        }

        let is_last_cell = i + 1 == length;
        if is_last_cell {
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
    override_str: Option<&String>,
) -> fmt::Result {
    let theres_no_border = borders.iter().all(|l| l.main.is_none());
    if theres_no_border || borders.is_empty() {
        return Ok(());
    }

    let mut override_str = override_str.map(|s| s.to_owned());
    for (i, border) in borders.iter().enumerate().take(widths.len()) {
        if let Some(left_connector) = border.connector1 {
            let connector = override_str
                .as_mut()
                .and_then(|s| {
                    s.chars().next().map(|c| {
                        let _ = s.drain(..c.len_utf8());
                        c
                    })
                })
                .unwrap_or(left_connector);
            write!(f, "{}", connector)?
        }

        if let Some(main) = border.main {
            let mut width = widths[i];
            if let Some(s) = override_str.as_mut() {
                while !s.is_empty() && width > 0 {
                    match s.chars().next() {
                        Some(c) => {
                            write!(f, "{}", c)?;
                            width -= 1;
                            let _ = s.drain(..c.len_utf8());
                        }
                        None => break,
                    }
                }
            }

            while width > 0 {
                write!(f, "{}", main)?;
                width -= 1;
            }
        }

        let is_last_cell = i + 1 == widths.len();
        if is_last_cell {
            if let Some(right_connector) = border.connector2 {
                let connector = override_str
                    .as_mut()
                    .and_then(|s| {
                        s.chars().next().map(|c| {
                            let _ = s.drain(..c.len_utf8());
                            c
                        })
                    })
                    .unwrap_or(right_connector);
                write!(f, "{}", connector)?
            }
        }
    }

    writeln!(f)?;

    Ok(())
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

fn columns_width(
    cells: &mut [Vec<Vec<&str>>],
    styles: &mut [Vec<Style>],
    borders: &[Vec<BorderLine>],
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
    spans.remove(&0);

    spans.into_iter().for_each(|span| {
        adjust_width(
            &mut widths,
            styles,
            borders,
            count_rows,
            count_columns,
            span,
        );
    });

    widths
}

fn adjust_width(
    widths: &mut [Vec<usize>],
    styles: &[Vec<Style>],
    borders: &[Vec<BorderLine>],
    count_rows: usize,
    count_columns: usize,
    span: usize,
) {
    let ranges = (0..count_columns)
        .map(|col| (col, col + span))
        .take_while(|&(_, end)| end <= count_columns);

    for (start, end) in ranges.clone() {
        adjust_range_width(widths, styles, borders, count_rows, start, end);
    }

    // sometimes the adjustment of later stages affect the adjastement of privious stages.
    // therefore we check if this is the case and re run the adjustement one more time.
    for (start, end) in ranges {
        let is_range_complete = is_range_complete(styles, widths, borders, count_rows, start, end);
        if !is_range_complete {
            adjust_range_width(widths, styles, borders, count_rows, start, end);
        }
    }
}

fn adjust_range_width(
    widths: &mut [Vec<usize>],
    styles: &[Vec<Style>],
    borders: &[Vec<BorderLine>],
    count_rows: usize,
    start_column: usize,
    end_column: usize,
) {
    if count_rows == 0 {
        return;
    }
    let span = end_column - start_column;

    // find max width of a column range
    let (max_row, max_width) = (0..count_rows)
        .map(|row| {
            let width = row_width(
                &styles[row],
                &widths[row],
                &borders[row],
                start_column,
                end_column,
            );
            (row, width)
        })
        .max_by_key(|&(_, width)| width)
        .unwrap_or_default();

    // might happen when we filtered every cell
    if max_width == 0 {
        return;
    }

    // increase the widths
    (0..count_rows)
        .filter(|&row| row != max_row)
        .filter(|&row| !is_there_out_of_scope_cell(&styles[row], start_column, end_column)) // ignore the cell we do handle this case later on
        .for_each(|row| {
            let row_width = row_width(
                &styles[row],
                &widths[row],
                &borders[row],
                start_column,
                end_column,
            );

            let diff = max_width - row_width;

            inc_cells_width(
                &mut widths[row],
                &styles[row],
                start_column,
                end_column,
                diff,
            );
        });

    // fixing the rows with out_of_scope cells
    //
    // these cells may not have correct width, therefore
    // we replace these cells's width with
    // a width of cells with the same span and on the same column.
    (0..count_rows)
        .filter(|&row| row != max_row)
        .filter(|&row| is_there_out_of_scope_cell(&styles[row], start_column, end_column))
        .for_each(|row| {
            (start_column..end_column)
                .filter(|&col| is_cell_visible(&styles[row], col))
                .for_each(|col| {
                    let cell_with_the_same_cell = (0..count_rows)
                        .filter(|&r| r != max_row)
                        .filter(|&r| r != row)
                        .filter(|&r| !is_row_bigger_than_span(&styles[r], span))
                        .filter(|&r| {
                            !is_there_out_of_scope_cell(&styles[r], start_column, end_column)
                        })
                        .find(|&r| styles[r][col].span == styles[row][col].span);

                    if let Some(r) = cell_with_the_same_cell {
                        widths[row][col] = widths[r][col];
                    }
                })
        });
}

fn is_there_out_of_scope_cell(styles: &[Style], start_column: usize, end_column: usize) -> bool {
    let first_cell_is_invisible = !is_cell_visible(styles, start_column);
    let any_cell_out_of_scope = (start_column..end_column)
        .filter(|&col| is_cell_visible(styles, col))
        .any(|col| !is_cell_in_scope(styles, col, end_column));

    first_cell_is_invisible || any_cell_out_of_scope
}

fn is_cell_in_scope(styles: &[Style], col: usize, end_col: usize) -> bool {
    styles[col].span + col <= end_col
}

fn is_row_bigger_than_span(styles: &[Style], span: usize) -> bool {
    styles[0].span > span
}

fn is_cell_visible(row_styles: &[Style], column: usize) -> bool {
    let is_span_zero = row_styles[column].span == 0;
    let is_cell_overriden = row_styles[..column]
        .iter()
        .enumerate()
        .any(|(col, style)| style.span > column - col);

    !is_span_zero && !is_cell_overriden
}

fn is_range_complete(
    styles: &[Vec<Style>],
    widths: &[Vec<usize>],
    borders: &[Vec<BorderLine>],
    count_rows: usize,
    start_column: usize,
    end_column: usize,
) -> bool {
    let is_not_complete = (0..count_rows)
        .filter(|&row| !is_there_out_of_scope_cell(&styles[row], start_column, end_column))
        .map(|row| {
            row_width(
                &styles[row],
                &widths[row],
                &borders[row],
                start_column,
                end_column,
            )
        })
        .fold(None, |mut acc, width| {
            match acc {
                Some((w, true)) if w != width => {
                    acc = Some((0, false));
                }
                None => {
                    acc = Some((width, true));
                }
                _ => {}
            };

            acc
        });

    matches!(is_not_complete, Some((_, true)))
}

fn row_width(
    styles: &[Style],
    widths: &[usize],
    borders: &[BorderLine],
    column_start: usize,
    column_end: usize,
) -> usize {
    let width = (column_start..column_end)
        .filter(|&i| is_cell_visible(styles, i))
        .filter(|&i| is_cell_in_scope(styles, i, column_end))
        .map(|i| widths[i])
        .sum::<usize>();

    let border_count = if column_end - column_start == 0 {
        0
    } else {
        (column_start..column_end)
            .filter(|&i| is_cell_visible(styles, i))
            .filter(|&i| is_cell_in_scope(styles, i, column_end))
            .filter(|&i| {
                if i == column_start {
                    false
                } else {
                    borders[i].connector1.is_some()
                }
            })
            .count()
    };

    width + border_count
}

fn inc_cells_width(
    widths: &mut [usize],
    styles: &[Style],
    start_range: usize,
    end_range: usize,
    inc: usize,
) {
    (0..inc)
        .zip(
            (start_range..end_range)
                .filter(|&i| is_cell_visible(styles, i))
                .cycle(),
        )
        .for_each(|(_, i)| widths[i] += 1);
}

fn cell_width(cell: &[&str], style: &Style) -> usize {
    let content_width = cell.iter().map(|l| string_width(l)).max().unwrap_or(0);
    content_width + style.padding.left.size + style.padding.right.size
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
    content_height + style.padding.top.size + style.padding.bottom.size
}

fn normalized_width(
    widths: &[Vec<usize>],
    styles: &[Vec<Style>],
    count_rows: usize,
    count_columns: usize,
) -> Vec<usize> {
    let mut v = vec![0; count_columns];
    let mut skip = 0;
    for col in 0..count_columns {
        if skip > 0 {
            skip -= 1;
            continue;
        }

        let min_spanned_row = (0..count_rows)
            .filter(|&row| styles[row][col].span > 0)
            .min_by(|&x, &y| styles[x][col].span.cmp(&styles[y][col].span));

        if let Some(row) = min_spanned_row {
            let span = styles[row][col].span;
            let mut width = widths[row][col] - (span - 1);

            for col in (col..col + span).cycle() {
                if width == 0 {
                    break;
                }

                v[col] += 1;
                width -= 1;
            }

            skip += span - 1;
        }
    }

    v
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
            return Ok(vec![BorderLine::default(); self.count_columns]);
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
        for column in 0..self.count_columns {
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

        line[self.count_columns - 1].connector2 = self.get_vertical_char(row, self.count_columns);

        Ok(line)
    }

    // we can take only a border of a cell
    // which is a pitty,
    // would be cool if we could take a border of any Entity
    fn get_border(&self, row: usize, column: usize) -> Option<Border> {
        if row > self.count_rows || column > self.count_columns {
            return None;
        }

        let cell = CellBorderIndex::new(row, column);

        let border = Border {
            top: self.get_horizontal_char(cell.top().0, cell.top().1),
            bottom: self.get_horizontal_char(cell.bottom().0, cell.bottom().1),
            left: self.get_vertical_char(cell.left().0, cell.left().1),
            right: self.get_vertical_char(cell.right().0, cell.right().1),
            left_top_corner: self.get_intersection_char(cell.top_left()),
            left_bottom_corner: self.get_intersection_char(cell.bottom_left()),
            right_top_corner: self.get_intersection_char(cell.top_right()),
            right_bottom_corner: self.get_intersection_char(cell.bottom_right()),
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

#[derive(PartialEq, Eq, Debug, Hash, Clone)]
struct CellBorderIndex {
    row: usize,
    col: usize,
}

impl CellBorderIndex {
    fn new(row: usize, col: usize) -> Self {
        Self { row, col }
    }

    fn top(&self) -> GridPosition {
        (self.row, self.col)
    }

    fn bottom(&self) -> GridPosition {
        (self.row + 1, self.col)
    }

    fn left(&self) -> GridPosition {
        (self.row, self.col)
    }

    fn right(&self) -> GridPosition {
        (self.row, self.col + 1)
    }

    fn top_left(&self) -> GridPosition {
        (self.row, self.col)
    }

    fn top_right(&self) -> GridPosition {
        (self.row, self.col + 1)
    }

    fn bottom_left(&self) -> GridPosition {
        (self.row + 1, self.col)
    }

    fn bottom_right(&self) -> GridPosition {
        (self.row + 1, self.col + 1)
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
