//! The module contains [`Peaker`] trait and its implementations to be used in [`Height`] and [`Width`].
//!
//! [`Width`]: crate::settings::width::Width
//! [`Height`]: crate::settings::height::Height

mod left;
mod max;
mod min;
mod none;
mod right;

/// A strategy of width function.
/// It determines the order how the function is applied.
pub trait Peaker {
    /// This function returns a column index which will be changed.
    /// Or `None` if no changes are necessary.
    fn peak(&mut self, min_widths: &[usize], widths: &[usize]) -> Option<usize>;
}

pub use left::PriorityLeft;
pub use max::PriorityMax;
pub use min::PriorityMin;
pub use none::PriorityNone;
pub use right::PriorityRight;
