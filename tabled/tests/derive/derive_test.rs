#![cfg(feature = "derive")]

use tabled::Tabled;

use super::{sstr, test_enum, test_struct, test_tuple};

test_tuple!(
    tuple_empty,
    declare: { { } }
    init: {}
    define: { }
    assert_headers: { [] }
    assert_fields: { [] }
);

test_tuple!(
    tuple_basic,
    declare: { { u32 sstr } }
    init: {}
    define: { 20202503 "Hello World" }
    assert_headers: { ["0", "1"] }
    assert_fields: { ["20202503", "Hello World"] }
);

test_enum!(
    enum_basic,
    declare: { { Security Embedded Frontend Unknown } }
    init: {}
    assert_headers: { ["Security", "Embedded", "Frontend", "Unknown"] }
    assert: {
        Security => ["+", "", "", ""],
        Embedded => ["", "+", "", ""],
        Frontend => ["", "", "+", ""],
        Unknown =>  ["", "", "", "+"],
    }
);

test_enum!(
    enum_diverse,
    declare: { { A { a: u8, b: i32 } B(sstr) K } }
    init: {}
    assert_headers: { ["A", "B", "K"] }
    assert: {
        A { a: 1, b: 2 } => ["+", "", ""],
        B("") => ["", "+",  ""],
        K => ["", "", "+"],
    }
);

#[test]
fn basic() {
    #[derive(Tabled)]
    struct St;
    let st = St;

    assert!(st.fields().is_empty());
    assert!(St::headers().is_empty());
    assert_eq!(St::LENGTH, 0);
}

test_struct!(
    struct_empty,
    declare: { {} }
    init: {}
    define: { }
    assert_headers: { [] }
    assert_fields: { [] }
);

test_struct!(
    struct_general,
    declare: {
        {
            f1: u8,
            f2: sstr,
        }
    }
    init: {}
    define: { f1: 0, f2: "v2" }
    assert_headers: { ["f1", "f2"] }
    assert_fields: { ["0", "v2"] }
);

mod __ {
    #[test]
    fn dont_import_the_trait() {
        #[derive(tabled::Tabled)]
        struct __;
    }
}

// TODO: MOVE TO UI
// #[test]
// fn order_compile_fail_when_order_is_bigger_then_count_fields() {
//     #[derive(Tabled)]
//     struct St(#[tabled(order = 3)] u8, u8, u8);
// }

// TODO: MOVE TO UI
//
// #[test]
// fn wrong_rename_all_panic_when_used_as_not_first() {
//     #[derive(Tabled)]
//     #[tabled(rename_all = "UPPERCASE")]
//     #[tabled(rename_all = "some wrong case")]
//     struct Struct1 {
//         field: usize,
//     }

//     let st = Struct1 { field: 789 };

//     assert_eq!(Struct1::headers(), vec!["FIELD"],);
//     assert_eq!(st.fields(), vec!["789"]);

//     #[derive(Tabled)]
//     #[tabled(rename_all = "UPPERCASE", rename_all = "some wrong case")]
//     struct Struct2 {
//         field: usize,
//     }

//     let st = Struct2 { field: 789 };

//     assert_eq!(Struct1::headers(), vec!["FIELD"],);
//     assert_eq!(st.fields(), vec!["789"]);
// }

// TODO: MOVE TO UI
//
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
