//! Module contains a list of backends for pretty print tables.

pub mod compact;

#[cfg(feature = "std")]
#[cfg_attr(docsrs, doc(cfg(feature = "std")))]
pub mod spanned;
