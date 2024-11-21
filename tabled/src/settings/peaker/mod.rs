//! The module contains [`Priority`] and [`Peaker`] trait,
//! its implementations to be used in [`Height`] and [`Width`].
//!
//! [`Width`]: crate::settings::width::Width
//! [`Height`]: crate::settings::height::Height

mod left;
mod max;
mod min;
mod none;
mod right;

pub use left::PriorityLeft;
pub use max::PriorityMax;
pub use min::PriorityMin;
pub use none::PriorityNone;
pub use right::PriorityRight;

/// A strategy of width function.
/// It determines the order how a function is applied.
///
/// For example which column we shall peak to truncate when doing width alogorithms.
pub trait Peaker {
    /// This function returns an index which will be changed.
    /// Or `None` if no changes are necessary.
    ///
    /// When [`None`] returned the alogorithm must be stopped.
    fn peak(&mut self, mins: &[usize], values: &[usize]) -> Option<usize>;
}

/// An abstract factory to construct different [`Peaker`] methods.
///
/// ```
/// # use tabled::{Table, settings::{Style, peaker::Priority, Width}};
/// # use testing_table::assert_table;
/// #
/// let data = [
///     ("1", "Hello", 100),
///     ("2", "World", 1000),
/// ];
///
/// let mut table = Table::new(data);
/// table.with(Style::modern());
/// table.with(Width::wrap(15).priority(Priority::max(false)));
///
/// let output = table.to_string();
///
/// assert_table!(
///     output,
///     "┌───┬────┬────┐"
///     "│ & │ &s │ i3 │"
///     "│ s │ tr │ 2  │"
///     "│ t │    │    │"
///     "│ r │    │    │"
///     "├───┼────┼────┤"
///     "│ 1 │ He │ 10 │"
///     "│   │ ll │ 0  │"
///     "│   │ o  │    │"
///     "├───┼────┼────┤"
///     "│ 2 │ Wo │ 10 │"
///     "│   │ rl │ 00 │"
///     "│   │ d  │    │"
///     "└───┴────┴────┘"
/// );
/// ```
#[derive(Debug, Clone, Copy, Default, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Priority;

impl Priority {
    /// Returns a [`Peaker`] which goes over list one by one,
    /// in order from left to right with no prioritization,
    /// just peaking each value after another.
    ///
    /// ```
    /// # use tabled::{Table, settings::{Style, peaker::Priority, Width}};
    /// # use testing_table::assert_table;
    /// #
    /// let data = [
    ///     ("1", "Hello", 100),
    ///     ("2", "World", 1000),
    /// ];
    ///
    /// let mut table = Table::new(data);
    /// table.with(Style::modern());
    /// table.with(Width::wrap(15).priority(Priority::none()));
    ///
    /// let output = table.to_string();
    ///
    /// assert_table!(
    ///     output,
    ///     "┌───┬────┬────┐"
    ///     "│ & │ &s │ i3 │"
    ///     "│ s │ tr │ 2  │"
    ///     "│ t │    │    │"
    ///     "│ r │    │    │"
    ///     "├───┼────┼────┤"
    ///     "│ 1 │ He │ 10 │"
    ///     "│   │ ll │ 0  │"
    ///     "│   │ o  │    │"
    ///     "├───┼────┼────┤"
    ///     "│ 2 │ Wo │ 10 │"
    ///     "│   │ rl │ 00 │"
    ///     "│   │ d  │    │"
    ///     "└───┴────┴────┘"
    /// );
    /// ```
    pub fn none() -> PriorityNone {
        PriorityNone::new()
    }

    /// Returns a [`Peaker`] which goes over list peacking a minimum value,
    /// and prioritizing a chosen side when equal values are met.
    ///
    /// ```
    /// # use tabled::{Table, settings::{Style, peaker::Priority, Width}};
    /// # use testing_table::assert_table;
    /// #
    /// let data = [
    ///     ("1", "Hello", 100),
    ///     ("2", "World", 1000),
    /// ];
    ///
    /// let mut table = Table::new(data);
    /// table.with(Style::modern());
    /// table.with(Width::wrap(15).priority(Priority::min(true)));
    ///
    /// let output = table.to_string();
    ///
    /// assert_table!(
    ///     output,
    ///     "┌──┬───────┬──┐"
    ///     "│  │ &str  │  │"
    ///     "├──┼───────┼──┤"
    ///     "│  │ Hello │  │"
    ///     "├──┼───────┼──┤"
    ///     "│  │ World │  │"
    ///     "└──┴───────┴──┘"
    /// );
    /// ```
    pub fn min(right: bool) -> PriorityMin {
        PriorityMin::new(right)
    }

    /// Returns a [`Peaker`] which goes over list peacking a maximum value,
    /// and prioritizing a chosen side when equal values are met.
    ///
    /// ```
    /// # use tabled::{Table, settings::{Style, peaker::Priority, Width}};
    /// # use testing_table::assert_table;
    /// #
    /// let data = [
    ///     ("1", "Hello", 100),
    ///     ("2", "World", 1000),
    /// ];
    ///
    /// let mut table = Table::new(data);
    /// table.with(Style::modern());
    /// table.with(Width::wrap(15).priority(Priority::max(true)));
    ///
    /// let output = table.to_string();
    ///
    /// assert_table!(
    ///     output,
    ///     "┌────┬────┬───┐"
    ///     "│ &s │ &s │ i │"
    ///     "│ tr │ tr │ 3 │"
    ///     "│    │    │ 2 │"
    ///     "├────┼────┼───┤"
    ///     "│ 1  │ He │ 1 │"
    ///     "│    │ ll │ 0 │"
    ///     "│    │ o  │ 0 │"
    ///     "├────┼────┼───┤"
    ///     "│ 2  │ Wo │ 1 │"
    ///     "│    │ rl │ 0 │"
    ///     "│    │ d  │ 0 │"
    ///     "│    │    │ 0 │"
    ///     "└────┴────┴───┘"
    /// );
    /// ```
    pub fn max(right: bool) -> PriorityMax {
        PriorityMax::new(right)
    }

    /// Returns a [`Peaker`] which goes over list peacking a left most value as far as possible.
    ///
    /// ```
    /// # use tabled::{Table, settings::{Style, peaker::Priority, Width}};
    /// # use testing_table::assert_table;
    /// #
    /// let data = [
    ///     ("1", "Hello", 100),
    ///     ("2", "World", 1000),
    /// ];
    ///
    /// let mut table = Table::new(data);
    /// table.with(Style::modern());
    /// table.with(Width::wrap(15).priority(Priority::left()));
    ///
    /// let output = table.to_string();
    ///
    /// assert_table!(
    ///     output,
    ///     "┌──┬───┬──────┐"
    ///     "│  │ & │ i32  │"
    ///     "│  │ s │      │"
    ///     "│  │ t │      │"
    ///     "│  │ r │      │"
    ///     "├──┼───┼──────┤"
    ///     "│  │ H │ 100  │"
    ///     "│  │ e │      │"
    ///     "│  │ l │      │"
    ///     "│  │ l │      │"
    ///     "│  │ o │      │"
    ///     "├──┼───┼──────┤"
    ///     "│  │ W │ 1000 │"
    ///     "│  │ o │      │"
    ///     "│  │ r │      │"
    ///     "│  │ l │      │"
    ///     "│  │ d │      │"
    ///     "└──┴───┴──────┘"
    /// );
    /// ```
    pub fn left() -> PriorityLeft {
        PriorityLeft::new()
    }

    /// Returns a [`Peaker`] which goes over list peacking a right most value as far as possible.
    ///
    /// ```
    /// # use tabled::{Table, settings::{Style, peaker::Priority, Width}};
    /// # use testing_table::assert_table;
    /// #
    /// let data = [
    ///     ("1", "Hello", 100),
    ///     ("2", "World", 1000),
    /// ];
    ///
    /// let mut table = Table::new(data);
    /// table.with(Style::modern());
    /// table.with(Width::wrap(15).priority(Priority::right()));
    ///
    /// let output = table.to_string();
    ///
    /// assert_table!(
    ///     output,
    ///     "┌──────┬───┬──┐"
    ///     "│ &str │ & │  │"
    ///     "│      │ s │  │"
    ///     "│      │ t │  │"
    ///     "│      │ r │  │"
    ///     "├──────┼───┼──┤"
    ///     "│ 1    │ H │  │"
    ///     "│      │ e │  │"
    ///     "│      │ l │  │"
    ///     "│      │ l │  │"
    ///     "│      │ o │  │"
    ///     "├──────┼───┼──┤"
    ///     "│ 2    │ W │  │"
    ///     "│      │ o │  │"
    ///     "│      │ r │  │"
    ///     "│      │ l │  │"
    ///     "│      │ d │  │"
    ///     "└──────┴───┴──┘"
    /// );
    /// ```
    pub fn right() -> PriorityRight {
        PriorityRight::new()
    }
}
