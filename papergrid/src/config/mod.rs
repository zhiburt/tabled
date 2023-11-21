//! A module which contains a general settings which might be used in other grid implementations.

mod alignment;
mod border;
mod borders;
mod entity;
mod formatting;
mod horizontal_line;
mod indent;
mod position;
mod sides;
mod vertical_line;

pub mod compact;
#[cfg(feature = "std")]
pub mod spanned;

pub use alignment::{AlignmentHorizontal, AlignmentVertical};
pub use border::Border;
pub use borders::Borders;
pub use entity::{Entity, EntityIterator};
pub use formatting::Formatting;
pub use horizontal_line::HorizontalLine;
pub use indent::Indent;
pub use position::Position;
pub use sides::Sides;
pub use vertical_line::VerticalLine;
