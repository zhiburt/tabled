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
        };

        let (headers, fields) = infer_type(1);

        assert_eq!(vec!["0".to_owned()], headers);
        assert_eq!(vec!["1".to_owned()], fields);
    }
}
