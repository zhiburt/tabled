//! Module contains a list of table representatives.
//!
//! ## [`table`]
//!
//! A default table implementation.
//!
//! ## [`iter`]
//!
//! Just like [`table`] but it's API is a bit different to serve better in context
//! where there are a memory limits.
//!
//! ## [`extended`]
//!
//! Has a table which is useful for large amount of data.

pub mod compact;

#[cfg(feature = "std")]
#[cfg_attr(docsrs, doc(cfg(feature = "std")))]
pub mod extended;
#[cfg(feature = "std")]
#[cfg_attr(docsrs, doc(cfg(feature = "std")))]
pub mod iter;
#[cfg(feature = "std")]
#[cfg_attr(docsrs, doc(cfg(feature = "std")))]
pub mod table;

// todo: make it private/public