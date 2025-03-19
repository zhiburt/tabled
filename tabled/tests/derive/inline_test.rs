#![cfg(feature = "derive")]

use tabled::Tabled;

use super::{sstr, test_enum, test_struct};

test_enum!(
    inline_enum_as_whole_and_rename,
    declare: {
        #[tabled(inline, rename_all = "snake_case")]
        {
            AbsdEgh { a: u8, b: i32 }
            B(String)
            K
        }
    }
    init: {}
    assert_headers: { ["Test"] }
    assert: {
        AbsdEgh { a: 0, b: 0 }  => ["absd_egh"],
        B(String::new()) => ["b"],
        K => ["k"],
    }
);

test_enum!(
    inline_enum_as_whole_and_rename_inner,
    declare: {
        #[tabled(inline)]
        {
            #[tabled(rename_all = "snake_case")]
            AbsdEgh { a: u8, b: i32 }
            #[tabled(rename_all = "lowercase")]
            B(String)
            K
        }
    }
    init: {}
    assert_headers: { ["Test"] }
    assert: {
        AbsdEgh { a: 0, b: 0 }  => ["absd_egh"],
        B(String::new()) => ["b"],
        K => ["K"],
    }
);

test_enum!(
    inline_variant,
    declare: {
        {
            #[tabled(inline("Auto::"))]
            Auto {
                #[tabled(rename = "mod")]
                model: sstr,
                engine: sstr
            }
            #[tabled(inline)]
            Bikecycle(
                #[tabled(rename = "name")]
                sstr,
                #[tabled(inline)]
                Bike
            )
            Skateboard
        }
    }
    init: {
        #[derive(Tabled)]
        struct Bike { brand: sstr, price: f32 }
    }
    assert_headers: { ["Auto::mod", "Auto::engine", "name", "brand", "price", "Skateboard"] }
    assert: {
        Skateboard => ["", "", "", "", "", "+"],
        Auto { model: "Mini", engine: "v8" } => ["Mini", "v8", "", "", "", ""],
        Bikecycle("A bike", Bike { brand: "Canyon", price: 2000.0 })=> ["", "", "A bike", "Canyon", "2000", ""],
    }
);

test_enum!(
    enum_inline_field_with_display_function,
    declare: {
        {
            #[tabled(inline("backend::"))]
            Backend {
                #[tabled(display = "display", rename = "name")]
                value: sstr
            }
            Frontend
        }
    }
    init: {
        fn display(_: sstr) -> String {
            "asd".to_string()
        }
    }
    assert_headers: { ["backend::name", "Frontend"] }
    assert: {
        Backend { value: "123" } => ["asd", ""],
        Frontend => ["", "+"],
    }
);

test_enum!(
    enum_inline_field_with_display_self_function,
    declare: {
        {
            #[tabled(inline("backend::"))]
            Backend {
                #[tabled()]
                #[tabled(display("display", self), rename = "name")]
                value: sstr
            }
            Frontend
        }
    }
    init: {
        fn display<T>(_opt: &sstr, _: &T) -> String {
            "asd".to_string()
        }
    }
    assert_headers: { ["backend::name", "Frontend"] }
    assert: {
        Backend { value: "123" } => ["asd", ""],
        Frontend => ["", "+"],
    }
);

test_enum!(
    rename_all_enum_inherited_inside_struct_enum,
    declare: {
        #[tabled(rename_all = "snake_case")]
        {
            #[tabled(inline)]
            VariantName1 { some_field_1: u8, some_field_2: i32 }
            VariantName2(String)
            VariantName3
        }
    }
    init: {}
    assert_headers: { ["some_field_1", "some_field_2", "variant_name2", "variant_name3"] }
    assert: {
        VariantName1 { some_field_1: 0, some_field_2: -1 } => ["0", "-1", "", ""],
        VariantName2(String::from("Hello World")) => ["", "", "+", ""],
        VariantName3 => ["", "", "", "+"],
    }
);

test_enum!(
    rename_all_enum_inherited_inside_struct_override_by_rename_enum,
    declare: {
        #[tabled(rename_all = "snake_case")]
        {
            #[tabled(inline)]
            VariantName1 {
                #[tabled(rename = "f1")]
                some_field_1: u8,
                #[tabled(rename = "f2")]
                some_field_2: i32,
            }
            VariantName2(String)
            VariantName3
        }
    }
    init: {}
    assert_headers: { ["f1", "f2", "variant_name2", "variant_name3"] }
    assert: {
        VariantName1 { some_field_1: 0, some_field_2: -1 } => ["0", "-1", "", ""],
        VariantName2(String::from("Hello World")) => ["", "", "+", ""],
        VariantName3 => ["", "", "", "+"],
    }
);

test_enum!(
    rename_all_enum_inherited_inside_struct_override_by_rename_all_enum,
    declare: {
        #[tabled(rename_all = "snake_case")]
        {
            #[tabled(inline)]
            VariantName1 {
                #[tabled(rename_all = "UPPERCASE")]
                some_field_1: u8,
                #[tabled(rename_all = "CamelCase")]
                some_field_2: i32,
            }
            VariantName2(String)
            VariantName3
        }
    }
    init: {}
    assert_headers: { ["SOMEFIELD1", "someField2", "variant_name2", "variant_name3"] }
    assert: {
        VariantName1 { some_field_1: 0, some_field_2: -1 } => ["0", "-1", "", ""],
        VariantName2(String::from("Hello World")) => ["", "", "+", ""],
        VariantName3 => ["", "", "", "+"],
    }
);

test_enum!(
    rename_all_variant_inherited_inside_struct_enum,
    declare: {
        #[tabled(rename_all = "snake_case")]
        {
            #[tabled(inline)]
            #[tabled(rename_all = "snake_case")]
            VariantName1 {
                some_field_1: u8,
                some_field_2: i32,
            }
            VariantName2(String)
            VariantName3
        }
    }
    init: {}
    assert_headers: { ["some_field_1", "some_field_2", "variant_name2", "variant_name3"] }
    assert: {
        VariantName1 { some_field_1: 0, some_field_2: -1 } => ["0", "-1", "", ""],
        VariantName2(String::from("Hello World")) => ["", "", "+", ""],
        VariantName3 => ["", "", "", "+"],
    }
);

test_enum!(
    rename_all_variant_inherited_inside_struct_enum_overridden_by_rename,
    declare: {
        #[tabled(rename_all = "snake_case")]
        {
            #[tabled(inline, rename_all = "snake_case")]
            VariantName1 {
                #[tabled(rename = "f1")]
                some_field_1: u8,
                #[tabled(rename = "f2")]
                some_field_2: i32,
            }
            VariantName2(String)
            VariantName3
        }
    }
    init: {}
    assert_headers: { ["f1", "f2", "variant_name2", "variant_name3"] }
    assert: {
        VariantName1 { some_field_1: 0, some_field_2: -1 } => ["0", "-1", "", ""],
        VariantName2(String::from("Hello World")) => ["", "", "+", ""],
        VariantName3 => ["", "", "", "+"],
    }
);

test_enum!(
    rename_all_variant_inherited_inside_struct_override_by_rename_all_enum,
    declare: {
        #[tabled(rename_all = "snake_case")]
        {
            #[tabled(rename_all = "snake_case", inline)]
            VariantName1 {
                #[tabled(rename_all = "UPPERCASE")]
                some_field_1: u8,
                #[tabled(rename_all = "CamelCase")]
                some_field_2: i32,
            }
            VariantName2(String)
            VariantName3
        }
    }
    init: {}
    assert_headers: { ["SOMEFIELD1", "someField2", "variant_name2", "variant_name3"] }
    assert: {
        VariantName1 { some_field_1: 0, some_field_2: -1 } => ["0", "-1", "", ""],
        VariantName2(String::from("Hello World")) => ["", "", "+", ""],
        VariantName3 => ["", "", "", "+"],
    }
);

test_enum!(
    inline_field_with_display_function,
    declare: {
        {
            #[tabled(inline("backend::"))]
            Backend {
                #[tabled(display = "display", rename = "name")]
                value: sstr
            }
            Frontend
        }
    }
    init: {
        fn display(_: sstr) -> String {
            "asd".to_string()
        }
    }
    assert_headers: { ["backend::name", "Frontend"] }
    assert: {
        Backend { value: "123" } => ["asd", ""],
        Frontend => ["", "+"],
    }
);

test_enum!(
    inline_field_with_display_self_function,
    declare: {
        {
            #[tabled(inline("backend::"))]
            Backend {
                #[tabled()]
                #[tabled(display("display", self), rename = "name")]
                value: sstr
            }
            Frontend
        }
    }
    init: {
        fn display<T>(_opt: &sstr, _: &T) -> String {
            "asd".to_string()
        }
    }
    assert_headers: { ["backend::name", "Frontend"] }
    assert: {
        Backend { value: "123" } => ["asd", ""],
        Frontend => ["", "+"],
    }
);

test_struct!(
    ignore_display_with_when_used_with_inline,
    declare: {
        {
            f1: sstr,
            f2: sstr,
            #[tabled(display = "print", inline)]
            f3: usize,
        }
    }
    init: {}
    define: { f1: "123", f2: "456", f3: 789 }
    assert_headers: { ["f1", "f2", "usize"] }
    assert_fields: { ["123", "456", "789"] }
);

test_struct!(
    ignore_display_with_when_used_with_inline_2,
    declare: {
        {
            f1: sstr,
            f2: sstr,
            #[tabled(display = "print", )]
            #[tabled(inline)]
            f3: usize,
        }
    }
    init: {}
    define: { f1: "123", f2: "456", f3: 789 }
    assert_headers: { ["f1", "f2", "usize"] }
    assert_fields: { ["123", "456", "789"] }
);

test_struct!(
    struct_inline,
    declare: {
        {
            #[tabled(inline = true)]
            id: u8,
            name: sstr,
            #[tabled(inline)]
            ed: Education
        }
    }
    init: {
        #[derive(Tabled)]
        struct Education { uni: sstr, graduated: bool }
    }
    define: {
        id: 0, name: "Maxim", ed: Education { uni: "BNTU", graduated: true }
    }
    assert_headers: { ["u8", "name","uni","graduated"] }
    assert_fields: { ["0", "Maxim", "BNTU", "true"] }
);

test_struct!(
    struct_inline_with_prefix,
    declare: {
        {
            #[tabled(rename = "it's an ignored option", inline)]
            id: u8,
            name: sstr,
            #[tabled(inline("education::"))]
            ed: Education,
        }
    }
    init: {
        #[derive(Tabled)]
        struct Education { uni: sstr, graduated: bool }
    }
    define: {
        id: 0, name: "Maxim", ed: Education { uni: "BNTU", graduated: true }}
    assert_headers: { ["u8", "name","education::uni","education::graduated"] }
    assert_fields: { ["0", "Maxim", "BNTU", "true"] }
);

test_enum!(
    struct_skipped_fields_not_implement_display_enum,
    declare: {
        {
            #[tabled(inline("A::"))]
            A {
                name: sstr
            }
            #[tabled(inline("B::"))]
            B {
                issue: usize,
                name: sstr,
                #[tabled(skip)]
                _gem: (),
            }
            #[tabled(inline("C::"))]
            C(usize, #[tabled(skip)] (), sstr)
            D
        }
    }
    init: {}
    assert_headers: { ["A::name", "B::issue", "B::name", "C::0", "C::2", "D"] }
    assert: {
        A { name: "nrdxp" } => ["nrdxp", "", "", "", "", ""],
        B { _gem: (), issue: 32, name: "nrdxp" } => ["", "32", "nrdxp", "", "", ""],
        C(32, (), "nrdxp") => ["", "", "", "32", "nrdxp", ""],
        D => ["", "", "", "", "", "+"],
    }
);

test_enum!(
    inline_enum_as_whole,
    declare: {
        #[tabled(inline)] {
            AbsdEgh { a: u8, b: i32 }
            B(String)
            K
        }
    }
    init: {}
    assert_headers: { ["Test"] }
    assert: {
        AbsdEgh { a: 0, b: 0 }  => ["AbsdEgh"],
        B(String::new()) => ["B"],
        K => ["K"],
    }
);

test_enum!(
    inline_enum_name,
    declare: {
        #[tabled(inline("A struct name"))]
        {
            AbsdEgh { a: u8, b: i32 }
            B(String)
            K
        }
    }
    init: {}
    assert_headers: { ["A struct name"] }
    assert: {
        AbsdEgh { a: 0, b: 0 }  => ["AbsdEgh"],
        B(String::new()) => ["B"],
        K => ["K"],
    }
);

test_enum!(
    enum_order_0_inlined,
    declare: {
        #[tabled(inline)]
        {
            #[tabled(order = 1)]
            V1(u8)
            V2(u8)
            V3(u8)
        }
    }
    init: {}
    assert_headers: { ["Test"] }
    assert: {
        V1(0) => ["V1"],
        V2(0) => ["V2"],
        V3(0) => ["V3"],
    }
);

#[test]
fn test_format_enum_inline() {
    #[derive(Tabled)]
    enum Struct {
        #[tabled(inline)]
        Variant1 {
            #[tabled(rename = "cccc")]
            #[tabled(format("{} {}", self.character, self.lang))]
            character: char,
            lang: u8,
        },
        Variant2(char),
        Variant3,
    }

    assert_eq!(Struct::headers(), ["cccc", "lang", "Variant2", "Variant3"]);
    assert_eq!(
        Struct::Variant1 {
            character: 'c',
            lang: b'c'
        }
        .fields(),
        ["c 99", "99", "", ""]
    );
    assert_eq!(Struct::Variant2('c').fields(), ["", "", "+", ""]);
    assert_eq!(Struct::Variant3.fields(), ["", "", "", "+"]);
}
