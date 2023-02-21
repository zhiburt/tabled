//! The module contains a html primitives.

use std::fmt::Display;

use tabled::grid::util::string::get_lines;

/// A HTML element representation.
#[derive(Debug, Clone)]
pub struct HtmlElement {
    tag: String,
    attrs: Vec<Attribute>,
    value: Option<HtmlValue>,
}

impl HtmlElement {
    /// Creates new HTML element
    ///
    /// None value considered to represent void element.
    pub fn new<S: Into<String>>(tag: S, attrs: Vec<Attribute>, value: Option<HtmlValue>) -> Self {
        Self {
            tag: tag.into(),
            attrs,
            value,
        }
    }

    /// Creates a new HTML element with no content.
    pub fn void<S: Into<String>>(tag: S, attrs: Vec<Attribute>) -> Self {
        Self {
            tag: tag.into(),
            attrs,
            value: None,
        }
    }

    /// Returns a tag value of an element.
    pub fn tag(&self) -> &str {
        &self.tag
    }

    /// Returns a attributes of an element.
    pub fn attrs(&self) -> &[Attribute] {
        &self.attrs
    }

    /// Returns a value of an element.
    pub fn value(&self) -> Option<&HtmlValue> {
        self.value.as_ref()
    }

    /// Visits all desending elements starting from itself.
    pub fn visit<V: HtmlVisitor>(&self, mut visitor: V) {
        self.__visit(&mut visitor);
    }

    /// Visits all desending elements starting from itself.
    pub fn visit_mut<V: HtmlVisitorMut>(&mut self, mut visitor: V) {
        self.__visit_mut(&mut visitor);
    }

    fn __visit<V: HtmlVisitor>(&self, visitor: &mut V) -> bool {
        let ok = visitor.visit_element(self);
        if !ok {
            return false;
        }

        let value = match &self.value {
            Some(content) => content,
            None => return true,
        };

        match value {
            HtmlValue::Elements(elements) => {
                for e in elements {
                    let ok = e.__visit(visitor);
                    if !ok {
                        return false;
                    }
                }
            }
            HtmlValue::Content(_) => {}
        }

        true
    }

    fn __visit_mut<V: HtmlVisitorMut>(&mut self, visitor: &mut V) -> bool {
        let ok = visitor.visit_element_mut(self);
        if !ok {
            return false;
        }

        let value = match &mut self.value {
            Some(content) => content,
            None => return true,
        };

        match value {
            HtmlValue::Elements(elements) => {
                for e in elements {
                    let ok = e.__visit_mut(visitor);
                    if !ok {
                        return false;
                    }
                }
            }
            HtmlValue::Content(_) => {}
        }

        true
    }
}

impl Display for HtmlElement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        build_html(f, self)
    }
}

/// HtmlValue represents a children elements of an HTML element.
#[derive(Debug, Clone)]
pub enum HtmlValue {
    /// Children elements.
    Elements(Vec<HtmlElement>),
    /// A string content.
    Content(String),
}

/// Attribute represents a HTML `key=value` attribute pair.
#[derive(Debug, Clone)]
pub struct Attribute {
    key: String,
    value: String,
}

impl Attribute {
    /// Creates a new attribute.
    pub fn new<K: Into<String>, V: Into<String>>(key: K, value: V) -> Self {
        Self {
            key: key.into(),
            value: value.into(),
        }
    }

    /// Returns a key.
    pub fn key(&self) -> &str {
        &self.key
    }

    /// Returns a value.
    pub fn value(&self) -> &str {
        &self.value
    }
}

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

fn build_html(f: impl std::fmt::Write, e: &HtmlElement) -> std::fmt::Result {
    struct Builder<W> {
        writer: W,
        tab: usize,
        result: std::fmt::Result,
    }

    impl<W> Builder<W> {
        fn new(writer: W, tab: usize) -> Self {
            Self {
                writer,
                tab,
                result: Ok(()),
            }
        }
    }

    impl<W: std::fmt::Write> HtmlVisitor for Builder<W> {
        fn visit_element(&mut self, e: &HtmlElement) -> bool {
            let space = " ".repeat(self.tab);

            let is_void_element = e.value().is_none();
            if is_void_element {
                self.result = write!(self.writer, "{space}");
                self.result = print_tag(&mut self.writer, e.tag(), e.attrs());
                return true;
            }

            if let Some(val) = e.value() {
                self.result = write!(self.writer, "{space}");
                self.result = print_tag(&mut self.writer, e.tag(), e.attrs());
                self.result = writeln!(self.writer);

                match val {
                    HtmlValue::Elements(elems) => {
                        self.tab += 4;
                        for e in elems {
                            self.visit_element(e);
                            self.result = writeln!(self.writer);
                        }
                        self.tab -= 4;
                    }
                    HtmlValue::Content(content) => {
                        let space = " ".repeat(self.tab + 4);
                        for line in get_lines(content) {
                            self.result = write!(self.writer, "{space}");
                            self.result = writeln!(self.writer, "{line}");
                        }
                    }
                }

                self.result = write!(self.writer, "{space}");
                self.result = write!(self.writer, "</{}>", e.tag());
            }

            false
        }
    }

    let mut builder = Builder::new(f, 0);
    e.visit(&mut builder);

    builder.result
}

fn print_tag(mut f: impl std::fmt::Write, tag: &str, attrs: &[Attribute]) -> std::fmt::Result {
    if attrs.is_empty() {
        return write!(f, "<{tag}>");
    }

    write!(f, "<{tag} ")?;
    print_attributes(&mut f, attrs)?;
    write!(f, ">")
}

fn print_attributes(mut f: impl std::fmt::Write, attrs: &[Attribute]) -> std::fmt::Result {
    for (i, attr) in attrs.iter().enumerate() {
        if i > 0 {
            f.write_char(' ')?;
        }

        f.write_str(attr.key())?;
        f.write_char('=')?;
        write!(f, "{:?}", attr.value())?;
    }

    Ok(())
}
