//! Module contains a list of implementations of [`Estimate`] and [`Dimension`].

mod const_dimension;

#[cfg(feature = "std")]
mod comlete_dimension;
#[cfg(feature = "std")]
mod static_dimension;

#[cfg(feature = "std")]
#[cfg_attr(docsrs, doc(cfg(feature = "std")))]
pub use self::{
    comlete_dimension::CompleteDimension,
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
