//! Module contains a list of implementations of [`Estimate`] and [`Dimension`].

mod comlete_dimension;
mod const_dimension;
mod static_dimension;

pub use comlete_dimension::CompleteDimension;
pub use const_dimension::{ConstDimension, ConstSize};
pub use static_dimension::{ExactList, StaticDimension};

pub use papergrid::dimension::{
    compact::CompactGridDimension, spanned::SpannedGridDimension, Dimension, Estimate,
};
