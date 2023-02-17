use table_to_html::html::{Attribute, HtmlElement, HtmlValue, HtmlVisitor, HtmlVisitorMut};

#[test]
fn html_built_element() {
    let table = HtmlElement::new(
        "table",
        vec![],
        Some(HtmlValue::Elements(vec![HtmlElement::new(
            "tr",
            vec![Attribute::new("id", "tr1")],
            Some(HtmlValue::Elements(vec![HtmlElement::new(
                "td",
                vec![],
                Some(HtmlValue::Content(String::from("Hello Wolrd"))),
            )])),
        )])),
    );

    let buf = table.to_string();

    assert_eq!(
        buf,
        concat!(
            "<table>\n",
            "    <tr id=\"tr1\">\n",
            "        <td>\n",
            "            Hello Wolrd\n",
            "        </td>\n",
            "    </tr>\n",
            "</table>"
        )
    )
}

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
                    Some(HtmlValue::Content(String::from("Hello Wolrd"))),
                ),
                HtmlElement::new(
                    "td",
                    vec![],
                    Some(HtmlValue::Content(String::from("Hello"))),
                ),
                HtmlElement::new(
                    "td",
                    vec![],
                    Some(HtmlValue::Content(String::from("Wolrd"))),
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
    struct Visitor(usize);

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
                    Some(HtmlValue::Content(String::from("Wolrd"))),
                ),
            ])),
        )])),
    );

    let mut visitor = Visitor(0);
    table.visit_mut(&mut visitor);

    let buf = table.to_string();

    assert_eq!(
        buf,
        concat!(
            "<table>\n",
            "    <tr id=\"tr1\">\n",
            "        <p>\n",
            "            Hello World\n",
            "        </p>\n",
            "        <p>\n",
            "            Hello World\n",
            "        </p>\n",
            "        <p>\n",
            "            Hello World\n",
            "        </p>\n",
            "    </tr>\n",
            "</table>"
        )
    );
}
