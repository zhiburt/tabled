//! This module contains settings for render strategy of papergrid.
//!
//! [`TrimStrategy`] and [`AlignmentStrategy`] allows to set [`Alignment`] settings.
//!
//! [`TabSize`] sets a default tab size.
//!
//! [`Charset`] responsible for special char treatment.
//!
//! [`Alignment`]: crate::settings::Alignment

mod alignment_strategy;
mod charset;
mod tab_size;
mod trim_strategy;

pub use alignment_strategy::AlignmentStrategy;
pub use charset::{Charset, CleanCharset};
pub use tab_size::TabSize;
pub use trim_strategy::TrimStrategy;
