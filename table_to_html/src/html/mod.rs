//! The module contains a html primitives.

mod attr;
mod html_element;
mod value;

pub use attr::Attribute;
pub use html_element::HtmlElement;
pub use value::HtmlValue;

/// A visitor which traverses a HTML elements tree.
pub trait HtmlVisitor {
    /// Visit an element.
    fn visit_element(&mut self, table: &HtmlElement) -> bool;
}

impl<T> HtmlVisitor for &mut T
where
    T: HtmlVisitor,
{
    fn visit_element(&mut self, table: &HtmlElement) -> bool {
        T::visit_element(self, table)
    }
}

/// A visitor which traverses a HTML elements tree, while mutating the tree.
pub trait HtmlVisitorMut {
    /// Visit an element.
    fn visit_element_mut(&mut self, table: &mut HtmlElement) -> bool;
}

impl<T> HtmlVisitorMut for &mut T
where
    T: HtmlVisitorMut,
{
    fn visit_element_mut(&mut self, table: &mut HtmlElement) -> bool {
        T::visit_element_mut(self, table)
    }
}
