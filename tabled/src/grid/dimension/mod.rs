//! Module contains a list of implementations of [`Estimate`] and [`Dimension`].

mod const_dimension;
mod pool_table_dimension;

#[cfg(feature = "std")]
mod complete_dimension;
#[cfg(feature = "std")]
mod complete_dimension_vec_records;
#[cfg(feature = "std")]
mod peekable_dimension;
#[cfg(feature = "std")]
mod static_dimension;

#[cfg(feature = "std")]
#[cfg_attr(docsrs, doc(cfg(feature = "std")))]
pub use self::{
    complete_dimension::CompleteDimension,
    complete_dimension_vec_records::CompleteDimensionVecRecords,
    peekable_dimension::PeekableDimension,
    static_dimension::{DimensionValue, StaticDimension},
};
pub use const_dimension::{ConstDimension, ConstSize};
pub use papergrid::dimension::{Dimension, Estimate};
pub use pool_table_dimension::{DimensionPriority, PoolTableDimension};

#[cfg(feature = "std")]
#[cfg_attr(docsrs, doc(cfg(feature = "std")))]
pub use papergrid::dimension::{
    compact::CompactGridDimension, spanned::SpannedGridDimension,
    spanned_vec_records::SpannedVecRecordsDimension,
};
