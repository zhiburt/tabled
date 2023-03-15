//! Module contains a list of implementations of [`Estimate`] and [`Dimension`].

mod const_dimension;

#[cfg(feature = "std")]
mod complete_dimension;
#[cfg(feature = "std")]
mod peekable_dimension;
#[cfg(feature = "std")]
mod static_dimension;

#[cfg(feature = "std")]
#[cfg_attr(docsrs, doc(cfg(feature = "std")))]
pub use self::{
    complete_dimension::CompleteDimension,
    peekable_dimension::PeekableDimension,
    static_dimension::{ExactList, StaticDimension},
};
pub use const_dimension::{ConstDimension, ConstSize};
pub use papergrid::dimension::{Dimension, Estimate};

#[cfg(feature = "std")]
#[cfg_attr(docsrs, doc(cfg(feature = "std")))]
pub use papergrid::dimension::compact::CompactGridDimension;
#[cfg(feature = "std")]
#[cfg_attr(docsrs, doc(cfg(feature = "std")))]
pub use papergrid::dimension::spanned::SpannedGridDimension;
