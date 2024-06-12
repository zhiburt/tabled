use table_to_html::html::{Attribute, HtmlElement, HtmlValue, HtmlVisitor, HtmlVisitorMut};
use testing_table::{assert_table, test_table};

test_table!(
    html_built_element,
    HtmlElement::new(
        "table",
        vec![],
        Some(HtmlValue::Elements(vec![HtmlElement::new(
            "tr",
            vec![Attribute::new("id", "tr1")],
            Some(HtmlValue::Elements(vec![HtmlElement::new(
                "td",
                vec![],
                Some(HtmlValue::Content(String::from("Hello World"))),
            )])),
        )])),
    ),
    "<table>"
    "    <tr id=\"tr1\">"
    "        <td>"
    "            Hello World"
    "        </td>"
    "    </tr>"
    "</table>"
);

#[test]
fn html_element_visitor() {
    struct Visitor(usize);

    impl HtmlVisitor for Visitor {
        fn visit_element(&mut self, _: &HtmlElement) -> bool {
            self.0 += 1;
            true
        }
    }

    let table = HtmlElement::new(
        "table",
        vec![],
        Some(HtmlValue::Elements(vec![HtmlElement::new(
            "tr",
            vec![Attribute::new("id", "tr1")],
            Some(HtmlValue::Elements(vec![
                HtmlElement::new(
                    "td",
                    vec![],
                    Some(HtmlValue::Content(String::from("Hello World"))),
                ),
                HtmlElement::new(
                    "td",
                    vec![],
                    Some(HtmlValue::Content(String::from("Hello"))),
                ),
                HtmlElement::new(
                    "td",
                    vec![],
                    Some(HtmlValue::Content(String::from("World"))),
                ),
            ])),
        )])),
    );

    let mut visitor = Visitor(0);
    table.visit(&mut visitor);

    assert_eq!(visitor.0, 5)
}

#[test]
fn html_element_visitor_mut() {
    struct Visitor;

    impl HtmlVisitorMut for Visitor {
        fn visit_element_mut(&mut self, e: &mut HtmlElement) -> bool {
            if e.tag() == "td" {
                *e = HtmlElement::new(
                    "p",
                    vec![],
                    Some(HtmlValue::Content("Hello World".to_owned())),
                )
            }

            true
        }
    }

    let mut table = HtmlElement::new(
        "table",
        vec![],
        Some(HtmlValue::Elements(vec![HtmlElement::new(
            "tr",
            vec![Attribute::new("id", "tr1")],
            Some(HtmlValue::Elements(vec![
                HtmlElement::new("td", vec![], Some(HtmlValue::Content(String::from("1234")))),
                HtmlElement::new(
                    "td",
                    vec![],
                    Some(HtmlValue::Content(String::from("Hello"))),
                ),
                HtmlElement::new(
                    "td",
                    vec![],
                    Some(HtmlValue::Content(String::from("World"))),
                ),
            ])),
        )])),
    );

    let mut visitor = Visitor;
    table.visit_mut(&mut visitor);

    assert_table!(
        table,
        "<table>"
        "    <tr id=\"tr1\">"
        "        <p>"
        "            Hello World"
        "        </p>"
        "        <p>"
        "            Hello World"
        "        </p>"
        "        <p>"
        "            Hello World"
        "        </p>"
        "    </tr>"
        "</table>"
    );
}
