#![cfg(feature = "derive")]

use tabled::Tabled;

use super::{sstr, test_enum, test_struct, test_tuple};

test_tuple!(
    tuple_rename_all,
    declare: { #[tabled(rename_all = "UPPERCASE")] { u8 sstr } }
    init: {}
    define: { 0 "123" }
    assert_headers: { ["0", "1"] }
    assert_fields: { ["0", "123"] }
);

test_tuple!(
    tuple_rename_all_field,
    declare: { { u8 #[tabled(rename_all = "UPPERCASE")] sstr } }
    init: { }
    define: { 0 "123" }
    assert_headers: { ["0", "1"] }
    assert_fields: { ["0", "123"] }
);

test_tuple!(
    tuple_rename_all_field_with_rename_0,
    declare: { { u8 #[tabled(rename_all = "UPPERCASE", rename = "Something")] sstr } }
    init: {}
    define: { 0 "123" }
    assert_headers: { ["0", "Something"] }
    assert_fields: { ["0", "123"] }
);

test_tuple!(
    tuple_rename_all_field_with_rename_1,
    declare: { { u8 #[tabled(rename = "Something")] #[tabled(rename_all = "UPPERCASE")] sstr } }
    init: { }
    define: { 0 "123" }
    assert_headers: { ["0", "Something"] }
    assert_fields: { ["0", "123"] }
);

test_enum!(
    rename_all_variant,
    declare: {
        {
            #[tabled(rename_all = "snake_case")]
            VariantName1 { a: u8, b: i32 }
            #[tabled(rename_all = "UPPERCASE")]
            VariantName2(String)
            VariantName3
        }
    }
    init: {}
    assert_headers: { ["variant_name1", "VARIANTNAME2", "VariantName3"] }
    assert: {
        VariantName1 { a: 0, b: -1 } => ["+", "", ""],
        VariantName2(String::from("Hello World")) => ["", "+", ""],
        VariantName3 => ["", "", "+"],
    }
);

test_enum!(
    rename_all_enum,
    declare: {
        #[tabled(rename_all = "snake_case")]
        {
            VariantName1 { a: u8, b: i32 }
            VariantName2(String)
            VariantName3
        }
    }
    init: {}
    assert_headers: { ["variant_name1", "variant_name2", "variant_name3"] }
    assert: {
        VariantName1 { a: 0, b: -1 } => ["+", "", ""],
        VariantName2(String::from("Hello World")) => ["", "+", ""],
        VariantName3 => ["", "", "+"],
    }
);

test_struct!(
    struct_rename_all,
    declare: { #[tabled(rename_all = "UPPERCASE")] { f1: u8, f2: sstr } }
    init: {}
    define: { f1: 0, f2: "v2" }
    assert_headers: { ["F1", "F2"] }
    assert_fields: { ["0", "v2"] }
);

test_struct!(
    struct_rename_all_override_in_field_by_rename,
    declare: { #[tabled(rename_all = "UPPERCASE")] { #[tabled(rename = "213213")] f1: u8, f2: sstr } }
    init: {}
    define: { f1: 0, f2: "v2" }
    assert_headers: { ["213213", "F2"] }
    assert_fields: { ["0", "v2"] }
);

test_struct!(
    struct_rename_all_override_in_field_by_rename_all,
    declare: { #[tabled(rename_all = "UPPERCASE")] { #[tabled(rename_all = "lowercase")] f1: u8, f2: sstr } }
    init: {}
    define: { f1: 0, f2: "v2" }
    assert_headers: { ["f1", "F2"] }
    assert_fields: { ["0", "v2"] }
);

test_struct!(
    struct_rename_all_field,
    declare: { { #[tabled(rename_all = "lowercase")] f1: u8, #[tabled(rename_all = "UPPERCASE")] f2: sstr } }
    init: {}
    define: { f1: 0, f2: "v2" }
    assert_headers: { ["f1", "F2"] }
    assert_fields: { ["0", "v2"] }
);

test_struct!(
    struct_rename_all_field_overridden_by_rename,
    declare: { { #[tabled(rename_all = "lowercase", rename = "Hello")] f1: u8, #[tabled(rename_all = "UPPERCASE")] f2: sstr } }
    init: {}
    define: { f1: 0, f2: "v2" }
    assert_headers: { ["Hello", "F2"] }
    assert_fields: { ["0", "v2"] }
);

#[test]
fn rename_all_variants() {
    macro_rules! test_case {
        ( $name:ident, $case:expr ) => {
            #[allow(dead_code)]
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
fn rename_all_gets_last_value() {
    #[derive(Tabled)]
    #[tabled(rename_all = "UPPERCASE")]
    #[tabled(rename_all = "PascalCase")]
    struct Struct1 {
        field: usize,
    }

    let st = Struct1 { field: 789 };

    assert_eq!(Struct1::headers(), vec!["Field"],);
    assert_eq!(st.fields(), vec!["789"]);

    #[derive(Tabled)]
    #[tabled(rename_all = "UPPERCASE", rename_all = "PascalCase")]
    struct Struct2 {
        field: usize,
    }

    let st = Struct2 { field: 789 };

    assert_eq!(Struct1::headers(), vec!["Field"],);
    assert_eq!(st.fields(), vec!["789"]);
}
