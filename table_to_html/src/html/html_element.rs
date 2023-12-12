use std::fmt::Display;

use tabled::grid::util::string::get_lines;

use crate::html::{Attribute, HtmlValue, HtmlVisitor, HtmlVisitorMut};

/// A HTML element representation.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct HtmlElement {
    tag: String,
    attrs: Vec<Attribute>,
    value: Option<HtmlValue>,
}

impl HtmlElement {
    /// Creates new HTML element
    ///
    /// None value considered to represent void element.
    pub fn new<S>(tag: S, attrs: Vec<Attribute>, value: Option<HtmlValue>) -> Self
    where
        S: Into<String>,
    {
        Self::__new(tag.into(), attrs, value)
    }

    /// Creates a new HTML element with no content.
    pub fn void<S>(tag: S, attrs: Vec<Attribute>) -> Self
    where
        S: Into<String>,
    {
        Self::__new(tag.into(), attrs, None)
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
    pub fn visit<V>(&self, mut visitor: V)
    where
        V: HtmlVisitor,
    {
        self.__visit(&mut visitor);
    }

    /// Visits all desending elements starting from itself.
    pub fn visit_mut<V>(&mut self, mut visitor: V)
    where
        V: HtmlVisitorMut,
    {
        self.__visit_mut(&mut visitor);
    }

    fn __visit<V>(&self, visitor: &mut V) -> bool
    where
        V: HtmlVisitor,
    {
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

    fn __visit_mut<V>(&mut self, visitor: &mut V) -> bool
    where
        V: HtmlVisitorMut,
    {
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

    fn __new(tag: String, attrs: Vec<Attribute>, value: Option<HtmlValue>) -> Self {
        Self { tag, attrs, value }
    }
}

impl Display for HtmlElement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        build_html(f, self)
    }
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
