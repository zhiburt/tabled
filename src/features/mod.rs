pub mod alignment;
pub mod format;
pub mod formatting;
#[allow(unreachable_pub)]
pub mod highlight;
pub mod locator;
pub mod style;
pub mod width;

#[cfg(feature = "color")]
pub mod border_colored;
#[cfg(feature = "color")]
pub mod color;
#[cfg(feature = "color")]
pub mod margin_color;
#[cfg(feature = "color")]
pub mod padding_color;

pub(crate) mod border;
pub(crate) mod border_text;
pub(crate) mod concat;
pub(crate) mod disable;
pub(crate) mod extract;
pub(crate) mod margin;
pub mod merge;
pub(crate) mod padding;
pub(crate) mod panel;
pub(crate) mod rotate;
pub(crate) mod span;
