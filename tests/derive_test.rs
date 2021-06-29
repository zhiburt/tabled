use tabled::Tabled;

mod tupple_structure {
    use super::*;

    #[test]
    fn rename_field() {
        #[derive(Tabled)]
        struct St(u8, #[header("field 2")] &'static str);

        let st = St(0, "123");

        assert_eq!(vec!["0".to_owned(), "123".to_owned()], st.fields());
        assert_eq!(vec!["0".to_owned(), "field 2".to_owned()], St::headers());
    }

    #[test]
    fn hide_field() {
        #[derive(Tabled)]
        struct St(
            #[header(hidden = true)] u8,
            #[header("field 2", hidden)] &'static str,
            &'static str,
        );

        let st = St(0, "v2", "123");

        assert_eq!(vec!["123".to_owned()], st.fields());
        assert_eq!(vec!["2".to_owned()], St::headers());
    }

    #[allow(dead_code)]
    #[test]
    fn display_with() {
        fn display_option(o: &Option<&'static str>) -> String {
            match o {
                Some(s) => format!("some {}", s),
                None => "none".to_string(),
            }
        }

        #[derive(Tabled)]
        struct St(
            u8,
            #[field(display_with = "display_option")] Option<&'static str>,
        );

        let st = St(0, Some("v2"));

        assert_eq!(vec!["0".to_owned(), "some v2".to_owned()], st.fields());
        assert_eq!(vec!["0".to_owned(), "1".to_owned()], St::headers());
    }

    #[allow(dead_code)]
    #[test]
    fn display_with_self_static_method() {
        #[derive(Tabled)]
        struct St(
            u8,
            #[field(display_with = "Self::display_option")] Option<&'static str>,
        );

        impl St {
            fn display_option(o: &Option<&'static str>) -> String {
                match o {
                    Some(s) => format!("some {}", s),
                    None => "none".to_string(),
                }
            }
        }

        let st = St(0, Some("v2"));

        assert_eq!(vec!["0".to_owned(), "some v2".to_owned()], st.fields());
        assert_eq!(vec!["0".to_owned(), "1".to_owned()], St::headers());
    }

    #[test]
    fn tuple() {
        #[derive(Tabled)]
        struct St(u8, &'static str);
        let st = St(0, "v2");

        assert_eq!(vec!["0".to_owned(), "v2".to_owned()], st.fields());
        assert_eq!(vec!["0".to_owned(), "1".to_owned()], St::headers());
    }

    #[test]
    fn empty() {
        #[derive(Tabled)]
        struct St();
        let st = St();

        assert!(st.fields().is_empty());
        assert!(St::headers().is_empty());
    }

    #[test]
    fn with_lifetime() {
        #[derive(Tabled)]
        struct St<'a>(&'a i8);
        let st = St(&1);

        assert_eq!(vec!["0".to_owned()], St::headers());
        assert_eq!(vec!["1".to_owned()], st.fields());
    }

    #[test]
    fn with_generic() {
        #[derive(Tabled)]
        struct St<T: std::fmt::Display>(T);

        fn infer_type<T: std::fmt::Display>(v: T) -> (Vec<String>, Vec<String>) {
            let st = St(v);
            (<St<T> as Tabled>::headers(), st.fields())
        }

        let (headers, fields) = infer_type(1);

        assert_eq!(vec!["0".to_owned()], headers);
        assert_eq!(vec!["1".to_owned()], fields);
    }
}

mod enum_ {
    use super::*;

    #[test]
    fn basic() {
        #[allow(dead_code)]
        #[derive(Tabled)]
        enum Domain {
            Security,
            Embeded,
            Frontend,
            Unknown,
        }

        assert_eq!(
            vec![
                "Security".to_owned(),
                "Embeded".to_owned(),
                "Frontend".to_owned(),
                "Unknown".to_owned(),
            ],
            Domain::headers()
        );

        assert_eq!(
            vec!["+".to_owned(), "".to_owned(), "".to_owned(), "".to_owned(),],
            Domain::Security.fields()
        );
    }

    #[test]
    fn diverse_enum() {
        #[allow(dead_code)]
        #[derive(Tabled)]
        enum E {
            A { a: u8, b: i32 },
            B(String),
            K,
        }

        assert_eq!(
            vec!["A".to_owned(), "B".to_owned(), "K".to_owned()],
            E::headers()
        );
        assert_eq!(
            vec!["+".to_owned(), "".to_owned(), "".to_owned()],
            E::A { a: 1, b: 2 }.fields()
        );
        assert_eq!(
            vec!["".to_owned(), "".to_owned(), "+".to_owned()],
            E::K.fields()
        );
        assert_eq!(
            vec!["".to_owned(), "+".to_owned(), "".to_owned()],
            E::B(String::new()).fields()
        );
    }

    #[test]
    fn rename_variant() {
        #[allow(dead_code)]
        #[derive(Tabled)]
        enum E {
            #[header("Variant 1")]
            A {
                a: u8,
                b: i32,
            },
            #[header(name = "Variant 2")]
            B(String),
            K,
        }

        assert_eq!(
            vec![
                "Variant 1".to_owned(),
                "Variant 2".to_owned(),
                "K".to_owned()
            ],
            E::headers()
        );
    }

    #[test]
    fn hide_variant() {
        #[allow(dead_code)]
        #[derive(Tabled)]
        enum E {
            A {
                a: u8,
                b: i32,
            },
            #[header(hidden = true)]
            B(String),
            K,
        }

        assert_eq!(vec!["A".to_owned(), "K".to_owned()], E::headers());
        assert_eq!(
            vec!["+".to_owned(), "".to_owned()],
            E::A { a: 1, b: 2 }.fields()
        );
        assert_eq!(vec!["".to_owned(), "+".to_owned()], E::K.fields());
        assert!(E::B(String::new()).fields().is_empty());
    }

    #[test]
    fn inline_variant() {
        #[derive(Tabled)]
        enum Vehicle {
            #[header(inline("Auto::"))]
            Auto {
                #[header("mod")]
                model: &'static str,
                engine: &'static str,
            },
            #[header(inline)]
            Bikecycle(#[header("name")] &'static str, #[header(inline)] Bike),
            Skateboard,
        }

        #[derive(Tabled)]
        struct Bike {
            brand: &'static str,
            price: f32,
        }

        assert_eq!(
            vec![
                "Auto::mod".to_owned(),
                "Auto::engine".to_owned(),
                "name".to_owned(),
                "brand".to_owned(),
                "price".to_owned(),
                "Skateboard".to_owned(),
            ],
            Vehicle::headers()
        );

        assert_eq!(
            vec![
                "".to_owned(),
                "".to_owned(),
                "".to_owned(),
                "".to_owned(),
                "".to_owned(),
                "+".to_owned(),
            ],
            Vehicle::Skateboard.fields()
        );

        assert_eq!(
            vec![
                "Mini".to_owned(),
                "v8".to_owned(),
                "".to_owned(),
                "".to_owned(),
                "".to_owned(),
                "".to_owned(),
            ],
            Vehicle::Auto {
                model: "Mini",
                engine: "v8"
            }
            .fields()
        );
        assert_eq!(
            vec![
                "".to_owned(),
                "".to_owned(),
                "A bike".to_owned(),
                "Canyon".to_owned(),
                "2000".to_owned(),
                "".to_owned(),
            ],
            Vehicle::Bikecycle(
                "A bike",
                Bike {
                    brand: "Canyon",
                    price: 2000.0
                }
            )
            .fields()
        );
    }

    #[test]
    fn inline_field_with_display_function() {
        #[derive(Tabled)]
        enum Developer {
            #[header(inline("backend::"))]
            Backend {
                #[header("name")]
                #[field(display_with = "display")]
                specific: &'static str,
            },
            Frontend,
        }
        fn display(_: &'static str) -> String {
            "asd".to_string()
        }

        assert_eq!(
            vec!["backend::name".to_owned(), "Frontend".to_owned()],
            Developer::headers()
        );
        assert_eq!(
            vec!["asd".to_owned(), "".to_owned()],
            Developer::Backend { specific: "123" }.fields(),
        );
        assert_eq!(
            vec!["".to_owned(), "+".to_owned()],
            Developer::Frontend.fields(),
        );
    }

    #[test]
    fn with_display() {
        #[allow(dead_code)]
        #[derive(Tabled)]
        enum Fact {
            #[field(inline)]
            Known(#[field(display_with = "Self::format::<4>")] &'static str),
            Unknown,
        }

        impl Fact {
            fn format<const ID: usize>(_: &'static str) -> String {
                ID.to_string()
            }
        }

        assert_eq!(vec!["0".to_owned(), "Unknown".to_owned(),], Fact::headers());
        assert_eq!(
            vec!["4".to_owned(), "".to_owned(),],
            Fact::Known("Hello World").fields()
        );
        assert_eq!(vec!["".to_owned(), "+".to_owned(),], Fact::Unknown.fields());
    }
}

mod unit {
    use super::*;

    #[test]
    fn basic() {
        #[derive(Tabled)]
        struct St;
        let st = St;

        assert!(st.fields().is_empty());
        assert!(St::headers().is_empty());
    }
}

mod structure {
    use super::*;

    #[test]
    fn general() {
        #[derive(Tabled)]
        struct St {
            f1: u8,
            f2: &'static str,
        }

        let st = St { f1: 0, f2: "v2" };
        assert_eq!(vec!["0".to_owned(), "v2".to_owned()], st.fields());
        assert_eq!(vec!["f1".to_owned(), "f2".to_owned()], St::headers());
    }

    #[test]
    fn empty() {
        #[derive(Tabled)]
        struct St {}

        let st = St {};
        assert!(st.fields().is_empty());
        assert!(St::headers().is_empty());
    }

    #[test]
    fn rename_field() {
        #[derive(Tabled)]
        struct St {
            #[header(name = "field 1")]
            f1: u8,
            #[header("field 2")]
            f2: &'static str,
        }

        let st = St { f1: 0, f2: "v2" };
        assert_eq!(vec!["0".to_owned(), "v2".to_owned()], st.fields());
        assert_eq!(
            vec!["field 1".to_owned(), "field 2".to_owned()],
            St::headers()
        );
    }

    #[allow(dead_code)]
    #[test]
    fn hide_field() {
        #[derive(Tabled)]
        struct St {
            #[header(hidden = true)]
            f1: u8,
            #[header("field 2", hidden)]
            f2: &'static str,
            f3: &'static str,
        }

        let st = St {
            f1: 0,
            f2: "v2",
            f3: "123",
        };

        assert_eq!(vec!["123".to_owned()], st.fields());
        assert_eq!(vec!["f3".to_owned()], St::headers());
    }

    #[test]
    fn inline() {
        #[derive(Tabled)]
        struct Person {
            #[header(inline = true)]
            id: u8,
            name: &'static str,
            #[header(inline)]
            ed: Education,
        }

        #[derive(Tabled)]
        struct Education {
            uni: &'static str,
            graduated: bool,
        }

        let p = Person {
            id: 0,
            name: "Maxim",
            ed: Education {
                uni: "BNTU",
                graduated: true,
            },
        };

        assert_eq!(
            vec![
                "0".to_owned(),
                "Maxim".to_owned(),
                "BNTU".to_owned(),
                "true".to_owned()
            ],
            p.fields()
        );
        assert_eq!(
            vec![
                "u8".to_owned(),
                "name".to_owned(),
                "uni".to_owned(),
                "graduated".to_owned()
            ],
            Person::headers()
        );
    }

    #[test]
    fn inline_with_rename_prefix() {
        #[derive(Tabled)]
        struct Person {
            #[header("it's an ignored option", inline)] // does nothing
            id: u8,
            name: &'static str,
            #[header(inline("education::"))]
            ed: Education,
        }

        #[derive(Tabled)]
        struct Education {
            uni: &'static str,
            graduated: bool,
        }

        let p = Person {
            id: 0,
            name: "Maxim",
            ed: Education {
                uni: "BNTU",
                graduated: true,
            },
        };

        assert_eq!(
            vec![
                "0".to_owned(),
                "Maxim".to_owned(),
                "BNTU".to_owned(),
                "true".to_owned()
            ],
            p.fields()
        );
        assert_eq!(
            vec![
                "u8".to_owned(),
                "name".to_owned(),
                "education::uni".to_owned(),
                "education::graduated".to_owned()
            ],
            Person::headers()
        );
    }

    #[allow(dead_code)]
    #[test]
    fn display_with() {
        fn display_option(o: &Option<&'static str>) -> String {
            match o {
                Some(s) => format!("some {}", s),
                None => "none".to_string(),
            }
        }

        #[derive(Tabled)]
        struct St {
            f1: u8,
            #[field(display_with = "display_option")]
            f2: Option<&'static str>,
        }

        let st = St {
            f1: 0,
            f2: Some("v2"),
        };

        assert_eq!(vec!["0".to_owned(), "some v2".to_owned()], st.fields());
        assert_eq!(vec!["f1".to_owned(), "f2".to_owned()], St::headers());
    }

    #[allow(dead_code)]
    #[test]
    fn display_with_self_static_method() {
        #[derive(Tabled)]
        struct St {
            f1: u8,
            #[field(display_with = "Self::display_option")]
            f2: Option<&'static str>,
        }

        impl St {
            fn display_option(o: &Option<&'static str>) -> String {
                match o {
                    Some(s) => format!("some {}", s),
                    None => "none".to_string(),
                }
            }
        }

        let st = St {
            f1: 0,
            f2: Some("v2"),
        };

        assert_eq!(vec!["0".to_owned(), "some v2".to_owned()], st.fields());
        assert_eq!(vec!["f1".to_owned(), "f2".to_owned()], St::headers());
    }
}
