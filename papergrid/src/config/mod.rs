//! A module which contains a general settings which might be used in other grid implementations.

mod alignment;
mod border;
mod borders;
mod entity;
mod indent;
mod line;
mod position;
mod sides;

pub mod compact;
#[cfg(feature = "std")]
pub mod spanned;

pub use alignment::{AlignmentHorizontal, AlignmentVertical};
pub use border::Border;
pub use borders::Borders;
pub use entity::{Entity, EntityIterator};
pub use indent::Indent;
pub use line::Line;
pub use position::Position;
pub use sides::Sides;
