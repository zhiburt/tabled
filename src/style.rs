use std::{borrow::Cow, marker::PhantomData};

#[allow(unused)]
use crate::Table;
use crate::TableOption;
use papergrid::{Border, Entity, Grid, Settings};

/// Style is responsible for a look of a [Table].
///
/// # Example
///
/// ```rust,no_run
/// use tabled::{Table, Style, style::Line};
/// let data = vec!["Hello", "2021"];
/// let table = Table::new(&data).with(
///                 Style::NO_BORDER
///                     .frame_bottom(Some(Line::short('*', ' ')))
///                     .split(Some(Line::short('*', ' ')))
///                     .inner(' ')
///             )
///             .to_string();
///
/// println!("{}", table);
/// ```
#[derive(Debug, Clone)]
pub struct Style {
    frame: Frame,
    header_split_line: Option<Line>,
    split: Option<Line>,
    inner_split_char: Option<char>,
}

impl Style {
    pub const EMPTY: CustomStyle<(), (), (), (), (), (), ()> = CustomStyle::empty();

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
    pub const ASCII: CustomStyle<
        TopLine,
        BottomLine,
        LeftLine,
        RightLine,
        InnerHorizontalLine,
        InnerVerticalLine,
        HeaderLine,
    > = CustomStyle::new(Self::new(
        Frame {
            bottom: Some(Line::bordered('-', '+', '+', '+')),
            top: Some(Line::bordered('-', '+', '+', '+')),
            left: Some('|'),
            right: Some('|'),
        },
        Some(Line::bordered('-', '+', '+', '+')),
        Some(Line::bordered('-', '+', '+', '+')),
        Some('|'),
    ));

    /// Noborder style looks like the following table
    ///
    /// ```text
    ///      id   destribution             link
    ///      0       Fedora       https://getfedora.org/
    ///      2      OpenSUSE     https://www.opensuse.org/
    ///      3    Endeavouros    https://endeavouros.com/
    /// ```
    #[deprecated(note = "Renamed to BLANK")]
    pub const NO_BORDER: Self = Self::new(Frame::empty(), None, None, Some(' '));

    /// Noborder style looks like the following table
    ///
    /// ```text
    ///      id   destribution             link
    ///      0       Fedora       https://getfedora.org/
    ///      2      OpenSUSE     https://www.opensuse.org/
    ///      3    Endeavouros    https://endeavouros.com/
    /// ```
    pub const BLANK: Self = Self::new(Frame::empty(), None, None, Some(' '));

    /// Psql style looks like the following table
    ///
    /// ```text
    ///      id | destribution |           link
    ///     ----+--------------+---------------------------
    ///      0  |    Fedora    |  https://getfedora.org/
    ///      2  |   OpenSUSE   | https://www.opensuse.org/
    ///      3  | Endeavouros  | https://endeavouros.com/
    /// ```
    pub const PSQL: Self = Self::new(Frame::empty(), Some(Line::short('-', '+')), None, Some('|'));

    /// Github_markdown style looks like the following table
    ///
    /// ```text
    ///     | id | destribution |           link            |
    ///     |----+--------------+---------------------------|
    ///     | 0  |    Fedora    |  https://getfedora.org/   |
    ///     | 2  |   OpenSUSE   | https://www.opensuse.org/ |
    ///     | 3  | Endeavouros  | https://endeavouros.com/  |
    /// ```
    pub const GITHUB_MARKDOWN: Self = Self::new(
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

    /// Pseudo style looks like the following table
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
    pub const PSEUDO: Self = Self::new(
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

    /// Pseudo_clean style looks like the following table
    ///
    /// ```text
    ///     ┌────┬──────────────┬───────────────────────────┐
    ///     │ id │ destribution │           link            │
    ///     ├────┼──────────────┼───────────────────────────┤
    ///     │ 0  │    Fedora    │  https://getfedora.org/   │
    ///     │ 2  │   OpenSUSE   │ https://www.opensuse.org/ │
    ///     │ 3  │ Endeavouros  │ https://endeavouros.com/  │
    ///     └────┴──────────────┴───────────────────────────┘
    /// ```
    pub const PSEUDO_CLEAN: Self = Self::new(
        Frame {
            left: Some('│'),
            right: Some('│'),
            bottom: Some(Line::bordered('─', '┴', '└', '┘')),
            top: Some(Line::bordered('─', '┬', '┌', '┐')),
        },
        Some(Line::bordered('─', '┼', '├', '┤')),
        None,
        Some('│'),
    );

    /// Extended style looks like the following table
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
    pub const EXTENDED: Self = Self::new(
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

    /// ASCII Dots style looks like the following table
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
    pub const ASCII_DOTS: Self = Self::new(
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
    pub const RE_STRUCTURED_TEXT: Self = Self::new(
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

    // #[deprecated(note = "The name is not explicit. Use ASCII constant instead.")]
    // pub fn default() -> Self {
    //     Self::ASCII
    // }

    // #[deprecated(note = "The name is not explicit. Use ASCII constant instead.")]
    // pub fn ascii() -> Self {
    //     Self::ASCII
    // }

    #[deprecated(note = "The name is not explicit. Use NO_BORDER constant instead.")]
    pub fn noborder() -> Self {
        Self::BLANK
    }

    #[deprecated(note = "The name is not explicit. Use PSQL constant instead.")]
    pub fn psql() -> Self {
        Self::PSQL
    }

    #[deprecated(note = "The name is not explicit. Use GITHUB_MARKDOWN constant instead.")]
    pub fn github_markdown() -> Self {
        Self::GITHUB_MARKDOWN
    }

    #[deprecated(note = "The name is not explicit. Use PSEUDO constant instead.")]
    pub fn pseudo() -> Self {
        Self::PSEUDO
    }

    #[deprecated(note = "The name is not explicit. Use PSEUDO_CLEAN constant instead.")]
    pub fn pseudo_clean() -> Self {
        Self::PSEUDO_CLEAN
    }

    #[deprecated(note = "Use EXTENDED constant instead.")]
    pub fn extended() -> Self {
        Self::EXTENDED
    }

    /// Left frame character.
    pub fn frame_left(mut self, frame: Option<char>) -> Self {
        self.frame.left = frame;
        self
    }

    /// Right frame character.
    pub fn frame_right(mut self, frame: Option<char>) -> Self {
        self.frame.right = frame;
        self
    }

    /// The header's top line.
    ///
    /// It's suppose that [Self::frame_bottom] and [Self::split]  has the same type of [Line] short or bordered.  
    pub fn frame_top(mut self, frame: Option<Line>) -> Self {
        self.frame.top = frame;
        self
    }

    /// The footer's bottom line.
    ///
    /// It's suppose that [Self::frame_top] and [Self::split] has the same type of [Line] short or bordered.
    pub fn frame_bottom(mut self, frame: Option<Line>) -> Self {
        self.frame.bottom = frame;
        self
    }

    /// The header's bottom line.
    pub fn header(mut self, line: Option<Line>) -> Self {
        self.header_split_line = line;
        self
    }

    /// Row split line.
    ///
    /// [Self::frame_top] and [Self::frame_bottom]
    pub fn split(mut self, line: Option<Line>) -> Self {
        self.header_split_line = line.clone();
        self.split = line;
        self
    }

    /// Inner split character.
    pub fn inner(mut self, c: char) -> Self {
        self.inner_split_char = Some(c);
        self
    }

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
pub struct Line {
    main: char,
    intersection: Option<char>,
    left_corner: Option<char>,
    right_corner: Option<char>,
}

impl Line {
    /// A line for frame styles.
    pub const fn bordered(main: char, intersection: char, left: char, right: char) -> Self {
        Self {
            main,
            intersection: Some(intersection),
            left_corner: Some(left),
            right_corner: Some(right),
        }
    }

    /// A line for no-frame styles.
    pub const fn short(main: char, intersection: char) -> Self {
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

impl TableOption for Style {
    fn change(&mut self, grid: &mut Grid) {
        grid.clear_split_grid();
        grid.clear_overide_split_lines();

        let count_rows = grid.count_rows();
        let count_columns = grid.count_columns();
        for row in 0..count_rows {
            for column in 0..count_columns {
                let border = make_style(self, row, column, count_rows, count_columns);

                grid.set(
                    &Entity::Cell(row, column),
                    Settings::default().border(border).border_restriction(false),
                );
            }
        }
    }
}

fn make_style(
    style: &Style,
    row: usize,
    column: usize,
    count_rows: usize,
    count_columns: usize,
) -> Border {
    let is_first_row = row == 0;
    let is_last_row = row + 1 == count_rows;
    let is_first_column = column == 0;
    let is_last_column = column + 1 == count_columns;

    let mut border = match (is_first_row, is_last_row, is_first_column, is_last_column) {
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
            bottom: style.header_split_line.as_ref().map(|l| l.main),
            left: style.frame.left,
            left_top_corner: style.frame.top.as_ref().and_then(|l| l.left_corner),
            left_bottom_corner: style.header_split_line.as_ref().and_then(|l| l.left_corner),
            right: style.frame.right,
            right_top_corner: style.frame.top.as_ref().and_then(|l| l.right_corner),
            right_bottom_corner: style
                .header_split_line
                .as_ref()
                .and_then(|l| l.right_corner),
        },
        (false, true, true, true) => Border {
            top: style.frame.bottom.as_ref().map(|l| l.main),
            bottom: style.frame.bottom.as_ref().map(|l| l.main),
            left: style.frame.left,
            left_top_corner: style.frame.bottom.as_ref().and_then(|l| l.left_corner),
            left_bottom_corner: style.frame.bottom.as_ref().and_then(|l| l.left_corner),
            right: style.frame.right,
            right_top_corner: style.frame.bottom.as_ref().and_then(|l| l.right_corner),
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
            bottom: style.header_split_line.as_ref().map(|l| l.main),
            left: style.frame.left,
            left_top_corner: style.frame.top.as_ref().and_then(|l| l.left_corner),
            left_bottom_corner: style.header_split_line.as_ref().and_then(|l| l.left_corner),
            right: style.inner_split_char,
            right_top_corner: style.frame.top.as_ref().and_then(|l| l.intersection),
            right_bottom_corner: style
                .header_split_line
                .as_ref()
                .and_then(|l| l.intersection),
        },
        (true, false, false, true) => Border {
            top: style.frame.top.as_ref().map(|l| l.main),
            bottom: style.header_split_line.as_ref().map(|l| l.main),
            left: style.inner_split_char,
            left_top_corner: style.frame.top.as_ref().and_then(|l| l.intersection),
            left_bottom_corner: style
                .header_split_line
                .as_ref()
                .and_then(|l| l.intersection),
            right: style.frame.right,
            right_top_corner: style.frame.top.as_ref().and_then(|l| l.right_corner),
            right_bottom_corner: style
                .header_split_line
                .as_ref()
                .and_then(|l| l.right_corner),
        },
        (true, false, false, false) => Border {
            top: style.frame.top.as_ref().map(|l| l.main),
            bottom: style.header_split_line.as_ref().map(|l| l.main),
            left: style.inner_split_char,
            left_top_corner: style.frame.top.as_ref().and_then(|l| l.intersection),
            left_bottom_corner: style
                .header_split_line
                .as_ref()
                .and_then(|l| l.intersection),
            right: style.inner_split_char,
            right_top_corner: style.frame.top.as_ref().and_then(|l| l.intersection),
            right_bottom_corner: style
                .header_split_line
                .as_ref()
                .and_then(|l| l.intersection),
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
    };

    // handle header
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
            border.left_top_corner = style.header_split_line.as_ref().and_then(|l| l.left_corner);
        } else {
            border.left_top_corner = style
                .header_split_line
                .as_ref()
                .and_then(|l| l.intersection);
        }
    }

    border
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

#[derive(Debug, Clone)]
pub struct CustomStyle<L, R, T, B, IH, IV, H> {
    inner: Style,
    _l_border: PhantomData<L>,
    _r_border: PhantomData<R>,
    _t_border: PhantomData<T>,
    _b_border: PhantomData<B>,
    _i_h_border: PhantomData<IH>,
    _i_v_border: PhantomData<IV>,
    _h_border: PhantomData<H>,
}

#[derive(Debug, Clone)]
pub struct LeftLine;
#[derive(Debug, Clone)]
pub struct RightLine;
#[derive(Debug, Clone)]
pub struct TopLine;
#[derive(Debug, Clone)]
pub struct BottomLine;
#[derive(Debug, Clone)]
pub struct InnerVerticalLine;
#[derive(Debug, Clone)]
pub struct InnerHorizontalLine;
#[derive(Debug, Clone)]
pub struct HeaderLine;

impl<T, B, L, R, IH, IV, H> CustomStyle<T, B, L, R, IH, IV, H> {
    const fn new(style: Style) -> Self {
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

impl CustomStyle<(), (), (), (), (), (), ()> {
    const fn empty() -> Self {
        Self {
            inner: Style::new(Frame::empty(), None, None, None),
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
    pub const fn top(self, c: char) -> CustomStyle<TopLine, B, L, R, IH, IV, H> {
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

    pub const fn bottom(self, c: char) -> CustomStyle<T, BottomLine, L, R, IH, IV, H> {
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

    pub const fn left(self, c: char) -> CustomStyle<T, B, LeftLine, R, IH, IV, H> {
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

    pub const fn right(mut self, c: char) -> CustomStyle<T, B, L, RightLine, IH, IV, H> {
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

    pub const fn horizontal(self, c: char) -> CustomStyle<T, B, L, R, InnerHorizontalLine, IV, H> {
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

        if style.header_split_line.is_none() {
            style.header_split_line = Some(Line {
                intersection: split.intersection,
                left_corner: split.left_corner,
                right_corner: split.right_corner,
                main: split.main,
            });
        }

        style.split = Some(split);

        CustomStyle::new(style)
    }

    pub const fn vertical(self, c: char) -> CustomStyle<T, B, L, R, IH, InnerVerticalLine, H> {
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

    pub const fn header(self, c: char) -> CustomStyle<T, B, L, R, IH, IV, HeaderLine> {
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

impl<B, R, IH, IV, H> CustomStyle<TopLine, B, LeftLine, R, IH, IV, H> {
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

impl<B, L, IH, IV, H> CustomStyle<TopLine, B, L, RightLine, IH, IV, H> {
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

impl<T, L, IH, IV, H> CustomStyle<T, BottomLine, L, RightLine, IH, IV, H> {
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

impl<T, R, IH, IV, H> CustomStyle<T, BottomLine, LeftLine, R, IH, IV, H> {
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

impl<T, B, R, IV, H> CustomStyle<T, B, LeftLine, R, InnerHorizontalLine, IV, H> {
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

impl<T, B, L, IV, H> CustomStyle<T, B, L, RightLine, InnerHorizontalLine, IV, H> {
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

impl<B, L, R, IH, H> CustomStyle<TopLine, B, L, R, IH, InnerVerticalLine, H> {
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

impl<T, L, R, IH, H> CustomStyle<T, BottomLine, L, R, IH, InnerVerticalLine, H> {
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

impl<B, L, R, IH, IV, H> CustomStyle<TopLine, B, L, R, IH, IV, H> {
    pub const fn top_off(self) -> CustomStyle<(), B, L, R, IH, IV, H> {
        let mut style = self.inner;
        style.frame.top = None;
        CustomStyle::new(style)
    }
}

impl<T, L, R, IH, IV, H> CustomStyle<T, BottomLine, L, R, IH, IV, H> {
    pub const fn bottom_off(self) -> CustomStyle<T, (), L, R, IH, IV, H> {
        let mut style = self.inner;
        style.frame.bottom = None;
        CustomStyle::new(style)
    }
}

impl<T, B, R, IH, IV, H> CustomStyle<T, B, LeftLine, R, IH, IV, H> {
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

impl<T, B, L, IH, IV, H> CustomStyle<T, B, L, RightLine, IH, IV, H> {
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

impl<T, B, L, R, IV, H> CustomStyle<T, B, L, R, InnerHorizontalLine, IV, H> {
    pub const fn horizontal_off(mut self) -> CustomStyle<T, B, L, R, (), IV, H> {
        self.inner.split = None;
        self.inner.header_split_line = None;
        CustomStyle::new(self.inner)
    }
}

impl<T, B, L, R, IH, H> CustomStyle<T, B, L, R, IH, InnerVerticalLine, H> {
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

impl<T, B, L, R, IH, IV> CustomStyle<T, B, L, R, IH, IV, HeaderLine> {
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
