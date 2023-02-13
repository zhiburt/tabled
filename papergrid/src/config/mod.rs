//! A module which contains [GridConfig] which is responsible for grid configuration.

mod alignment;
mod border;
mod borders;
mod indent;
mod sides;
mod position;

pub use alignment::{AlignmentHorizontal, AlignmentVertical};
pub use border::Border;
pub use borders::Borders;
pub use indent::Indent;
pub use sides::Sides;
pub use position::Position;
