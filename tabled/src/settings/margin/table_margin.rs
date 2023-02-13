use crate::{
    grid::config::Indent,
    grid::{
        compact::CompactConfig,
        spanned::config::{GridConfig, Margin as GridMargin},
    },
    settings::TableOption,
};

/// Margin is responsible for a left/right/top/bottom outer indent of a grid.
///
/// ```rust,no_run
/// # use tabled::{settings::margin::Margin, Table};
/// # let data: Vec<&'static str> = Vec::new();
/// let table = Table::new(&data)
///     .with(Margin::new(1, 1, 1, 1).fill('>', '<', 'V', '^'));
/// ```
#[derive(Debug, Clone)]
pub struct Margin(GridMargin);

impl Margin {
    /// Construct's an Margin object.
    ///
    /// It uses space(' ') as a default fill character.
    /// To set a custom character you can use [`Margin::fill`] function.
    pub const fn new(left: usize, right: usize, top: usize, bottom: usize) -> Self {
        Self(GridMargin {
            top: Indent::spaced(top),
            bottom: Indent::spaced(bottom),
            left: Indent::spaced(left),
            right: Indent::spaced(right),
        })
    }

    /// The function, sets a characters for the margin on an each side.
    pub const fn fill(mut self, left: char, right: char, top: char, bottom: char) -> Self {
        self.0.left.fill = left;
        self.0.right.fill = right;
        self.0.top.fill = top;
        self.0.bottom.fill = bottom;
        self
    }
}

impl<R, D> TableOption<R, D, GridConfig> for Margin {
    fn change(&mut self, _: &mut R, cfg: &mut GridConfig, _: &mut D) {
        cfg.set_margin(self.0);
    }
}

impl<R, D> TableOption<R, D, CompactConfig> for Margin {
    fn change(&mut self, _: &mut R, cfg: &mut CompactConfig, _: &mut D) {
        cfg.set_margin(self.0);
    }
}
