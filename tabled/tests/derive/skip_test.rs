#![cfg(feature = "derive")]

use tabled::Tabled;

use super::{sstr, test_enum, test_struct, test_tuple};

test_tuple!(
    tuple_skip_0,
    declare: {
        {
            #[tabled(skip)]
            #[allow(dead_code)]
            u8
            #[tabled(rename = "...", skip)]
            #[allow(dead_code)]
            sstr
            sstr
        }
    }
    init: {}
    define: { 0 "v2" "123" }
    assert_headers: { ["2"] }
    assert_fields: { ["123"] }
);

test_tuple!(
    tuple_skip_1,
    declare: {
        {
            #[tabled(skip)]
            #[allow(dead_code)]
            u8
            #[tabled(skip)]
            #[tabled(rename = "...")]
            #[allow(dead_code)]
            sstr
            sstr
        }
    }
    init: {}
    define: { 0 "v2" "123" }
    assert_headers: { ["2"] }
    assert_fields: { ["123"] }
);

test_enum!(
    enum_skip_variant,
    declare: {
        #[allow(dead_code)]
        {
            A { a: u8, b: i32 }
            #[tabled(skip)]
            B(sstr)
            K
        }
    }
    init: {}
    assert_headers: { ["A", "K"] }
    assert: {
        A { a: 1, b: 2 } => ["+", ""],
        B("") => ["", ""],
        K => ["", "+"],
    }
);

test_struct!(
    struct_skip,
    declare: {
        {
            #[tabled(skip)]
            #[allow(dead_code)]
            f1: u8,
            #[tabled(rename = "field 2", skip)]
            #[allow(dead_code)]
            f2: sstr,
            f3: sstr,
        }
    }
    init: {}
    define: { f1: 0, f2: "v2", f3: "123" }
    assert_headers: { ["f3"] }
    assert_fields: { ["123"] }
);

test_struct!(
    struct_skip_true,
    declare: {
        {
            #[tabled(skip = true)]
            #[allow(dead_code)]
            f1: u8,
            #[tabled(rename = "field 2", skip = true)]
            #[allow(dead_code)]
            f2: sstr,
            f3: sstr,
        }
    }
    init: {}
    define: { f1: 0, f2: "v2", f3: "123" }
    assert_headers: { ["f3"] }
    assert_fields: { ["123"] }
);

test_tuple!(
    tuple_skipped_fields_not_implement_display_tuple,
    declare: { { #[tabled(skip)] () sstr } }
    init: {}
    define: { () "123" }
    assert_headers: { ["1"] }
    assert_fields: { ["123"] }
);

test_struct!(
    struct_skipped_fields_not_implement_display_struct,
    declare: { { #[tabled(skip)] _unit: (), s: sstr } }
    init: {}
    define: { _unit: (), s: "123" }
    assert_headers: { ["s"] }
    assert_fields: { ["123"] }
);

test_struct!(
    struct_skipped_fields_not_implement_display_struct_in_inline,
    declare: { { s: sstr, #[tabled(inline)] f: S1 } }
    init: {
        #[derive(Tabled)]
        struct S1 {
            #[tabled(skip)]
            _unit: (),
            s: sstr,
        }
    }
    define: { s: "123", f: S1 { _unit: (), s: "..." } }
    assert_headers: { ["s", "s"] }
    assert_fields: { ["123", "..."] }
);

#[test]
fn test_skip_enum_0() {
    #[allow(dead_code)]
    #[derive(Tabled)]
    enum Letters {
        Vowels {
            character: char,
            lang: u8,
        },
        Consonant(char),
        #[tabled(skip)]
        Digit,
    }

    assert_eq!(Letters::headers(), vec!["Vowels", "Consonant"]);
    assert_eq!(Letters::Consonant('c').fields(), vec!["", "+"]);
    assert_eq!(Letters::Digit.fields(), vec!["", ""]);
}

#[test]
fn test_reimport_trait_by_crate_attribute() {
    pub mod new_module {
        pub trait Tabled {
            const LENGTH: usize;

            fn fields(&self) -> Vec<std::borrow::Cow<'_, str>>;
            fn headers() -> Vec<std::borrow::Cow<'static, str>>;
        }
    }

    mod tabled {}

    #[allow(dead_code)]
    #[derive(Tabled)]
    #[tabled(crate = "new_module")]
    enum Letters {
        Vowels {
            character: char,
            lang: u8,
        },
        Consonant(char),
        #[tabled(skip)]
        Digit,
    }

    assert_eq!(
        <Letters as new_module::Tabled>::headers(),
        vec!["Vowels", "Consonant"]
    );
    assert_eq!(
        <Letters as new_module::Tabled>::fields(&Letters::Consonant('c')),
        vec!["", "+"]
    );
    assert_eq!(
        <Letters as new_module::Tabled>::fields(&Letters::Digit),
        vec!["", ""]
    );
}
