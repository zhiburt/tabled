#![cfg(feature = "derive")]

use tabled::Tabled;

use super::{sstr, test_enum, test_struct, test_tuple};

test_tuple!(
    tuple_rename,
    declare: { { u8 #[tabled(rename = "field 2")] sstr } }
    init: {}
    define: { 0 "123" }
    assert_headers: { ["0", "field 2"] }
    assert_fields: { ["0", "123"] }
);

test_enum!(
    enum_rename_variant,
    declare: { { #[tabled(rename = "Variant 1")] A { a: u8, b: i32 } #[tabled(rename = "Variant 2")] B(sstr) K } }
    init: {}
    assert_headers: { ["Variant 1", "Variant 2", "K"] }
    assert: {
        A { a: 1, b: 2 } => ["+", "", ""],
        B("") => ["", "+",  ""],
        K => ["", "", "+"],
    }
);

test_struct!(
    struct_rename,
    declare: {
        {
            #[tabled(rename = "field 1")]
            f1: u8,
            #[tabled(rename = "field 2")]
            f2: sstr,
        }
    }
    init: {}
    define: { f1: 0, f2: "v2" }
    assert_headers: { ["field 1", "field 2"] }
    assert_fields: { ["0", "v2"] }
);
