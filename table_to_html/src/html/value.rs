use crate::html::HtmlElement;

/// HtmlValue represents a children elements of an HTML element.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum HtmlValue {
    /// Children elements.
    Elements(Vec<HtmlElement>),
    /// A string content.
    Content(String),
}
