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
//! Has a table which is usefull for large amount of data.

pub mod extended;
pub mod iter;
pub mod table;
