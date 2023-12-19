//! This module contains a configuration of a Border to set its color via [`BorderColor`].

use core::marker::PhantomData;

use crate::{
    grid::{
        config::{Border as GridBorder, ColoredConfig, Entity},
        records::{ExactRecords, Records},
    },
    settings::{style::On, CellOption, Color, TableOption},
};

/// Border represents a border color of a Cell.
///
/// ```text
///                         top border
///                             |
///                             V
/// corner top left ------> +_______+  <---- corner top left
///                         |       |
/// left border ----------> |  cell |  <---- right border
///                         |       |
/// corner bottom right --> +_______+  <---- corner bottom right
///                             ^
///                             |
///                        bottom border
/// ```
///
/// # Example
///
/// ```rust,no_run
/// # use tabled::{Table, settings::{style::{Style, BorderColor}, object::Rows, Color}};
/// # let data: Vec<&'static str> = Vec::new();
/// let table = Table::new(&data)
///     .with(Style::ascii())
///     .modify(Rows::single(0), BorderColor::new().set_top(Color::FG_RED));
/// ```
#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct BorderColor<T, B, L, R> {
    inner: GridBorder<Color>,
    _top: PhantomData<T>,
    _bottom: PhantomData<B>,
    _left: PhantomData<L>,
    _right: PhantomData<R>,
}

impl<T, B, L, R> BorderColor<T, B, L, R> {
    pub(crate) const fn from_border(inner: GridBorder<Color>) -> BorderColor<T, B, L, R> {
        BorderColor {
            inner,
            _top: PhantomData,
            _bottom: PhantomData,
            _left: PhantomData,
            _right: PhantomData,
        }
    }
}

impl BorderColor<(), (), (), ()> {
    /// Creates an empty border.
    pub const fn new() -> Self {
        Self::from_border(GridBorder::empty())
    }
}

impl BorderColor<On, On, On, On> {
    /// This function constructs a cell borders with all sides set.
    #[allow(clippy::too_many_arguments)]
    pub const fn full(
        top: Color,
        bottom: Color,
        left: Color,
        right: Color,
        top_left: Color,
        top_right: Color,
        bottom_left: Color,
        bottom_right: Color,
    ) -> Self {
        Self::from_border(GridBorder::full(
            top,
            bottom,
            left,
            right,
            top_left,
            top_right,
            bottom_left,
            bottom_right,
        ))
    }

    /// This function constructs a cell borders with all sides's char set to a given color.
    /// It behaves like [`BorderColor::full`] with the same color set to each side.
    pub fn filled(c: Color) -> Self {
        Self::full(
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
}

impl<T, B, L, R> BorderColor<T, B, L, R> {
    /// Set a top border color.
    pub fn set_top(mut self, c: Color) -> BorderColor<On, B, L, R> {
        self.inner.top = Some(c);
        BorderColor::from_border(self.inner)
    }

    /// Set a bottom border color.
    pub fn set_bottom(mut self, c: Color) -> BorderColor<T, On, L, R> {
        self.inner.bottom = Some(c);
        BorderColor::from_border(self.inner)
    }

    /// Set a left border color.
    pub fn set_left(mut self, c: Color) -> BorderColor<T, B, On, R> {
        self.inner.left = Some(c);
        BorderColor::from_border(self.inner)
    }

    /// Set a right border color.
    pub fn set_right(mut self, c: Color) -> BorderColor<T, B, L, On> {
        self.inner.right = Some(c);
        BorderColor::from_border(self.inner)
    }

    /// Converts a border into a general data structure.
    pub fn into_inner(self) -> GridBorder<Color> {
        self.inner
    }
}

impl<T, B, L> BorderColor<T, B, L, On> {
    /// Get a right color.
    pub fn get_right(&self) -> Color {
        get_color(self.inner.right.clone())
    }
}

impl<T, B, R> BorderColor<T, B, On, R> {
    /// Get a left color.
    pub fn get_left(&self) -> Color {
        get_color(self.inner.left.clone())
    }
}

impl<B, L, R> BorderColor<On, B, L, R> {
    /// Get a top color.
    pub fn get_top(&self) -> Color {
        get_color(self.inner.top.clone())
    }
}

impl<T, L, R> BorderColor<T, On, L, R> {
    /// Get a bottom color.
    pub fn get_bottom(&self) -> Color {
        get_color(self.inner.bottom.clone())
    }
}

impl<B, R> BorderColor<On, B, On, R> {
    /// Set a top left intersection color.
    pub fn set_corner_top_left(mut self, c: Color) -> Self {
        self.inner.left_top_corner = Some(c);
        self
    }

    /// Get a top left intersection color.
    pub fn get_corner_top_left(&self) -> Color {
        get_color(self.inner.left_top_corner.clone())
    }
}

impl<B, L> BorderColor<On, B, L, On> {
    /// Set a top right intersection color.
    pub fn set_corner_top_right(mut self, c: Color) -> Self {
        self.inner.right_top_corner = Some(c);
        self
    }

    /// Get a top right intersection color.
    pub fn get_corner_top_right(&self) -> Color {
        get_color(self.inner.right_top_corner.clone())
    }
}

impl<T, R> BorderColor<T, On, On, R> {
    /// Set a bottom left intersection color.
    pub fn set_corner_bottom_left(mut self, c: Color) -> Self {
        self.inner.left_bottom_corner = Some(c);
        self
    }

    /// Get a bottom left intersection color.
    pub fn get_corner_bottom_left(&self) -> Color {
        get_color(self.inner.left_bottom_corner.clone())
    }
}

impl<T, L> BorderColor<T, On, L, On> {
    /// Set a bottom right intersection color.
    pub fn set_corner_bottom_right(mut self, c: Color) -> Self {
        self.inner.right_bottom_corner = Some(c);
        self
    }

    /// Get a bottom left intersection color.
    pub fn get_corner_bottom_right(&self) -> Color {
        get_color(self.inner.right_bottom_corner.clone())
    }
}

impl<T, B, L, R> From<BorderColor<T, B, L, R>> for GridBorder<Color> {
    fn from(value: BorderColor<T, B, L, R>) -> Self {
        value.inner
    }
}

impl<Data, T, B, L, R> CellOption<Data, ColoredConfig> for BorderColor<T, B, L, R>
where
    Data: Records + ExactRecords,
{
    fn change(self, records: &mut Data, cfg: &mut ColoredConfig, entity: Entity) {
        let count_rows = records.count_rows();
        let count_columns = records.count_columns();

        let border_color = self.inner.clone().convert();

        for pos in entity.iter(count_rows, count_columns) {
            cfg.set_border_color(pos, border_color.clone());
        }
    }
}

impl<Data, D, T, B, L, R> TableOption<Data, ColoredConfig, D> for BorderColor<T, B, L, R>
where
    Data: Records + ExactRecords,
{
    fn change(self, records: &mut Data, cfg: &mut ColoredConfig, _: &mut D) {
        let count_rows = records.count_rows();
        let count_columns = records.count_columns();

        let border_color = self.inner.clone().convert();

        for row in 0..count_rows {
            for col in 0..count_columns {
                cfg.set_border_color((row, col), border_color.clone());
            }
        }
    }
}

fn get_color(c: Option<Color>) -> Color {
    match c {
        Some(c) => c,
        None => unreachable!(),
    }
}
