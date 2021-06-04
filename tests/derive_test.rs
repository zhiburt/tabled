// Copyright (c) 2021 Maxim Zhiburt
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.

use tabled::Tabled;

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
    fn rename_structure_field() {
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

    #[test]
    fn structure_hidden_field() {
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
    fn enum_hidden_variant() {
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
    fn rename_enum_variant() {
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
    fn empty() {
        #[derive(Tabled)]
        struct St {}

        let st = St {};
        assert!(st.fields().is_empty());
        assert!(St::headers().is_empty());
    }

    #[test]
    fn unit() {
        #[derive(Tabled)]
        struct St;
        let st = St;

        assert!(st.fields().is_empty());
        assert!(St::headers().is_empty());
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
    fn empty_tuple() {
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

    #[test]
    fn enum_structure() {
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
}
