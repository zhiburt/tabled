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
    fn display_with_static_method() {
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

    #[allow(dead_code)]
    #[test]
    fn display_with_self_static_method() {
        #[derive(Tabled)]
        struct St(
            u8,
            #[tabled(display_with("Self::display_option", args))] Option<&'static str>,
        );

        impl St {
            fn display_option(&self) -> String {
                match self.1 {
                    Some(s) => format!("some {}", s),
                    None => "none".to_string(),
                }
            }
        }

        let st = St(0, Some("v2"));

        assert_eq!(vec!["0".to_owned(), "some v2".to_owned()], st.fields());
        assert_eq!(vec!["0".to_owned(), "1".to_owned()], St::headers());
    }

    #[allow(dead_code)]
    #[test]
    fn display_with_self_static_method2() {
        #[derive(Tabled)]
        struct St(
            u8,
            #[tabled(display_with("Self::display_option", args))] Option<&'static str>,
        );

        impl St {
            fn display_option(o: &St) -> String {
                match o.1 {
                    Some(s) => format!("some {}", s),
                    None => "none".to_string(),
                }
            }
        }

        let st = St(0, Some("v2"));

        assert_eq!(vec!["0".to_owned(), "some v2".to_owned()], st.fields());
        assert_eq!(vec!["0".to_owned(), "1".to_owned()], St::headers());
    }

    #[allow(dead_code)]
    #[test]
    fn display_with_self_func() {
        #[derive(Tabled)]
        struct St(
            u8,
            #[tabled(display_with("display_option", args))] Option<&'static str>,
        );

        fn display_option(o: &St) -> String {
            match o.1 {
                Some(s) => format!("some {}", s),
                None => "none".to_string(),
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

    #[test]
    fn rename_all_field() {
        #[derive(Tabled)]
        struct St(u8, #[tabled(rename_all = "UPPERCASE")] &'static str);

        let st = St(0, "123");

        assert_eq!(vec!["0".to_owned(), "123".to_owned()], st.fields());
        assert_eq!(vec!["0".to_owned(), "1".to_owned()], St::headers());
    }

    #[test]
    fn rename_all_field_with_rename() {
        #[derive(Tabled)]
        struct St(
            u8,
            #[tabled(rename_all = "UPPERCASE", rename = "Something")] &'static str,
        );

        let st = St(0, "123");

        assert_eq!(vec!["0".to_owned(), "123".to_owned()], st.fields());
        assert_eq!(vec!["0".to_owned(), "Something".to_owned()], St::headers());
    }

    #[test]
    fn rename_all_tuple() {
        #[derive(Tabled)]
        #[tabled(rename_all = "UPPERCASE")]
        struct St(u8, &'static str);

        let st = St(0, "123");

        assert_eq!(vec!["0".to_owned(), "123".to_owned()], st.fields());
        assert_eq!(vec!["0".to_owned(), "1".to_owned()], St::headers());
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
    fn inline_field_with_display_self_function() {
        #[derive(Tabled)]
        enum Developer {
            #[tabled(inline("backend::"))]
            Backend {
                #[tabled(rename = "name")]
                #[tabled(display_with("display", args))]
                specific: &'static str,
            },
            Frontend,
        }
        fn display(_: &Developer) -> String {
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

    #[test]
    fn with_display_self() {
        #[allow(dead_code)]
        #[derive(Tabled)]
        enum Fact {
            #[tabled(inline)]
            Known(#[tabled(display_with("Self::format::<4>", args))] &'static str),
            Unknown,
        }

        impl Fact {
            fn format<const ID: usize>(_: &Fact) -> String {
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

    #[test]
    fn rename_all_variant() {
        #[allow(dead_code)]
        #[derive(Tabled)]
        enum E {
            #[tabled(rename_all = "snake_case")]
            VariantName1 {
                a: u8,
                b: i32,
            },
            #[tabled(rename_all = "UPPERCASE")]
            VariantName2(String),
            K,
        }

        assert_eq!(E::headers(), ["variant_name1", "VARIANTNAME2", "K"]);
    }

    #[test]
    fn rename_all_enum() {
        #[allow(dead_code)]
        #[derive(Tabled)]
        #[tabled(rename_all = "snake_case")]
        enum E {
            VariantName1 { a: u8, b: i32 },
            VariantName2(String),
            K,
        }

        assert_eq!(E::headers(), ["variant_name1", "variant_name2", "k"]);
    }

    #[test]
    fn rename_all_enum_inhirited_inside_struct_enum() {
        #[allow(dead_code)]
        #[derive(Tabled)]
        #[tabled(rename_all = "snake_case")]
        enum E {
            #[tabled(inline)]
            VariantName1 {
                some_field_1: u8,
                some_field_2: i32,
            },
            VariantName2(String),
            K,
        }

        assert_eq!(
            E::headers(),
            ["some_field_1", "some_field_2", "variant_name2", "k"]
        );
    }

    #[test]
    fn rename_all_enum_inhirited_inside_struct_override_by_rename_enum() {
        #[allow(dead_code)]
        #[derive(Tabled)]
        #[tabled(rename_all = "snake_case")]
        enum E {
            #[tabled(inline)]
            VariantName1 {
                #[tabled(rename = "f1")]
                some_field_1: u8,
                #[tabled(rename = "f2")]
                some_field_2: i32,
            },
            VariantName2(String),
            K,
        }

        assert_eq!(E::headers(), ["f1", "f2", "variant_name2", "k"]);
    }

    #[test]
    fn rename_all_enum_inhirited_inside_struct_override_by_rename_all_enum() {
        #[allow(dead_code)]
        #[derive(Tabled)]
        #[tabled(rename_all = "snake_case")]
        enum E {
            #[tabled(inline)]
            VariantName1 {
                #[tabled(rename_all = "UPPERCASE")]
                some_field_1: u8,
                #[tabled(rename_all = "CamelCase")]
                some_field_2: i32,
            },
            VariantName2(String),
            K,
        }

        assert_eq!(
            E::headers(),
            ["SOMEFIELD1", "someField2", "variant_name2", "k"]
        );
    }

    #[test]
    fn rename_all_variant_inhirited_inside_struct_enum() {
        #[allow(dead_code)]
        #[derive(Tabled)]
        enum E {
            #[tabled(rename_all = "snake_case")]
            #[tabled(inline)]
            VariantName1 {
                some_field_1: u8,
                some_field_2: i32,
            },
            VariantName2(String),
            K,
        }

        assert_eq!(
            E::headers(),
            ["some_field_1", "some_field_2", "VariantName2", "K"]
        );
    }

    #[test]
    fn rename_all_variant_inhirited_inside_struct_override_by_rename_enum() {
        #[allow(dead_code)]
        #[derive(Tabled)]
        enum E {
            #[tabled(inline, rename_all = "snake_case")]
            VariantName1 {
                #[tabled(rename = "f1")]
                some_field_1: u8,
                #[tabled(rename = "f2")]
                some_field_2: i32,
            },
            VariantName2(String),
            K,
        }

        assert_eq!(E::headers(), ["f1", "f2", "VariantName2", "K"]);
    }

    #[test]
    fn rename_all_variant_inhirited_inside_struct_override_by_rename_all_enum() {
        #[allow(dead_code)]
        #[derive(Tabled)]
        enum E {
            #[tabled(rename_all = "snake_case", inline)]
            VariantName1 {
                #[tabled(rename_all = "UPPERCASE")]
                some_field_1: u8,
                #[tabled(rename_all = "CamelCase")]
                some_field_2: i32,
            },
            VariantName2(String),
            K,
        }

        assert_eq!(
            E::headers(),
            ["SOMEFIELD1", "someField2", "VariantName2", "K"]
        );
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

    #[allow(dead_code)]
    #[test]
    fn display_with_2_self_static_method() {
        #[derive(Tabled)]
        struct St {
            f1: u8,
            #[tabled(display_with("Self::display_option"))]
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

    #[allow(dead_code)]
    #[test]
    fn display_with_self_self_static_method() {
        #[derive(Tabled)]
        struct St {
            f1: u8,
            #[tabled(display_with("Self::display_option", args))]
            f2: Option<&'static str>,
        }

        impl St {
            fn display_option(o: &St) -> String {
                match o.f2 {
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

    #[allow(dead_code)]
    #[test]
    fn display_with_self_2_self_static_method() {
        #[derive(Tabled)]
        struct St {
            f1: u8,
            #[tabled(display_with("Self::display_option", args))]
            f2: Option<&'static str>,
        }

        impl St {
            fn display_option(&self) -> String {
                match self.f2 {
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

    #[allow(dead_code)]
    #[test]
    fn display_with_self_3_self_static_method() {
        #[derive(Tabled)]
        struct St {
            f1: u8,
            #[tabled(display_with("display_option", args))]
            f2: Option<&'static str>,
        }

        fn display_option(o: &St) -> String {
            match o.f2 {
                Some(s) => format!("some {}", s),
                None => "none".to_string(),
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

    #[test]
    fn rename_all_test() {
        #[derive(Tabled)]
        #[tabled(rename_all = "UPPERCASE")]
        struct St {
            f1: u8,
            f2: &'static str,
        }

        let st = St { f1: 0, f2: "v2" };
        assert_eq!(st.fields(), ["0", "v2"]);
        assert_eq!(St::headers(), ["F1", "F2"],);
    }

    #[test]
    fn rename_all_override_in_field_by_rename_test() {
        #[derive(Tabled)]
        #[tabled(rename_all = "UPPERCASE")]
        struct St {
            #[tabled(rename = "213213")]
            f1: u8,
            f2: &'static str,
        }

        let st = St { f1: 0, f2: "v2" };
        assert_eq!(st.fields(), ["0", "v2"]);
        assert_eq!(St::headers(), ["213213", "F2"],);
    }

    #[test]
    fn rename_all_override_in_field_by_rename_all_test() {
        #[derive(Tabled)]
        #[tabled(rename_all = "UPPERCASE")]
        struct St {
            #[tabled(rename_all = "lowercase")]
            f1: u8,
            f2: &'static str,
        }

        let st = St { f1: 0, f2: "v2" };
        assert_eq!(st.fields(), ["0", "v2"]);
        assert_eq!(St::headers(), ["f1", "F2"],);
    }

    #[test]
    fn rename_all_field_test() {
        #[derive(Tabled)]
        struct St {
            #[tabled(rename_all = "lowercase")]
            f1: u8,
            #[tabled(rename_all = "UPPERCASE")]
            f2: &'static str,
        }

        let st = St { f1: 0, f2: "v2" };
        assert_eq!(st.fields(), ["0", "v2"]);
        assert_eq!(St::headers(), ["f1", "F2"],);
    }

    #[test]
    fn rename_all_field_overriden_by_rename_test() {
        #[derive(Tabled)]
        struct St {
            #[tabled(rename_all = "lowercase", rename = "Hello")]
            f1: u8,
            #[tabled(rename_all = "UPPERCASE")]
            f2: &'static str,
        }

        let st = St { f1: 0, f2: "v2" };
        assert_eq!(st.fields(), ["0", "v2"]);
        assert_eq!(St::headers(), ["Hello", "F2"],);
    }
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

#[test]
fn display_with_used_with_inline() {
    #[derive(Tabled)]
    struct Struct1 {
        f1: &'static str,
        f2: &'static str,
        #[tabled(display_with = "print", inline)]
        f3: usize,
    }

    #[allow(dead_code)]
    fn print<T>(_: T) -> String {
        String::new()
    }

    let st = Struct1 {
        f1: "123",
        f2: "456",
        f3: 789,
    };

    assert_eq!(Struct1::headers(), vec!["f1", "f2", "usize"],);
    assert_eq!(st.fields(), vec!["123", "456", "789"]);
}

#[test]
fn display_with_used_with_inline_2() {
    #[derive(Tabled)]
    struct Struct1 {
        f1: &'static str,
        f2: &'static str,
        #[tabled(display_with = "print")]
        #[tabled(inline)]
        f3: usize,
    }

    #[allow(dead_code)]
    fn print<T>(_: T) -> String {
        String::new()
    }

    let st = Struct1 {
        f1: "123",
        f2: "456",
        f3: 789,
    };

    assert_eq!(Struct1::headers(), vec!["f1", "f2", "usize"],);
    assert_eq!(st.fields(), vec!["123", "456", "789"]);
}

#[test]
fn display_with_rename() {
    #[derive(Tabled)]
    struct Struct1 {
        f1: &'static str,
        f2: &'static str,
        #[tabled(display_with = "print", rename = "Field 3")]
        f3: usize,
    }

    #[allow(dead_code)]
    fn print<T>(_: T) -> String {
        String::new()
    }

    let st = Struct1 {
        f1: "123",
        f2: "456",
        f3: 789,
    };

    assert_eq!(Struct1::headers(), vec!["f1", "f2", "Field 3"],);
    assert_eq!(st.fields(), vec!["123", "456", ""]);
}

#[test]
fn display_with_rename_2() {
    #[derive(Tabled)]
    struct Struct1 {
        f1: &'static str,
        f2: &'static str,
        #[tabled(display_with = "print")]
        #[tabled(rename = "Field 3")]
        f3: usize,
    }

    #[allow(dead_code)]
    fn print<T>(_: T) -> String {
        String::new()
    }

    let st = Struct1 {
        f1: "123",
        f2: "456",
        f3: 789,
    };

    assert_eq!(Struct1::headers(), vec!["f1", "f2", "Field 3"],);
    assert_eq!(st.fields(), vec!["123", "456", ""]);
}

#[test]
fn display_with_rename_all() {
    #[derive(Tabled)]
    struct Struct1 {
        f1: &'static str,
        f2: &'static str,
        #[tabled(display_with = "print", rename_all = "UPPERCASE")]
        f3: usize,
    }

    #[allow(dead_code)]
    fn print<T>(_: T) -> String {
        String::new()
    }

    let st = Struct1 {
        f1: "123",
        f2: "456",
        f3: 789,
    };

    assert_eq!(Struct1::headers(), vec!["f1", "f2", "F3"],);
    assert_eq!(st.fields(), vec!["123", "456", ""]);
}

#[test]
fn rename_all_variants() {
    macro_rules! test_case {
        ( $name:ident, $case:expr ) => {
            #[derive(Tabled)]
            #[tabled(rename_all = $case)]
            struct $name {
                field: usize,
            }
        };
    }

    test_case!(S1, "UPPERCASE");
    test_case!(S2, "lowercase");
    test_case!(S3, "camelCase");
    test_case!(S4, "PascalCase");
    test_case!(S5, "snake_case");
    test_case!(S6, "SCREAMING_SNAKE_CASE");
    test_case!(S7, "kebab-case");
    test_case!(S8, "verbatimcase");
}

#[test]
fn wrong_rename_all_doesnt_panic_when_used_as_not_first() {
    #[derive(Tabled)]
    #[tabled(rename_all = "UPPERCASE")]
    #[tabled(rename_all = "some wrong case")]
    struct Struct1 {
        field: usize,
    }

    let st = Struct1 { field: 789 };

    assert_eq!(Struct1::headers(), vec!["FIELD"],);
    assert_eq!(st.fields(), vec!["789"]);

    #[derive(Tabled)]
    #[tabled(rename_all = "UPPERCASE", rename_all = "some wrong case")]
    struct Struct2 {
        field: usize,
    }

    let st = Struct2 { field: 789 };

    assert_eq!(Struct1::headers(), vec!["FIELD"],);
    assert_eq!(st.fields(), vec!["789"]);
}


#[test]
fn rename_all_gets_first_value() {
    #[derive(Tabled)]
    #[tabled(rename_all = "UPPERCASE")]
    #[tabled(rename_all = "PascalCase")]
    struct Struct1 {
        field: usize,
    }

    let st = Struct1 { field: 789 };

    assert_eq!(Struct1::headers(), vec!["FIELD"],);
    assert_eq!(st.fields(), vec!["789"]);

    #[derive(Tabled)]
    #[tabled(rename_all = "UPPERCASE", rename_all = ""PascalCase"")]
    struct Struct2 {
        field: usize,
    }

    let st = Struct2 { field: 789 };
    
    assert_eq!(Struct1::headers(), vec!["FIELD"],);
    assert_eq!(st.fields(), vec!["789"]);
}
