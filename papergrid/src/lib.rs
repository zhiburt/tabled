//! Papergrid is a library for generating text-based tables for display
//!
//! # Example
//! ```rust
//!     use papergrid::{Grid, Entity, Settings};
//!     let mut grid = Grid::new(2, 2);
//!
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
    cmp::{self, max},
    collections::{BTreeMap, HashMap, HashSet},
    fmt::{self, Display, Write},
    hash::Hash,
    ops::{Bound, RangeBounds},
};

pub const DEFAULT_BORDERS: Borders = Borders {
    top: Some(Symbol::from_char('-')),
    top_left: Some(Symbol::from_char('+')),
    top_right: Some(Symbol::from_char('+')),
    top_intersection: Some(Symbol::from_char('+')),

    bottom: Some(Symbol::from_char('-')),
    bottom_left: Some(Symbol::from_char('+')),
    bottom_right: Some(Symbol::from_char('+')),
    bottom_intersection: Some(Symbol::from_char('+')),

    horizontal: Some(Symbol::from_char('-')),
    horizontal_left: Some(Symbol::from_char('+')),
    horizontal_right: Some(Symbol::from_char('+')),

    vertical_left: Some(Symbol::from_char('|')),
    vertical_right: Some(Symbol::from_char('|')),
    vertical_intersection: Some(Symbol::from_char('|')),

    intersection: Some(Symbol::from_char('+')),
};

const DEFAULT_BORDER_HORIZONTAL_CHAR: char = ' ';

const DEFAULT_BORDER_HORIZONTAL_SYMBOL: Symbol = Symbol::from_char(' ');
const DEFAULT_BORDER_VERTICAL_SYMBOL: Symbol = Symbol::from_char(' ');

const DEFAULT_BORDER_VERTICAL_SYMBOL_REF: &Symbol = &DEFAULT_BORDER_VERTICAL_SYMBOL;

const DEFAULT_INDENT_FILL_CHAR: char = ' ';

/// Grid provides a set of methods for building a text-based table
#[derive(Debug, Clone)]
pub struct Grid {
    size: (usize, usize),
    cells: Vec<Vec<String>>,
    styles: HashMap<Entity, Style>,
    margin: Margin,
    theme: Theme,
    override_split_lines: HashMap<usize, String>,
    spans: BTreeMap<(usize, usize), HashSet<usize>>,
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
    ///     assert_eq!(
    ///          grid.to_string(),
    ///          "+++\n\
    ///           +++\n\
    ///           +++\n"
    ///     )
    /// ```
    pub fn new(rows: usize, columns: usize) -> Self {
        let mut styles = HashMap::with_capacity(1);
        styles.insert(Entity::Global, Style::default());

        Grid {
            size: (rows, columns),
            cells: vec![vec![String::new(); columns]; rows],
            styles,
            margin: Margin::default(),
            theme: Theme::new(),
            override_split_lines: HashMap::new(),
            spans: BTreeMap::new(),
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
    ///     assert_eq!(
    ///          grid.to_string(),
    ///          "+-----+-----+\n\
    ///           |row 1|row 1|\n\
    ///           +-----+-----+\n\
    ///           |row 2|row 2|\n\
    ///           +-----+-----+\n"
    ///     )
    /// ```
    pub fn set(&mut self, entity: Entity, settings: Settings) {
        let is_style_changes = settings.padding.is_some()
            || settings.alignment_h.is_some()
            || settings.alignment_v.is_some()
            || settings.span.is_some()
            || settings.formatting.is_some();

        if is_style_changes {
            self.remove_inherited_styles(entity);
            let style = self.style_mut(entity);

            if let Some(padding) = settings.padding {
                style.padding = padding;
            }

            if let Some(alignment_h) = settings.alignment_h {
                style.alignment_h = alignment_h;
            }

            if let Some(alignment_v) = settings.alignment_v {
                style.alignment_v = alignment_v;
            }

            if let Some(formatting) = settings.formatting {
                style.formatting = formatting;
            }
        }

        if let Some(text) = settings.text {
            self.set_text(entity, text);
        }

        if let Some(border) = settings.border {
            self.set_border(entity, border);
        }

        if let Some(span) = settings.span {
            match entity {
                Entity::Global => (),
                Entity::Column(_) => (),
                Entity::Row(_) => (),
                Entity::Cell(row, col) => self.set_span(span, row, col),
            }
        }
    }

    fn set_span(&mut self, mut span: usize, row: usize, mut col: usize) {
        if row >= self.count_rows() {
            return;
        }

        // It's a default span so we can do nothing.
        if span == 1 {
            return;
        }

        if col == 0 && span == 0 {
            return;
        }

        if col + span > self.count_columns() {
            span = self.count_columns() - col;
        }

        if span == 0 && col > 0 {
            match closest_visible(self, row, col - 1) {
                Some(c) => {
                    span += 1 + col - c;
                    col = c;
                }
                None => return,
            }
        }

        self.spans
            .entry((col, col + span))
            .and_modify(|rows| {
                rows.insert(row);
            })
            .or_insert_with(|| {
                let mut m = HashSet::with_capacity(1);
                m.insert(row);
                m
            });

        // it may happen that a colided span will be left so we checks if there's one
        // like we insert (0, 3) but (0, 2) was in a set.
        // such span makes no sense so we delete it.

        for span in 0..span {
            let mut do_remove = false;
            if let Some(rows) = self.spans.get_mut(&(col, col + span)) {
                rows.remove(&row);
                do_remove = rows.is_empty();
            }

            if do_remove {
                self.spans.remove(&(col, col + span));
            }
        }
    }

    pub fn margin(&mut self, margin: Margin) {
        self.margin = margin
    }

    pub fn get_margin(&self) -> &Margin {
        &self.margin
    }

    pub fn clear_theme(&mut self) {
        self.theme = Theme::new();
        self.override_split_lines.clear();
    }

    pub fn set_borders(&mut self, borders: Borders) {
        self.theme.borders = borders;
    }

    pub fn get_borders(&self) -> &Borders {
        &self.theme.borders
    }

    pub fn set_border(&mut self, entity: Entity, border: Border) {
        match entity {
            Entity::Global => {
                for col in 0..self.count_columns() {
                    for row in 0..self.count_rows() {
                        self.theme.override_border((row, col), border.clone());
                    }
                }
            }
            Entity::Column(col) => {
                for row in 0..self.count_rows() {
                    self.theme.override_border((row, col), border.clone());
                }
            }
            Entity::Row(row) => {
                for col in 0..self.count_columns() {
                    self.theme.override_border((row, col), border.clone());
                }
            }
            Entity::Cell(row, col) => {
                self.theme.override_border((row, col), border);
            }
        }
    }

    pub fn set_split_line(&mut self, row: usize, line: Line) {
        self.theme.override_line(row, line)
    }

    /// get_cell_settings returns a settings of a cell
    pub fn get_settings(&self, row: usize, col: usize) -> Settings {
        let style = self.style(Entity::Cell(row, col));
        let content = &self.cells[row][col];
        let border = self.get_border(row, col);
        let span = self
            .spans
            .iter()
            .find(|((c, _), rows)| rows.contains(&row) && *c == col)
            .map(|((start, end), _)| end - start);

        Settings {
            text: Some(content.clone()),
            padding: Some(Padding {
                left: style.padding.left,
                right: style.padding.right,
                top: style.padding.top,
                bottom: style.padding.bottom,
            }),
            border: Some(border),
            alignment_h: Some(style.alignment_h),
            alignment_v: Some(style.alignment_v),
            formatting: None,
            span,
        }
    }

    pub fn get_border(&self, row: usize, col: usize) -> Border {
        let mut border = self
            .theme
            .get_border((row, col), self.count_rows(), self.count_columns());

        // make sure that there's no user defined lines
        // in which case we use spaces.

        let mut top_set = border.top.is_some();
        let mut bottom_set = border.bottom.is_some();
        let mut left_set = border.left.is_some();
        let mut right_set = border.right.is_some();

        if border.top.is_none() && has_horizontal(self, row) {
            border.top = Some(DEFAULT_BORDER_HORIZONTAL_SYMBOL);
            top_set = true;
        }

        if border.bottom.is_none() && has_horizontal(self, row + 1) {
            border.bottom = Some(DEFAULT_BORDER_HORIZONTAL_SYMBOL);
            bottom_set = true;
        }

        if border.left.is_none() && has_vertical(self, col) {
            border.left = Some(DEFAULT_BORDER_VERTICAL_SYMBOL);
            left_set = true;
        }

        if border.right.is_none() && has_vertical(self, col + 1) {
            border.right = Some(DEFAULT_BORDER_VERTICAL_SYMBOL);
            right_set = true;
        }

        if border.left_top_corner.is_none() && top_set && left_set {
            border.left_top_corner = Some(DEFAULT_BORDER_VERTICAL_SYMBOL);
        }

        if border.left_bottom_corner.is_none() && bottom_set && left_set {
            border.left_bottom_corner = Some(DEFAULT_BORDER_VERTICAL_SYMBOL);
        }

        if border.right_top_corner.is_none() && top_set && right_set {
            border.right_top_corner = Some(DEFAULT_BORDER_VERTICAL_SYMBOL);
        }

        if border.right_bottom_corner.is_none() && bottom_set && right_set {
            border.right_bottom_corner = Some(DEFAULT_BORDER_VERTICAL_SYMBOL);
        }

        border
    }

    pub fn style(&self, entity: Entity) -> &Style {
        match entity {
            Entity::Column(column) => {
                if let Some(style) = self.styles.get(&Entity::Column(column)) {
                    return style;
                }
            }
            Entity::Row(row) => {
                if let Some(style) = self.styles.get(&Entity::Row(row)) {
                    return style;
                }
            }
            Entity::Cell(row, col) => {
                if let Some(style) = self.styles.get(&Entity::Cell(row, col)) {
                    return style;
                }

                if let Some(style) = self.styles.get(&Entity::Column(col)) {
                    return style;
                }

                if let Some(style) = self.styles.get(&Entity::Row(row)) {
                    return style;
                }
            }
            Entity::Global => (),
        }

        // unreachable!("there's a Entity::Global setting guaranteed in the map")
        self.styles.get(&Entity::Global).unwrap()
    }

    fn style_mut(&mut self, entity: Entity) -> &mut Style {
        if self.styles.contains_key(&entity) {
            return self.styles.get_mut(&entity).unwrap();
        }

        let style = self.style(entity).clone();
        self.styles.insert(entity, style);
        self.styles.get_mut(&entity).unwrap()
    }

    fn remove_inherited_styles(&mut self, entity: Entity) {
        match entity {
            Entity::Global => self.styles.retain(|k, _| matches!(k, Entity::Global)),
            Entity::Column(col) => self
                .styles
                .retain(move |k, _| !matches!(k, Entity::Cell(_, c) if *c == col)),
            Entity::Row(row) => self
                .styles
                .retain(move |k, _| !matches!(k, Entity::Cell(r, _) if *r == row)),
            Entity::Cell(_, _) => {}
        }
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

    pub fn set_text(&mut self, entity: Entity, text: String) {
        // let style = self.style(entity);
        // replace_tab(&mut text, style.formatting.tab_width);
        self._set_text(entity, text);
    }

    fn _set_text(&mut self, entity: Entity, text: String) {
        match entity {
            Entity::Cell(row, col) => {
                self.cells[row][col] = text;
            }
            Entity::Column(col) => {
                for row in 0..self.count_rows() {
                    self.cells[row][col] = text.clone();
                }
            }
            Entity::Row(row) => {
                for col in 0..self.count_columns() {
                    self.cells[row][col] = text.clone();
                }
            }
            Entity::Global => {
                for row in 0..self.count_rows() {
                    for col in 0..self.count_columns() {
                        self.cells[row][col] = text.clone();
                    }
                }
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
        new_grid.theme = self.theme.clone();

        for (new_row, row) in (start_row..end_row).enumerate() {
            for (new_column, column) in (start_column..end_column).enumerate() {
                let settings = self.get_settings(row, column);
                new_grid.set(Entity::Cell(new_row, new_column), settings);
            }
        }

        new_grid
    }

    /// Returns a total width of table, including split lines.
    pub fn total_width(&self) -> usize {
        let count_rows = self.count_rows();
        let count_columns = self.count_columns();
        if count_rows == 0 || count_columns == 0 {
            return 0;
        }

        let widths = columns_width(self);

        total_width(self, &widths, &self.margin)
    }

    pub fn override_split_line(&mut self, row: usize, line: impl Into<String>) {
        self.override_split_lines.insert(row, line.into());
    }

    // hide it by feature?
    // 'private'
    pub fn build_widths(&self) -> Vec<usize> {
        columns_width(self)
    }

    pub fn build_cells_widths(&self) -> Vec<Vec<usize>> {
        let widths = columns_width(self);

        let mut cells_widths = Vec::with_capacity(self.count_rows());
        for row in 0..self.count_rows() {
            let mut row_widths = Vec::with_capacity(self.count_columns());
            for col in 0..self.count_columns() {
                let width = if is_cell_visible(self, (row, col)) {
                    grid_cell_width(self, &widths, (row, col))
                } else {
                    0
                };

                row_widths.push(width);
            }

            cells_widths.push(row_widths);
        }

        cells_widths
    }

    pub fn collect_cells(&self) -> Vec<Vec<Vec<String>>> {
        let count_rows = self.count_rows();
        let count_columns = self.count_columns();

        let mut rows = vec![Vec::with_capacity(self.count_columns()); self.count_rows()];
        (0..count_rows).for_each(|row| {
            (0..count_columns).for_each(|col| {
                let style = self.style(Entity::Cell(row, col));
                let content = replace_tab(&self.cells[row][col], style.formatting.tab_width);

                // fixme: I guess it can be done in a different place?
                let lines: Vec<_> = content.lines().map(|l| l.to_owned()).collect();
                rows[row].push(lines);
            });
        });

        rows
    }

    pub fn is_cell_visible(&self, pos: Position) -> bool {
        let is_cell_overriden = is_cell_overriden(self, pos);
        !is_cell_overriden
    }
}

#[derive(Debug, Clone, Default, Eq, PartialEq)]
pub struct Border {
    pub top: Option<Symbol>,
    pub bottom: Option<Symbol>,
    pub left: Option<Symbol>,
    pub left_top_corner: Option<Symbol>,
    pub left_bottom_corner: Option<Symbol>,
    pub right: Option<Symbol>,
    pub right_top_corner: Option<Symbol>,
    pub right_bottom_corner: Option<Symbol>,
}

impl Border {
    /// This function constructs a cell borders with all sides set.
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        top: impl Into<Symbol>,
        bottom: impl Into<Symbol>,
        left: impl Into<Symbol>,
        right: impl Into<Symbol>,
        top_left: impl Into<Symbol>,
        top_right: impl Into<Symbol>,
        bottom_left: impl Into<Symbol>,
        bottom_right: impl Into<Symbol>,
    ) -> Self {
        Self {
            top: Some(top.into()),
            bottom: Some(bottom.into()),
            right: Some(right.into()),
            right_top_corner: Some(top_right.into()),
            right_bottom_corner: Some(bottom_right.into()),
            left: Some(left.into()),
            left_bottom_corner: Some(bottom_left.into()),
            left_top_corner: Some(top_left.into()),
        }
    }

    /// This function constructs a cell borders with all sides's char set to a given character.
    /// It behaives like [Border::new] with the same character set to each side.
    pub fn filled(c: impl Into<Symbol>) -> Self {
        let c = c.into();
        Self::new(
            c.clone(),
            c.clone(),
            c.clone(),
            c.clone(),
            c.clone(),
            c.clone(),
            c.clone(),
            c,
        )
    }

    /// Set a top border character.
    pub fn top(mut self, c: impl Into<Symbol>) -> Self {
        self.top = Some(c.into());
        self
    }

    /// Set a bottom border character.
    pub fn bottom(mut self, c: impl Into<Symbol>) -> Self {
        self.bottom = Some(c.into());
        self
    }

    /// Set a left border character.
    pub fn left(mut self, c: impl Into<Symbol>) -> Self {
        self.left = Some(c.into());
        self
    }

    /// Set a right border character.
    pub fn right(mut self, c: impl Into<Symbol>) -> Self {
        self.right = Some(c.into());
        self
    }

    /// Set a top left intersection character.
    pub fn top_left_corner(mut self, c: impl Into<Symbol>) -> Self {
        self.left_top_corner = Some(c.into());
        self
    }

    /// Set a top right intersection character.
    pub fn top_right_corner(mut self, c: impl Into<Symbol>) -> Self {
        self.right_top_corner = Some(c.into());
        self
    }

    /// Set a bottom left intersection character.
    pub fn bottom_left_corner(mut self, c: impl Into<Symbol>) -> Self {
        self.left_bottom_corner = Some(c.into());
        self
    }

    /// Set a bottom right intersection character.
    pub fn bottom_right_corner(mut self, c: impl Into<Symbol>) -> Self {
        self.right_bottom_corner = Some(c.into());
        self
    }
}

/// Entity a structure which represent a set of cells.
#[derive(PartialEq, Eq, Debug, Hash, Clone, Copy)]
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

// impl Entity {
//     // fn cells<'a>(&self, grid: &'a Grid) -> impl Iterator<Item=(usize, usize)> + 'a {

//     // }
// }

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Style {
    pub padding: Padding,
    pub alignment_h: AlignmentHorizontal,
    pub alignment_v: AlignmentVertical,
    pub formatting: Formatting,
}

impl Default for Style {
    fn default() -> Self {
        Self {
            padding: Padding::default(),
            alignment_h: AlignmentHorizontal::Left,
            alignment_v: AlignmentVertical::Top,
            formatting: Formatting {
                horizontal_trim: false,
                vertical_trim: false,
                allow_lines_alignement: false,
                tab_width: 4,
            },
        }
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct Formatting {
    pub horizontal_trim: bool,
    pub vertical_trim: bool,
    pub allow_lines_alignement: bool,
    pub tab_width: usize,
}

#[derive(Default, Debug, Clone, Copy)]
pub struct Margin {
    pub top: Indent,
    pub bottom: Indent,
    pub left: Indent,
    pub right: Indent,
}

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
pub struct Padding {
    pub top: Indent,
    pub bottom: Indent,
    pub left: Indent,
    pub right: Indent,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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

/// AlignmentHorizontal represents an horizontal alignment of a cell content.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AlignmentHorizontal {
    Center,
    Left,
    Right,
}

impl AlignmentHorizontal {
    fn align_with_max_width(
        &self,
        f: &mut fmt::Formatter<'_>,
        text: &str,
        width: usize,
        text_width: usize,
        max_text_width: usize,
    ) -> fmt::Result {
        let diff = width - text_width;

        match self {
            AlignmentHorizontal::Left => Self::align(f, text, 0, diff),
            AlignmentHorizontal::Right => {
                let max_diff = width - max_text_width;
                let rest = diff - max_diff;
                Self::align(f, text, max_diff, rest)
            }
            AlignmentHorizontal::Center => {
                let max_diff = width - max_text_width;
                let left = max_diff / 2;
                let rest = diff - left;
                Self::align(f, text, left, rest)
            }
        }
    }

    fn align(f: &mut fmt::Formatter<'_>, text: &str, left: usize, right: usize) -> fmt::Result {
        repeat_char(f, ' ', left)?;
        f.write_str(text)?;
        repeat_char(f, ' ', right)?;
        Ok(())
    }
}

/// AlignmentVertical represents an vertical alignment of a cell content.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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
    border: Option<Border>,
    span: Option<usize>,
    alignment_h: Option<AlignmentHorizontal>,
    alignment_v: Option<AlignmentVertical>,
    formatting: Option<Formatting>,
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

    /// Set a formatting settings.
    ///
    /// It overades them even if any were not set.
    pub fn formatting(mut self, formatting: Formatting) -> Self {
        self.formatting = Some(formatting);
        self
    }
}

impl fmt::Display for Grid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let count_rows = self.count_rows();
        let count_columns = self.count_columns();

        if count_rows == 0 || count_columns == 0 {
            return Ok(());
        }

        let heights = rows_height(self);
        let widths = columns_width(self);

        print_grid(f, self, widths, heights)
    }
}

fn build_line_cell(
    f: &mut fmt::Formatter<'_>,
    line_index: usize,
    cell: &str,
    style: &Style,
    width: usize,
    height: usize,
) -> fmt::Result {
    let cell_height = count_lines(cell);
    if style.formatting.vertical_trim {
        let cell = skip_empty_lines(cell, cell_height);
        let cell_height = cell.clone().count();
        build_format_line(f, line_index, cell, style, width, height, cell_height)
    } else {
        build_format_line(
            f,
            line_index,
            cell.lines(),
            style,
            width,
            height,
            cell_height,
        )
    }
}

fn build_format_line<'a>(
    f: &mut fmt::Formatter<'_>,
    line_index: usize,
    mut cell: impl Iterator<Item = &'a str>,
    style: &Style,
    width: usize,
    height: usize,
    cell_height: usize,
) -> Result<(), fmt::Error> {
    let top_indent = top_indent(cell_height, style, height);
    if top_indent > line_index {
        return repeat_char(f, style.padding.top.fill, width);
    }

    let cell_line_index = line_index - top_indent;
    let cell_has_this_line = cell_height > cell_line_index;
    // happens when other cells have bigger height
    if !cell_has_this_line {
        return repeat_char(f, style.padding.bottom.fill, width);
    }

    if style.formatting.allow_lines_alignement {
        let mut line = cell.nth(cell_line_index).unwrap();
        if style.formatting.horizontal_trim && style.formatting.allow_lines_alignement {
            line = line.trim();
        } else if style.formatting.horizontal_trim {
            line = line.trim_end();
        };

        let line_width = string_width(line);

        line_with_width(f, line, width, line_width, line_width, style)
    } else {
        let (max_line_width, (text, line_width)) =
            cell.enumerate().fold((0, ("", 0)), |mut acc, (i, line)| {
                if i == cell_line_index {
                    let line = if style.formatting.horizontal_trim
                        && style.formatting.allow_lines_alignement
                    {
                        line.trim()
                    } else if style.formatting.horizontal_trim {
                        line.trim_end()
                    } else {
                        line
                    };

                    acc.1 = (line, string_width(line));
                }

                let line = if style.formatting.horizontal_trim {
                    line.trim_end()
                } else {
                    line
                };

                let len = string_width(line);

                if acc.0 < len {
                    acc.0 = len;
                }

                acc
            });

        line_with_width(f, text, width, line_width, max_line_width, style)
    }
}

fn skip_empty_lines(s: &str, length: usize) -> impl Iterator<Item = &'_ str> + Clone {
    let is_empty = |s: &&str| s.trim().is_empty();
    let end_lines = s.lines().rev().take_while(is_empty).count();
    let n = length - end_lines;
    s.lines().take(n).skip_while(is_empty)
}

fn top_indent(cell_height: usize, style: &Style, height: usize) -> usize {
    let height = height - style.padding.top.size;
    let indent = style.alignment_v.top_ident(height, cell_height);

    indent + style.padding.top.size
}

fn repeat_symbol(f: &mut fmt::Formatter<'_>, c: &Symbol, n: usize) -> fmt::Result {
    if n > 0 {
        for _ in 0..n {
            c.fmt(f)?;
        }
    }
    Ok(())
}

fn repeat_char(f: &mut fmt::Formatter<'_>, c: char, n: usize) -> fmt::Result {
    if n > 0 {
        for _ in 0..n {
            f.write_char(c)?;
        }
    }
    Ok(())
}

fn line_with_width(
    f: &mut fmt::Formatter<'_>,
    text: &str,
    width: usize,
    width_text: usize,
    width_text_max: usize,
    style: &Style,
) -> fmt::Result {
    let left_indent = style.padding.left;
    let right_indent = style.padding.right;
    let alignment = style.alignment_h;

    repeat_char(f, left_indent.fill, left_indent.size)?;
    let width = width - left_indent.size - right_indent.size;
    alignment.align_with_max_width(f, text, width, width_text, width_text_max)?;
    repeat_char(f, right_indent.fill, right_indent.size)?;

    Ok(())
}

pub fn strip(s: &str, width: usize) -> String {
    #[cfg(not(feature = "color"))]
    {
        s.chars().take(width).collect::<String>()
    }
    #[cfg(feature = "color")]
    {
        let width = to_byte_length(s, width);
        ansi_str::AnsiStr::ansi_cut(s, ..width)
    }
}

#[cfg(feature = "color")]
fn to_byte_length(s: &str, width: usize) -> usize {
    s.chars().take(width).map(|c| c.len_utf8()).sum::<usize>()
}

#[cfg(not(feature = "color"))]
pub fn string_width(text: &str) -> usize {
    unicode_width::UnicodeWidthStr::width(text)
}

#[cfg(feature = "color")]
pub fn string_width(text: &str) -> usize {
    let b = strip_ansi_escapes::strip(text.as_bytes()).unwrap();
    let s = std::str::from_utf8(&b).unwrap();
    unicode_width::UnicodeWidthStr::width(s)
}

#[cfg(not(feature = "color"))]
pub fn string_width_multiline(text: &str) -> usize {
    text.lines()
        .map(unicode_width::UnicodeWidthStr::width)
        .max()
        .unwrap_or(0)
}

#[cfg(feature = "color")]
pub fn string_width_multiline(text: &str) -> usize {
    let b = strip_ansi_escapes::strip(text.as_bytes()).unwrap();
    let s = std::str::from_utf8(&b).unwrap();

    s.lines()
        .map(unicode_width::UnicodeWidthStr::width)
        .max()
        .unwrap_or(0)
}

fn string_width_multiline_tab(text: &str, tab_width: usize) -> usize {
    #[cfg(feature = "color")]
    {
        let b = strip_ansi_escapes::strip(text.as_bytes()).unwrap();
        let s = std::str::from_utf8(&b).unwrap();

        s.lines()
            .map(|line| width_with_tab(line, tab_width))
            .max()
            .unwrap_or(0)
    }
    #[cfg(not(feature = "color"))]
    {
        text.lines()
            .map(|line| width_with_tab(line, tab_width))
            .max()
            .unwrap_or(0)
    }
}

fn width_with_tab(text: &str, tab_width: usize) -> usize {
    let width = unicode_width::UnicodeWidthStr::width(text);
    let count_tabs = count_tabs(text);

    width + count_tabs * tab_width
}

fn columns_width(grid: &Grid) -> Vec<usize> {
    let mut widths = Vec::with_capacity(grid.count_columns());
    for col in 0..grid.count_columns() {
        let mut max = 0;

        for row in 0..grid.count_rows() {
            if !is_simple_cell(grid, (row, col)) {
                continue;
            }

            let width = get_cell_width(grid, (row, col));
            max = cmp::max(width, max);
        }

        widths.push(max);
    }

    adjust_spans(grid, &mut widths);

    widths
}

fn adjust_spans(grid: &Grid, widths: &mut [usize]) {
    if grid.spans.is_empty() {
        return;
    }

    for (&(start, end), rows) in &grid.spans {
        adjust_range(grid, rows.iter().cloned(), widths, start, end);
    }
}

fn adjust_range(
    grid: &Grid,
    rows: impl ExactSizeIterator<Item = usize>,
    widths: &mut [usize],
    start: usize,
    end: usize,
) {
    if rows.len() == 0 {
        return;
    }

    let max_span_width = rows
        .map(|row| get_cell_width(grid, (row, start)))
        .max()
        .unwrap_or(0);
    let range_width = range_width(grid, start, end, widths);

    if range_width >= max_span_width {
        return;
    }

    inc_range_width(widths, max_span_width - range_width, start, end);
}

fn get_cell_width(grid: &Grid, (row, col): Position) -> usize {
    let style = grid.style(Entity::Cell(row, col));
    let text = &grid.cells[row][col];
    let width = string_width_multiline_tab(text, style.formatting.tab_width);
    width + style.padding.left.size + style.padding.right.size
}

fn range_width(grid: &Grid, start: usize, end: usize, widths: &[usize]) -> usize {
    let count_borders = count_borders_in_range(grid, start, end);
    let range_width = widths[start..end].iter().sum::<usize>();
    count_borders + range_width
}

fn is_cell_visible(grid: &Grid, pos: Position) -> bool {
    let is_cell_overriden = is_cell_overriden(grid, pos);
    !is_cell_overriden
}

fn is_cell_overriden(grid: &Grid, pos: Position) -> bool {
    grid.spans
        .iter()
        .any(|(&(start, end), rows)| pos.1 > start && pos.1 < end && rows.contains(&pos.0))
}

fn is_simple_cell(grid: &Grid, pos: Position) -> bool {
    let is_spanned = grid
        .spans
        .iter()
        .any(|(&(start, end), rows)| pos.1 >= start && pos.1 < end && rows.contains(&pos.0));

    !is_spanned
}

fn count_borders_in_range(grid: &Grid, start: usize, end: usize) -> usize {
    (start..end)
        .skip(1)
        .filter(|&i| has_vertical(grid, i))
        .count()
}

fn inc_range_width(widths: &mut [usize], size: usize, start: usize, end: usize) {
    if widths.is_empty() {
        return;
    }

    let span = end - start;
    let one = size / span;
    let rest = size - span * one;

    let mut i = start;
    while i < end {
        if i == start {
            widths[i] += one + rest;
        } else {
            widths[i] += one;
        }

        i += 1;
    }
}

fn closest_visible(grid: &Grid, row: usize, mut col: usize) -> Option<usize> {
    loop {
        if is_cell_visible(grid, (row, col)) {
            return Some(col);
        }

        if col == 0 {
            return None;
        }

        col -= 1;
    }
}

fn rows_height(grid: &Grid) -> impl Iterator<Item = usize> + '_ {
    (0..grid.count_rows()).map(move |row| {
        let mut max_height = 0;
        (0..grid.count_columns()).for_each(|col| {
            let cell = &grid.cells[row][col];
            let style = grid.style(Entity::Cell(row, col));
            let cell_height = cell_height(cell, style);
            max_height = max(max_height, cell_height);
        });

        max_height
    })
}

fn cell_height(cell: &str, style: &Style) -> usize {
    let is_there_padding = style.padding.left.size > 0 || style.padding.right.size > 0;
    let mut content_height = count_lines(cell);
    if content_height == 0 && is_there_padding {
        content_height = 1;
    }

    content_height + style.padding.top.size + style.padding.bottom.size
}

fn replace_tab(text: &str, n: usize) -> String {
    // it's a general case which probably must be faster?
    if n == 4 {
        text.replace('\t', "    ")
    } else {
        let mut text = text.to_owned();
        replace_tab_range(&mut text, n);
        text
    }
}

fn replace_tab_range(cell: &mut String, n: usize) -> &str {
    let mut skip = 0;
    while let &Some(pos) = &cell[skip..].find('\t') {
        let pos = skip + pos;

        let is_escaped = pos > 0 && cell.get(pos - 1..pos) == Some("\\");
        if is_escaped {
            skip = pos + 1;
        } else if n == 0 {
            cell.remove(pos);
            skip = pos;
        } else {
            // I'am not sure which version is faster a loop of 'replace'
            // or allacation of a string for replacement;
            cell.replace_range(pos..pos + 1, &" ".repeat(n));
            skip = pos + 1;
        }

        if cell.is_empty() || skip >= cell.len() {
            break;
        }
    }
    cell
}

// only valid to call for stabilized widths.
fn total_width(grid: &Grid, widths: &[usize], margin: &Margin) -> usize {
    if grid.count_rows() == 0 {
        return 0;
    }

    let content_width = (0..grid.count_columns())
        .filter(|&col| is_cell_visible(grid, (0, col)))
        .map(|col| grid_cell_width(grid, widths, (0, col)))
        .sum::<usize>();

    let count_borders = if grid.count_columns() == 0 {
        0
    } else {
        let rest_borders = (0..grid.count_columns())
            .filter(|&col| is_cell_visible(grid, (0, col)))
            .map(|col| has_vertical(grid, col))
            .filter(|b| *b)
            .count();
        let last_col_border = has_vertical(grid, grid.count_columns()) as usize;

        last_col_border + rest_borders
    };

    content_width + count_borders + margin.left.size + margin.right.size
}

#[derive(Debug, Clone)]
struct Theme {
    borders: Borders,
    override_borders: BordersMap,
    override_lines: HashMap<usize, Line>,
}

#[derive(Debug, Clone, Default)]
pub struct Borders {
    pub top: Option<Symbol>,
    pub top_left: Option<Symbol>,
    pub top_right: Option<Symbol>,
    pub top_intersection: Option<Symbol>,

    pub bottom: Option<Symbol>,
    pub bottom_left: Option<Symbol>,
    pub bottom_right: Option<Symbol>,
    pub bottom_intersection: Option<Symbol>,

    pub horizontal: Option<Symbol>,
    pub horizontal_left: Option<Symbol>,
    pub horizontal_right: Option<Symbol>,

    pub vertical_left: Option<Symbol>,
    pub vertical_intersection: Option<Symbol>,
    pub vertical_right: Option<Symbol>,

    pub intersection: Option<Symbol>,
}

#[derive(Debug, Clone)]
struct BordersMap {
    vertical: HashMap<Position, Symbol>,
    horizontal: HashMap<Position, Symbol>,
    intersection: HashMap<Position, Symbol>,
}

#[derive(Debug, Clone, Default)]
pub struct Line {
    pub horizontal: Option<Symbol>,
    pub intersection: Option<Symbol>,
    pub left: Option<Symbol>,
    pub right: Option<Symbol>,
}

pub type Position = (usize, usize);

impl Theme {
    fn new() -> Self {
        Self {
            borders: DEFAULT_BORDERS,
            override_borders: BordersMap {
                vertical: HashMap::new(),
                horizontal: HashMap::new(),
                intersection: HashMap::new(),
            },
            override_lines: HashMap::new(),
        }
    }

    fn override_border(&mut self, pos: Position, border: Border) {
        if let Some(c) = border.top {
            self.override_borders.horizontal.insert(pos, c);
        }

        if let Some(c) = border.bottom {
            self.override_borders
                .horizontal
                .insert((pos.0 + 1, pos.1), c);
        }

        if let Some(c) = border.left {
            self.override_borders.vertical.insert(pos, c);
        }

        if let Some(c) = border.right {
            self.override_borders.vertical.insert((pos.0, pos.1 + 1), c);
        }

        if let Some(c) = border.left_top_corner {
            self.override_borders.intersection.insert((pos.0, pos.1), c);
        }

        if let Some(c) = border.left_bottom_corner {
            self.override_borders
                .intersection
                .insert((pos.0 + 1, pos.1), c);
        }

        if let Some(c) = border.right_top_corner {
            self.override_borders
                .intersection
                .insert((pos.0, pos.1 + 1), c);
        }

        if let Some(c) = border.right_bottom_corner {
            self.override_borders
                .intersection
                .insert((pos.0 + 1, pos.1 + 1), c);
        }
    }

    fn override_line(&mut self, row: usize, line: Line) {
        self.override_lines.insert(row, line);
    }

    // we can take only a border of a cell
    // which is a pitty,
    // would be cool if we could take a border of any Entity
    fn get_border(&self, pos: Position, count_rows: usize, count_cols: usize) -> Border {
        let is_first_row = pos.0 == 0;
        let is_last_row = pos.0 + 1 == count_rows;
        let is_first_col = pos.1 == 0;
        let is_last_col = pos.1 + 1 == count_cols;

        let top = if is_first_row {
            &self.borders.top
        } else {
            &self.borders.horizontal
        };

        let bottom = if is_last_row {
            &self.borders.bottom
        } else {
            &self.borders.horizontal
        };

        let left = if is_first_col {
            &self.borders.vertical_left
        } else {
            &self.borders.vertical_intersection
        };

        let left_top_corner = if is_first_row && is_first_col {
            &self.borders.top_left
        } else if is_first_col {
            &self.borders.horizontal_left
        } else {
            &self.borders.intersection
        };

        let left_bottom_corner = if is_last_row && is_first_col {
            &self.borders.bottom_left
        } else if is_first_col {
            &self.borders.horizontal_left
        } else {
            &self.borders.intersection
        };

        let right = if is_last_col {
            &self.borders.vertical_right
        } else {
            &self.borders.vertical_intersection
        };

        let right_top_corner = if is_first_row && is_last_col {
            &self.borders.top_right
        } else if is_last_col {
            &self.borders.horizontal_right
        } else {
            &self.borders.intersection
        };

        let right_bottom_corner = if is_last_row && is_last_col {
            &self.borders.bottom_right
        } else if is_last_col {
            &self.borders.horizontal_right
        } else {
            &self.borders.intersection
        };

        let mut border = Border {
            top: top.clone(),
            bottom: bottom.clone(),
            left: left.clone(),
            left_top_corner: left_top_corner.clone(),
            left_bottom_corner: left_bottom_corner.clone(),
            right: right.clone(),
            right_top_corner: right_top_corner.clone(),
            right_bottom_corner: right_bottom_corner.clone(),
        };

        if let Some(line) = self.override_lines.get(&pos.0) {
            border.top = line.horizontal.clone().or(border.top);

            if is_first_col {
                border.left_top_corner = line.left.clone().or(border.left_top_corner);
            } else {
                border.left_top_corner = line.intersection.clone().or(border.left_top_corner);
            }

            if is_last_col {
                border.right_top_corner = line.right.clone().or(border.right_top_corner);
            } else {
                border.right_top_corner = line.intersection.clone().or(border.right_top_corner);
            }
        }

        if let Some(line) = self.override_lines.get(&(pos.0 + 1)) {
            border.bottom = line.horizontal.clone().or(border.bottom);

            if is_first_col {
                border.left_bottom_corner = line.left.clone().or(border.left_bottom_corner);
            } else {
                border.left_bottom_corner = line.intersection.clone().or(border.left_bottom_corner);
            }

            if is_last_col {
                border.right_bottom_corner = line.right.clone().or(border.right_bottom_corner);
            } else {
                border.right_bottom_corner =
                    line.intersection.clone().or(border.right_bottom_corner);
            }
        }

        if let Some(b) = self.get_override_border(&pos) {
            border.top = b.top.or(border.top);
            border.bottom = b.bottom.or(border.bottom);
            border.left = b.left.or(border.left);
            border.left_top_corner = b.left_top_corner.or(border.left_top_corner);
            border.left_bottom_corner = b.left_bottom_corner.or(border.left_bottom_corner);
            border.right = b.right.or(border.right);
            border.right_top_corner = b.right_top_corner.or(border.right_top_corner);
            border.right_bottom_corner = b.right_bottom_corner.or(border.right_bottom_corner);
        }

        border
    }

    fn get_vertical(&self, pos: Position, count_cols: usize) -> Option<&Symbol> {
        let use_left = pos.1 == 0;
        let use_right = pos.1 == count_cols;

        if let Some(b) = self.override_borders.vertical.get(&pos) {
            return Some(b);
        }

        if use_right {
            self.borders.vertical_right.as_ref()
        } else if use_left {
            self.borders.vertical_left.as_ref()
        } else {
            self.borders.vertical_intersection.as_ref()
        }
    }

    fn get_horizontal(&self, pos: Position, count_rows: usize) -> Option<&Symbol> {
        let use_top = pos.0 == 0;
        let use_bottom = pos.0 == count_rows;

        if let Some(b) = self.override_borders.horizontal.get(&pos) {
            return Some(b);
        }

        if let Some(line) = self.override_lines.get(&pos.0) {
            if line.horizontal.is_some() {
                return line.horizontal.as_ref();
            }
        }

        if use_top {
            self.borders.top.as_ref()
        } else if use_bottom {
            self.borders.bottom.as_ref()
        } else {
            self.borders.horizontal.as_ref()
        }
    }

    fn get_intersection(
        &self,
        pos: Position,
        count_rows: usize,
        count_cols: usize,
    ) -> Option<&Symbol> {
        let use_top = pos.0 == 0;
        let use_bottom = pos.0 == count_rows;

        let use_left = pos.1 == 0;
        let use_right = pos.1 == count_cols;

        if let Some(b) = self.override_borders.intersection.get(&pos) {
            return Some(b);
        }

        if let Some(line) = self.override_lines.get(&pos.0) {
            if use_left && line.left.is_some() {
                return line.left.as_ref();
            }

            if use_right && line.right.is_some() {
                return line.right.as_ref();
            }

            if !use_right && !use_left && line.intersection.is_some() {
                return line.intersection.as_ref();
            }
        }

        if use_top && use_left {
            self.borders.top_left.as_ref()
        } else if use_top && use_right {
            self.borders.top_right.as_ref()
        } else if use_bottom && use_left {
            self.borders.bottom_left.as_ref()
        } else if use_bottom && use_right {
            self.borders.bottom_right.as_ref()
        } else if use_top {
            self.borders.top_intersection.as_ref()
        } else if use_bottom {
            self.borders.bottom_intersection.as_ref()
        } else if use_left {
            self.borders.horizontal_left.as_ref()
        } else if use_right {
            self.borders.horizontal_right.as_ref()
        } else {
            self.borders.intersection.as_ref()
        }
    }

    fn get_override_border(&self, pos: &Position) -> Option<Border> {
        let top = self.override_borders.horizontal.get(pos);
        let bottom = self.override_borders.horizontal.get(&(pos.0 + 1, pos.1));
        let left = self.override_borders.vertical.get(pos);
        let right = self.override_borders.vertical.get(&(pos.0, pos.1 + 1));
        let left_top = self.override_borders.intersection.get(pos);
        let left_bottom = self.override_borders.intersection.get(&(pos.0 + 1, pos.1));
        let right_top = self.override_borders.intersection.get(&(pos.0, pos.1 + 1));
        let right_bottom = self
            .override_borders
            .intersection
            .get(&(pos.0 + 1, pos.1 + 1));

        if top.is_some()
            || bottom.is_some()
            || left.is_some()
            || right.is_some()
            || left_top.is_some()
            || left_bottom.is_some()
            || right_top.is_some()
            || right_bottom.is_some()
        {
            return Some(Border {
                top: top.cloned(),
                bottom: bottom.cloned(),
                left: left.cloned(),
                left_top_corner: left_top.cloned(),
                left_bottom_corner: left_bottom.cloned(),
                right: right.cloned(),
                right_top_corner: right_top.cloned(),
                right_bottom_corner: right_bottom.cloned(),
            });
        }

        None
    }
}

/// A single character representation.
///
/// It uses String to support ANSI colors.
#[cfg(feature = "color")]
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Symbol(InnerSymbol);

#[cfg(feature = "color")]
#[derive(Debug, Clone, Eq, PartialEq)]
enum InnerSymbol {
    Ansi(String),
    Char(char),
}

/// A single character representation.
///
/// It uses String to support ANSI colors.
#[cfg(not(feature = "color"))]
#[derive(Debug, Clone, Default, Eq, PartialEq)]
pub struct Symbol(char);

impl Symbol {
    #[cfg(feature = "color")]
    pub fn ansi(s: String) -> Option<Self> {
        let mut chars = s.chars();
        let c = chars.next();
        let no_other_chars = chars.next().is_none();
        drop(chars);
        match c {
            Some(c) if no_other_chars => return Some(Self(InnerSymbol::Char(c))),
            _ => (),
        }

        if string_width(&s) != 1 {
            return None;
        }

        Some(Self(InnerSymbol::Ansi(s)))
    }

    pub const fn from_char(c: char) -> Self {
        #[cfg(feature = "color")]
        {
            Self(InnerSymbol::Char(c))
        }

        #[cfg(not(feature = "color"))]
        {
            Self(c)
        }
    }
}

#[cfg(feature = "color")]
impl Default for Symbol {
    fn default() -> Self {
        Self(InnerSymbol::Char(char::default()))
    }
}

impl From<char> for Symbol {
    fn from(c: char) -> Self {
        Self::from_char(c)
    }
}

impl fmt::Display for Symbol {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        #[cfg(feature = "color")]
        {
            match &self.0 {
                InnerSymbol::Ansi(s) => f.write_str(s),
                InnerSymbol::Char(c) => f.write_char(*c),
            }
        }

        #[cfg(not(feature = "color"))]
        {
            f.write_char(self.0)
        }
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
        (Bound::Excluded(_), _) => {
            unreachable!("A start bound can't be excluded")
        }
    }
}

fn print_grid(
    f: &mut fmt::Formatter,
    grid: &Grid,
    widths: Vec<usize>,
    mut heights: impl Iterator<Item = usize>,
) -> fmt::Result {
    let table_width = row_width_grid(grid, &widths);
    print_margin_top(f, &grid.margin, table_width)?;

    for row in 0..grid.count_rows() {
        print_split_line(f, grid, &widths, table_width, row)?;

        let height = heights.next().unwrap();

        for i in 0..height {
            print_margin_left(f, &grid.margin)?;

            for col in 0..grid.count_columns() {
                let border = grid.get_border(row, col);

                if is_cell_visible(grid, (row, col)) {
                    if let Some(c) = border.left {
                        c.fmt(f)?;
                    }

                    let style = grid.style(Entity::Cell(row, col));
                    let width = grid_cell_width(grid, &widths, (row, col));
                    let text = replace_tab(&grid.cells[row][col], style.formatting.tab_width);

                    build_line_cell(f, i, &text, style, width, height)?;
                }

                let is_last_column = col + 1 == grid.count_columns();
                if is_last_column {
                    if let Some(c) = border.right {
                        c.fmt(f)?;
                    }
                }
            }

            print_margin_right(f, &grid.margin)?;

            f.write_char('\n')?;
        }

        let is_last_row = row + 1 == grid.count_rows();
        if is_last_row {
            print_split_line(f, grid, &widths, table_width, row + 1)?;
        }
    }

    print_margin_bottom(f, &grid.margin, table_width)?;

    Ok(())
}

fn grid_cell_width(grid: &Grid, widths: &[usize], pos: Position) -> usize {
    let span = grid
        .spans
        .iter()
        .find(|((col, _), rows)| *col == pos.1 && rows.contains(&pos.0))
        .map(|(span, _)| span);
    match span {
        Some(&(start, end)) => range_width(grid, start, end, widths),
        None => widths[pos.1],
    }
}

fn print_margin_top(f: &mut fmt::Formatter, margin: &Margin, table_width: usize) -> fmt::Result {
    let size = table_width + margin.left.size + margin.right.size;
    for _ in 0..margin.top.size {
        repeat_char(f, margin.top.fill, size)?;
        f.write_char('\n')?
    }

    Ok(())
}

fn print_margin_bottom(f: &mut fmt::Formatter, margin: &Margin, table_width: usize) -> fmt::Result {
    let size = table_width + margin.left.size + margin.right.size;
    for _ in 0..margin.bottom.size {
        repeat_char(f, margin.bottom.fill, size)?;
        f.write_char('\n')?
    }

    Ok(())
}

fn print_margin_left(f: &mut fmt::Formatter, margin: &Margin) -> fmt::Result {
    repeat_char(f, margin.left.fill, margin.left.size)
}

fn print_margin_right(f: &mut fmt::Formatter, margin: &Margin) -> fmt::Result {
    repeat_char(f, margin.right.fill, margin.right.size)
}

fn print_split_line(
    f: &mut fmt::Formatter,
    grid: &Grid,
    widths: &[usize],
    max_width: usize,
    row: usize,
) -> fmt::Result {
    if !has_horizontal(grid, row) {
        return Ok(());
    }

    print_margin_left(f, &grid.margin)?;

    let mut char_skip = 0;
    let override_text = grid.override_split_lines.get(&row);
    if let Some(text) = override_text {
        let text = strip(text, max_width);
        let line = text.lines().next().unwrap();
        char_skip = string_width(line);
        f.write_str(line)?;
    }

    for (col, width) in widths.iter().enumerate() {
        if col == 0 {
            let left = grid
                .theme
                .get_intersection((row, col), grid.count_rows(), grid.count_columns())
                .or_else(|| {
                    if has_vertical(grid, col) {
                        Some(DEFAULT_BORDER_VERTICAL_SYMBOL_REF)
                    } else {
                        None
                    }
                });

            if let Some(c) = left {
                if char_skip == 0 {
                    c.fmt(f)?;
                } else {
                    char_skip -= 1;
                }
            }
        }

        let mut width = *width;
        if char_skip > 0 {
            let sub = cmp::min(width, char_skip);
            width -= sub;
            char_skip -= sub;
        }

        let main = grid.theme.get_horizontal((row, col), grid.count_rows());
        match main {
            Some(c) => repeat_symbol(f, c, width)?,
            None => repeat_char(f, DEFAULT_BORDER_HORIZONTAL_CHAR, width)?,
        }

        let right = grid
            .theme
            .get_intersection((row, col + 1), grid.count_rows(), grid.count_columns())
            .or_else(|| {
                if has_vertical(grid, col + 1) {
                    Some(DEFAULT_BORDER_VERTICAL_SYMBOL_REF)
                } else {
                    None
                }
            });

        if let Some(c) = right {
            if char_skip == 0 {
                c.fmt(f)?;
            } else {
                char_skip -= 1;
            }
        }
    }

    print_margin_right(f, &grid.margin)?;

    f.write_char('\n')?;

    Ok(())
}

fn row_width_grid(grid: &Grid, widths: &[usize]) -> usize {
    let row_width = widths.iter().sum::<usize>();
    let count_borders = (0..grid.count_columns())
        .filter(|&col| has_vertical(grid, col))
        .count()
        + has_vertical(grid, grid.count_columns()) as usize;

    row_width + count_borders
}

pub fn has_vertical(grid: &Grid, col: usize) -> bool {
    (0..grid.count_rows())
        .map(|row| grid.theme.get_vertical((row, col), grid.count_columns()))
        .any(|c| c.is_some())
}

fn has_horizontal(grid: &Grid, row: usize) -> bool {
    (0..grid.count_columns())
        .map(|col| grid.theme.get_horizontal((row, col), grid.count_rows()))
        .any(|c| c.is_some())
}

fn count_tabs(s: &str) -> usize {
    bytecount::count(s.as_bytes(), b'\t')
}

fn count_lines(s: &str) -> usize {
    let mut count = bytecount::count(s.as_bytes(), b'\n');

    // we need to identify if the last char is '\n' for this matter we need to strip the string
    #[cfg(feature = "color")]
    {
        let ends_with_new_line = ansi_str::AnsiStr::ansi_ends_with(s, "\n");
        if !ends_with_new_line {
            count += 1;
        }
    }
    #[cfg(not(feature = "color"))]
    {
        let ends_with_new_line = s.ends_with('\n');
        if !ends_with_new_line {
            count += 1;
        }
    }

    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn replace_tab_test() {
        assert_eq!(replace_tab("123\t\tabc\t", 3), "123      abc   ");

        assert_eq!(replace_tab("\t", 0), "");
        assert_eq!(replace_tab("\t", 3), "   ");
        assert_eq!(replace_tab("123\tabc", 3), "123   abc");
        assert_eq!(replace_tab("123\tabc\tzxc", 0), "123abczxc");

        assert_eq!(replace_tab("\\t", 0), "\\t");
        assert_eq!(replace_tab("\\t", 4), "\\t");
        assert_eq!(replace_tab("123\\tabc", 0), "123\\tabc");
        assert_eq!(replace_tab("123\\tabc", 4), "123\\tabc");
    }

    #[test]
    fn string_width_emojie_test() {
        // ...emojis such as “joy”, which normally take up two columns when printed in a terminal
        // https://github.com/mgeisler/textwrap/pull/276
        assert_eq!(string_width("🎩"), 2);
        assert_eq!(string_width("Rust 💕"), 7);
        assert_eq!(string_width_multiline("Go 👍\nC 😎"), 5);
    }

    #[test]
    fn horizontal_aligment_test() {
        use std::fmt;

        struct F<'a>(&'a str, AlignmentHorizontal, usize);

        impl fmt::Display for F<'_> {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                let w = string_width(self.0);
                self.1.align_with_max_width(f, self.0, self.2, w, w)
            }
        }

        assert_eq!(F("AAA", AlignmentHorizontal::Right, 4).to_string(), " AAA");
        assert_eq!(F("AAA", AlignmentHorizontal::Left, 4).to_string(), "AAA ");
        assert_eq!(F("AAA", AlignmentHorizontal::Center, 4).to_string(), "AAA ");
        assert_eq!(F("🎩", AlignmentHorizontal::Center, 4).to_string(), " 🎩 ");
        assert_eq!(F("🎩", AlignmentHorizontal::Center, 3).to_string(), "🎩 ");

        #[cfg(feature = "color")]
        {
            use owo_colors::OwoColorize;
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
        use owo_colors::OwoColorize;
        assert_eq!(string_width(&"hello world".red().to_string()), 11);
        assert_eq!(string_width(&"hello\nworld".blue().to_string()), 5);
        assert_eq!(string_width("\u{1b}[34m0\u{1b}[0m"), 1);
        assert_eq!(string_width(&"0".red().to_string()), 1);
    }
}
