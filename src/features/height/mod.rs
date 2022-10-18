//! The module contains [`Height`] structure which is responsible for a table and cell height.

use papergrid::{height::HeightEstimator, records::Records, Estimate, GridConfig};

use crate::measurment::Measurment;

mod cell_height_increase;
mod cell_height_limit;
mod height_list;
mod table_height_increase;
mod table_height_limit;

pub use cell_height_increase::CellHeightIncrease;
pub use cell_height_limit::CellHeightLimit;
pub use height_list::HeightList;
pub use table_height_increase::TableHeightIncrease;
pub use table_height_limit::TableHeightLimit;

/// Height is a abstract factory for height settings.
///
/// # Example
///
/// ```
/// use tabled::{Table, Height};
///
/// let data = vec![
///     ("Some data", "here", "and here"),
///     ("Some data on a next", "line", "right here"),
/// ];
///
/// let table = Table::new(data)
///     .with(Height::increase(10))
///     .with(Height::limit(10))
///     .to_string();
///
/// assert_eq!(
///     table,
///     "+---------------------+------+------------+\n\
///      | &str                | &str | &str       |\n\
///      |                     |      |            |\n\
///      +---------------------+------+------------+\n\
///      | Some data           | here | and here   |\n\
///      |                     |      |            |\n\
///      +---------------------+------+------------+\n\
///      | Some data on a next | line | right here |\n\
///      |                     |      |            |\n\
///      +---------------------+------+------------+",
/// )
/// ```
#[derive(Debug)]
pub struct Height;

impl Height {
    /// Create [`CellHeightIncrease`] to set a table/cell height.
    ///
    /// # Example
    ///
    /// ## Cell height
    ///
    /// ```
    /// use tabled::{Table, Height, Modify, object::Columns};
    ///
    /// let data = vec![
    ///     ("Some data", "here", "and here"),
    ///     ("Some data on a next", "line", "right here"),
    /// ];
    ///
    /// let table = Table::new(data)
    ///     .with(Modify::new(Columns::first()).with(Height::increase(5)))
    ///     .to_string();
    ///
    /// assert_eq!(
    ///     table,
    ///     "+---------------------+------+------------+\n\
    ///      | &str                | &str | &str       |\n\
    ///      |                     |      |            |\n\
    ///      |                     |      |            |\n\
    ///      |                     |      |            |\n\
    ///      |                     |      |            |\n\
    ///      +---------------------+------+------------+\n\
    ///      | Some data           | here | and here   |\n\
    ///      |                     |      |            |\n\
    ///      |                     |      |            |\n\
    ///      |                     |      |            |\n\
    ///      |                     |      |            |\n\
    ///      +---------------------+------+------------+\n\
    ///      | Some data on a next | line | right here |\n\
    ///      |                     |      |            |\n\
    ///      |                     |      |            |\n\
    ///      |                     |      |            |\n\
    ///      |                     |      |            |\n\
    ///      +---------------------+------+------------+"
    /// )
    /// ```
    ///
    /// ## Table height
    ///
    /// ```
    /// use tabled::{Table, Height};
    ///
    /// let data = vec![
    ///     ("Some data", "here", "and here"),
    ///     ("Some data on a next", "line", "right here"),
    /// ];
    ///
    /// let table = Table::new(data)
    ///     .with(Height::increase(10))
    ///     .to_string();
    ///
    /// assert_eq!(
    ///     table,
    ///     "+---------------------+------+------------+\n\
    ///      | &str                | &str | &str       |\n\
    ///      |                     |      |            |\n\
    ///      +---------------------+------+------------+\n\
    ///      | Some data           | here | and here   |\n\
    ///      |                     |      |            |\n\
    ///      +---------------------+------+------------+\n\
    ///      | Some data on a next | line | right here |\n\
    ///      |                     |      |            |\n\
    ///      +---------------------+------+------------+",
    /// )
    /// ```
    pub fn increase<W>(width: W) -> CellHeightIncrease<W>
    where
        W: Measurment<Height>,
    {
        CellHeightIncrease::new(width)
    }

    /// Create [`CellHeightLimit`] to set a table/cell height.
    ///
    /// # Example
    ///
    /// ## Cell height
    ///
    /// ```
    /// use tabled::{Table, Height, Modify, object::Columns};
    ///
    /// let data = vec![
    ///     ("Some\ndata", "here", "and here"),
    ///     ("Some\ndata on a next", "line", "right here"),
    /// ];
    ///
    /// let table = Table::new(data)
    ///     .with(Modify::new(Columns::first()).with(Height::limit(1)))
    ///     .to_string();
    ///
    /// assert_eq!(
    ///     table,
    ///     "+------+------+------------+\n\
    ///      | &str | &str | &str       |\n\
    ///      +------+------+------------+\n\
    ///      | Some | here | and here   |\n\
    ///      +------+------+------------+\n\
    ///      | Some | line | right here |\n\
    ///      +------+------+------------+"
    /// )
    /// ```
    ///
    /// ## Table height
    ///
    /// ```
    /// use tabled::{Table, Height};
    ///
    /// let data = vec![
    ///     ("Some\ndata", "here", "and here"),
    ///     ("Some\ndata on a next", "line", "right here"),
    /// ];
    ///
    /// let table = Table::new(&data)
    ///     .with(Height::limit(6))
    ///     .to_string();
    ///
    /// assert_eq!(
    ///     table,
    ///     "+----------------+------+------------+\n\
    ///      +----------------+------+------------+\n\
    ///      | Some           | here | and here   |\n\
    ///      +----------------+------+------------+\n\
    ///      | Some           | line | right here |\n\
    ///      +----------------+------+------------+",
    /// );
    ///
    /// let table = Table::new(&data)
    ///     .with(Height::limit(1))
    ///     .to_string();
    ///
    /// assert_eq!(
    ///     table,
    ///     "+----------------+------+------------+\n\
    ///      +----------------+------+------------+\n\
    ///      +----------------+------+------------+\n\
    ///      +----------------+------+------------+",
    /// );
    /// ```
    pub fn limit<W>(width: W) -> CellHeightLimit<W>
    where
        W: Measurment<Height>,
    {
        CellHeightLimit::new(width)
    }

    /// Create [`HeightList`] to set a table height to a constant list of row heights.
    ///
    /// Notice if you provide a list with `.len()` less than `Table::count_rows` then it will have no affect.
    ///
    /// # Example
    ///
    /// ```
    /// use tabled::{Table, Height, Modify, object::Columns};
    ///
    /// let data = vec![
    ///     ("Some\ndata", "here", "and here"),
    ///     ("Some\ndata on a next", "line", "right here"),
    /// ];
    ///
    /// let table = Table::new(data)
    ///     .with(Height::list([1, 0, 2]))
    ///     .to_string();
    ///
    /// assert_eq!(
    ///     table,
    ///     "+----------------+------+------------+\n\
    ///      | &str           | &str | &str       |\n\
    ///      +----------------+------+------------+\n\
    ///      +----------------+------+------------+\n\
    ///      | Some           | line | right here |\n\
    ///      | data on a next |      |            |\n\
    ///      +----------------+------+------------+",
    /// )
    /// ```
    pub fn list<I>(rows: I) -> HeightList
    where
        I: IntoIterator<Item = usize>,
    {
        HeightList::new(rows.into_iter().collect())
    }
}

pub(crate) fn get_table_total_height<R, E>(records: &R, cfg: &GridConfig, ctrl: &E) -> usize
where
    R: Records,
    E: Estimate<R>,
{
    ctrl.total()
        + cfg.count_horizontal(records.count_rows())
        + cfg.get_margin().top.size
        + cfg.get_margin().bottom.size
}

pub(crate) fn get_table_total_height2<R>(records: &R, cfg: &GridConfig) -> (usize, Vec<usize>)
where
    R: Records,
{
    let mut ctrl = HeightEstimator::default();
    ctrl.estimate(records, cfg);
    let total = <HeightEstimator as Estimate<R>>::total(&ctrl)
        + cfg.count_horizontal(records.count_rows())
        + cfg.get_margin().top.size
        + cfg.get_margin().bottom.size;

    (total, ctrl.into())
}
