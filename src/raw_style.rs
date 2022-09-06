//! This module contains [`RawStyle`] structure, which is analogues to [`Style`] but not generic,
//! so sometimes it can be used more conviently.

use std::collections::HashMap;

use papergrid::{records::Records, Borders};

use crate::{
    style::{HorizontalLine, Line, VerticalLine},
    Style, Table, TableOption,
};

/// A raw style data, which can be produced safely from [`Style`].
///
/// It can be useful in order to not have a generics and be able to use it as a variable more conveniently.
#[derive(Default, Debug, Clone)]
pub struct RawStyle {
    borders: Borders<char>,
    horizontals: HashMap<usize, Line>,
    verticals: HashMap<usize, Line>,
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

    /// Set horizontal border lines.
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
    /// style.set_horizontals(lines);
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
    pub fn set_horizontals(&mut self, lines: HashMap<usize, Line>) -> &mut Self {
        self.horizontals = lines;
        self
    }

    /// Set vertical border lines.
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
    /// style.set_verticals(lines);
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
    ///         "=======╠=====\n",
    ///         " &str  ═ i32 \n",
    ///         "======= =====\n",
    ///         " Hello ═ 0   \n",
    ///         " Hello ═ 1   \n",
    ///         " Hello ═ 2   \n",
    ///         "=======╣=====",
    ///     ),
    /// )
    /// ```
    pub fn set_verticals(&mut self, lines: HashMap<usize, Line>) -> &mut Self {
        self.verticals = lines;
        self
    }

    /// Returns a [`RawStyle`] version which can set colors.
    #[cfg_attr(docsrs, doc(cfg(feature = "color")))]
    #[cfg(feature = "color")]
    pub fn colored(self) -> crate::raw_style_colored::RawStyleColored {
        crate::raw_style_colored::RawStyleColored::from(self)
    }
}

impl From<Borders<char>> for RawStyle {
    fn from(borders: Borders<char>) -> Self {
        Self {
            borders,
            horizontals: HashMap::new(),
            verticals: HashMap::new(),
        }
    }
}

impl<R> TableOption<R> for RawStyle
where
    R: Records,
{
    fn change(&mut self, table: &mut Table<R>) {
        if table.is_empty() {
            return;
        }

        let (count_rows, count_cols) = table.shape();

        let cfg = table.get_config_mut();
        cfg.clear_theme();
        cfg.set_borders(self.borders.clone());

        if count_rows > 1 {
            for (&row, line) in &self.horizontals {
                if line.is_empty() {
                    cfg.remove_horizontal_line(row);
                } else {
                    cfg.set_horizontal_line(row, papergrid::HorizontalLine::from(*line));
                }
            }
        }

        if count_cols > 1 {
            for (&col, line) in &self.verticals {
                if line.is_empty() {
                    cfg.remove_vertical_line(col);
                } else {
                    cfg.set_vertical_line(col, papergrid::VerticalLine::from(*line));
                }
            }
        }
        
        table.destroy_width_cache();
    }
}

impl<T, B, L, R, H, V, HLines, VLines> From<Style<T, B, L, R, H, V, HLines, VLines>> for RawStyle
where
    HLines: IntoIterator<Item = HorizontalLine>,
    VLines: IntoIterator<Item = VerticalLine>,
{
    fn from(style: Style<T, B, L, R, H, V, HLines, VLines>) -> Self {
        let horizontals = style
            .horizontals
            .into_iter()
            .flat_map(|hr| {
                let index = hr.index;
                hr.line.map(|line| (index, line))
            })
            .collect();

        let verticals = style
            .verticals
            .into_iter()
            .flat_map(|hr| {
                let index = hr.index;
                hr.line.map(|line| (index, line))
            })
            .collect();

        Self {
            borders: style.borders,
            horizontals,
            verticals,
        }
    }
}
