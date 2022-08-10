use papergrid::{records::Records, AnsiColor, Border, Entity};

use crate::{symbol::Symbol, CellOption, Table};

/// BorderColored represents a colored border of a Cell.
///
/// ```rust,no_run
/// # use owo_colors::OwoColorize;
/// # use tabled::{style::{Style, Symbol, BorderColored}, object::Rows, Table, Modify};
/// #
/// # let data: Vec<&'static str> = Vec::new();
/// #
/// let c = Symbol::ansi("#".red().to_string()).unwrap();
/// let table = Table::new(&data)
///     .with(Style::ascii())
///     .with(Modify::new(Rows::single(0)).with(BorderColored::default().top(c)));
/// ```
#[cfg_attr(docsrs, doc(cfg(feature = "color")))]
#[derive(Debug, Clone, Default, Eq, PartialEq)]
pub struct BorderColored(papergrid::Border<Symbol>);

impl BorderColored {
    /// Set a top border character.
    pub fn top(mut self, c: Symbol) -> Self {
        self.0.top = Some(c);
        self
    }

    /// Set a bottom border character.
    pub fn bottom(mut self, c: Symbol) -> Self {
        self.0.bottom = Some(c);
        self
    }

    /// Set a left border character.
    pub fn left(mut self, c: Symbol) -> Self {
        self.0.left = Some(c);
        self
    }

    /// Set a right border character.
    pub fn right(mut self, c: Symbol) -> Self {
        self.0.right = Some(c);
        self
    }

    /// Set a top left intersection character.
    pub fn top_left_corner(mut self, c: Symbol) -> Self {
        self.0.left_top_corner = Some(c);
        self
    }

    /// Set a top right intersection character.
    pub fn top_right_corner(mut self, c: Symbol) -> Self {
        self.0.right_top_corner = Some(c);
        self
    }

    /// Set a bottom left intersection character.
    pub fn bottom_left_corner(mut self, c: Symbol) -> Self {
        self.0.left_bottom_corner = Some(c);
        self
    }

    /// Set a bottom right intersection character.
    pub fn bottom_right_corner(mut self, c: Symbol) -> Self {
        self.0.right_bottom_corner = Some(c);
        self
    }

    /// This function constructs a cell borders with all sides's char set to a given character.
    /// It behaives like [Border::new] with the same character set to each side.
    pub fn filled(c: Symbol) -> Self {
        Self(papergrid::Border {
            top: Some(c.clone()),
            bottom: Some(c.clone()),
            left: Some(c.clone()),
            right: Some(c.clone()),
            left_bottom_corner: Some(c.clone()),
            left_top_corner: Some(c.clone()),
            right_bottom_corner: Some(c.clone()),
            right_top_corner: Some(c),
        })
    }
}

impl<R> CellOption<R> for BorderColored
where
    for<'a> &'a R: Records,
{
    fn change_cell(&mut self, table: &mut Table<R>, entity: Entity) {
        let (border, color) = split_border_colored(self);

        let (count_rows, count_cols) = table.shape();
        for pos in entity.iter(count_rows, count_cols) {
            let cfg = table.get_config_mut();
            cfg.set_border_color(pos, color.clone());
            cfg.set_border(pos, border.clone());
        }
    }
}

impl From<BorderColored> for Border<AnsiColor> {
    fn from(val: BorderColored) -> Self {
        let border = val;
        let mut b = Border::default();
        if let Some(s) = &border.0.left {
            b.left = s.color().cloned().map(AnsiColor::from);
        }

        if let Some(s) = &border.0.right {
            b.right = s.color().cloned().map(AnsiColor::from);
        }

        if let Some(s) = &border.0.top {
            b.top = s.color().cloned().map(AnsiColor::from);
        }

        if let Some(s) = &border.0.bottom {
            b.bottom = s.color().cloned().map(AnsiColor::from);
        }

        if let Some(s) = &border.0.left_top_corner {
            b.left_top_corner = s.color().cloned().map(AnsiColor::from);
        }

        if let Some(s) = &border.0.right_top_corner {
            b.right_top_corner = s.color().cloned().map(AnsiColor::from);
        }

        if let Some(s) = &border.0.left_bottom_corner {
            b.left_bottom_corner = s.color().cloned().map(AnsiColor::from);
        }

        if let Some(s) = &border.0.right_bottom_corner {
            b.right_bottom_corner = s.color().cloned().map(AnsiColor::from);
        }

        b
    }
}

fn split_border_colored(b: &BorderColored) -> (Border<char>, Border<AnsiColor>) {
    let mut border = Border::default();
    let mut color: Border<AnsiColor> = Border::default();

    if let Some(s) = &b.0.left {
        border.left = Some(s.c());
        color.left = s.color().cloned().map(|c| c.into());
    }

    if let Some(s) = &b.0.right {
        border.right = Some(s.c());
        color.right = s.color().cloned().map(|c| c.into());
    }

    if let Some(s) = &b.0.top {
        border.top = Some(s.c());
        color.top = s.color().cloned().map(|c| c.into());
    }

    if let Some(s) = &b.0.bottom {
        border.bottom = Some(s.c());
        color.bottom = s.color().cloned().map(|c| c.into());
    }

    if let Some(s) = &b.0.left_top_corner {
        border.left_top_corner = Some(s.c());
        color.left_top_corner = s.color().cloned().map(|c| c.into());
    }

    if let Some(s) = &b.0.right_top_corner {
        border.right_top_corner = Some(s.c());
        color.right_top_corner = s.color().cloned().map(|c| c.into());
    }

    if let Some(s) = &b.0.left_bottom_corner {
        border.left_bottom_corner = Some(s.c());
        color.left_bottom_corner = s.color().cloned().map(|c| c.into());
    }

    if let Some(s) = &b.0.right_bottom_corner {
        border.right_bottom_corner = Some(s.c());
        color.right_bottom_corner = s.color().cloned().map(|c| c.into());
    }

    (border, color)
}
