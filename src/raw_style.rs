//! This module contains [`RawStyle`] structure, which is analogues to [`Style`] but not generic,
//! so sometimes it can be used more conviently.

use std::collections::HashMap;

use papergrid::{records::Records, Borders};

use crate::{style::Line, Style, Table, TableOption};

/// A raw style data, which can be produced safely from [`Style`].
///
/// It can be useful in order to not have a generics and be able to use it as a variable more conveniently.
#[derive(Debug, Clone)]
pub struct RawStyle {
    borders: Borders<char>,
    lines: HashMap<usize, Line>,
}

impl RawStyle {
    /// Set a top border character.
    pub fn set_top(&mut self, s: Option<char>) -> &mut Self {
        self.borders.top = s;
        self
    }

    /// Set a bottom border character.
    pub fn set_bottom(&mut self, s: Option<char>) -> &mut Self {
        self.borders.bottom = s;
        self
    }

    /// Set a left border character.
    pub fn set_left(&mut self, s: Option<char>) -> &mut Self {
        self.borders.vertical_left = s;
        self
    }

    /// Set a right border character.
    pub fn set_right(&mut self, s: Option<char>) -> &mut Self {
        self.borders.vertical_right = s;
        self
    }

    /// Set a top split border character.
    pub fn set_top_split(&mut self, s: Option<char>) -> &mut Self {
        self.borders.top_intersection = s;
        self
    }

    /// Set a bottom split character.
    pub fn set_bottom_split(&mut self, s: Option<char>) -> &mut Self {
        self.borders.bottom_intersection = s;
        self
    }

    /// Set a left split character.
    pub fn set_left_split(&mut self, s: Option<char>) -> &mut Self {
        self.borders.horizontal_left = s;
        self
    }

    /// Set a right split character.
    pub fn set_right_split(&mut self, s: Option<char>) -> &mut Self {
        self.borders.horizontal_right = s;
        self
    }

    /// Set an internal character.
    pub fn set_internal_split(&mut self, s: Option<char>) -> &mut Self {
        self.borders.intersection = s;
        self
    }

    /// Set a vertical character.
    pub fn set_vertical(&mut self, s: Option<char>) -> &mut Self {
        self.borders.vertical = s;
        self
    }

    /// Set a horizontal character.
    pub fn set_horizontal(&mut self, s: Option<char>) -> &mut Self {
        self.borders.horizontal = s;
        self
    }

    /// Set a character for a top left corner.
    pub fn set_top_left(&mut self, s: Option<char>) -> &mut Self {
        self.borders.top_left = s;
        self
    }

    /// Set a character for a top right corner.
    pub fn set_top_right(&mut self, s: Option<char>) -> &mut Self {
        self.borders.top_right = s;
        self
    }

    /// Set a character for a bottom left corner.
    pub fn set_bottom_left(&mut self, s: Option<char>) -> &mut Self {
        self.borders.bottom_left = s;
        self
    }

    /// Set a character for a bottom right corner.
    pub fn set_bottom_right(&mut self, s: Option<char>) -> &mut Self {
        self.borders.bottom_right = s;
        self
    }

    /// Set border lines.
    ///
    /// # Example
    ///
    /// ```
    /// use std::collections::HashMap;
    /// use tabled::{style::{Style, Line}, raw_style::RawStyle, TableIteratorExt};
    ///
    /// let mut style = RawStyle::from(Style::re_structured_text());
    ///
    /// let mut lines = HashMap::new();
    /// lines.insert(1, Style::extended().get_horizontal());
    /// style.set_lines(lines);
    ///
    /// let table = (0..3)
    ///    .map(|i| ("Hello", i))
    ///    .table()
    ///    .with(style)
    ///    .to_string();
    ///
    /// assert_eq!(
    ///     table,
    ///     concat!(
    ///         " ======= ===== \n",
    ///         "  &str    i32  \n",
    ///         "╠═══════╬═════╣\n",
    ///         "  Hello   0    \n",
    ///         "  Hello   1    \n",
    ///         "  Hello   2    \n",
    ///         " ======= ===== ",
    ///     ),
    /// )
    /// ```
    pub fn set_lines(&mut self, lines: HashMap<usize, Line>) -> &mut Self {
        self.lines = lines;
        self
    }
}

impl From<Borders<char>> for RawStyle {
    fn from(borders: Borders<char>) -> Self {
        Self {
            borders,
            lines: HashMap::new(),
        }
    }
}

#[cfg(feature = "color")]
impl RawStyle {
    /// Returns a [`RawStyle`] version which can set colors.
    #[cfg_attr(docsrs, doc(cfg(feature = "color")))]
    pub fn colored(self) -> crate::raw_style_colored::RawStyleColored {
        crate::raw_style_colored::RawStyleColored::from(self)
    }
}

impl<R> TableOption<R> for RawStyle
where
    R: Records,
{
    fn change(&mut self, table: &mut Table<R>) {
        table.get_config_mut().clear_theme();
        table.get_config_mut().set_borders(self.borders.clone());

        if table.shape().0 > 1 {
            for (&row, line) in &self.lines {
                if line.is_empty() {
                    table.get_config_mut().remove_split_line(row);
                } else {
                    table
                        .get_config_mut()
                        .set_split_line(row, line.clone().into());
                }
            }
        }
    }
}

impl<T, B, L, R, H, V, Lines> From<Style<T, B, L, R, H, V, Lines>> for RawStyle
where
    Lines: IntoIterator<Item = (usize, Line)>,
{
    fn from(style: Style<T, B, L, R, H, V, Lines>) -> Self {
        Self {
            borders: style.borders,
            lines: style.lines.into_iter().collect(),
        }
    }
}
