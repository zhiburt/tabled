//! The module contains a varieity of configurations of table, which often
//! changes not a single setting.
//! As such they are making relatively big changes to the configuration.

mod colorization;
mod column_names;

pub use colorization::{Colorization, ExactColorization};
pub use column_names::ColumnNames;
