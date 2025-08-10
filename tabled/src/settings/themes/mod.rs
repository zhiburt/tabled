//! The module contains a variety of configurations of table, which often
//! changes not a single setting.
//! As such they are making relatively big changes to the configuration.

mod border_correction;
mod colorization;
mod column_names;
mod layout;
mod row_names;
mod theme;

pub use border_correction::BorderCorrection;
pub use colorization::{Colorization, ExactColorization};
pub use column_names::ColumnNames;
pub use layout::Layout;
pub use row_names::RowNames;
pub use theme::Theme;
