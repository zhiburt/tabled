//! This module contains a list of primitives to help to modify a [`Table`].
//!
//! [`Table`]: crate::Table

mod format_config;
mod format_content;
mod format_positioned;

pub use format_config::FormatConfig;
pub use format_content::FormatContent;
pub use format_positioned::FormatContentPositioned;

/// A formatting function of particular cells on a [`Table`].
///
/// [`Table`]: crate::Table
#[derive(Debug)]
pub struct Format;

impl Format {
    /// This function creates a new [`Format`] instance, so
    /// it can be used as a grid setting.
    ///
    /// # Example
    ///
    /// ```
    /// use tabled::{Table, settings::{Format, object::Rows, Modify}};
    ///
    /// let data = vec![
    ///     (0, "Grodno", true),
    ///     (1, "Minsk", true),
    ///     (2, "Hamburg", false),
    ///     (3, "Brest", true),
    /// ];
    ///
    /// let table = Table::new(&data)
    ///                .with(Modify::new(Rows::new(1..)).with(Format::content(|s| format!(": {} :", s))))
    ///                .to_string();
    ///
    /// assert_eq!(
    ///     table,
    ///     "+-------+-------------+-----------+\n\
    ///      | i32   | &str        | bool      |\n\
    ///      +-------+-------------+-----------+\n\
    ///      | : 0 : | : Grodno :  | : true :  |\n\
    ///      +-------+-------------+-----------+\n\
    ///      | : 1 : | : Minsk :   | : true :  |\n\
    ///      +-------+-------------+-----------+\n\
    ///      | : 2 : | : Hamburg : | : false : |\n\
    ///      +-------+-------------+-----------+\n\
    ///      | : 3 : | : Brest :   | : true :  |\n\
    ///      +-------+-------------+-----------+"
    /// );
    /// ```
    pub fn content<F>(f: F) -> FormatContent<F>
    where
        F: FnMut(&str) -> String,
    {
        FormatContent::new(f)
    }

    /// This function creates a new [`FormatContentPositioned`], so
    /// it can be used as a grid setting.
    ///
    /// It's different from [`Format::content`] as it also provides a row and column index.
    ///
    /// # Example
    ///
    /// ```
    /// use tabled::{Table, settings::{Format, object::Rows, Modify}};
    ///
    /// let data = vec![
    ///     (0, "Grodno", true),
    ///     (1, "Minsk", true),
    ///     (2, "Hamburg", false),
    ///     (3, "Brest", true),
    /// ];
    ///
    /// let table = Table::new(&data)
    ///                .with(Modify::new(Rows::single(0)).with(Format::positioned(|_, (_, col)| col.to_string())))
    ///                .to_string();
    ///
    /// assert_eq!(
    ///     table,
    ///     "+---+---------+-------+\n\
    ///      | 0 | 1       | 2     |\n\
    ///      +---+---------+-------+\n\
    ///      | 0 | Grodno  | true  |\n\
    ///      +---+---------+-------+\n\
    ///      | 1 | Minsk   | true  |\n\
    ///      +---+---------+-------+\n\
    ///      | 2 | Hamburg | false |\n\
    ///      +---+---------+-------+\n\
    ///      | 3 | Brest   | true  |\n\
    ///      +---+---------+-------+"
    /// );
    /// ```
    pub fn positioned<F>(f: F) -> FormatContentPositioned<F>
    where
        F: FnMut(&str, (usize, usize)) -> String,
    {
        FormatContentPositioned::new(f)
    }

    /// This function creates [`FormatConfig`] function to modify a table config.
    ///
    /// # Example
    ///
    /// ```
    /// use tabled::{
    ///     Table,
    ///     settings::{Format, object::Rows, Modify},
    ///     grid::config::ColoredConfig,
    /// };
    ///
    /// let data = vec![
    ///     (0, "Grodno", true),
    ///     (1, "Minsk", true),
    ///     (2, "Hamburg", false),
    ///     (3, "Brest", true),
    /// ];
    ///
    /// let table = Table::new(&data)
    ///                .with(Format::config(|cfg: &mut ColoredConfig| cfg.set_justification((0,1).into(), '.')))
    ///                .to_string();
    ///
    /// assert_eq!(
    ///     table,
    ///     "+-----+---------+-------+\n\
    ///      | i32 | &str... | bool  |\n\
    ///      +-----+---------+-------+\n\
    ///      | 0   | Grodno  | true  |\n\
    ///      +-----+---------+-------+\n\
    ///      | 1   | Minsk   | true  |\n\
    ///      +-----+---------+-------+\n\
    ///      | 2   | Hamburg | false |\n\
    ///      +-----+---------+-------+\n\
    ///      | 3   | Brest   | true  |\n\
    ///      +-----+---------+-------+"
    /// );
    /// ```
    pub fn config<F>(f: F) -> FormatConfig<F> {
        FormatConfig(f)
    }
}
