//! This module contains a list of primitives which can be applied to change [Table] style.
//!
//! [Table]: crate::Table

use std::{borrow::Cow, marker::PhantomData};

use crate::{object::Cell, CellOption, Highlight, TableOption};
use papergrid::{Entity, Grid, Settings};

/// Style is represents a theme of a [Table].
///
/// It can be Mofified extensively, look at [CustomStyle] methods.
///
/// # Example
///
/// ```rust,no_run
/// use tabled::{Table, Style};
///
/// let style = Style::ascii()
///                 .bottom('*')
///                 .inner_intersection(' ');
///
/// let data = vec!["Hello", "2021"];
/// let table = Table::new(&data).with(style).to_string();
///
/// println!("{}", table);
/// ```
///
/// [Table]: crate::Table
pub struct Style;

impl Style {
    /// Empty style is a style with no styling options on,
    ///
    /// ```text
    ///      id  destribution            link
    ///      0      Fedora      https://getfedora.org/
    ///      2     OpenSUSE    https://www.opensuse.org/
    ///      3   Endeavouros   https://endeavouros.com/
    /// ```
    ///
    /// Note: The cells in the example have 1-left and 1-right indent.
    ///
    /// It's usefull as a scratch style to build a custom one.
    ///
    /// ```rust,no_run
    /// # use tabled::Style;
    /// let style = Style::empty()
    ///     .top('*')
    ///     .bottom('*')
    ///     .header('x')
    ///     .vertical('#')
    ///     .bottom_intersection('^')
    ///     .top_intersection('*');
    /// ```
    pub const fn empty() -> CustomStyle<(), (), (), (), (), (), ()> {
        CustomStyle::new(Self::EMPTY)
    }

    /// Blank style looks like the following table
    ///
    /// ```text
    ///      id   destribution             link
    ///      0       Fedora       https://getfedora.org/
    ///      2      OpenSUSE     https://www.opensuse.org/
    ///      3    Endeavouros    https://endeavouros.com/
    /// ```
    pub const fn blank() -> CustomStyle<(), (), (), (), (), On, ()> {
        CustomStyle::new(Self::BLANK)
    }

    /// Default style looks like the following table
    ///
    /// ```text
    ///     +----+--------------+---------------------------+
    ///     | id | destribution |           link            |
    ///     +----+--------------+---------------------------+
    ///     | 0  |    Fedora    |  https://getfedora.org/   |
    ///     +----+--------------+---------------------------+
    ///     | 2  |   OpenSUSE   | https://www.opensuse.org/ |
    ///     +----+--------------+---------------------------+
    ///     | 3  | Endeavouros  | https://endeavouros.com/  |
    ///     +----+--------------+---------------------------+
    /// ```
    pub const fn ascii() -> CustomStyle<On, On, On, On, On, On, On> {
        CustomStyle::new(Self::ASCII)
    }

    /// Dots style looks like the following table
    ///
    /// ```text
    ///     .................................................
    ///     : id : destribution :           link            :
    ///     :....:..............:...........................:
    ///     : 0  :    Fedora    :  https://getfedora.org/   :
    ///     :....:..............:...........................:
    ///     : 2  :   OpenSUSE   : https://www.opensuse.org/ :
    ///     :....:..............:...........................:
    ///     : 3  : Endeavouros  : https://endeavouros.com/  :
    ///     :....:..............:...........................:
    /// ```
    pub const fn dots() -> CustomStyle<On, On, On, On, On, On, On> {
        CustomStyle::new(Self::DOTS)
    }

    /// Psql style looks like the following table
    ///
    /// ```text
    ///      id | destribution |           link
    ///     ----+--------------+---------------------------
    ///      0  |    Fedora    |  https://getfedora.org/
    ///      2  |   OpenSUSE   | https://www.opensuse.org/
    ///      3  | Endeavouros  | https://endeavouros.com/
    /// ```
    pub const fn psql() -> CustomStyle<(), (), (), (), (), On, On> {
        CustomStyle::new(Self::PSQL)
    }

    /// Github_markdown style looks like the following table
    ///
    /// ```text
    ///     | id | destribution |           link            |
    ///     |----+--------------+---------------------------|
    ///     | 0  |    Fedora    |  https://getfedora.org/   |
    ///     | 2  |   OpenSUSE   | https://www.opensuse.org/ |
    ///     | 3  | Endeavouros  | https://endeavouros.com/  |
    /// ```
    pub const fn github_markdown() -> CustomStyle<(), (), On, On, (), On, On> {
        CustomStyle::new(Self::GITHUB_MARKDOWN)
    }

    /// Modern style looks like the following table.
    ///
    /// Beware: It uses UTF8 characters.
    ///
    /// ```text
    ///     ┌────┬──────────────┬───────────────────────────┐
    ///     │ id │ destribution │           link            │
    ///     ├────┼──────────────┼───────────────────────────┤
    ///     │ 0  │    Fedora    │  https://getfedora.org/   │
    ///     ├────┼──────────────┼───────────────────────────┤
    ///     │ 2  │   OpenSUSE   │ https://www.opensuse.org/ │
    ///     ├────┼──────────────┼───────────────────────────┤
    ///     │ 3  │ Endeavouros  │ https://endeavouros.com/  │
    ///     └────┴──────────────┴───────────────────────────┘
    /// ```
    pub const fn modern() -> CustomStyle<On, On, On, On, On, On, On> {
        CustomStyle::new(Self::MODERN)
    }

    /// Extended style looks like the following table
    ///
    /// Beware: It uses UTF8 characters.
    ///
    /// ```text
    ///     ╔════╦══════════════╦═══════════════════════════╗
    ///     ║ id ║ destribution ║           link            ║
    ///     ╠════╬══════════════╬═══════════════════════════╣
    ///     ║ 0  ║    Fedora    ║  https://getfedora.org/   ║
    ///     ╠════╬══════════════╬═══════════════════════════╣
    ///     ║ 2  ║   OpenSUSE   ║ https://www.opensuse.org/ ║
    ///     ╠════╬══════════════╬═══════════════════════════╣
    ///     ║ 3  ║ Endeavouros  ║ https://endeavouros.com/  ║
    ///     ╚════╩══════════════╩═══════════════════════════╝
    /// ```
    pub const fn extended() -> CustomStyle<On, On, On, On, On, On, On> {
        CustomStyle::new(Self::EXTENDED)
    }

    /// ReStructuredText style looks like the following table
    ///
    /// ```text
    ///     ==== ============== ===========================
    ///      id   destribution             link            
    ///     ==== ============== ===========================
    ///      0       Fedora       https://getfedora.org/   
    ///      2      OpenSUSE     https://www.opensuse.org/
    ///      3    Endeavouros    https://endeavouros.com/  
    ///     ==== ============== ===========================
    /// ```
    pub const fn re_structured_text() -> CustomStyle<On, On, (), (), (), On, On> {
        CustomStyle::new(Self::RE_STRUCTURED_TEXT)
    }

    const EMPTY: StyleSettings =
        StyleSettings::new(Frame::empty(), Line::empty(), Line::empty(), Line::empty());

    const ASCII: StyleSettings = StyleSettings::new(
        Frame::full(
            Line::new('-', '+'),
            Line::new('-', '+'),
            Line::new('|', '+'),
            Line::new('|', '+'),
            ('+', '+', '+', '+'),
        ),
        Line::new('-', '+'),
        Line::new('-', '+'),
        Line::new('|', '+'),
    );

    const BLANK: StyleSettings = StyleSettings::new(
        Frame::empty(),
        Line::empty(),
        Line::empty(),
        Line::new(' ', ' '),
    );

    const PSQL: StyleSettings = StyleSettings::new(
        Frame::empty(),
        Line::empty(),
        Line::new('-', '+'),
        Line::new('|', '+'),
    );

    const GITHUB_MARKDOWN: StyleSettings = StyleSettings::new(
        Frame::bordered(
            Line::empty(),
            Line::empty(),
            Line::new('|', '|'),
            Line::new('|', '|'),
        ),
        Line::empty(),
        Line::new('-', '+'),
        Line::new('|', '|'),
    );

    const MODERN: StyleSettings = StyleSettings::new(
        Frame::full(
            Line::new('─', '┬'),
            Line::new('─', '┴'),
            Line::new('│', '├'),
            Line::new('│', '┤'),
            ('┌', '┐', '└', '┘'),
        ),
        Line::new('─', '┼'),
        Line::new('─', '┼'),
        Line::new('│', '┼'),
    );

    const EXTENDED: StyleSettings = StyleSettings::new(
        Frame::full(
            Line::new('═', '╦'),
            Line::new('═', '╩'),
            Line::new('║', '╠'),
            Line::new('║', '╣'),
            ('╔', '╗', '╚', '╝'),
        ),
        Line::new('═', '╬'),
        Line::new('═', '╬'),
        Line::new('║', '╬'),
    );

    const DOTS: StyleSettings = StyleSettings::new(
        Frame::full(
            Line::new('.', '.'),
            Line::new('.', ':'),
            Line::new(':', ':'),
            Line::new(':', ':'),
            ('.', '.', ':', ':'),
        ),
        Line::new('.', ':'),
        Line::new('.', ':'),
        Line::new(':', ':'),
    );

    const RE_STRUCTURED_TEXT: StyleSettings = StyleSettings::new(
        Frame::bordered(
            Line::new('=', ' '),
            Line::new('=', ' '),
            Line::empty(),
            Line::empty(),
        ),
        Line::empty(),
        Line::new('=', ' '),
        Line::new(' ', ' '),
    );
}

#[derive(Debug, Clone)]
pub struct StyleSettings {
    frame: Frame,
    horizontal: Line,
    header: Line,
    vertical: Line,
}

impl StyleSettings {
    const fn new(frame: Frame, horizontal: Line, header: Line, vertical: Line) -> Self {
        Self {
            frame,
            horizontal,
            header,
            vertical,
        }
    }

    const fn is_there_vertical(&self) -> bool {
        self.horizontal.intersection.is_some()
            || self.vertical.main.is_some()
            || self.vertical.intersection.is_some()
    }
}

/// Line represents a horizontal line on a [Table].
#[derive(Debug, Clone, Default)]
struct Line {
    main: Option<char>,
    intersection: Option<char>,
}

impl Line {
    /// Create a new line.
    const fn new(main: char, intersection: char) -> Self {
        Self {
            main: Some(main),
            intersection: Some(intersection),
        }
    }

    /// A line which doesn't exists.
    const fn empty() -> Self {
        Self {
            main: None,
            intersection: None,
        }
    }

    /// Chechks if any symbol is set in line.
    const fn is_empty(&self) -> bool {
        self.main.is_none() && self.intersection.is_none()
    }
}

#[derive(Debug, Clone, Default)]
struct Frame {
    top: Line,
    bottom: Line,
    left: Line,
    right: Line,
    corner_top_left: Option<char>,
    corner_top_right: Option<char>,
    corner_bottom_left: Option<char>,
    corner_bottom_right: Option<char>,
}

impl Frame {
    /// Creates a full border.
    const fn full(
        top: Line,
        bottom: Line,
        left: Line,
        right: Line,
        (top_left, top_right, bottom_left, bottom_right): (char, char, char, char),
    ) -> Self {
        Self {
            top,
            bottom,
            left,
            right,
            corner_top_left: Some(top_left),
            corner_top_right: Some(top_right),
            corner_bottom_left: Some(bottom_left),
            corner_bottom_right: Some(bottom_right),
        }
    }

    /// Creates a frame top, bottom, left and right borders.
    const fn bordered(top: Line, bottom: Line, left: Line, right: Line) -> Self {
        Self {
            bottom,
            top,
            left,
            right,
            corner_top_left: None,
            corner_top_right: None,
            corner_bottom_left: None,
            corner_bottom_right: None,
        }
    }

    /// Creates an empty border.
    const fn empty() -> Self {
        Self {
            bottom: Line::empty(),
            top: Line::empty(),
            left: Line::empty(),
            right: Line::empty(),
            corner_top_left: None,
            corner_top_right: None,
            corner_bottom_left: None,
            corner_bottom_right: None,
        }
    }
}

impl TableOption for StyleSettings {
    fn change(&mut self, grid: &mut Grid) {
        grid.clear_split_grid();
        grid.clear_overide_split_lines();

        let count_rows = grid.count_rows();
        let count_columns = grid.count_columns();
        for row in 0..count_rows {
            for column in 0..count_columns {
                let mut border = make_style(self, row, column, count_rows, count_columns);
                make_style_header(&mut border, self, row, column, count_rows, count_columns);

                grid.set(
                    Entity::Cell(row, column),
                    Settings::default().border(border).border_restriction(false),
                );
            }
        }
    }
}

fn make_style(
    style: &StyleSettings,
    row: usize,
    column: usize,
    count_rows: usize,
    count_columns: usize,
) -> Border {
    let is_first_row = row == 0;
    let is_last_row = row + 1 == count_rows;
    let is_first_column = column == 0;
    let is_last_column = column + 1 == count_columns;

    let inner_intersection = style.horizontal.intersection;
    let frame_left_intersection = if style.horizontal.is_empty() {
        None
    } else {
        style.frame.left.intersection
    };
    let frame_right_intersection = if style.horizontal.is_empty() {
        None
    } else {
        style.frame.right.intersection
    };

    match (is_first_row, is_last_row, is_first_column, is_last_column) {
        // A table with a single cell
        (true, true, true, true) => Border {
            top: style.frame.top.main.map(From::from),
            bottom: style.frame.bottom.main.map(From::from),
            right: style.frame.right.main.map(From::from),
            right_top_corner: style.frame.corner_top_right.map(From::from),
            right_bottom_corner: style.frame.corner_bottom_right.map(From::from),
            left: style.frame.left.main.map(From::from),
            left_top_corner: style.frame.corner_top_left.map(From::from),
            left_bottom_corner: style.frame.corner_bottom_left.map(From::from),
        },
        // Single row
        (true, true, true, false) => Border {
            top: style.frame.top.main.map(From::from),
            bottom: style.frame.bottom.main.map(From::from),
            left: style.frame.left.main.map(From::from),
            left_top_corner: style.frame.corner_top_left.map(From::from),
            left_bottom_corner: style.frame.corner_bottom_left.map(From::from),
            right: style.vertical.main.map(From::from),
            right_top_corner: style.frame.top.intersection.map(From::from),
            right_bottom_corner: style.frame.bottom.intersection.map(From::from),
        },
        (true, true, false, true) => Border {
            top: style.frame.top.main.map(From::from),
            bottom: style.frame.bottom.main.map(From::from),
            left: style.frame.left.main.map(From::from),
            left_top_corner: style.frame.top.intersection.map(From::from),
            left_bottom_corner: style.frame.bottom.intersection.map(From::from),
            right: style.vertical.main.map(From::from),
            right_top_corner: style.frame.corner_top_right.map(From::from),
            right_bottom_corner: style.frame.corner_bottom_right.map(From::from),
        },
        (true, true, false, false) => Border {
            top: style.frame.top.main.map(From::from),
            bottom: style.frame.bottom.main.map(From::from),
            left: style.vertical.main.map(From::from),
            left_top_corner: style.frame.corner_top_left.map(From::from),
            left_bottom_corner: style.frame.bottom.intersection.map(From::from),
            right: style.vertical.main.map(From::from),
            right_top_corner: style.frame.top.intersection.map(From::from),
            right_bottom_corner: style.frame.bottom.intersection.map(From::from),
        },
        // Single column
        (true, false, true, true) => Border {
            top: style.frame.top.main.map(From::from),
            bottom: style.horizontal.main.map(From::from),
            left: style.frame.left.main.map(From::from),
            left_top_corner: style.frame.corner_top_left.map(From::from),
            left_bottom_corner: style.frame.left.main.map(From::from),
            right: style.frame.right.main.map(From::from),
            right_top_corner: style.frame.corner_top_right.map(From::from),
            right_bottom_corner: frame_right_intersection.map(From::from),
        },
        (false, true, true, true) => Border {
            top: style.horizontal.main.map(From::from),
            bottom: style.frame.bottom.main.map(From::from),
            left: style.frame.left.main.map(From::from),
            left_top_corner: frame_left_intersection.map(From::from),
            left_bottom_corner: style.frame.corner_bottom_left.map(From::from),
            right: style.frame.right.main.map(From::from),
            right_top_corner: frame_right_intersection.map(From::from),
            right_bottom_corner: style.frame.corner_bottom_right.map(From::from),
        },
        (false, false, true, true) => Border {
            top: style.horizontal.main.map(From::from),
            bottom: style.horizontal.main.map(From::from),
            left: style.frame.left.main.map(From::from),
            left_top_corner: frame_left_intersection.map(From::from),
            left_bottom_corner: frame_left_intersection.map(From::from),
            right: style.frame.right.main.map(From::from),
            right_top_corner: frame_right_intersection.map(From::from),
            right_bottom_corner: frame_right_intersection.map(From::from),
        },
        // General
        (true, false, true, false) => Border {
            top: style.frame.top.main.map(Symbol::from),
            bottom: style.horizontal.main.map(Symbol::from),
            left: style.frame.left.main.map(Symbol::from),
            right: style.vertical.main.map(Symbol::from),
            left_top_corner: style.frame.corner_top_left.map(Symbol::from),
            left_bottom_corner: frame_left_intersection.map(Symbol::from),
            right_top_corner: style.frame.top.intersection.map(Symbol::from),
            right_bottom_corner: inner_intersection.map(Symbol::from),
        },
        (true, false, false, true) => Border {
            top: style.frame.top.main.map(Symbol::from),
            bottom: style.horizontal.main.map(Symbol::from),
            left: style.vertical.main.map(Symbol::from),
            right: style.frame.right.main.map(Symbol::from),
            left_top_corner: style.frame.top.intersection.map(Symbol::from),
            left_bottom_corner: inner_intersection.map(Symbol::from),
            right_top_corner: style.frame.corner_top_right.map(Symbol::from),
            right_bottom_corner: frame_right_intersection.map(Symbol::from),
        },
        (true, false, false, false) => Border {
            top: style.frame.top.main.map(Symbol::from),
            bottom: style.horizontal.main.map(Symbol::from),
            left: style.vertical.main.map(Symbol::from),
            right: style.vertical.main.map(Symbol::from),
            left_top_corner: style.frame.top.intersection.map(Symbol::from),
            left_bottom_corner: inner_intersection.map(Symbol::from),
            right_top_corner: style.frame.top.intersection.map(Symbol::from),
            right_bottom_corner: inner_intersection.map(Symbol::from),
        },
        (false, true, true, false) => Border {
            top: style.horizontal.main.map(Symbol::from),
            bottom: style.frame.bottom.main.map(Symbol::from),
            left: style.frame.left.main.map(Symbol::from),
            right: style.vertical.main.map(Symbol::from),
            left_top_corner: frame_left_intersection.map(Symbol::from),
            left_bottom_corner: style.frame.corner_bottom_left.map(Symbol::from),
            right_top_corner: inner_intersection.map(Symbol::from),
            right_bottom_corner: style.frame.bottom.intersection.map(Symbol::from),
        },
        (false, true, false, true) => Border {
            top: style.horizontal.main.map(Symbol::from),
            bottom: style.frame.bottom.main.map(Symbol::from),
            left: style.vertical.main.map(Symbol::from),
            right: style.frame.right.main.map(Symbol::from),
            left_top_corner: inner_intersection.map(Symbol::from),
            left_bottom_corner: style.frame.bottom.intersection.map(Symbol::from),
            right_top_corner: frame_right_intersection.map(Symbol::from),
            right_bottom_corner: style.frame.corner_bottom_right.map(Symbol::from),
        },
        (false, true, false, false) => Border {
            top: style.horizontal.main.map(Symbol::from),
            bottom: style.frame.bottom.main.map(Symbol::from),
            left: style.vertical.main.map(Symbol::from),
            right: style.vertical.main.map(Symbol::from),
            left_top_corner: inner_intersection.map(Symbol::from),
            left_bottom_corner: style.frame.bottom.intersection.map(Symbol::from),
            right_top_corner: inner_intersection.map(Symbol::from),
            right_bottom_corner: style.frame.bottom.intersection.map(Symbol::from),
        },
        (false, false, true, false) => Border {
            top: style.horizontal.main.map(Symbol::from),
            bottom: style.horizontal.main.map(Symbol::from),
            left: style.frame.left.main.map(Symbol::from),
            right: style.vertical.main.map(Symbol::from),
            left_top_corner: frame_left_intersection.map(Symbol::from),
            left_bottom_corner: frame_left_intersection.map(Symbol::from),
            right_top_corner: inner_intersection.map(Symbol::from),
            right_bottom_corner: inner_intersection.map(Symbol::from),
        },
        (false, false, false, true) => Border {
            top: style.horizontal.main.map(Symbol::from),
            bottom: style.horizontal.main.map(Symbol::from),
            left: style.vertical.main.map(Symbol::from),
            right: style.frame.right.main.map(Symbol::from),
            left_top_corner: inner_intersection.map(Symbol::from),
            left_bottom_corner: inner_intersection.map(Symbol::from),
            right_top_corner: frame_right_intersection.map(Symbol::from),
            right_bottom_corner: frame_right_intersection.map(Symbol::from),
        },
        (false, false, false, false) => Border {
            top: style.horizontal.main.map(Symbol::from),
            bottom: style.horizontal.main.map(Symbol::from),
            left: style.vertical.main.map(Symbol::from),
            right: style.vertical.main.map(Symbol::from),
            left_top_corner: inner_intersection.map(Symbol::from),
            left_bottom_corner: inner_intersection.map(Symbol::from),
            right_top_corner: inner_intersection.map(Symbol::from),
            right_bottom_corner: inner_intersection.map(Symbol::from),
        },
    }
}

fn make_style_header(
    border: &mut Border,
    style: &StyleSettings,
    row: usize,
    column: usize,
    count_rows: usize,
    count_columns: usize,
) {
    let is_first_column = column == 0;
    let is_last_column = column + 1 == count_columns;

    let is_single_cell = row + 1 == count_rows && column + 1 == count_columns;
    if is_single_cell {
        return;
    }

    if !style.header.is_empty() {
        if row == 1 {
            border.top = style.header.main.map(Symbol::from);

            if is_last_column {
                border.right_top_corner = style.frame.right.intersection.map(Symbol::from);
            } else {
                border.right_top_corner = style.header.intersection.map(Symbol::from);
            }

            if is_first_column {
                border.left_top_corner = style.frame.left.intersection.map(Symbol::from);
            } else {
                border.left_top_corner = style.header.intersection.map(Symbol::from);
            }
        }

        if row == 0 {
            border.bottom = style.header.main.map(Symbol::from);

            if is_last_column {
                border.right_bottom_corner = style.frame.right.intersection.map(Symbol::from);
            } else {
                border.right_bottom_corner = style.header.intersection.map(Symbol::from);
            }

            if is_first_column {
                border.left_bottom_corner = style.frame.left.intersection.map(Symbol::from);
            } else {
                border.left_bottom_corner = style.header.intersection.map(Symbol::from);
            }
        }
    } else if count_columns > 1 {
        if row == 1 {
            border.top = None;
            border.right_top_corner = None;
            border.left_top_corner = None;
        }

        if row == 0 {
            border.bottom = None;
            border.right_bottom_corner = None;
            border.left_bottom_corner = None;
        }
    }
}

/// TopBorderText writes a custom text on a top border.
///
/// # Example
///
/// ```rust
/// use tabled::{Table, style::BorderText};
/// let table = Table::new(["Hello World"])
///     .with(BorderText::first("+-.table"));
///
/// assert_eq!(
///     table.to_string(),
///     "+-.table------+\n\
///      |    &str     |\n\
///      +-------------+\n\
///      | Hello World |\n\
///      +-------------+\n"
/// );
/// ```
pub struct BorderText<'a> {
    // todo: offset from which we start overriding border
    // offset: usize,
    text: Cow<'a, str>,
    row: SplitLineIndex,
}

enum SplitLineIndex {
    First,
    Last,
    Line(usize),
}

impl<'a> BorderText<'a> {
    pub fn new<S: Into<Cow<'a, str>>>(line: usize, text: S) -> Self {
        Self {
            text: text.into(),
            row: SplitLineIndex::Line(line),
        }
    }

    pub fn first<S: Into<Cow<'a, str>>>(text: S) -> Self {
        Self {
            text: text.into(),
            row: SplitLineIndex::First,
        }
    }

    pub fn last<S: Into<Cow<'a, str>>>(text: S) -> Self {
        Self {
            text: text.into(),
            row: SplitLineIndex::Last,
        }
    }
}

impl<'a> TableOption for BorderText<'a> {
    fn change(&mut self, grid: &mut Grid) {
        let row = match self.row {
            SplitLineIndex::First => 0,
            SplitLineIndex::Last => grid.count_rows(),
            SplitLineIndex::Line(row) => {
                if row > grid.count_rows() {
                    return;
                }

                row
            }
        };

        grid.override_split_line(row, self.text.as_ref())
    }
}

/// CustomStyle represents a style controlling a valid state of it.
///
/// For example.
/// It doesn't allow to call method [CustomStyle::top_left_corner] unless [CustomStyle::left] and [CustomStyle::top] is set.
#[derive(Debug, Clone)]
pub struct CustomStyle<Top, Bottom, Left, Right, Horizontal, Vertical, Header> {
    inner: StyleSettings,
    _l_border: PhantomData<Left>,
    _r_border: PhantomData<Right>,
    _t_border: PhantomData<Top>,
    _b_border: PhantomData<Bottom>,
    _i_h_border: PhantomData<Horizontal>,
    _i_v_border: PhantomData<Vertical>,
    _h_border: PhantomData<Header>,
}

#[derive(Debug, Clone)]
pub struct On;

impl<Top, Bottom, Left, Rright, Horizontal, Vertical, Header>
    CustomStyle<Top, Bottom, Left, Rright, Horizontal, Vertical, Header>
{
    const fn new(style: StyleSettings) -> Self {
        Self {
            inner: style,
            _b_border: PhantomData,
            _l_border: PhantomData,
            _r_border: PhantomData,
            _t_border: PhantomData,
            _i_h_border: PhantomData,
            _i_v_border: PhantomData,
            _h_border: PhantomData,
        }
    }
}

impl<T, B, L, R, IH, IV, H> CustomStyle<T, B, L, R, IH, IV, H> {
    /// Sets a top border.
    ///
    /// Any corners and intersections which were set will be overriden.
    pub const fn top(self, c: char) -> CustomStyle<On, B, L, R, IH, IV, H> {
        let mut style = self.inner;

        style.frame.top.main = Some(c);

        if !style.frame.left.is_empty() {
            style.frame.corner_top_left = Some(c);
        }

        if !style.frame.right.is_empty() {
            style.frame.corner_top_right = Some(c);
        }

        if style.is_there_vertical() {
            style.frame.top.intersection = Some(c);
        }

        CustomStyle::new(style)
    }

    /// Sets a bottom border.
    ///
    /// Any corners and intersections which were set will be overriden.
    pub const fn bottom(self, c: char) -> CustomStyle<T, On, L, R, IH, IV, H> {
        let mut style = self.inner;
        style.frame.bottom.main = Some(c);

        if !style.frame.left.is_empty() {
            style.frame.corner_bottom_left = Some(c);
        }

        if !style.frame.right.is_empty() {
            style.frame.corner_bottom_right = Some(c);
        }

        if style.is_there_vertical() {
            style.frame.bottom.intersection = Some(c);
        }

        CustomStyle::new(style)
    }

    /// Sets a left border.
    ///
    /// Any corners and intersections which were set will be overriden.
    pub const fn left(self, c: char) -> CustomStyle<T, B, On, R, IH, IV, H> {
        let mut style = self.inner;
        style.frame.left.main = Some(c);

        if !style.frame.top.is_empty() {
            style.frame.corner_top_left = Some(c);
        }

        if !style.frame.bottom.is_empty() {
            style.frame.corner_bottom_left = Some(c);
        }

        if !style.horizontal.is_empty() {
            style.frame.left.intersection = Some(c);
        }

        CustomStyle::new(style)
    }

    /// Sets a right border.
    ///
    /// Any corners and intersections which were set will be overriden.
    pub const fn right(self, c: char) -> CustomStyle<T, B, L, On, IH, IV, H> {
        let mut style = self.inner;
        style.frame.right.main = Some(c);

        if !style.frame.top.is_empty() {
            style.frame.corner_top_right = Some(c);
        }

        if !style.frame.bottom.is_empty() {
            style.frame.corner_bottom_right = Some(c);
        }

        if !style.horizontal.is_empty() {
            style.frame.right.intersection = Some(c);
        }

        CustomStyle::new(style)
    }

    /// Sets a horizontal split line.
    ///
    /// It doesn't include a header split line.
    /// It must be set via its own method [Self::header].
    ///
    /// Any corners and intersections which were set will be overriden.
    pub const fn horizontal(self, c: char) -> CustomStyle<T, B, L, R, On, IV, H> {
        let mut style = self.inner;
        style.horizontal.main = Some(c);

        if style.horizontal.intersection.is_some() {
            style.horizontal.intersection = Some(c);
        }

        if !style.vertical.is_empty() {
            style.vertical.intersection = Some(c);
            style.horizontal.intersection = Some(c);
        }

        if !style.frame.left.is_empty() {
            style.frame.left.intersection = Some(c);
        }

        if !style.frame.right.is_empty() {
            style.frame.right.intersection = Some(c);
        }

        CustomStyle::new(style)
    }

    /// Sets a vertical split line.
    ///
    /// Any corners and intersections which were set will be overriden.
    pub const fn vertical(self, c: char) -> CustomStyle<T, B, L, R, IH, On, H> {
        let mut style = self.inner;
        style.vertical.main = Some(c);

        if !style.horizontal.is_empty() {
            style.horizontal.intersection = Some(c);
        }

        if !style.header.is_empty() {
            style.header.intersection = Some(c);
        }

        if !style.frame.top.is_empty() {
            style.frame.top.intersection = Some(c);
        }

        if !style.frame.bottom.is_empty() {
            style.frame.bottom.intersection = Some(c);
        }

        CustomStyle::new(style)
    }

    /// Sets a 1st horizontal split line.
    ///
    /// Any corners and intersections which were set will be overriden.
    pub const fn header(self, c: char) -> CustomStyle<T, B, L, R, IH, IV, On> {
        let mut style = self.inner;
        style.header.main = Some(c);

        if !style.vertical.is_empty() {
            style.header.intersection = Some(c);
        }

        CustomStyle::new(style)
    }
}

impl<B, R, IH, IV, H> CustomStyle<On, B, On, R, IH, IV, H> {
    /// Sets a top left corner.
    pub const fn top_left_corner(self, c: char) -> Self {
        let mut style = self.inner;
        style.frame.corner_top_left = Some(c);

        CustomStyle::new(style)
    }
}

impl<B, L, IH, IV, H> CustomStyle<On, B, L, On, IH, IV, H> {
    /// Sets a top right corner.
    pub const fn top_right_corner(self, c: char) -> Self {
        let mut style = self.inner;
        style.frame.corner_top_right = Some(c);

        CustomStyle::new(style)
    }
}

impl<T, L, IH, IV, H> CustomStyle<T, On, L, On, IH, IV, H> {
    /// Sets a bottom right corner.
    pub const fn bottom_right_corner(self, c: char) -> Self {
        let mut style = self.inner;
        style.frame.corner_bottom_right = Some(c);

        CustomStyle::new(style)
    }
}

impl<T, R, IH, IV, H> CustomStyle<T, On, On, R, IH, IV, H> {
    /// Sets a bottom left corner.
    pub const fn bottom_left_corner(self, c: char) -> Self {
        let mut style = self.inner;
        style.frame.corner_bottom_left = Some(c);

        CustomStyle::new(style)
    }
}

impl<T, B, R, IV, H> CustomStyle<T, B, On, R, On, IV, H> {
    /// Sets a left intersection char.
    pub const fn left_intersection(self, c: char) -> Self {
        let mut style = self.inner;
        style.frame.left.intersection = Some(c);

        CustomStyle::new(style)
    }
}

impl<T, B, L, IV, H> CustomStyle<T, B, L, On, On, IV, H> {
    /// Sets a right intersection char.
    pub const fn right_intersection(self, c: char) -> Self {
        let mut style = self.inner;
        style.frame.right.intersection = Some(c);

        CustomStyle::new(style)
    }
}

impl<B, L, R, IH, H> CustomStyle<On, B, L, R, IH, On, H> {
    /// Sets a top intersection char.
    pub const fn top_intersection(self, c: char) -> Self {
        let mut style = self.inner;
        style.frame.top.intersection = Some(c);

        CustomStyle::new(style)
    }
}

impl<T, L, R, IH, H> CustomStyle<T, On, L, R, IH, On, H> {
    /// Sets a bottom intersection char.
    pub const fn bottom_intersection(self, c: char) -> Self {
        let mut style = self.inner;
        style.frame.bottom.intersection = Some(c);

        CustomStyle::new(style)
    }
}

impl<T, B, L, R, H> CustomStyle<T, B, L, R, On, On, H> {
    /// Sets an inner intersection char.
    /// A char between horizontal and vertical split lines.
    pub const fn inner_intersection(mut self, c: char) -> Self {
        self.inner.horizontal.intersection = Some(c);
        self.inner.vertical.intersection = Some(c);
        CustomStyle::new(self.inner)
    }
}

impl<T, B, L, R, IH> CustomStyle<T, B, L, R, IH, On, On> {
    /// Sets an intersection char of a 1st horizontal split line.
    pub const fn header_intersection(mut self, c: char) -> Self {
        self.inner.header.intersection = Some(c);
        CustomStyle::new(self.inner)
    }
}

impl<B, L, R, IH, IV, H> CustomStyle<On, B, L, R, IH, IV, H> {
    /// Removes top border.
    pub const fn top_off(mut self) -> CustomStyle<(), B, L, R, IH, IV, H> {
        self.inner.frame.top = Line::empty();
        self.inner.frame.corner_top_left = None;
        self.inner.frame.corner_top_right = None;

        CustomStyle::new(self.inner)
    }
}

impl<T, L, R, IH, IV, H> CustomStyle<T, On, L, R, IH, IV, H> {
    /// Removes bottom border.
    pub const fn bottom_off(mut self) -> CustomStyle<T, (), L, R, IH, IV, H> {
        self.inner.frame.bottom = Line::empty();
        self.inner.frame.corner_bottom_left = None;
        self.inner.frame.corner_bottom_right = None;

        CustomStyle::new(self.inner)
    }
}

impl<T, B, R, IH, IV, H> CustomStyle<T, B, On, R, IH, IV, H> {
    /// Removes left border.
    pub const fn left_off(mut self) -> CustomStyle<T, B, (), R, IH, IV, H> {
        self.inner.frame.left = Line::empty();
        self.inner.frame.corner_top_left = None;
        self.inner.frame.corner_bottom_left = None;

        CustomStyle::new(self.inner)
    }
}

impl<T, B, L, IH, IV, H> CustomStyle<T, B, L, On, IH, IV, H> {
    /// Removes right border.
    pub const fn right_off(mut self) -> CustomStyle<T, B, L, (), IH, IV, H> {
        self.inner.frame.right = Line::empty();
        self.inner.frame.corner_top_right = None;
        self.inner.frame.corner_bottom_right = None;

        CustomStyle::new(self.inner)
    }
}

impl<T, B, L, R, IV, H> CustomStyle<T, B, L, R, On, IV, H> {
    /// Removes horizontal split lines.
    ///
    /// Not including 1st split line.
    pub const fn horizontal_off(mut self) -> CustomStyle<T, B, L, R, (), IV, H> {
        self.inner.horizontal = Line::empty();
        self.inner.vertical.intersection = None;

        CustomStyle::new(self.inner)
    }
}

impl<T, B, L, R, IH, H> CustomStyle<T, B, L, R, IH, On, H> {
    /// Removes vertical split lines.
    pub const fn vertical_off(mut self) -> CustomStyle<T, B, L, R, IH, (), H> {
        self.inner.vertical = Line::empty();
        self.inner.horizontal.intersection = None;
        self.inner.header.intersection = None;
        self.inner.frame.top.intersection = None;
        self.inner.frame.bottom.intersection = None;

        CustomStyle::new(self.inner)
    }
}

impl<T, B, L, R, IH, IV> CustomStyle<T, B, L, R, IH, IV, On> {
    /// Removes 1st horizontal split line.
    pub const fn header_off(mut self) -> CustomStyle<T, B, L, R, IH, IV, ()> {
        self.inner.header = Line::empty();
        CustomStyle::new(self.inner)
    }
}

impl<T, B, L, R, IH, IV, H> TableOption for CustomStyle<T, B, L, R, IH, IV, H> {
    fn change(&mut self, grid: &mut Grid) {
        self.inner.change(grid);
    }
}

/// Border represents a border of a Cell.
///
/// ```rust,no_run
///   # use tabled::{style::{Style, Border}, object::Rows, Table, Modify};
///   # let data: Vec<&'static str> = Vec::new();
///     let table = Table::new(&data)
///         .with(Style::ascii())
///         .with(Modify::new(Rows::single(0)).with(Border::default().top('x')));
/// ```
pub use papergrid::Border;

impl CellOption for Border {
    fn change_cell(&mut self, grid: &mut Grid, row: usize, column: usize) {
        Highlight::new(Cell(row, column), self.clone()).change(grid);
    }
}

/// Symbol represents a character of a border.
///
/// It's only needed when used with `color` feature flag.
///
/// ```rust,no_run
///   # use owo_colors::OwoColorize;
///   # use tabled::{style::{Border, Symbol}, object::Rows, TableIteratorExt, Modify};
///   #
///   # let data: Vec<&'static str> = Vec::new();
///     let colored_char = "#".red().to_string();
///     let table = data.table()
///         .with(Modify::new(Rows::single(0)).with(Border::filled(Symbol::ansi(colored_char).unwrap())));
/// ```
#[cfg(feature = "color")]
pub use papergrid::Symbol;

#[cfg(not(feature = "color"))]
use papergrid::Symbol;
