//! Module contains a list of backends for pretty print tables.

pub mod compact;

#[cfg(feature = "std")]
pub mod iterable;

#[cfg(feature = "std")]
pub mod peekable;
