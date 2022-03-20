//! This module contains a list of Styles which can be applied to change [Table] styles.

use std::{borrow::Cow, marker::PhantomData};

#[allow(unused)]
use crate::Table;
use crate::{CellOption, Highlight, TableOption};
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

    const EMPTY: StyleSettings = StyleSettings::new(Frame::empty(), None, None, None);

    const ASCII: StyleSettings = StyleSettings::new(
        Frame {
            bottom: Some(Line::bordered('-', '+', '+', '+')),
            top: Some(Line::bordered('-', '+', '+', '+')),
            left: Some('|'),
            right: Some('|'),
        },
        Some(Line::bordered('-', '+', '+', '+')),
        Some(Line::bordered('-', '+', '+', '+')),
        Some('|'),
    );

    const BLANK: StyleSettings = StyleSettings::new(Frame::empty(), None, None, Some(' '));

    const PSQL: StyleSettings =
        StyleSettings::new(Frame::empty(), Some(Line::short('-', '+')), None, Some('|'));

    const GITHUB_MARKDOWN: StyleSettings = StyleSettings::new(
        Frame {
            left: Some('|'),
            right: Some('|'),
            bottom: None,
            top: None,
        },
        Some(Line::bordered('-', '+', '|', '|')),
        None,
        Some('|'),
    );

    const MODERN: StyleSettings = StyleSettings::new(
        Frame {
            left: Some('│'),
            right: Some('│'),
            bottom: Some(Line::bordered('─', '┴', '└', '┘')),
            top: Some(Line::bordered('─', '┬', '┌', '┐')),
        },
        Some(Line::bordered('─', '┼', '├', '┤')),
        Some(Line::bordered('─', '┼', '├', '┤')),
        Some('│'),
    );

    const EXTENDED: StyleSettings = StyleSettings::new(
        Frame {
            left: Some('║'),
            right: Some('║'),
            bottom: Some(Line::bordered('═', '╩', '╚', '╝')),
            top: Some(Line::bordered('═', '╦', '╔', '╗')),
        },
        Some(Line::bordered('═', '╬', '╠', '╣')),
        Some(Line::bordered('═', '╬', '╠', '╣')),
        Some('║'),
    );

    const DOTS: StyleSettings = StyleSettings::new(
        Frame {
            bottom: Some(Line::bordered('.', ':', ':', ':')),
            top: Some(Line::bordered('.', '.', '.', '.')),
            left: Some(':'),
            right: Some(':'),
        },
        Some(Line::bordered('.', ':', ':', ':')),
        None,
        Some(':'),
    );

    const RE_STRUCTURED_TEXT: StyleSettings = StyleSettings::new(
        Frame {
            bottom: Some(Line::short('=', ' ')),
            top: Some(Line::short('=', ' ')),
            left: None,
            right: None,
        },
        Some(Line::short('=', ' ')),
        None,
        Some(' '),
    );
}

#[derive(Debug, Clone)]
pub struct StyleSettings {
    frame: Frame,
    header_split_line: Option<Line>,
    split: Option<Line>,
    inner_split_char: Option<char>,
}

impl StyleSettings {
    const fn new(
        frame: Frame,
        header: Option<Line>,
        split: Option<Line>,
        inner: Option<char>,
    ) -> Self {
        Self {
            frame,
            split,
            header_split_line: header,
            inner_split_char: inner,
        }
    }
}

/// Line represents a horizontal line on a [Table].
#[derive(Debug, Clone, Default)]
struct Line {
    main: char,
    intersection: Option<char>,
    left_corner: Option<char>,
    right_corner: Option<char>,
}

impl Line {
    /// A line for frame styles.
    const fn bordered(main: char, intersection: char, left: char, right: char) -> Self {
        Self {
            main,
            intersection: Some(intersection),
            left_corner: Some(left),
            right_corner: Some(right),
        }
    }

    /// A line for no-frame styles.
    const fn short(main: char, intersection: char) -> Self {
        Self {
            main,
            intersection: Some(intersection),
            left_corner: None,
            right_corner: None,
        }
    }
}

#[derive(Debug, Clone, Default)]
struct Frame {
    top: Option<Line>,
    bottom: Option<Line>,
    left: Option<char>,
    right: Option<char>,
}

impl Frame {
    const fn empty() -> Self {
        Self {
            bottom: None,
            top: None,
            left: None,
            right: None,
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
                    &Entity::Cell(row, column),
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

    match (is_first_row, is_last_row, is_first_column, is_last_column) {
        // A table with a single cell
        (true, true, true, true) => Border {
            top: style.frame.top.as_ref().map(|l| l.main),
            bottom: style.frame.bottom.as_ref().map(|l| l.main),
            right: style.frame.right,
            right_top_corner: style.frame.top.as_ref().and_then(|l| l.right_corner),
            right_bottom_corner: style.frame.bottom.as_ref().and_then(|l| l.right_corner),
            left: style.frame.left,
            left_top_corner: style.frame.top.as_ref().and_then(|l| l.left_corner),
            left_bottom_corner: style.frame.bottom.as_ref().and_then(|l| l.left_corner),
        },
        // Single row
        (true, true, true, false) => Border {
            top: style.frame.top.as_ref().map(|l| l.main),
            bottom: style.frame.bottom.as_ref().map(|l| l.main),
            left: style.frame.left,
            left_top_corner: style.frame.top.as_ref().and_then(|l| l.left_corner),
            left_bottom_corner: style.frame.bottom.as_ref().and_then(|l| l.left_corner),
            right: style.inner_split_char,
            right_top_corner: style.frame.top.as_ref().and_then(|l| l.intersection),
            right_bottom_corner: style.frame.bottom.as_ref().and_then(|l| l.intersection),
        },
        (true, true, false, true) => Border {
            top: style.frame.top.as_ref().map(|l| l.main),
            bottom: style.frame.bottom.as_ref().map(|l| l.main),
            left: style.frame.left,
            left_top_corner: style.frame.top.as_ref().and_then(|l| l.intersection),
            left_bottom_corner: style.frame.bottom.as_ref().and_then(|l| l.intersection),
            right: style.inner_split_char,
            right_top_corner: style.frame.top.as_ref().and_then(|l| l.right_corner),
            right_bottom_corner: style.frame.bottom.as_ref().and_then(|l| l.right_corner),
        },
        (true, true, false, false) => Border {
            top: style.frame.top.as_ref().map(|l| l.main),
            bottom: style.frame.bottom.as_ref().map(|l| l.main),
            left: style.inner_split_char,
            left_top_corner: style.frame.top.as_ref().and_then(|l| l.left_corner),
            left_bottom_corner: style.frame.bottom.as_ref().and_then(|l| l.intersection),
            right: style.inner_split_char,
            right_top_corner: style.frame.top.as_ref().and_then(|l| l.intersection),
            right_bottom_corner: style.frame.bottom.as_ref().and_then(|l| l.intersection),
        },
        // Single column
        (true, false, true, true) => Border {
            top: style.frame.top.as_ref().map(|l| l.main),
            bottom: style.split.as_ref().map(|l| l.main),
            left: style.frame.left,
            left_top_corner: style.frame.top.as_ref().and_then(|l| l.left_corner),
            left_bottom_corner: style.split.as_ref().and_then(|l| l.left_corner),
            right: style.frame.right,
            right_top_corner: style.frame.top.as_ref().and_then(|l| l.right_corner),
            right_bottom_corner: style.split.as_ref().and_then(|l| l.right_corner),
        },
        (false, true, true, true) => Border {
            top: style.split.as_ref().map(|l| l.main),
            bottom: style.frame.bottom.as_ref().map(|l| l.main),
            left: style.frame.left,
            left_top_corner: style.split.as_ref().and_then(|l| l.left_corner),
            left_bottom_corner: style.frame.bottom.as_ref().and_then(|l| l.left_corner),
            right: style.frame.right,
            right_top_corner: style.split.as_ref().and_then(|l| l.right_corner),
            right_bottom_corner: style.frame.bottom.as_ref().and_then(|l| l.right_corner),
        },
        (false, false, true, true) => Border {
            top: style.split.as_ref().map(|l| l.main),
            bottom: style.split.as_ref().map(|l| l.main),
            left: style.frame.left,
            left_top_corner: style.split.as_ref().and_then(|l| l.left_corner),
            left_bottom_corner: style.split.as_ref().and_then(|l| l.left_corner),
            right: style.frame.right,
            right_top_corner: style.split.as_ref().and_then(|l| l.right_corner),
            right_bottom_corner: style.split.as_ref().and_then(|l| l.right_corner),
        },
        // General
        (true, false, true, false) => Border {
            top: style.frame.top.as_ref().map(|l| l.main),
            bottom: style.split.as_ref().map(|l| l.main),
            left: style.frame.left,
            left_top_corner: style.frame.top.as_ref().and_then(|l| l.left_corner),
            left_bottom_corner: style.split.as_ref().and_then(|l| l.left_corner),
            right: style.inner_split_char,
            right_top_corner: style.frame.top.as_ref().and_then(|l| l.intersection),
            right_bottom_corner: style.split.as_ref().and_then(|l| l.intersection),
        },
        (true, false, false, true) => Border {
            top: style.frame.top.as_ref().map(|l| l.main),
            bottom: style.split.as_ref().map(|l| l.main),
            left: style.inner_split_char,
            left_top_corner: style.frame.top.as_ref().and_then(|l| l.intersection),
            left_bottom_corner: style.split.as_ref().and_then(|l| l.intersection),
            right: style.frame.right,
            right_top_corner: style.frame.top.as_ref().and_then(|l| l.right_corner),
            right_bottom_corner: style.split.as_ref().and_then(|l| l.right_corner),
        },
        (true, false, false, false) => Border {
            top: style.frame.top.as_ref().map(|l| l.main),
            bottom: style.split.as_ref().map(|l| l.main),
            left: style.inner_split_char,
            left_top_corner: style.frame.top.as_ref().and_then(|l| l.intersection),
            left_bottom_corner: style.split.as_ref().and_then(|l| l.intersection),
            right: style.inner_split_char,
            right_top_corner: style.frame.top.as_ref().and_then(|l| l.intersection),
            right_bottom_corner: style.split.as_ref().and_then(|l| l.intersection),
        },
        (false, true, true, false) => Border {
            top: style.split.as_ref().map(|l| l.main),
            bottom: style.frame.bottom.as_ref().map(|l| l.main),
            left: style.frame.left,
            left_top_corner: style.split.as_ref().and_then(|l| l.left_corner),
            left_bottom_corner: style.frame.bottom.as_ref().and_then(|l| l.left_corner),
            right: style.inner_split_char,
            right_top_corner: style.split.as_ref().and_then(|l| l.intersection),
            right_bottom_corner: style.frame.bottom.as_ref().and_then(|l| l.intersection),
        },
        (false, true, false, true) => Border {
            top: style.split.as_ref().map(|l| l.main),
            bottom: style.frame.bottom.as_ref().map(|l| l.main),
            left: style.inner_split_char,
            left_top_corner: style.split.as_ref().and_then(|l| l.intersection),
            left_bottom_corner: style.frame.bottom.as_ref().and_then(|l| l.intersection),
            right: style.frame.right,
            right_top_corner: style.split.as_ref().and_then(|l| l.right_corner),
            right_bottom_corner: style.frame.bottom.as_ref().and_then(|l| l.right_corner),
        },
        (false, true, false, false) => Border {
            top: style.split.as_ref().map(|l| l.main),
            bottom: style.frame.bottom.as_ref().map(|l| l.main),
            left: style.inner_split_char,
            left_top_corner: style.split.as_ref().and_then(|l| l.intersection),
            left_bottom_corner: style.frame.bottom.as_ref().and_then(|l| l.intersection),
            right: style.inner_split_char,
            right_top_corner: style.split.as_ref().and_then(|l| l.intersection),
            right_bottom_corner: style.frame.bottom.as_ref().and_then(|l| l.intersection),
        },
        (false, false, true, false) => Border {
            top: style.split.as_ref().map(|l| l.main),
            bottom: style.split.as_ref().map(|l| l.main),
            left: style.frame.left,
            left_top_corner: style.split.as_ref().and_then(|l| l.left_corner),
            left_bottom_corner: style.split.as_ref().and_then(|l| l.left_corner),
            right: style.inner_split_char,
            right_top_corner: style.split.as_ref().and_then(|l| l.intersection),
            right_bottom_corner: style.split.as_ref().and_then(|l| l.intersection),
        },
        (false, false, false, true) => Border {
            top: style.split.as_ref().map(|l| l.main),
            bottom: style.split.as_ref().map(|l| l.main),
            left: style.inner_split_char,
            left_top_corner: style.split.as_ref().and_then(|l| l.intersection),
            left_bottom_corner: style.split.as_ref().and_then(|l| l.intersection),
            right: style.frame.right,
            right_top_corner: style.split.as_ref().and_then(|l| l.right_corner),
            right_bottom_corner: style.split.as_ref().and_then(|l| l.right_corner),
        },
        (false, false, false, false) => Border {
            top: style.split.as_ref().map(|l| l.main),
            bottom: style.split.as_ref().map(|l| l.main),
            left: style.inner_split_char,
            left_top_corner: style.split.as_ref().and_then(|l| l.intersection),
            left_bottom_corner: style.split.as_ref().and_then(|l| l.intersection),
            right: style.inner_split_char,
            right_top_corner: style.split.as_ref().and_then(|l| l.intersection),
            right_bottom_corner: style.split.as_ref().and_then(|l| l.intersection),
        },
    }
}

fn make_style_header(
    border: &mut Border,
    style: &StyleSettings,
    row: usize,
    column: usize,
    _count_rows: usize,
    count_columns: usize,
) {
    let is_first_column = column == 0;
    let is_last_column = column + 1 == count_columns;

    if style.header_split_line.is_some() {
        if row == 1 {
            border.top = style.header_split_line.as_ref().map(|l| l.main);

            if is_last_column {
                border.right_top_corner = style
                    .header_split_line
                    .as_ref()
                    .and_then(|l| l.right_corner);
            } else {
                border.right_top_corner = style
                    .header_split_line
                    .as_ref()
                    .and_then(|l| l.intersection);
            }

            if is_first_column {
                border.left_top_corner =
                    style.header_split_line.as_ref().and_then(|l| l.left_corner);
            } else {
                border.left_top_corner = style
                    .header_split_line
                    .as_ref()
                    .and_then(|l| l.intersection);
            }
        }

        if row == 0 {
            border.bottom = style.header_split_line.as_ref().map(|l| l.main);

            if is_last_column {
                border.right_bottom_corner = style
                    .header_split_line
                    .as_ref()
                    .and_then(|l| l.right_corner);
            } else {
                border.right_bottom_corner = style
                    .header_split_line
                    .as_ref()
                    .and_then(|l| l.intersection);
            }

            if is_first_column {
                border.left_bottom_corner =
                    style.header_split_line.as_ref().and_then(|l| l.left_corner);
            } else {
                border.left_bottom_corner = style
                    .header_split_line
                    .as_ref()
                    .and_then(|l| l.intersection);
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

/// Style is responsible for a look of a [Table].
///
/// # Example
///
/// ```rust
/// use tabled::{Table, style::TopBorderText};
/// let table = Table::new(["Hello World"])
///     .with(TopBorderText::new("+-.table"));
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
pub struct TopBorderText<'a> {
    // todo: offset from which we start overriding border
    // offset: usize,
    text: Cow<'a, str>,
}

impl<'a> TopBorderText<'a> {
    pub fn new<S: Into<Cow<'a, str>>>(text: S) -> Self {
        Self { text: text.into() }
    }
}

impl<'a> TableOption for TopBorderText<'a> {
    fn change(&mut self, grid: &mut Grid) {
        grid.override_split_line(0, self.text.as_ref())
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

        let mut top_line = Line {
            main: c,
            intersection: None,
            left_corner: None,
            right_corner: None,
        };

        if let Some(top) = style.frame.top {
            top_line.intersection = top.intersection;
            top_line.left_corner = top.left_corner;
            top_line.left_corner = top.left_corner;
        }

        if style.frame.left.is_some() {
            top_line.left_corner = Some(c);
        }

        if style.frame.right.is_some() {
            top_line.right_corner = Some(c);
        }

        if style.inner_split_char.is_some() {
            top_line.intersection = Some(c);
        }

        style.frame.top = Some(top_line);

        CustomStyle::new(style)
    }

    /// Sets a bottom border.
    ///
    /// Any corners and intersections which were set will be overriden.
    pub const fn bottom(self, c: char) -> CustomStyle<T, On, L, R, IH, IV, H> {
        let mut style = self.inner;
        let mut bottom = match style.frame.bottom {
            Some(mut line) => {
                line.main = c;
                line
            }
            None => Line {
                main: c,
                intersection: None,
                left_corner: None,
                right_corner: None,
            },
        };

        if style.frame.left.is_some() {
            bottom.left_corner = Some(c);
        }

        if style.frame.right.is_some() {
            bottom.right_corner = Some(c);
        }

        if style.inner_split_char.is_some() {
            bottom.intersection = Some(c);
        }

        style.frame.bottom = Some(bottom);

        CustomStyle::new(style)
    }

    /// Sets a left border.
    ///
    /// Any corners and intersections which were set will be overriden.
    pub const fn left(self, c: char) -> CustomStyle<T, B, On, R, IH, IV, H> {
        let mut style = self.inner;
        style.frame.left = Some(c);

        if let Some(mut top) = style.frame.top {
            top.left_corner = Some(c);
            style.frame.top = Some(top);
        }

        if let Some(mut bottom) = style.frame.bottom {
            bottom.left_corner = Some(c);
            style.frame.bottom = Some(bottom);
        }

        if let Some(mut split) = style.split {
            split.left_corner = Some(c);
            style.split = Some(split);
        }

        if let Some(mut split) = style.header_split_line {
            split.left_corner = Some(c);
            style.header_split_line = Some(split);
        }

        CustomStyle::new(style)
    }

    /// Sets a right border.
    ///
    /// Any corners and intersections which were set will be overriden.
    pub const fn right(mut self, c: char) -> CustomStyle<T, B, L, On, IH, IV, H> {
        self.inner.frame.right = Some(c);

        if let Some(mut top) = self.inner.frame.top {
            top.right_corner = Some(c);
            self.inner.frame.top = Some(top);
        }

        if let Some(mut bottom) = self.inner.frame.bottom {
            bottom.right_corner = Some(c);
            self.inner.frame.bottom = Some(bottom);
        }

        if let Some(mut split) = self.inner.split {
            split.right_corner = Some(c);
            self.inner.split = Some(split);
        }

        if let Some(mut split) = self.inner.header_split_line {
            split.right_corner = Some(c);
            self.inner.header_split_line = Some(split);
        }

        CustomStyle::new(self.inner)
    }

    /// Sets a horizontal split line.
    ///
    /// It doesn't include a header split line.
    /// It must be set via its own method [Self::header].
    ///
    /// Any corners and intersections which were set will be overriden.
    pub const fn horizontal(self, c: char) -> CustomStyle<T, B, L, R, On, IV, H> {
        let mut style = self.inner;
        let mut split = match style.split {
            Some(line) => line,
            None => Line {
                main: '\0',
                intersection: None,
                left_corner: None,
                right_corner: None,
            },
        };

        split.main = c;

        if style.frame.left.is_some() {
            split.left_corner = Some(c);
        }

        if style.frame.right.is_some() {
            split.right_corner = Some(c);
        }

        if style.inner_split_char.is_some() {
            split.intersection = Some(c);
        }

        style.split = Some(split);

        CustomStyle::new(style)
    }

    /// Sets a vertical split line.
    ///
    /// Any corners and intersections which were set will be overriden.
    pub const fn vertical(self, c: char) -> CustomStyle<T, B, L, R, IH, On, H> {
        let mut style = self.inner;
        style.inner_split_char = Some(c);

        if let Some(mut split) = style.split {
            split.intersection = Some(c);
            style.split = Some(split);
        }

        if let Some(mut top) = style.frame.top {
            top.intersection = Some(c);
            style.frame.top = Some(top);
        }

        if let Some(mut bottom) = style.frame.bottom {
            bottom.intersection = Some(c);
            style.frame.bottom = Some(bottom);
        }

        if let Some(mut split) = style.header_split_line {
            split.intersection = Some(c);
            style.header_split_line = Some(split);
        }

        CustomStyle::new(style)
    }

    /// Sets a 1st horizontal split line.
    ///
    /// Any corners and intersections which were set will be overriden.
    pub const fn header(self, c: char) -> CustomStyle<T, B, L, R, IH, IV, On> {
        let mut style = self.inner;
        let mut split = match style.header_split_line {
            Some(line) => line,
            None => Line {
                main: '\0',
                intersection: None,
                left_corner: None,
                right_corner: None,
            },
        };

        split.main = c;

        if style.frame.left.is_some() {
            split.left_corner = Some(c);
        }

        if style.frame.right.is_some() {
            split.right_corner = Some(c);
        }

        if style.inner_split_char.is_some() {
            split.intersection = Some(c);
        }

        style.header_split_line = Some(split);

        CustomStyle::new(style)
    }
}

impl<B, R, IH, IV, H> CustomStyle<On, B, On, R, IH, IV, H> {
    /// Sets a top left corner.
    pub const fn top_left_corner(self, c: char) -> Self {
        let mut style = self.inner;
        match style.frame.top {
            Some(mut top) => {
                top.left_corner = Some(c);
                style.frame.top = Some(top);
            }
            None => unreachable!(),
        }

        CustomStyle::new(style)
    }
}

impl<B, L, IH, IV, H> CustomStyle<On, B, L, On, IH, IV, H> {
    /// Sets a top right corner.
    pub const fn top_right_corner(self, c: char) -> Self {
        let mut style = self.inner;
        match style.frame.top {
            Some(mut top) => {
                top.right_corner = Some(c);
                style.frame.top = Some(top);
            }
            None => unreachable!(),
        }

        CustomStyle::new(style)
    }
}

impl<T, L, IH, IV, H> CustomStyle<T, On, L, On, IH, IV, H> {
    /// Sets a bottom right corner.
    pub const fn bottom_right_corner(self, c: char) -> Self {
        let mut style = self.inner;
        match style.frame.bottom {
            Some(mut bottom) => {
                bottom.right_corner = Some(c);
                style.frame.bottom = Some(bottom);
            }
            None => unreachable!(),
        }

        CustomStyle::new(style)
    }
}

impl<T, R, IH, IV, H> CustomStyle<T, On, On, R, IH, IV, H> {
    /// Sets a bottom left corner.
    pub const fn bottom_left_corner(self, c: char) -> Self {
        let mut style = self.inner;
        match style.frame.bottom {
            Some(mut bottom) => {
                bottom.left_corner = Some(c);
                style.frame.bottom = Some(bottom);
            }
            None => unreachable!(),
        }

        CustomStyle::new(style)
    }
}

impl<T, B, R, IV, H> CustomStyle<T, B, On, R, On, IV, H> {
    /// Sets a left intersection char.
    pub const fn left_intersection(self, c: char) -> Self {
        let mut style = self.inner;
        match style.split {
            Some(mut split) => {
                split.left_corner = Some(c);
                style.split = Some(split);
            }
            None => unreachable!(),
        }

        match style.header_split_line {
            Some(mut split) => {
                split.left_corner = Some(c);
                style.header_split_line = Some(split);
            }
            None => unreachable!(),
        }

        CustomStyle::new(style)
    }
}

impl<T, B, L, IV, H> CustomStyle<T, B, L, On, On, IV, H> {
    /// Sets a right intersection char.
    pub const fn right_intersection(self, c: char) -> Self {
        let mut style = self.inner;
        match style.split {
            Some(mut split) => {
                split.right_corner = Some(c);
                style.split = Some(split);
            }
            None => unreachable!(),
        }

        match style.header_split_line {
            Some(mut split) => {
                split.right_corner = Some(c);
                style.header_split_line = Some(split);
            }
            None => unreachable!(),
        }

        CustomStyle::new(style)
    }
}

impl<B, L, R, IH, H> CustomStyle<On, B, L, R, IH, On, H> {
    /// Sets a top intersection char.
    pub const fn top_intersection(self, c: char) -> Self {
        let mut style = self.inner;
        match style.frame.top {
            Some(mut top) => {
                top.intersection = Some(c);
                style.frame.top = Some(top);
            }
            None => unreachable!(),
        }

        CustomStyle::new(style)
    }
}

impl<T, L, R, IH, H> CustomStyle<T, On, L, R, IH, On, H> {
    /// Sets a bottom intersection char.
    pub const fn bottom_intersection(self, c: char) -> Self {
        let mut style = self.inner;
        match style.frame.bottom {
            Some(mut bottom) => {
                bottom.intersection = Some(c);
                style.frame.bottom = Some(bottom);
            }
            None => unreachable!(),
        }

        CustomStyle::new(style)
    }
}

impl<T, B, L, R, H> CustomStyle<T, B, L, R, On, On, H> {
    /// Sets an inner intersection char.
    /// A char between horizontal and vertical split lines.
    pub const fn inner_intersection(mut self, c: char) -> Self {
        match self.inner.split {
            Some(mut split) => {
                split.intersection = Some(c);
                self.inner.split = Some(split);
            }
            None => unreachable!(),
        }

        CustomStyle::new(self.inner)
    }
}

impl<T, B, L, R, IH> CustomStyle<T, B, L, R, IH, On, On> {
    /// Sets an intersection char of a 1st horizontal split line.
    pub const fn header_intersection(mut self, c: char) -> Self {
        match self.inner.header_split_line {
            Some(mut split) => {
                split.intersection = Some(c);
                self.inner.header_split_line = Some(split);
            }
            None => unreachable!(),
        }

        CustomStyle::new(self.inner)
    }
}

impl<T, B, R, IH, IV> CustomStyle<T, B, On, R, IH, IV, On> {
    /// Sets an left intersection char of a 1st horizontal split line.
    pub const fn left_header_intersection(self, c: char) -> Self {
        let mut style = self.inner;
        match style.header_split_line {
            Some(mut split) => {
                split.left_corner = Some(c);
                style.header_split_line = Some(split);
            }
            None => unreachable!(),
        }

        CustomStyle::new(style)
    }
}

impl<T, B, L, IH, IV> CustomStyle<T, B, L, On, IH, IV, On> {
    /// Sets an right intersection char of a 1st horizontal split line.
    pub const fn right_header_intersection(self, c: char) -> Self {
        let mut style = self.inner;
        match style.header_split_line {
            Some(mut split) => {
                split.right_corner = Some(c);
                style.header_split_line = Some(split);
            }
            None => unreachable!(),
        }

        CustomStyle::new(style)
    }
}

impl<B, L, R, IH, IV, H> CustomStyle<On, B, L, R, IH, IV, H> {
    /// Removes top border.
    pub const fn top_off(self) -> CustomStyle<(), B, L, R, IH, IV, H> {
        let mut style = self.inner;
        style.frame.top = None;
        CustomStyle::new(style)
    }
}

impl<T, L, R, IH, IV, H> CustomStyle<T, On, L, R, IH, IV, H> {
    /// Removes bottom border.
    pub const fn bottom_off(self) -> CustomStyle<T, (), L, R, IH, IV, H> {
        let mut style = self.inner;
        style.frame.bottom = None;
        CustomStyle::new(style)
    }
}

impl<T, B, R, IH, IV, H> CustomStyle<T, B, On, R, IH, IV, H> {
    /// Removes left border.
    pub const fn left_off(self) -> CustomStyle<T, B, (), R, IH, IV, H> {
        let mut style = self.inner;
        style.frame.left = None;

        if let Some(mut top) = style.frame.top {
            top.left_corner = None;
            style.frame.top = Some(top);
        }

        if let Some(mut bottom) = style.frame.bottom {
            bottom.left_corner = None;
            style.frame.bottom = Some(bottom);
        }

        if let Some(mut split) = style.split {
            split.left_corner = None;
            style.split = Some(split);
        }

        if let Some(mut split) = style.header_split_line {
            split.left_corner = None;
            style.header_split_line = Some(split);
        }

        CustomStyle::new(style)
    }
}

impl<T, B, L, IH, IV, H> CustomStyle<T, B, L, On, IH, IV, H> {
    /// Removes right border.
    pub const fn right_off(mut self) -> CustomStyle<T, B, L, (), IH, IV, H> {
        self.inner.frame.right = None;

        if let Some(mut top) = self.inner.frame.top {
            top.right_corner = None;
            self.inner.frame.top = Some(top);
        }

        if let Some(mut bottom) = self.inner.frame.bottom {
            bottom.right_corner = None;
            self.inner.frame.bottom = Some(bottom);
        }

        if let Some(mut split) = self.inner.split {
            split.right_corner = None;
            self.inner.split = Some(split);
        }

        if let Some(mut split) = self.inner.header_split_line {
            split.right_corner = None;
            self.inner.header_split_line = Some(split);
        }

        CustomStyle::new(self.inner)
    }
}

impl<T, B, L, R, IV, H> CustomStyle<T, B, L, R, On, IV, H> {
    /// Removes horizontal split lines.
    ///
    /// Not including 1st split line.
    pub const fn horizontal_off(mut self) -> CustomStyle<T, B, L, R, (), IV, H> {
        self.inner.split = None;
        CustomStyle::new(self.inner)
    }
}

impl<T, B, L, R, IH, H> CustomStyle<T, B, L, R, IH, On, H> {
    /// Removes vertical split lines.
    pub const fn vertical_off(mut self) -> CustomStyle<T, B, L, R, IH, (), H> {
        self.inner.inner_split_char = None;

        if let Some(mut split) = self.inner.split {
            split.intersection = None;
            self.inner.split = Some(split);
        }

        if let Some(mut top) = self.inner.frame.top {
            top.intersection = None;
            self.inner.frame.top = Some(top);
        }

        if let Some(mut bottom) = self.inner.frame.bottom {
            bottom.intersection = None;
            self.inner.frame.bottom = Some(bottom);
        }

        if let Some(mut split) = self.inner.header_split_line {
            split.intersection = None;
            self.inner.header_split_line = Some(split);
        }

        CustomStyle::new(self.inner)
    }
}

impl<T, B, L, R, IH, IV> CustomStyle<T, B, L, R, IH, IV, On> {
    /// Removes 1st horizontal split line.
    pub const fn header_off(mut self) -> CustomStyle<T, B, L, R, IH, IV, ()> {
        self.inner.header_split_line = None;
        CustomStyle::new(self.inner)
    }
}

impl<T, B, L, R, IH, IV, H> TableOption for CustomStyle<T, B, L, R, IH, IV, H> {
    fn change(&mut self, grid: &mut Grid) {
        self.inner.change(grid);
    }
}

/// Border represents a style of a CellBorder.
///
/// ```rust,no_run
///   # use tabled::{style::{Style, Border}, Row, Table, Modify};
///   # let data: Vec<&'static str> = Vec::new();
///     let table = Table::new(&data)
///         .with(Style::ascii())
///         .with(Modify::new(Row(..1)).with(Border::default().top('x')));
/// ```
pub use papergrid::Border;

impl CellOption for Border {
    fn change_cell(&mut self, grid: &mut Grid, row: usize, column: usize) {
        Highlight::cell(row, column, self.clone()).change(grid);
    }
}
