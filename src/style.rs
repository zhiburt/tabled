#[allow(unused)]
use crate::Table;
use crate::TableOption;
use papergrid::{Border, Grid};

/// Style is responsible for a look of a [Table].
///
/// # Example
///
/// ```rust,no_run
/// use tabled::{Table, Style, style::Line};
/// let data = vec!["Hello", "2021"];
/// let table = Table::new(&data).with(
///                 Style::noborder()
///                     .frame_bottom(Some(Line::short('*', ' ')))
///                     .split(Some(Line::short('*', ' ')))
///                     .inner(' ')
///             )
///             .to_string();
///
/// println!("{}", table);
/// ```
pub struct Style {
    frame: Frame,
    header_split_line: Option<Line>,
    split: Option<Line>,
    inner_split_char: char,
}

impl Style {
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
    pub fn default() -> Self {
        let line = Line::bordered('-', '+', '+', '+');

        Self::new(
            Frame {
                bottom: Some(line.clone()),
                top: Some(line.clone()),
                left: Some('|'),
                right: Some('|'),
            },
            Some(line.clone()),
            Some(line),
            '|',
        )
    }

    /// Noborder style looks like the following table
    ///
    /// ```text
    ///      id   destribution             link
    ///      0       Fedora       https://getfedora.org/
    ///      2      OpenSUSE     https://www.opensuse.org/
    ///      3    Endeavouros    https://endeavouros.com/
    /// ```
    pub fn noborder() -> Self {
        Self::new(Frame::default(), None, None, ' ')
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
    pub fn psql() -> Self {
        Self::new(Frame::default(), Some(Line::short('-', '+')), None, '|')
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
    pub fn github_markdown() -> Self {
        Self::new(
            Frame {
                left: Some('|'),
                right: Some('|'),
                ..Default::default()
            },
            Some(Line::bordered('-', '+', '|', '|')),
            None,
            '|',
        )
    }
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
    pub fn pseudo() -> Self {
        Self::new(
            Frame {
                left: Some('│'),
                right: Some('│'),
                bottom: Some(Line::bordered('─', '┴', '└', '┘')),
                top: Some(Line::bordered('─', '┬', '┌', '┐')),
            },
            Some(Line::bordered('─', '┼', '├', '┤')),
            Some(Line::bordered('─', '┼', '├', '┤')),
            '│',
        )
    }

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
    pub fn pseudo_clean() -> Self {
        let mut pseudo = Self::pseudo();
        pseudo.split = None;
        pseudo
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
        self.inner_split_char = c;
        self
    }

    fn new(frame: Frame, header: Option<Line>, split: Option<Line>, inner: char) -> Self {
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
    intersection: char,
    left_corner: Option<char>,
    right_corner: Option<char>,
}

impl Line {
    /// A line for frame styles.
    pub fn bordered(main: char, intersection: char, left: char, right: char) -> Self {
        Self {
            intersection,
            main,
            left_corner: Some(left),
            right_corner: Some(right),
        }
    }

    /// A line for no-frame styles.
    pub fn short(main: char, intersection: char) -> Self {
        Self {
            main,
            intersection,
            ..Default::default()
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

impl TableOption for Style {
    fn change(&self, grid: &mut Grid) {
        let count_rows = grid.count_rows();
        for row in 0..count_rows {
            let border = grid.get_border_mut(row);
            make_style(self, border, row == 0, row == count_rows - 1);
        }
    }
}

fn make_style(style: &Style, border: &mut Border, is_first_row: bool, is_last_row: bool) {
    let border = border.empty();

    if is_first_row {
        if let Some(line) = &style.frame.top {
            border.top(
                line.main,
                line.intersection,
                line.left_corner,
                line.right_corner,
            );
        }

        if let Some(line) = &style.header_split_line {
            border.bottom(
                line.main,
                line.intersection,
                line.left_corner,
                line.right_corner,
            );
        }
    } else if is_last_row {
        if let Some(line) = &style.frame.bottom {
            border.bottom(
                line.main,
                line.intersection,
                line.left_corner,
                line.right_corner,
            );
        }
    } else if let Some(line) = &style.split {
        border.bottom(
            line.main,
            line.intersection,
            line.left_corner,
            line.right_corner,
        );
    }

    border.inner(
        Some(style.inner_split_char),
        style.frame.left,
        style.frame.right,
    );
}
