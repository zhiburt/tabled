//! Papergrid is a library for generating text-based tables for display
//!
//! # Example
//! ```rust
//!     use papergrid::{Grid, Entity, Settings};
//!     let mut grid = Grid::new(2, 2);
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
    collections::HashMap,
    fmt::{self, Display},
    iter,
};

/// Grid provides a set of methods for building a text-based table
pub struct Grid {
    size: (usize, usize),
    styles: HashMap<Entity, Style>,
    cells: Vec<Vec<String>>,
    style_resolver_horizontal: StyleResolverHorizontal,
    style_resolver_vertical: StyleResolverVertical,
}

impl Grid {
    /// The new method creates a grid instance with default styles.
    ///
    /// The size of the grid can not be changed after the instance is created.
    ///
    /// # Example
    ///
    /// ```rust
    ///     use papergrid::{Grid, Entity, Settings};
    ///     let mut grid = Grid::new(2, 2);
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
            style_resolver_horizontal: StyleResolverHorizontal::LeftToRight,
            style_resolver_vertical: StyleResolverVertical::TopToBottom,
        }
    }

    /// Set method is responsible for modification of cell/row/column.
    ///
    /// The method panics if incorrect cell/row/column index is given.
    ///
    /// # Example
    ///
    /// ```rust
    ///     use papergrid::{Grid, Entity, Settings};
    ///     let mut grid = Grid::new(2, 2);
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
            self.style_mut(entity).border = border;
        }
    }

    /// get_cell_settings returns a settings of a cell
    pub fn get_settings(&mut self, row: usize, column: usize) -> Settings {
        let style = self.style(&Entity::Cell(row, column));
        let content = &self.cells[row][column];
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

    pub fn set_style_resolvers(
        &mut self,
        horizontal: StyleResolverHorizontal,
        vertical: StyleResolverVertical,
    ) {
        self.style_resolver_horizontal = horizontal;
        self.style_resolver_vertical = vertical;
    }

    /// get_cell_content returns content without any style changes
    pub fn get_cell_content(&mut self, row: usize, column: usize) -> &str {
        self.cells[row][column].as_str()
    }

    /// Get_border_mut returns a border for a given cell.
    pub fn get_border_mut(&mut self, row: usize, column: usize) -> &mut Border {
        debug_assert!(row < self.count_rows());
        &mut self.style_mut(&Entity::Cell(row, column)).border
    }

    /// Count_rows returns an amount of rows on the grid
    pub fn count_rows(&self) -> usize {
        self.size.0
    }

    /// Count_rows returns an amount of columns on the grid
    pub fn count_columns(&self) -> usize {
        self.size.1
    }

    /// Insert row in a grid.
    pub fn insert_row(&mut self, index: usize) {
        self.cells
            .insert(index, vec![String::new(); self.count_columns()]);
        self.size.0 += 1;
    }

    /// Removes a `row` from a grid.
    ///
    /// The row index must be started from 0
    pub fn remove_row(&mut self, row: usize) {
        self.cells.remove(row);

        // shift styles
        self.styles.remove(&Entity::Row(row));
        for row in row + 1..self.count_rows() {
            if self.styles.contains_key(&Entity::Row(row)) {
                let prev = self.styles.remove(&Entity::Row(row)).unwrap();
                self.styles.insert(Entity::Row(row - 1), prev);
            }
        }

        for column in 0..self.count_columns() {
            self.styles.remove(&Entity::Cell(row, column));
        }
        for row in row + 1..self.count_rows() {
            for column in 0..self.count_columns() {
                if let Some(prev) = self.styles.remove(&Entity::Cell(row, column)) {
                    self.styles.insert(Entity::Cell(row - 1, column), prev);
                }
            }
        }

        self.size.0 -= 1;
    }

    /// Removes a `column` from a grid.
    ///
    /// The column index must be started from 0
    pub fn remove_column(&mut self, column: usize) {
        for row in 0..self.count_rows() {
            self.cells[row].remove(column);
        }

        // shift styles
        self.styles.remove(&Entity::Column(column));
        for column in column + 1..self.count_columns() {
            if self.styles.contains_key(&Entity::Column(column)) {
                let prev = self.styles.remove(&Entity::Column(column)).unwrap();
                self.styles.insert(Entity::Column(column - 1), prev);
            }
        }

        for row in 0..self.count_rows() {
            self.styles.remove(&Entity::Cell(row, column));
        }
        for column in column + 1..self.count_columns() {
            for row in 0..self.count_rows() {
                if let Some(prev) = self.styles.remove(&Entity::Cell(row, column)) {
                    self.styles.insert(Entity::Cell(row, column - 1), prev);
                }
            }
        }

        self.size.1 -= 1;
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
}

/// An algorithm which is responsible for style merging on horizontal intersections
#[derive(Debug, Clone)]
pub enum StyleResolverHorizontal {
    LeftToRight,
    RightToLeft,
}

/// An algorithm which is responsible for style merging on intersections
#[derive(Debug, Clone)]
pub enum StyleResolverVertical {
    TopToBottom,
    BottomToTop,
}

/// Border structure represent all borders of a row
#[derive(Debug, Clone)]
pub struct Border {
    pub top: Option<char>,
    pub bottom: Option<char>,
    pub left: Option<char>,
    pub left_top_corner: Option<char>,
    pub left_bottom_corner: Option<char>,
    pub right: Option<char>,
    pub right_top_corner: Option<char>,
    pub right_bottom_corner: Option<char>,
}

impl Border {
    #[allow(clippy::too_many_arguments)]
    fn new(
        top: Option<char>,
        bottom: Option<char>,
        right: Option<char>,
        top_right_corner: Option<char>,
        bottom_right_corner: Option<char>,
        left: Option<char>,
        top_left_corner: Option<char>,
        bottom_left_corner: Option<char>,
    ) -> Self {
        Self {
            top,
            bottom,
            right,
            left,
            left_top_corner: top_left_corner,
            right_top_corner: top_right_corner,
            left_bottom_corner: bottom_left_corner,
            right_bottom_corner: bottom_right_corner,
        }
    }

    /// empty returns a border with no walls
    pub fn empty() -> Self {
        Self::new(None, None, None, None, None, None, None, None)
    }

    /// full returns a border all walls
    #[allow(clippy::too_many_arguments)]
    pub fn full(
        top: char,
        bottom: char,
        right: char,
        left: char,
        top_left_intersection: char,
        top_right_intersection: char,
        bottom_left_intersection: char,
        bottom_right_intersection: char,
    ) -> Self {
        Self::new(
            Some(top),
            Some(bottom),
            Some(right),
            Some(top_right_intersection),
            Some(bottom_right_intersection),
            Some(left),
            Some(top_left_intersection),
            Some(bottom_left_intersection),
        )
    }

    /// right returns a border with a wall on the right
    pub fn right(
        mut self,
        right: char,
        top_right_intersection: char,
        bottom_right_intersection: char,
    ) -> Self {
        self.right = Some(right);
        self.right_top_corner = Some(top_right_intersection);
        self.right_bottom_corner = Some(bottom_right_intersection);

        self
    }

    /// left returns a border with a wall on the left
    pub fn left(
        mut self,
        left: char,
        top_left_intersection: char,
        bottom_left_intersection: char,
    ) -> Self {
        self.left = Some(left);
        self.left_top_corner = Some(top_left_intersection);
        self.left_bottom_corner = Some(bottom_left_intersection);

        self
    }

    /// top returns a border with a wall on the top
    pub fn top(mut self, top: char) -> Self {
        self.top = Some(top);
        self
    }

    /// bottom returns a border with a wall on the top
    pub fn bottom(mut self, bottom: char) -> Self {
        self.bottom = Some(bottom);
        self
    }
}

#[derive(Debug)]
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
    indent: Indent,
    alignment_h: AlignmentHorizontal,
    alignment_v: AlignmentVertical,
    border: Border,
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
            border: Border::full('-', '-', '|', '|', '+', '+', '+', '+'),
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
    pub fn border(mut self, border: Border) -> Self {
        self.border = Some(border);
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
        let widths = __columns_width(&mut cells, &mut styles, count_rows, count_columns);

        for row_index in 0..count_rows {
            if row_index == 0 {
                let borders = collect_row_top_line_styles(&styles[row_index]);
                println!("{:?}", borders);
                build_split_line(
                    f,
                    &self.style_resolver_horizontal,
                    &widths[row_index],
                    &borders,
                )?;
            }

            let borders = collect_row_center_line_styles(&styles[row_index]);
            build_row(
                f,
                &self.style_resolver_horizontal,
                &cells[row_index],
                &styles[row_index],
                &widths[row_index],
                row_heights[row_index],
                &borders,
            )?;

            let borders = collect_row_bottom_line_styles(&styles[row_index]);
            build_split_line(
                f,
                &self.style_resolver_horizontal,
                &widths[row_index],
                &borders,
            )?;
        }

        Ok(())
    }
}

fn build_row(
    f: &mut std::fmt::Formatter<'_>,
    resolver: &StyleResolverHorizontal,
    row: &[Vec<&str>],
    row_styles: &[Style],
    widths: &[usize],
    height: usize,
    border: &[BorderLine],
) -> fmt::Result {
    for _line in 0..height {
        build_line(f, resolver, row.len(), border, |f, column| {
            let cell = &row[column];
            let style = &row_styles[column];
            let width = widths[column];

            let top_indent = top_indent(cell, style, height);
            if top_indent > _line {
                return empty_line(f, width);
            }

            let cell_line_index = _line - top_indent;
            let is_cell_has_this_line = cell.len() > cell_line_index;
            if !is_cell_has_this_line {
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
        })?;
    }

    Ok(())
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
    resolver: &StyleResolverHorizontal,
    length: usize,
    styles: &[BorderLine],
    writer: F,
) -> fmt::Result {
    for i in 0..length {
        match resolver {
            StyleResolverHorizontal::LeftToRight => {
                write_option(f, styles[i].connector1)?;
                writer(f, i)?;
                if i + 1 == length || length == 1 {
                    write_option(f, styles[i].connector2)?;
                }
            }
            StyleResolverHorizontal::RightToLeft => {
                if i == 0 {
                    write_option(f, styles[i].connector1)?;
                }

                writer(f, i)?;
                write_option(f, styles[i].connector2)?;
            }
        };
    }

    writeln!(f)?;

    Ok(())
}

fn build_split_line(
    f: &mut std::fmt::Formatter<'_>,
    resolver: &StyleResolverHorizontal,
    widths: &[usize],
    border: &[BorderLine],
) -> fmt::Result {
    let theres_no_border = border.iter().all(|l| l.main.is_none());
    if theres_no_border || border.is_empty() {
        return Ok(());
    }

    build_line(f, resolver, widths.len(), border, |f, i| {
        write_option(f, border[i].main.map(|m| m.to_string().repeat(widths[i])))
    })
}

fn write_option<D: Display>(f: &mut std::fmt::Formatter<'_>, text: Option<D>) -> fmt::Result {
    match text {
        Some(text) => write!(f, "{}", text),
        None => Ok(()),
    }
}

fn collect_row_bottom_line_styles(row_styles: &[Style]) -> Vec<BorderLine> {
    let mut rows = Vec::with_capacity(row_styles.len());
    for style in row_styles {
        rows.push(BorderLine {
            main: style.border.bottom,
            connector1: style.border.left_bottom_corner,
            connector2: style.border.right_bottom_corner,
        });
    }

    rows
}

fn collect_row_top_line_styles(row_styles: &[Style]) -> Vec<BorderLine> {
    let mut rows = Vec::with_capacity(row_styles.len());
    for style in row_styles {
        rows.push(BorderLine {
            main: style.border.top,
            connector1: style.border.left_top_corner,
            connector2: style.border.right_top_corner,
        });
    }

    rows
}

fn collect_row_center_line_styles(row_styles: &[Style]) -> Vec<BorderLine> {
    let mut rows = Vec::with_capacity(row_styles.len());
    for style in row_styles {
        rows.push(BorderLine {
            main: None,
            connector1: style.border.left,
            connector2: style.border.right,
        });
    }

    rows
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

fn __columns_width(
    cells: &mut [Vec<Vec<&str>>],
    styles: &mut [Vec<Style>],
    count_rows: usize,
    count_columns: usize,
) -> Vec<Vec<usize>> {
    let mut widths = vec![vec! {0; count_columns}; count_rows];
    (0..count_rows).for_each(|row| {
        (0..count_columns).for_each(|column| {
            let cell = &cells[row][column];
            let style = &styles[row][column];
            if is_cell_visible(&styles[row], column) {
                widths[row][column] = cell_width(cell, style);
            } else {
                widths[row][column] = 0;
            }
        });
    });

    // check if we don't need to check all spans as it a heavy load function.
    // it suppose to save us time and resources
    let spans_was_used = styles
        .iter()
        .any(|row_styles| row_styles.iter().any(|style| style.span > 1));
    if spans_was_used {
        (1..count_columns + 1).for_each(|span| {
            __adjust_width(&mut widths, cells, styles, count_rows, count_columns, span);
        });
    } else {
        __adjust_width(&mut widths, cells, styles, count_rows, count_columns, 1);
    }

    // remove not visible cells to print everything correctly
    (0..count_rows).for_each(|row| {
        let mut n_removed = 0;
        (0..count_columns)
            .filter(|&column| !is_cell_visible(&styles[row], column))
            .collect::<Vec<_>>() // it's here becouse of borrow rules...
            .into_iter()
            .for_each(|column| {
                widths[row].remove(column - n_removed);
                cells[row].remove(column - n_removed);
                styles[row].remove(column - n_removed);
                n_removed += 1;
            });
    });

    widths
}

fn __adjust_width(
    widths: &mut [Vec<usize>],
    cells: &[Vec<Vec<&str>>],
    styles: &[Vec<Style>],
    count_rows: usize,
    count_columns: usize,
    span: usize,
) {
    (0..count_rows).for_each(|row| {
        (0..count_columns)
            .filter(|&column| styles[row][column].span == span)
            .filter(|&column| is_cell_visible(&styles[row], column))
            .for_each(|column| {
                let cell = &cells[row][column];
                let style = &styles[row][column];
                let cell_width = cell_width(cell, style);
                // calc other's width

                let others_width = (0..count_rows)
                    .filter(|&r| r != row)
                    .filter(|&r| {
                        styles[r][column..column + span]
                            .iter()
                            .map(|style| style.span)
                            .sum::<usize>()
                            <= span
                    })
                    .map(|r| row_width(&styles[r], &widths[r][column..column + span]))
                    .max()
                    .unwrap_or(0);

                if cell_width > others_width {
                    widths[row][column] = cell_width;

                    (0..count_rows)
                        .filter(|&r| r != row)
                        .filter(|&r| {
                            styles[r][column..column + span]
                                .iter()
                                .map(|style| style.span)
                                .sum::<usize>()
                                <= span
                        }) // not sure if it's correct
                        .for_each(|r| {
                            inc_width_to_cells(
                                &mut widths[r],
                                &styles[r],
                                column,
                                column + span,
                                cell_width,
                            );
                        });
                } else {
                    inc_width_to_cells(
                        &mut widths[row],
                        &styles[row],
                        column,
                        column + 1,
                        others_width,
                    );
                }
            });
    });
}

fn is_cell_visible(row_styles: &[Style], column: usize) -> bool {
    !row_styles[..column]
        .iter()
        .zip(column..)
        .any(|(style, span)| style.span > span)
}

// relyes on fix_spans
fn row_width(row_styles: &[Style], widths: &[usize]) -> usize {
    let w = (0..widths.len())
        .filter(|&i| is_cell_visible(row_styles, i))
        .map(|i| widths[i])
        .sum::<usize>();

    if w == 0 {
        return 0;
    }

    w + (0..widths.len())
        .filter(|&i| is_cell_visible(row_styles, i))
        .count()
        - 1
}

// relyes on fix_spans
fn inc_width_to_cells(
    widths: &mut [usize],
    row_styles: &[Style],
    start_range: usize,
    end_range: usize,
    width: usize,
) {
    let a = row_width(row_styles, &widths[start_range..end_range]);
    let diff = width - a;

    (0..diff)
        .zip(
            (start_range..end_range)
                .filter(|&i| is_cell_visible(row_styles, i))
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn build_line_test() {
        struct Foo(StyleResolverHorizontal);

        impl fmt::Display for Foo {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                let styles = [
                    BorderLine {
                        connector1: Some('+'),
                        connector2: Some('='),
                        main: Some('-'),
                    },
                    BorderLine {
                        connector1: Some('x'),
                        connector2: Some('y'),
                        main: Some('u'),
                    },
                    BorderLine {
                        connector1: Some('('),
                        connector2: Some(')'),
                        main: Some('*'),
                    },
                ];
                build_line(f, &self.0, styles.len(), &styles, |f, _| write!(f, "00000"))
            }
        }

        assert_eq!(
            Foo(StyleResolverHorizontal::LeftToRight).to_string(),
            "+00000x00000(00000)\n"
        );
        assert_eq!(
            Foo(StyleResolverHorizontal::RightToLeft).to_string(),
            "+00000=00000y00000)\n"
        );
    }

    #[test]
    fn grid_2x2_change_cell_border_test() {
        let mut grid = Grid::new(2, 2);
        grid.set(&Entity::Global, Settings::new().text("asd"));
        grid.set(
            &Entity::Cell(0, 1),
            Settings::new().border(Border::full('*', '^', '#', '@', '~', '!', '%', '&')),
        );
        let str = grid.to_string();
        assert_eq!(
            str,
            "+---~***!\n\
             |asd@asd#\n\
             +---%^^^&\n\
             |asd|asd|\n\
             +---+---+\n"
        )
    }

    #[test]
    fn grid_1x1_test() {
        let mut grid = Grid::new(1, 1);
        grid.set(&Entity::Cell(0, 0), Settings::new().text("asd"));
        let str = grid.to_string();
        assert_eq!(
            str,
            "+---+\n\
             |asd|\n\
             +---+\n"
        )
    }

    #[test]
    fn grid_2x2_test() {
        let mut grid = Grid::new(2, 2);
        grid.set(&Entity::Global, Settings::new().text("asd"));
        let str = grid.to_string();
        assert_eq!(
            str,
            "+---+---+\n\
             |asd|asd|\n\
             +---+---+\n\
             |asd|asd|\n\
             +---+---+\n"
        )
    }

    #[test]
    fn grid_2x2_entity_settings_test() {
        let mut grid = Grid::new(2, 2);
        grid.set(&Entity::Global, Settings::new().text("asd"));
        grid.set(&Entity::Column(0), Settings::new().text("zxc"));
        grid.set(&Entity::Row(0), Settings::new().text("qwe"));
        grid.set(&Entity::Cell(1, 1), Settings::new().text("iop"));
        let str = grid.to_string();
        assert_eq!(
            str,
            "+---+---+\n\
             |qwe|qwe|\n\
             +---+---+\n\
             |zxc|iop|\n\
             +---+---+\n"
        )
    }

    #[test]
    fn grid_2x2_alignment_test() {
        let mut grid = Grid::new(2, 2);
        grid.set(&Entity::Global, Settings::new().text("asd    "));
        grid.set(
            &Entity::Column(0),
            Settings::new().alignment(AlignmentHorizontal::Left),
        );
        grid.set(
            &Entity::Column(1),
            Settings::new().alignment(AlignmentHorizontal::Right),
        );
        let str = grid.to_string();

        assert_eq!(
            str,
            "+-------+-------+\n\
             |asd    |    asd|\n\
             +-------+-------+\n\
             |asd    |    asd|\n\
             +-------+-------+\n"
        )
    }

    #[test]
    fn grid_2x2_indent_test() {
        let mut grid = Grid::new(2, 2);
        grid.set(
            &Entity::Global,
            Settings::new().text("asd").indent(1, 1, 1, 1),
        );
        grid.set(&Entity::Column(0), Settings::new().indent(0, 0, 0, 0));
        let str = grid.to_string();

        assert_eq!(
            str,
            "+---+-----+\n\
             |asd|     |\n\
             |   | asd |\n\
             |   |     |\n\
             +---+-----+\n\
             |asd|     |\n\
             |   | asd |\n\
             |   |     |\n\
             +---+-----+\n"
        )
    }

    #[test]
    fn grid_2x2_vertical_resize_test() {
        let mut grid = Grid::new(2, 2);
        grid.set(&Entity::Global, Settings::new().text("asd"));
        grid.set(&Entity::Cell(1, 1), Settings::new().text("asd     "));
        let str = grid.to_string();
        assert_eq!(
            str,
            "+---+--------+\n\
             |asd|asd     |\n\
             +---+--------+\n\
             |asd|asd     |\n\
             +---+--------+\n"
        )
    }

    #[test]
    fn grid_2x2_without_frame_test() {
        let mut grid = Grid::new(2, 2);
        grid.set(&Entity::Global, Settings::new().text("asd"));

        {
            grid.set_style_resolvers(
                StyleResolverHorizontal::RightToLeft,
                StyleResolverVertical::BottomToTop,
            );
            grid.style_mut(&Entity::Global).border = Border::empty();
            grid.style_mut(&Entity::Cell(0, 0)).border = Border::empty().right(' ', ' ', ' ');
            grid.style_mut(&Entity::Cell(1, 0)).border = Border::empty().right(' ', ' ', ' ');

            let str = grid.to_string();
            assert_eq!(
                str,
                "asd asd\n\
                 asd asd\n"
            )
        }

        {
            grid.set_style_resolvers(
                StyleResolverHorizontal::LeftToRight,
                StyleResolverVertical::BottomToTop,
            );
            grid.style_mut(&Entity::Global).border = Border::empty();
            grid.style_mut(&Entity::Cell(0, 1)).border = Border::empty().left(' ', ' ', ' ');
            grid.style_mut(&Entity::Cell(1, 1)).border = Border::empty().left(' ', ' ', ' ');

            let str = grid.to_string();
            assert_eq!(
                str,
                "asd asd\n\
                 asd asd\n"
            )
        }
    }

    #[test]
    fn grid_2x2_custom_border_test() {
        let mut grid = Grid::new(2, 2);
        grid.set(&Entity::Global, Settings::new().text("asd"));
        grid.style_mut(&Entity::Cell(0, 0)).border =
            Border::empty().top('*').bottom('-').left('$', ' ', '+');
        grid.style_mut(&Entity::Cell(0, 1)).border = Border::empty()
            .top('*')
            .bottom('-')
            .left('@', ' ', '+')
            .right('%', ' ', '+');
        grid.style_mut(&Entity::Cell(1, 0)).border =
            Border::empty().bottom('*').left('#', '+', '\u{0020}');
        grid.style_mut(&Entity::Cell(1, 1)).border = Border::empty()
            .bottom('*')
            .left('^', '+', ' ')
            .right('!', '+', ' ');

        let str = grid.to_string();
        assert_eq!(
            str,
            " *** *** \n\
             $asd@asd%\n\
             +---+---+\n\
             #asd^asd!\n\
             \u{0020}*** *** \n"
        )
    }

    #[test]
    fn grid_2x2_remove_row_test() {
        let mut grid = Grid::new(2, 2);
        grid.set(&Entity::Global, Settings::new().text("asd"));
        grid.remove_row(0);
        let str = grid.to_string();
        assert_eq!(
            str,
            "+---+---+\n\
             |asd|asd|\n\
             +---+---+\n"
        )
    }

    #[test]
    fn grid_2x2_remove_column_test() {
        let mut grid = Grid::new(2, 2);
        grid.set(&Entity::Global, Settings::new().text("asd"));
        grid.remove_column(0);
        let str = grid.to_string();
        assert_eq!(
            str,
            "+---+\n\
             |asd|\n\
             +---+\n\
             |asd|\n\
             +---+\n"
        )
    }

    #[test]
    fn grid_3x2_test() {
        let mut grid = Grid::new(3, 2);
        grid.set(&Entity::Global, Settings::new().text("asd"));
        let str = grid.to_string();
        assert_eq!(
            str,
            "+---+---+\n\
             |asd|asd|\n\
             +---+---+\n\
             |asd|asd|\n\
             +---+---+\n\
             |asd|asd|\n\
             +---+---+\n"
        )
    }

    #[test]
    #[ignore = "I am not sure what is the right behaiviour here"]
    fn hieroglyph_handling() {
        let mut grid = Grid::new(1, 2);
        grid.set(&Entity::Cell(0, 0), Settings::new().text("哈哈"));
        grid.set(&Entity::Cell(0, 1), Settings::new().text("哈"));
        let s = grid.to_string();
        assert_eq!(
            s,
            "+----+--+\n\
             |哈哈  |哈 |\n\
             +----+--+\n"
        )
    }

    #[test]
    #[ignore = "I am not sure what is the right behaiviour here"]
    fn hieroglyph_multiline_handling() {
        let mut grid = Grid::new(1, 2);
        grid.set(&Entity::Cell(0, 0), Settings::new().text("哈哈"));
        grid.set(&Entity::Cell(0, 1), Settings::new().text("哈\n哈"));
        let s = grid.to_string();
        assert_eq!(
            s,
            "+----+--+\n\
             |哈哈  |哈 |\n\
             |    |哈 |\n\
             +----+--+\n"
        )
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

    #[test]
    fn grid_2x2_span_test() {
        let mut grid = Grid::new(2, 2);
        grid.set(&Entity::Global, Settings::new().text("asd"));
        grid.set(&Entity::Cell(0, 0), Settings::new().text("123").span(2));
        let str = grid.to_string();
        assert_eq!(
            str,
            "+-------+\n\
             |123    |\n\
             +-------+\n\
             |asd|asd|\n\
             +---+---+\n"
        )
    }
}
