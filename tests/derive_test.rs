use tabled::Tabled;

mod tupple_structure {
    use super::*;

    #[test]
    fn rename_tabled() {
        #[derive(Tabled)]
        struct St(u8, #[tabled(rename = "field 2")] &'static str);

        let st = St(0, "123");

        assert_eq!(vec!["0".to_owned(), "123".to_owned()], st.fields());
        assert_eq!(vec!["0".to_owned(), "field 2".to_owned()], St::headers());
    }

    #[test]
    fn skip_tabled() {
        #[derive(Tabled)]
        struct St(
            #[tabled(skip)] u8,
            #[tabled(rename = "field 2", skip)] &'static str,
            &'static str,
        );

        let st = St(0, "v2", "123");

        assert_eq!(vec!["123".to_owned()], st.fields());
        assert_eq!(vec!["2".to_owned()], St::headers());
        assert_eq!(St::LENGTH, 1);
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
            #[tabled(display_with = "display_option")] Option<&'static str>,
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
            #[tabled(display_with = "Self::display_option")] Option<&'static str>,
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
        assert_eq!(St::LENGTH, 2);
    }

    #[test]
    fn empty() {
        #[derive(Tabled)]
        struct St();
        let st = St();

        assert!(st.fields().is_empty());
        assert!(St::headers().is_empty());
        assert_eq!(St::LENGTH, 0);
    }

    #[test]
    fn with_lifetime() {
        #[derive(Tabled)]
        struct St<'a>(&'a i8);
        let st = St(&1);

        assert_eq!(vec!["0".to_owned()], St::headers());
        assert_eq!(vec!["1".to_owned()], st.fields());
        assert_eq!(St::LENGTH, 1);
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

        assert_eq!(St::<String>::LENGTH, 1);
    }

    #[test]
    fn unit_order_tabled() {
        {
            #[derive(Tabled)]
            struct St(#[tabled(order = 0)] u8, u8, u8);

            let st = St(0, 1, 2);

            assert_eq!(vec!["0", "1", "2"], st.fields());
            assert_eq!(vec!["0", "1", "2"], St::headers());
        }
        {
            #[derive(Tabled)]
            struct St(#[tabled(order = 1)] u8, u8, u8);

            let st = St(0, 1, 2);

            assert_eq!(vec!["1", "0", "2"], st.fields());
            assert_eq!(vec!["1", "0", "2"], St::headers());
        }
        {
            #[derive(Tabled)]
            struct St(#[tabled(order = 2)] u8, u8, u8);

            let st = St(0, 1, 2);

            assert_eq!(vec!["1", "2", "0"], st.fields());
            assert_eq!(vec!["1", "2", "0"], St::headers());
        }

        {
            #[derive(Tabled)]
            struct St(u8, #[tabled(order = 0)] u8, u8);

            let st = St(0, 1, 2);

            assert_eq!(vec!["1", "0", "2"], st.fields());
            assert_eq!(vec!["1", "0", "2"], St::headers());
        }
        {
            #[derive(Tabled)]
            struct St(u8, #[tabled(order = 1)] u8, u8);

            let st = St(0, 1, 2);

            assert_eq!(vec!["0", "1", "2"], st.fields());
            assert_eq!(vec!["0", "1", "2"], St::headers());
        }
        {
            #[derive(Tabled)]
            struct St(u8, #[tabled(order = 2)] u8, u8);

            let st = St(0, 1, 2);

            assert_eq!(vec!["0", "2", "1"], st.fields());
            assert_eq!(vec!["0", "2", "1"], St::headers());
        }

        {
            #[derive(Tabled)]
            struct St(u8, u8, #[tabled(order = 0)] u8);

            let st = St(0, 1, 2);

            assert_eq!(vec!["2", "0", "1"], st.fields());
            assert_eq!(vec!["2", "0", "1"], St::headers());
        }
        {
            #[derive(Tabled)]
            struct St(u8, u8, #[tabled(order = 1)] u8);

            let st = St(0, 1, 2);

            assert_eq!(vec!["0", "2", "1"], st.fields());
            assert_eq!(vec!["0", "2", "1"], St::headers());
        }
        {
            #[derive(Tabled)]
            struct St(u8, u8, #[tabled(order = 2)] u8);

            let st = St(0, 1, 2);

            assert_eq!(vec!["0", "1", "2"], st.fields());
            assert_eq!(vec!["0", "1", "2"], St::headers());
        }

        {
            #[derive(Tabled)]
            struct St(#[tabled(order = 2)] u8, u8, #[tabled(order = 0)] u8);

            let st = St(0, 1, 2);

            assert_eq!(vec!["2", "1", "0"], st.fields());
            assert_eq!(vec!["2", "1", "0"], St::headers());
        }
        {
            #[derive(Tabled)]
            struct St(#[tabled(order = 2)] u8, #[tabled(order = 1)] u8, u8);

            let st = St(0, 1, 2);

            assert_eq!(vec!["2", "1", "0"], st.fields());
            assert_eq!(vec!["2", "1", "0"], St::headers());
        }
        {
            #[derive(Tabled)]
            struct St(
                #[tabled(order = 2)] u8,
                #[tabled(order = 2)] u8,
                #[tabled(order = 1)] u8,
            );

            let st = St(0, 1, 2);

            assert_eq!(vec!["0", "2", "1"], st.fields());
            assert_eq!(vec!["0", "2", "1"], St::headers());
        }
        {
            #[derive(Tabled)]
            struct St(
                #[tabled(order = 2)] u8,
                #[tabled(order = 2)] u8,
                #[tabled(order = 2)] u8,
            );

            let st = St(0, 1, 2);

            assert_eq!(vec!["0", "1", "2"], st.fields());
            assert_eq!(vec!["0", "1", "2"], St::headers());
        }
        {
            #[derive(Tabled)]
            struct St(
                #[tabled(order = 1)] u8,
                #[tabled(order = 1)] u8,
                #[tabled(order = 1)] u8,
            );

            let st = St(0, 1, 2);

            assert_eq!(vec!["0", "2", "1"], st.fields());
            assert_eq!(vec!["0", "2", "1"], St::headers());
        }
        {
            #[derive(Tabled)]
            struct St(
                #[tabled(order = 2)] u8,
                #[tabled(order = 1)] u8,
                #[tabled(order = 0)] u8,
            );

            let st = St(0, 1, 2);

            assert_eq!(vec!["2", "1", "0"], st.fields());
            assert_eq!(vec!["2", "1", "0"], St::headers());
        }
    }

    // #[test]
    // fn order_compile_fail_when_order_is_bigger_then_count_fields() {
    //     #[derive(Tabled)]
    //     struct St(#[tabled(order = 3)] u8, u8, u8);
    // }
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
        assert_eq!(Domain::LENGTH, 4);
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
        assert_eq!(E::LENGTH, 3);
    }

    #[test]
    fn rename_variant() {
        #[allow(dead_code)]
        #[derive(Tabled)]
        enum E {
            #[tabled(rename = "Variant 1")]
            A {
                a: u8,
                b: i32,
            },
            #[tabled(rename = "Variant 2")]
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
    fn skip_variant() {
        #[allow(dead_code)]
        #[derive(Tabled)]
        enum E {
            A {
                a: u8,
                b: i32,
            },
            #[tabled(skip)]
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
        assert_eq!(E::LENGTH, 2);
    }

    #[test]
    fn inline_variant() {
        #[derive(Tabled)]
        enum Vehicle {
            #[tabled(inline("Auto::"))]
            Auto {
                #[tabled(rename = "mod")]
                model: &'static str,
                engine: &'static str,
            },
            #[tabled(inline)]
            Bikecycle(
                #[tabled(rename = "name")] &'static str,
                #[tabled(inline)] Bike,
            ),
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

        assert_eq!(Vehicle::LENGTH, 6);
    }

    #[test]
    fn inline_field_with_display_function() {
        #[derive(Tabled)]
        enum Developer {
            #[tabled(inline("backend::"))]
            Backend {
                #[tabled(rename = "name")]
                #[tabled(display_with = "display")]
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
            #[tabled(inline)]
            Known(#[tabled(display_with = "Self::format::<4>")] &'static str),
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
        assert_eq!(St::LENGTH, 0);
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
        assert_eq!(St::LENGTH, 2);
    }

    #[test]
    fn empty() {
        #[derive(Tabled)]
        struct St {}

        let st = St {};
        assert!(st.fields().is_empty());
        assert!(St::headers().is_empty());
        assert_eq!(St::LENGTH, 0);
    }

    #[test]
    fn rename_tabled() {
        #[derive(Tabled)]
        struct St {
            #[tabled(rename = "field 1")]
            f1: u8,
            #[tabled(rename = "field 2")]
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
    fn skip_tabled() {
        #[derive(Tabled)]
        struct St {
            #[tabled(skip)]
            f1: u8,
            #[tabled(rename = "field 2", skip)]
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
        assert_eq!(St::LENGTH, 1);
    }

    #[allow(dead_code)]
    #[test]
    fn skip_true_tabled() {
        #[derive(Tabled)]
        struct St {
            #[tabled(skip = true)]
            f1: u8,
            #[tabled(rename = "field 2", skip = true)]
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
        assert_eq!(St::LENGTH, 1);
    }

    #[test]
    fn inline() {
        #[derive(Tabled)]
        struct Person {
            #[tabled(inline = true)]
            id: u8,
            name: &'static str,
            #[tabled(inline)]
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

        assert_eq!(Person::LENGTH, 4);
    }

    #[test]
    fn inline_with_rename_prefix() {
        #[derive(Tabled)]
        struct Person {
            #[tabled(rename = "it's an ignored option", inline)] // does nothing
            id: u8,
            name: &'static str,
            #[tabled(inline("education::"))]
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
            #[tabled(display_with = "display_option")]
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
            #[tabled(display_with = "Self::display_option")]
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

    #[test]
    fn order_tabled() {
        {
            #[derive(Tabled)]
            struct St {
                #[tabled(order = 0)]
                f0: u8,
                f1: u8,
                f2: u8,
            }

            let st = St {
                f0: 0,
                f1: 1,
                f2: 2,
            };

            assert_eq!(vec!["0", "1", "2"], st.fields());
            assert_eq!(vec!["f0", "f1", "f2"], St::headers());
        }
        {
            #[derive(Tabled)]
            struct St {
                #[tabled(order = 1)]
                f0: u8,
                f1: u8,
                f2: u8,
            }

            let st = St {
                f0: 0,
                f1: 1,
                f2: 2,
            };

            assert_eq!(vec!["1", "0", "2"], st.fields());
            assert_eq!(vec!["f1", "f0", "f2"], St::headers());
        }
        {
            #[derive(Tabled)]
            struct St {
                #[tabled(order = 2)]
                f0: u8,
                f1: u8,
                f2: u8,
            }

            let st = St {
                f0: 0,
                f1: 1,
                f2: 2,
            };

            assert_eq!(vec!["1", "2", "0"], st.fields());
            assert_eq!(vec!["f1", "f2", "f0"], St::headers());
        }

        {
            #[derive(Tabled)]
            struct St {
                f0: u8,
                #[tabled(order = 0)]
                f1: u8,
                f2: u8,
            }

            let st = St {
                f0: 0,
                f1: 1,
                f2: 2,
            };

            assert_eq!(vec!["1", "0", "2"], st.fields());
            assert_eq!(vec!["f1", "f0", "f2"], St::headers());
        }
        {
            #[derive(Tabled)]
            struct St {
                f0: u8,
                #[tabled(order = 1)]
                f1: u8,
                f2: u8,
            }

            let st = St {
                f0: 0,
                f1: 1,
                f2: 2,
            };

            assert_eq!(vec!["0", "1", "2"], st.fields());
            assert_eq!(vec!["f0", "f1", "f2"], St::headers());
        }
        {
            #[derive(Tabled)]
            struct St {
                f0: u8,
                #[tabled(order = 2)]
                f1: u8,
                f2: u8,
            }

            let st = St {
                f0: 0,
                f1: 1,
                f2: 2,
            };

            assert_eq!(vec!["0", "2", "1"], st.fields());
            assert_eq!(vec!["f0", "f2", "f1"], St::headers());
        }

        {
            #[derive(Tabled)]
            struct St {
                f0: u8,
                f1: u8,
                #[tabled(order = 0)]
                f2: u8,
            }

            let st = St {
                f0: 0,
                f1: 1,
                f2: 2,
            };

            assert_eq!(vec!["2", "0", "1"], st.fields());
            assert_eq!(vec!["f2", "f0", "f1"], St::headers());
        }
        {
            #[derive(Tabled)]
            struct St {
                f0: u8,
                f1: u8,
                #[tabled(order = 1)]
                f2: u8,
            }

            let st = St {
                f0: 0,
                f1: 1,
                f2: 2,
            };

            assert_eq!(vec!["0", "2", "1"], st.fields());
            assert_eq!(vec!["f0", "f2", "f1"], St::headers());
        }
        {
            #[derive(Tabled)]
            struct St {
                f0: u8,
                f1: u8,
                #[tabled(order = 2)]
                f2: u8,
            }

            let st = St {
                f0: 0,
                f1: 1,
                f2: 2,
            };

            assert_eq!(vec!["0", "1", "2"], st.fields());
            assert_eq!(vec!["f0", "f1", "f2"], St::headers());
        }

        {
            #[derive(Tabled)]
            struct St {
                #[tabled(order = 2)]
                f0: u8,
                f1: u8,
                #[tabled(order = 0)]
                f2: u8,
            }

            let st = St {
                f0: 0,
                f1: 1,
                f2: 2,
            };

            assert_eq!(vec!["2", "1", "0"], st.fields());
            assert_eq!(vec!["f2", "f1", "f0"], St::headers());
        }
        {
            #[derive(Tabled)]
            struct St {
                #[tabled(order = 2)]
                f0: u8,
                #[tabled(order = 1)]
                f1: u8,
                f2: u8,
            }

            let st = St {
                f0: 0,
                f1: 1,
                f2: 2,
            };

            assert_eq!(vec!["2", "1", "0"], st.fields());
            assert_eq!(vec!["f2", "f1", "f0"], St::headers());
        }
        {
            #[derive(Tabled)]
            struct St {
                #[tabled(order = 2)]
                f0: u8,
                #[tabled(order = 2)]
                f1: u8,
                #[tabled(order = 1)]
                f2: u8,
            }

            let st = St {
                f0: 0,
                f1: 1,
                f2: 2,
            };

            assert_eq!(vec!["0", "2", "1"], st.fields());
            assert_eq!(vec!["f0", "f2", "f1"], St::headers());
        }
        {
            #[derive(Tabled)]
            struct St {
                #[tabled(order = 2)]
                f0: u8,
                #[tabled(order = 2)]
                f1: u8,
                #[tabled(order = 2)]
                f2: u8,
            }

            let st = St {
                f0: 0,
                f1: 1,
                f2: 2,
            };

            assert_eq!(vec!["0", "1", "2"], st.fields());
            assert_eq!(vec!["f0", "f1", "f2"], St::headers());
        }
        {
            #[derive(Tabled)]
            struct St {
                #[tabled(order = 2)]
                f0: u8,
                #[tabled(order = 1)]
                f1: u8,
                #[tabled(order = 0)]
                f2: u8,
            }

            let st = St {
                f0: 0,
                f1: 1,
                f2: 2,
            };

            assert_eq!(vec!["2", "1", "0"], st.fields());
            assert_eq!(vec!["f2", "f1", "f0"], St::headers());
        }
    }

    // #[test]
    // fn order_compile_fail_when_order_is_bigger_then_count_fields() {
    //     #[derive(Tabled)]
    //     struct St {
    //         #[tabled(order = 3)]
    //         f0: u8,
    //         f1: u8,
    //         f2: u8,
    //     }
    // }
}

#[test]
fn skipped_fields_may_not_implement_display() {
    {
        struct Something;

        #[derive(Tabled)]
        struct TupleStruct(#[tabled(skip)] Something, &'static str);

        let st = TupleStruct(Something, "nrdxp");

        assert_eq!(vec!["nrdxp".to_owned()], st.fields());
        assert_eq!(vec!["1".to_owned()], TupleStruct::headers());
    }

    {
        struct Something;

        #[derive(Tabled)]
        struct Struct {
            #[tabled(skip)]
            _gem: Something,
            name: &'static str,
        }

        let st = Struct {
            _gem: Something,
            name: "nrdxp",
        };

        assert_eq!(vec!["nrdxp".to_owned()], st.fields());
        assert_eq!(vec!["name".to_owned()], Struct::headers());
    }

    {
        struct Something;

        #[derive(Tabled)]
        struct Struct1 {
            #[tabled(skip = true)]
            _field1: Something,
            field2: &'static str,
        }

        #[derive(Tabled)]
        struct Struct2 {
            field1: &'static str,
            #[tabled(inline)]
            field2: Struct1,
        }

        let st = Struct2 {
            field1: "nrdxp",
            field2: Struct1 {
                _field1: Something,
                field2: "...",
            },
        };

        assert_eq!(
            vec!["field1".to_owned(), "field2".to_owned()],
            Struct2::headers()
        );
        assert_eq!(vec!["nrdxp".to_owned(), "...".to_owned()], st.fields());
    }

    {
        struct Something;

        #[derive(Tabled)]
        enum Enum {
            #[tabled(inline("A::"))]
            A {
                name: &'static str,
            },
            #[tabled(inline("B::"))]
            B {
                issue: usize,
                #[tabled(skip)]
                _gem: Something,
                name: &'static str,
            },
            #[tabled(inline("C::"))]
            C(usize, #[tabled(skip)] Something, &'static str),
            D,
        }

        assert_eq!(
            vec![
                "A::name".to_owned(),
                "B::issue".to_owned(),
                "B::name".to_owned(),
                "C::0".to_owned(),
                "C::2".to_owned(),
                "D".to_owned()
            ],
            Enum::headers()
        );

        let st = Enum::A { name: "nrdxp" };
        assert_eq!(
            vec![
                "nrdxp".to_owned(),
                "".to_owned(),
                "".to_owned(),
                "".to_owned(),
                "".to_owned(),
                "".to_owned()
            ],
            st.fields()
        );

        let st = Enum::B {
            _gem: Something,
            issue: 32,
            name: "nrdxp",
        };
        assert_eq!(
            vec![
                "".to_owned(),
                "32".to_owned(),
                "nrdxp".to_owned(),
                "".to_owned(),
                "".to_owned(),
                "".to_owned()
            ],
            st.fields()
        );

        let st = Enum::C(32, Something, "nrdxp");
        assert_eq!(
            vec![
                "".to_owned(),
                "".to_owned(),
                "".to_owned(),
                "32".to_owned(),
                "nrdxp".to_owned(),
                "".to_owned()
            ],
            st.fields()
        );

        let st = Enum::D;
        assert_eq!(
            vec![
                "".to_owned(),
                "".to_owned(),
                "".to_owned(),
                "".to_owned(),
                "".to_owned(),
                "+".to_owned()
            ],
            st.fields()
        );
    }
}
