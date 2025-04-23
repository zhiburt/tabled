//! Module contains a list of implementations of [`Estimate`] and [`Dimension`].

mod const_dimension;
mod pool_table_dimension;
mod zero_dimension;

#[cfg(feature = "std")]
mod complete_dimension;
#[cfg(feature = "std")]
mod static_dimension;

#[cfg(feature = "std")]
#[cfg_attr(docsrs, doc(cfg(feature = "std")))]
pub use self::{
    complete_dimension::CompleteDimension,
    static_dimension::{DimensionValue, StaticDimension},
};
pub use const_dimension::{ConstDimension, ConstSize};
pub use papergrid::dimension::{Dimension, Estimate};
pub use pool_table_dimension::{DimensionPriority, PoolTableDimension};
pub use zero_dimension::ZeroDimension;

#[cfg(feature = "std")]
#[cfg_attr(docsrs, doc(cfg(feature = "std")))]
pub use papergrid::dimension::{
    compact::CompactGridDimension, iterable::IterGridDimension, peekable::PeekableGridDimension,
};
