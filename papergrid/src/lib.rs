//! Papergrid is a library for generating text-based tables for display
//!
//! # Example
//! ```rust
//!     use papergrid::{Grid, Entity, Settings};
//!     let mut grid = Grid::new(2, 2);
//!     grid.set(Entity::Cell(0, 0), Settings::new().text("0-0"));
//!     grid.set(Entity::Cell(0, 1), Settings::new().text("0-1"));
//!     grid.set(Entity::Cell(1, 0), Settings::new().text("1-0"));
//!     grid.set(Entity::Cell(1, 1), Settings::new().text("1-1"));
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
    border_styles: Vec<Border>,
    styles: HashMap<Entity, Style>,
    cells: Vec<Vec<String>>,
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

        let border_styles = iter::repeat(Self::default_border()).take(rows).collect();

        Grid {
            size: (rows, columns),
            cells: vec![vec![String::new(); columns]; rows],
            border_styles,
            styles,
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
    ///     grid.set(Entity::Row(0), Settings::new().text("row 1"));
    ///     grid.set(Entity::Row(1), Settings::new().text("row 2"));
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
    pub fn set(&mut self, entity: Entity, settings: Settings) {
        if let Some(text) = settings.text {
            self.set_text(&entity, text);
        }

        if settings.indent.is_none()
            && settings.alignment_h.is_none()
            && settings.alignment_v.is_none()
        {
            return;
        }

        // Check for existed style and don't rewrite it totally in case it exists,
        // only change parts which are set in settings
        let mut s = self
            .styles
            .get(&entity)
            .map_or_else(Style::default, |s| s.clone());

        if let Some(indent) = settings.indent {
            s.indent = indent;
        }
        if let Some(alignment) = settings.alignment_h {
            s.alignment_h = alignment;
        }
        if let Some(alignment) = settings.alignment_v {
            s.alignment_v = alignment;
        }

        self.styles.insert(entity, s);
    }

    /// get_cell_content returns content without any style changes
    pub fn get_cell_content(&mut self, row: usize, column: usize) -> &str {
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

    /// Get_border_mut returns a border for a given row.
    /// The border can be modified.
    ///
    /// # Example
    ///
    /// ```rust
    ///    use papergrid::{Grid, Entity, Settings};
    ///    let mut grid = Grid::new(2, 2);
    ///    grid.set(Entity::Global, Settings::new().text("asd"));
    ///    grid.get_border_mut(0).empty()
    ///         .top('─', '┬', Some('┌'), Some('┐'))
    ///         .bottom('─', '┼', Some('├'), Some('┤'))
    ///         .inner(Some('│'), Some('│'), Some('│'));
    ///    grid.get_border_mut(1).empty()
    ///         .top('─', '┬', Some('┌'), Some('┐'))
    ///         .bottom('─', '┴', Some('└'), Some('┘'))
    ///         .inner(Some('│'), Some('│'), Some('│'));
    ///
    ///    let str = grid.to_string();
    ///    assert_eq!(
    ///        str,
    ///        "┌───┬───┐\n\
    ///         │asd│asd│\n\
    ///         ├───┼───┤\n\
    ///         │asd│asd│\n\
    ///         └───┴───┘\n"
    ///    )
    /// ```
    pub fn get_border_mut(&mut self, row: usize) -> &mut Border {
        debug_assert!(row < self.count_rows());
        &mut self.border_styles[row]
    }

    /// Remove_row removes a `row` from a grid.
    ///
    /// The row index must be started from 0
    pub fn remove_row(&mut self, row: usize) {
        self.cells.remove(row);
        self.border_styles.remove(row);
        self.size.0 -= 1;
    }

    /// Remove_row removes a `column` from a grid.
    ///
    /// The column index must be started from 0
    pub fn remove_column(&mut self, column: usize) {
        self.size.1 -= 1;
        for row in 0..self.count_rows() {
            self.cells[row].remove(column);
        }
    }

    fn set_text<S: Into<String>>(&mut self, entity: &Entity, text: S) {
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

    fn style(&self, row: usize, column: usize) -> Style {
        let v = [
            self.styles.get(&Entity::Cell(row, column)),
            self.styles.get(&Entity::Column(column)),
            self.styles.get(&Entity::Row(row)),
            self.styles.get(&Entity::Global),
        ];

        #[allow(clippy::manual_flatten)]
        for styles in &v {
            if let Some(style) = styles {
                return (*style).clone();
            }
        }

        unreachable!("there's a global settings guaranted in the map")
    }

    fn info(&self) -> Info<'_> {
        let count_rows = self.count_rows();
        let count_columns = self.count_columns();
        let mut column_widths = vec![0; count_columns];
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
        let mut rows = Vec::with_capacity(count_rows);

        (0..count_rows).for_each(|row_index| {
            let mut row = Vec::with_capacity(count_columns);

            (0..count_columns).for_each(|column_index| {
                let content = &self.cells[row_index][column_index];
                let cell: Vec<_> = content.lines().collect();
                let style = self.style(row_index, column_index);

                let content_height = cell.len();
                let cell_height = content_height + style.indent.top + style.indent.bottom;

                let content_width = string_width(content);
                let cell_width = content_width + style.indent.left + style.indent.right;

                column_widths[column_index] = max(column_widths[column_index], cell_width);
                row_heights[row_index] = max(row_heights[row_index], cell_height);
                row.push((cell, style));
            });

            rows.push(row);
        });

        Info {
            cells: rows,
            row_heights,
            column_widths,
        }
    }

    fn default_border() -> Border {
        Border {
            inner: LineStyle {
                main: Some('-'),
                intersection: Some('|'),
                left_intersection: Some('|'),
                right_intersection: Some('|'),
            },
            bottom_line: LineStyle {
                main: Some('-'),
                intersection: Some('+'),
                left_intersection: Some('+'),
                right_intersection: Some('+'),
            },
            top_line: LineStyle {
                main: Some('-'),
                intersection: Some('+'),
                left_intersection: Some('+'),
                right_intersection: Some('+'),
            },
        }
    }
}

#[derive(Debug)]
struct Info<'a> {
    cells: Vec<Vec<(Vec<&'a str>, Style)>>,
    row_heights: Vec<usize>,
    column_widths: Vec<usize>,
}

/// Settings represent setting of a particular cell
#[derive(Debug, Clone, Default)]
pub struct Settings {
    text: Option<String>,
    indent: Option<Indent>,
    alignment_h: Option<AlignmentHorizontal>,
    alignment_v: Option<AlignmentVertical>,
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
}

/// Border structure represent all borders of a row
#[derive(Debug, Clone)]
pub struct Border {
    top_line: LineStyle,
    bottom_line: LineStyle,
    inner: LineStyle,
}

impl Border {
    /// empty border returns a border for a row with no frame and no internal separation
    pub fn empty(&mut self) -> &mut Self {
        *self = Self {
            top_line: LineStyle::default(),
            bottom_line: LineStyle::default(),
            inner: LineStyle::default(),
        };

        self
    }

    /// The method sets a top border line.
    ///
    /// * `main` - is a character which is used for building line.
    /// * `intersection` - a character which is used for internal separation on the line.
    /// * `left_intersection` - a left border character.
    /// * `right_intersection` - a right border character.
    pub fn top(
        &mut self,
        main: char,
        intersection: char,
        left_intersection: Option<char>,
        right_intersection: Option<char>,
    ) -> &mut Self {
        self.top_line = LineStyle {
            main: Some(main),
            intersection: Some(intersection),
            left_intersection,
            right_intersection,
        };

        self
    }

    /// The method sets a bottom border line.
    ///
    /// * `main` - is a character which is used for building line.
    /// * `intersection` - a character which is used for internal separation on the line.
    /// * `left_intersection` - a left border character.
    /// * `right_intersection` - a right border character.
    pub fn bottom(
        &mut self,
        main: char,
        intersection: char,
        left_intersection: Option<char>,
        right_intersection: Option<char>,
    ) -> &mut Self {
        self.bottom_line = LineStyle {
            main: Some(main),
            intersection: Some(intersection),
            left_intersection,
            right_intersection,
        };

        self
    }

    /// The method sets an inner row symbols.
    ///
    /// * `intersection` - a character which is used for internal separation on the line.
    /// * `left_intersection` - a left border character.
    /// * `right_intersection` - a right border character.
    pub fn inner(
        &mut self,
        intersection: Option<char>,
        left_intersection: Option<char>,
        right_intersection: Option<char>,
    ) -> &mut Self {
        self.inner = LineStyle {
            main: None,
            intersection,
            left_intersection,
            right_intersection,
        };

        self
    }
}

#[derive(Debug, Clone, Default)]
struct LineStyle {
    main: Option<char>,
    intersection: Option<char>,
    left_intersection: Option<char>,
    right_intersection: Option<char>,
}

impl LineStyle {
    fn is_empty(&self) -> bool {
        self.left_intersection.is_none()
            && self.right_intersection.is_none()
            && self.intersection.is_none()
            && self.main.is_none()
    }
}

/// Entity a structure which represent a set of cells.
#[derive(PartialEq, Eq, Debug, Hash)]
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
struct Style {
    indent: Indent,
    alignment_h: AlignmentHorizontal,
    alignment_v: AlignmentVertical,
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
        }
    }
}

#[derive(Debug, Clone)]
struct Indent {
    top: usize,
    bottom: usize,
    left: usize,
    right: usize,
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

impl std::fmt::Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // It may happen when all cells removed via `remove_row`, `remove_column` methods
        if self.count_rows() == 0 || self.count_columns() == 0 {
            return Ok(());
        }

        let information = self.info();

        for (row_index, row) in information.cells.into_iter().enumerate() {
            let border = self
                .border_styles
                .get(row_index)
                .expect("it's expected that grid has N styles where N is an amount of rows");

            if row_index == 0 {
                build_line(f, &information.column_widths, &border.top_line)?;
            }

            build_row(
                f,
                row,
                &information.column_widths,
                information.row_heights[row_index],
                &border.inner,
            )?;

            build_line(f, &information.column_widths, &border.bottom_line)?;
        }

        Ok(())
    }
}

fn build_row(
    f: &mut std::fmt::Formatter<'_>,
    mut row: Vec<(Vec<&str>, Style)>,
    widths: &[usize],
    height: usize,
    border: &LineStyle,
) -> fmt::Result {
    let mut top_indents = Vec::with_capacity(row.len());
    for (cell, style) in &row {
        let content_height = cell.len();
        let height = height - style.indent.top - style.indent.bottom;

        let indent = style.alignment_v.top_ident(height, content_height);
        let indent = indent + style.indent.top;

        top_indents.push(indent);
    }

    for line_index in 0..height {
        write_option(f, border.left_intersection)?;

        for (column_index, (cell, style)) in row.iter_mut().enumerate() {
            if column_index != 0 {
                write_option(f, border.intersection)?;
            }

            let width = widths[column_index];

            if top_indents[column_index] > line_index {
                empty_line(f, width)?;
                continue;
            }

            let index = line_index - top_indents[column_index];
            match cell.get(index) {
                Some(s) => line(
                    f,
                    s,
                    width,
                    style.indent.left,
                    style.indent.right,
                    style.alignment_h,
                )?,
                None => {
                    empty_line(f, width)?;
                }
            }
        }

        write_option(f, border.right_intersection)?;

        writeln!(f)?;
    }

    Ok(())
}

fn empty_line(f: &mut std::fmt::Formatter<'_>, n: usize) -> fmt::Result {
    write!(f, "{: ^1$}", "", n)
}

fn line(
    f: &mut std::fmt::Formatter<'_>,
    text: &str,
    width: usize,
    left_indent: usize,
    right_indent: usize,
    alignment: AlignmentHorizontal,
) -> fmt::Result {
    write!(f, "{}", " ".repeat(left_indent))?;
    alignment.align(f, text, width - left_indent - right_indent)?;
    write!(f, "{}", " ".repeat(right_indent))?;
    Ok(())
}

fn build_line(
    f: &mut std::fmt::Formatter<'_>,
    cells_width: &[usize],
    border: &LineStyle,
) -> fmt::Result {
    if border.is_empty() {
        return Ok(());
    }

    write_option(f, border.left_intersection)?;

    for (i, w) in cells_width.iter().enumerate() {
        write_option(f, border.main.map(|m| m.to_string().repeat(*w)))?;

        if i != cells_width.len() - 1 {
            write_option(f, border.intersection)?;
        }
    }

    write_option(f, border.right_intersection)?;

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
fn string_width(text: &str) -> usize {
    real_string_width(text)
}

#[cfg(feature = "color")]
fn string_width(text: &str) -> usize {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn grid_1x1_test() {
        let mut grid = Grid::new(1, 1);
        grid.set(Entity::Cell(0, 0), Settings::new().text("asd"));
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
        grid.set(Entity::Global, Settings::new().text("asd"));
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
        grid.set(Entity::Global, Settings::new().text("asd"));
        grid.set(Entity::Column(0), Settings::new().text("zxc"));
        grid.set(Entity::Row(0), Settings::new().text("qwe"));
        grid.set(Entity::Cell(1, 1), Settings::new().text("iop"));
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
        grid.set(Entity::Global, Settings::new().text("asd    "));
        grid.set(
            Entity::Column(0),
            Settings::new().alignment(AlignmentHorizontal::Left),
        );
        grid.set(
            Entity::Column(1),
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
            Entity::Global,
            Settings::new().text("asd").indent(1, 1, 1, 1),
        );
        grid.set(Entity::Column(0), Settings::new().indent(0, 0, 0, 0));
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
        grid.set(Entity::Global, Settings::new().text("asd"));
        grid.set(Entity::Cell(1, 1), Settings::new().text("asd     "));
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
        grid.set(Entity::Global, Settings::new().text("asd"));
        grid.get_border_mut(0).empty().inner(Some(' '), None, None);
        grid.get_border_mut(1).empty().inner(Some(' '), None, None);

        let str = grid.to_string();
        assert_eq!(
            str,
            "asd asd\n\
             asd asd\n"
        )
    }

    #[test]
    fn grid_2x2_custom_border_test() {
        let mut grid = Grid::new(2, 2);
        grid.set(Entity::Global, Settings::new().text("asd"));

        grid.get_border_mut(0)
            .top('*', ' ', Some(' '), Some(' '))
            .inner(Some('@'), Some('$'), Some('%'));
        grid.get_border_mut(1)
            .top('*', ' ', Some(' '), Some(' '))
            .bottom('*', ' ', Some(' '), Some(' '))
            .inner(Some('^'), Some('#'), Some('!'));

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
        grid.set(Entity::Global, Settings::new().text("asd"));
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
        grid.set(Entity::Global, Settings::new().text("asd"));
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
        grid.set(Entity::Global, Settings::new().text("asd"));
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
        grid.set(Entity::Cell(0, 0), Settings::new().text("哈哈"));
        grid.set(Entity::Cell(0, 1), Settings::new().text("哈"));
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
        grid.set(Entity::Cell(0, 0), Settings::new().text("哈哈"));
        grid.set(Entity::Cell(0, 1), Settings::new().text("哈\n哈"));
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
}
