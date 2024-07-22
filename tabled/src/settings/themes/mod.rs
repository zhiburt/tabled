//! The module contains a variety of configurations of table, which often
//! changes not a single setting.
//! As such they are making relatively big changes to the configuration.

mod colorization;
mod column_names;
mod layout;
mod theme;

pub use colorization::{Colorization, ExactColorization};
pub use column_names::ColumnNames;
pub use layout::Layout;
pub use theme::Theme;
