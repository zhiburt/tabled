#![cfg(feature = "derive")]
#![cfg(feature = "std")]

use tabled::Tabled;

// https://users.rust-lang.org/t/create-a-struct-from-macro-rules/19829
macro_rules! test_tuple {
    (
        $test_name:ident,
        t: $(#[$struct_attr:meta])* { $( $(#[$attr:meta])* $ty:ty)* },
        init: { $($init:expr)* },
        expected: $headers:expr, $fields:expr,
        $(pre: { $($init_block:stmt)* })?
    ) => {
        #[test]
        fn $test_name() {
            $($($init_block)*)?

            #[derive(Tabled)]
            struct TestType(
                $( $(#[$attr])* $ty, )*
            );

            let value = TestType($($init,)*);

            let fields: Vec<&'static str> = $fields.to_vec();
            let headers: Vec<&'static str> = $headers.to_vec();

            assert_eq!(value.fields(), fields);
            assert_eq!(TestType::headers(), headers);
            assert_eq!(<TestType as Tabled>::LENGTH, headers.len());
            assert_eq!(<TestType as Tabled>::LENGTH, fields.len());
        }
    };
}

macro_rules! test_enum {
    (
        $test_name:ident,
        t: $(#[$struct_attr:meta])* { $( $(#[$var_attr:meta])*  $var:ident $({ $( $(#[$attr:meta])* $field:ident: $ty:ty),* $(,)? })? $(( $( $(#[$attr2:meta])* $ty2:ty),* $(,)? ))? )* },
        $(pre: { $($init_block:stmt)* })?
        headers: $headers:expr,
        tests:  $($init:expr => $expected:expr,)*
    ) => {
        #[allow(dead_code, unused_imports)]
        #[test]
        fn $test_name() {
            $($($init_block)*)?

            #[derive(Tabled)]
            $(#[$struct_attr])*
            enum TestType {
                $(
                    $(#[$var_attr])*
                    $var $({
                        $( $(#[$attr])* $field: $ty,)*
                    })?

                    $((
                        $( $(#[$attr2])* $ty2,)*
                    ))?
                ),*
            }

            let headers: Vec<&'static str> = $headers.to_vec();
            assert_eq!(TestType::headers(), headers);
            assert_eq!(<TestType as Tabled>::LENGTH, headers.len());

            {
                use TestType::*;
                $(
                    let variant = $init;
                    let fields: Vec<&'static str> = $expected.to_vec();
                    assert_eq!(variant.fields(), fields);
                )*
            }
        }
    };
}

macro_rules! test_struct {
    (
        $test_name:ident,
        t: $(#[$struct_attr:meta])* { $( $(#[$attr:meta])* $field:ident: $ty:ty),*  $(,)?}
        $(pre: { $($init_block:stmt)* })?
        init: { $( $val_field:ident: $val:expr),* $(,)?}
        expected: $headers:expr, $fields:expr  $(,)?
    ) => {

        #[allow(dead_code, unused_imports)]
        #[test]
        fn $test_name() {
            $($($init_block)*)?

            #[derive(Tabled)]
            $(#[$struct_attr])*
            struct TestType {
                $(
                    $(#[$attr])*
                    $field: $ty,
                )*
            }

            let value = TestType {
                $($val_field: $val,)*
            };

            let fields: Vec<&'static str> = $fields.to_vec();
            let headers: Vec<&'static str> = $headers.to_vec();
            assert_eq!(TestType::headers(), headers);
            assert_eq!(value.fields(), fields);
            assert_eq!(<TestType as Tabled>::LENGTH, headers.len());
            assert_eq!(<TestType as Tabled>::LENGTH, fields.len());
        }
    };
}

#[allow(non_camel_case_types)]
type sstr = &'static str;

mod tuple {
    use super::*;

    test_tuple!(basic, t: { u8 sstr }, init: { 0 "v2" }, expected: ["0", "1"], ["0", "v2"],);
    test_tuple!(empty, t: { }, init: { }, expected: [], [],);

    test_tuple!(rename, t: { u8 #[tabled(rename = "field 2")] sstr }, init: { 0 "123" }, expected: ["0", "field 2"], ["0", "123"],);

    test_tuple!(skip_0, t: { #[tabled(skip)] u8 #[tabled(rename = "field 2", skip)] sstr sstr }, init: { 0 "v2" "123" }, expected: ["2"], ["123"],);
    test_tuple!(skip_1, t: { #[tabled(skip)] u8 #[tabled(skip)] #[tabled(rename = "field 2")] sstr sstr }, init: { 0 "v2" "123" }, expected: ["2"], ["123"],);

    test_tuple!(order_0, t: { #[tabled(order = 0)] u8 u8 u8}, init: { 0 1 2 }, expected: ["0", "1", "2"], ["0", "1", "2"],);
    test_tuple!(order_1, t: { #[tabled(order = 1)] u8 u8 u8}, init: { 0 1 2 }, expected: ["1", "0", "2"], ["1", "0", "2"],);
    test_tuple!(order_2, t: { #[tabled(order = 2)] u8 u8 u8}, init: { 0 1 2 }, expected: ["1", "2", "0"], ["1", "2", "0"],);
    test_tuple!(order_3, t: { u8 #[tabled(order = 0)] u8 u8}, init: { 0 1 2 }, expected: ["1", "0", "2"], ["1", "0", "2"],);
    test_tuple!(order_4, t: { u8 #[tabled(order = 1)] u8 u8}, init: { 0 1 2 }, expected: ["0", "1", "2"], ["0", "1", "2"],);
    test_tuple!(order_5, t: { u8 #[tabled(order = 2)] u8 u8}, init: { 0 1 2 }, expected: ["0", "2", "1"], ["0", "2", "1"],);
    test_tuple!(order_6, t: { u8 u8 #[tabled(order = 0)] u8}, init: { 0 1 2 }, expected: ["2", "0", "1"], ["2", "0", "1"],);
    test_tuple!(order_7, t: { u8 u8 #[tabled(order = 1)] u8}, init: { 0 1 2 }, expected: ["0", "2", "1"], ["0", "2", "1"],);
    test_tuple!(order_8, t: { u8 u8 #[tabled(order = 2)] u8}, init: { 0 1 2 }, expected: ["0", "1", "2"], ["0", "1", "2"],);
    test_tuple!(order_9, t: { #[tabled(order = 2)] u8 u8 #[tabled(order = 0)] u8}, init: { 0 1 2 }, expected: ["2", "1", "0"], ["2", "1", "0"],);
    test_tuple!(order_10, t: { #[tabled(order = 2)] u8 #[tabled(order = 1)] u8 u8}, init: { 0 1 2 }, expected: ["2", "1", "0"], ["2", "1", "0"],);
    test_tuple!(order_11, t: { #[tabled(order = 2)] u8 #[tabled(order = 2)] u8 #[tabled(order = 1)] u8}, init: { 0 1 2 }, expected: ["0", "2", "1"], ["0", "2", "1"],);
    test_tuple!(order_12, t: { #[tabled(order = 2)] u8 #[tabled(order = 2)] u8 #[tabled(order = 2)] u8}, init: { 0 1 2 }, expected: ["0", "1", "2"], ["0", "1", "2"],);
    test_tuple!(order_13, t: { #[tabled(order = 1)] u8 #[tabled(order = 1)] u8 #[tabled(order = 1)] u8}, init: { 0 1 2 }, expected: ["0", "2", "1"], ["0", "2", "1"],);
    test_tuple!(order_14, t: { #[tabled(order = 2)] u8 #[tabled(order = 1)] u8 #[tabled(order = 0)] u8}, init: { 0 1 2 }, expected: ["2", "1", "0"], ["2", "1", "0"],);

    test_tuple!(rename_all, t: #[tabled(rename_all = "UPPERCASE")] { u8 sstr}, init: { 0 "123" }, expected: ["0", "1"], ["0", "123"],);
    test_tuple!(rename_all_field, t: { u8 #[tabled(rename_all = "UPPERCASE")] sstr}, init: { 0 "123" }, expected: ["0", "1"], ["0", "123"],);
    test_tuple!(rename_all_field_with_rename_0, t: { u8 #[tabled(rename_all = "UPPERCASE", rename = "Something")] sstr}, init: { 0 "123" }, expected: ["0", "Something"], ["0", "123"],);
    test_tuple!(rename_all_field_with_rename_1, t: { u8 #[tabled(rename = "Something")] #[tabled(rename_all = "UPPERCASE")] sstr}, init: { 0 "123" }, expected: ["0", "Something"], ["0", "123"],);

    test_tuple!(
        display_option,
        t: { u8 #[tabled(display_with = "display_option")] Option<sstr> },
        init: { 0 Some("v2") },
        expected: ["0", "1"], ["0", "some v2"],
        pre: {
            fn display_option(o: &Option<sstr>) -> String {
                match o {
                    Some(s) => format!("some {s}"),
                    None => "none".to_string(),
                }
            }
        }
    );

    test_tuple!(
        display_option_args,
        t: { u8 #[tabled(display_with("display_option", 1, "234"))] Option<sstr> },
        init: { 0 Some("v2") },
        expected: ["0", "1"], ["0", "some 1 234"],
        pre: {
            fn display_option(val: usize, text: &str) -> String {
                format!("some {val} {text}")
            }
        }
    );

    test_tuple!(
        display_option_self,
        t: { u8 #[tabled(display_with = "Self::display_option")] Option<sstr> },
        init: { 0 Some("v2") },
        expected: ["0", "1"], ["0", "some v2"],
        pre: {
            impl TestType {
                fn display_option(o: &Option<sstr>) -> String {
                    match o {
                        Some(s) => format!("some {s}"),
                        None => "none".to_string(),
                    }
                }
            }
        }
    );

    test_tuple!(
        display_option_self_2,
        t: { u8 #[tabled(display_with("Self::display_option", self))] Option<sstr> },
        init: { 0 Some("v2") },
        expected: ["0", "1"], ["0", "some v2"],
        pre: {
            impl TestType {
                fn display_option(o: &TestType) -> String {
                    match o.1 {
                        Some(s) => format!("some {s}"),
                        None => "none".to_string(),
                    }
                }
            }
        }
    );

    test_tuple!(
        display_option_self_3,
        t: { u8 #[tabled(display_with("display_option", self))] Option<sstr> },
        init: { 0 Some("v2") },
        expected: ["0", "1"], ["0", "some v2"],
        pre: {
            fn display_option(o: &TestType) -> String {
                match o.1 {
                    Some(s) => format!("some {s}"),
                    None => "none".to_string(),
                }
            }
        }
    );

    // #[test]
    // fn order_compile_fail_when_order_is_bigger_then_count_fields() {
    //     #[derive(Tabled)]
    //     struct St(#[tabled(order = 3)] u8, u8, u8);
    // }
}

mod enum_ {
    use super::*;

    test_enum!(
        basic,
        t: { Security Embedded Frontend Unknown },
        headers: ["Security", "Embedded", "Frontend", "Unknown"],
        tests:
            Security => ["+", "", "", ""],
            Embedded => ["", "+", "", ""],
            Frontend => ["", "", "+", ""],
            Unknown => ["", "", "", "+"],
    );

    test_enum!(
        diverse,
        t: { A { a: u8, b: i32 } B(sstr) K },
        headers: ["A", "B", "K"],
        tests:
            A { a: 1, b: 2 } => ["+", "", ""],
            B("") => ["", "+",  ""],
            K => ["", "", "+"],
    );

    test_enum!(
        rename_variant,
        t: { #[tabled(rename = "Variant 1")] A { a: u8, b: i32 } #[tabled(rename = "Variant 2")] B(sstr) K },
        headers: ["Variant 1", "Variant 2", "K"],
        tests:
            A { a: 1, b: 2 } => ["+", "", ""],
            B("") => ["", "+",  ""],
            K => ["", "", "+"],
    );

    test_enum!(
        skip_variant,
        t: { A { a: u8, b: i32 } #[tabled(skip)] B(sstr) K },
        headers: ["A", "K"],
        tests:
            A { a: 1, b: 2 } => ["+", ""],
            B("") => ["", ""],
            K => ["", "+"],
    );

    test_enum!(
        inline_variant,
        t: {
            #[tabled(inline("Auto::"))] Auto { #[tabled(rename = "mod")] model: sstr, engine: sstr }
            #[tabled(inline)] Bikecycle( #[tabled(rename = "name")] sstr, #[tabled(inline)] Bike )
            Skateboard
        },
        pre: {
            #[derive(Tabled)]
            struct Bike { brand: sstr, price: f32 }
        }
        headers: ["Auto::mod", "Auto::engine", "name", "brand", "price", "Skateboard"],
        tests:
            Skateboard => ["", "", "", "", "", "+"],
            Auto { model: "Mini", engine: "v8" } => ["Mini", "v8", "", "", "", ""],
            Bikecycle("A bike", Bike { brand: "Canyon", price: 2000.0 })=> ["", "", "A bike", "Canyon", "2000", ""],
    );

    test_enum!(
        inline_field_with_display_function,
        t: {
            #[tabled(inline("backend::"))]
            Backend { #[tabled(display_with = "display", rename = "name")] value: sstr }
            Frontend
        },
        pre: {
            fn display(_: sstr) -> String {
                "asd".to_string()
            }
        }
        headers: ["backend::name", "Frontend"],
        tests:
            Backend { value: "123" } => ["asd", ""],
            Frontend => ["", "+"],
    );

    test_enum!(
        inline_field_with_display_self_function,
        t: {
            #[tabled(inline("backend::"))]
            Backend { #[tabled()] #[tabled(display_with("display", self), rename = "name")] value: sstr }
            Frontend
        },
        pre: {
            fn display<T>(_: &T) -> String {
                "asd".to_string()
            }
        }
        headers: ["backend::name", "Frontend"],
        tests:
            Backend { value: "123" } => ["asd", ""],
            Frontend => ["", "+"],
    );

    test_enum!(
        with_display,
        t: {
            #[tabled(inline)]
            A(#[tabled(display_with = "format::<4>")] sstr)
            B
        },
        pre: {
            fn format<const ID: usize>(_: sstr) -> String {
                ID.to_string()
            }
        }
        headers: ["0", "B"],
        tests:
            A("") => ["4", ""],
            B => ["", "+"],
    );

    test_enum!(
        with_display_self,
        t: {
            #[tabled(inline)]
            A(#[tabled(display_with("Self::format::<4>", self))] sstr)
            B
        },
        pre: {
            impl TestType {
                fn format<const ID: usize>(&self) -> String {
                    ID.to_string()
                }
            }
        }
        headers: ["0", "B"],
        tests:
            A("") => ["4", ""],
            B => ["", "+"],
    );

    test_enum!(
        rename_all_variant,
        t: {
            #[tabled(rename_all = "snake_case")]
            VariantName1 { a: u8, b: i32 }
            #[tabled(rename_all = "UPPERCASE")]
            VariantName2(String)
            K
        },
        headers: ["variant_name1", "VARIANTNAME2", "K"],
        tests:
    );

    test_enum!(
        rename_all_enum,
        t: #[tabled(rename_all = "snake_case")] {
            VariantName1 { a: u8, b: i32 }
            VariantName2(String)
            K
        },
        headers: ["variant_name1", "variant_name2", "k"],
        tests:
    );

    test_enum!(
        rename_all_enum_inherited_inside_struct_enum,
        t: #[tabled(rename_all = "snake_case")] {
            #[tabled(inline)]
            VariantName1 { some_field_1: u8, some_field_2: i32 }
            VariantName2(String)
            K
        },
        headers: ["some_field_1", "some_field_2", "variant_name2", "k"],
        tests:
    );

    test_enum!(
        rename_all_enum_inherited_inside_struct_override_by_rename_enum,
        t: #[tabled(rename_all = "snake_case")] {
            #[tabled(inline)]
            VariantName1 {
                #[tabled(rename = "f1")]
                some_field_1: u8,
                #[tabled(rename = "f2")]
                some_field_2: i32,
            }
            VariantName2(String)
            K
        },
        headers: ["f1", "f2", "variant_name2", "k"],
        tests:
    );

    test_enum!(
        rename_all_enum_inherited_inside_struct_override_by_rename_all_enum,
        t: #[tabled(rename_all = "snake_case")] {
            #[tabled(inline)]
            VariantName1 {
                #[tabled(rename_all = "UPPERCASE")]
                some_field_1: u8,
                #[tabled(rename_all = "CamelCase")]
                some_field_2: i32,
            }
            VariantName2(String)
            K
        },
        headers: ["SOMEFIELD1", "someField2", "variant_name2", "k"],
        tests:
    );

    test_enum!(
        rename_all_variant_inherited_inside_struct_enum,
        t: #[tabled(rename_all = "snake_case")] {
            #[tabled(inline)]
            #[tabled(rename_all = "snake_case")]
            VariantName1 {
                some_field_1: u8,
                some_field_2: i32,
            }
            VariantName2(String)
            K
        },
        headers: ["some_field_1", "some_field_2", "variant_name2", "k"],
        tests:
    );

    test_enum!(
        rename_all_variant_inherited_inside_struct_enum_overridden_by_rename,
        t: #[tabled(rename_all = "snake_case")] {
            #[tabled(inline, rename_all = "snake_case")]
            VariantName1 {
                #[tabled(rename = "f1")]
                some_field_1: u8,
                #[tabled(rename = "f2")]
                some_field_2: i32,
            }
            VariantName2(String)
            K
        },
        headers: ["f1", "f2", "variant_name2", "k"],
        tests:
    );

    test_enum!(
        rename_all_variant_inherited_inside_struct_override_by_rename_all_enum,
        t: #[tabled(rename_all = "snake_case")] {
            #[tabled(rename_all = "snake_case", inline)]
            VariantName1 {
                #[tabled(rename_all = "UPPERCASE")]
                some_field_1: u8,
                #[tabled(rename_all = "CamelCase")]
                some_field_2: i32,
            }
            VariantName2(String)
            K
        },
        headers: ["SOMEFIELD1", "someField2", "variant_name2", "k"],
        tests:
    );

    test_enum!(
        inline_enum_as_whole,
        t: #[tabled(inline)] {
            AbsdEgh { a: u8, b: i32 }
            B(String)
            K
        },
        headers: ["TestType"],
        tests:
            AbsdEgh { a: 0, b: 0 }  => ["AbsdEgh"],
            B(String::new()) => ["B"],
            K => ["K"],
    );

    test_enum!(
        inline_enum_as_whole_and_rename,
        t:
        #[tabled(inline, rename_all = "snake_case")]
        {
            AbsdEgh { a: u8, b: i32 }
            B(String)
            K
        },
        headers: ["TestType"],
        tests:
            AbsdEgh { a: 0, b: 0 }  => ["absd_egh"],
            B(String::new()) => ["b"],
            K => ["k"],
    );

    test_enum!(
        inline_enum_as_whole_and_rename_inner,
        t: #[tabled(inline)] {
            #[tabled(rename_all = "snake_case")]
            AbsdEgh { a: u8, b: i32 }
            #[tabled(rename_all = "lowercase")]
            B(String)
            K
        },
        headers: ["TestType"],
        tests:
            AbsdEgh { a: 0, b: 0 }  => ["absd_egh"],
            B(String::new()) => ["b"],
            K => ["K"],
    );

    test_enum!(
        inline_enum_name,
        t: #[tabled(inline("A struct name"))] {
            AbsdEgh { a: u8, b: i32 }
            B(String)
            K
        },
        headers: ["A struct name"],
        tests:
            AbsdEgh { a: 0, b: 0 }  => ["AbsdEgh"],
            B(String::new()) => ["B"],
            K => ["K"],
    );

    test_enum!(
        enum_display_with_variant,
        t: {
            #[tabled(display_with = "display_variant1")]
            AbsdEgh { a: u8, b: i32 }
            #[tabled(display_with = "display_variant2::<200>")]
            B(String)
            #[tabled(display_with = "some::bar::display_variant1")]
            K
        },
        pre: {
            fn display_variant1() -> &'static str {
                "Hello World"
            }

            fn display_variant2<const VAL: usize>() -> String {
                format!("asd {VAL}")
            }

            pub mod some {
                pub mod bar {
                    pub fn display_variant1() -> &'static str {
                        "Hello World 123"
                    }
                }
            }
        }
        headers: ["AbsdEgh", "B", "K"],
        tests:
            AbsdEgh { a: 0, b: 0 }  => ["Hello World", "", ""],
            B(String::new()) => ["", "asd 200", ""],
            K => ["", "", "Hello World 123"],
    );

    test_enum!(
        enum_display_with_self_variant,
        t: {
            #[tabled(display_with("display_variant1", self))]
            AbsdEgh { a: u8, b: i32 }
            #[tabled(display_with("display_variant2::<200, _>", self))]
            B(String)
            #[tabled(display_with("some::bar::display_variant1", self))]
            K
        },
        pre: {
            fn display_variant1<D>(_: &D) -> &'static str {
                "Hello World"
            }

            fn display_variant2<const VAL: usize, D>(_: &D) -> String {
                format!("asd {VAL}")
            }

            pub mod some {
                pub mod bar {
                    pub fn display_variant1<D>(_: &D) -> &'static str {
                        "Hello World 123"
                    }
                }
            }
        }
        headers: ["AbsdEgh", "B", "K"],
        tests:
            AbsdEgh { a: 0, b: 0 }  => ["Hello World", "", ""],
            B(String::new()) => ["", "asd 200", ""],
            K => ["", "", "Hello World 123"],
    );

    test_enum!(
        enum_display_with_arguments,
        t: {
            #[tabled(display_with("display1", 1, 2, self))]
            AbsdEgh { a: u8, b: i32 }
            #[tabled(display_with("display2::<200>", "Hello World"))]
            B(String)
            #[tabled(display_with("display1", 100, 200, self))]
            K
        },
        pre: {
            fn display1<D>(val: usize, val2: usize, _: &D) -> String {
                format!("{val} {val2}")
            }

            fn display2<const VAL: usize>(val: &str) -> String {
                format!("asd {VAL} {val}")
            }
        }
        headers: ["AbsdEgh", "B", "K"],
        tests:
            AbsdEgh { a: 0, b: 0 }  => ["1 2", "", ""],
            B(String::new()) => ["", "asd 200 Hello World", ""],
            K => ["", "", "100 200"],
    );

    test_enum!(order_0, t: { #[tabled(order = 0)] V1(u8) V2(u8) V3(u8) }, headers: ["V1", "V2", "V3"], tests: V1(0) => ["+", "", ""], V2(0) => ["", "+", ""], V3(0) => ["", "", "+"],);
    test_enum!(order_1, t: { #[tabled(order = 1)] V1(u8) V2(u8) V3(u8) }, headers: ["V2", "V1", "V3"], tests: V1(0) => ["", "+", ""], V2(0) => ["+", "", ""], V3(0) => ["", "", "+"],);
    test_enum!(order_2, t: { #[tabled(order = 2)] V1(u8) V2(u8) V3(u8) }, headers: ["V2", "V3", "V1"], tests: V1(0) => ["", "", "+"], V2(0) => ["+", "", ""], V3(0) => ["", "+", ""],);
    test_enum!(order_3, t: { V1(u8) #[tabled(order = 0)] V2(u8) V3(u8) }, headers: ["V2", "V1", "V3"], tests: V1(0) => ["", "+", ""], V2(0) => ["+", "", ""], V3(0) => ["", "", "+"],);
    test_enum!(order_4, t: { V1(u8) #[tabled(order = 1)] V2(u8) V3(u8) }, headers: ["V1", "V2", "V3"], tests: V1(0) => ["+", "", ""], V2(0) => ["", "+", ""], V3(0) => ["", "", "+"],);
    test_enum!(order_5, t: { V1(u8) #[tabled(order = 2)] V2(u8) V3(u8) }, headers: ["V1", "V3", "V2"], tests: V1(0) => ["+", "", ""], V2(0) => ["", "", "+"], V3(0) => ["", "+", ""],);
    test_enum!(order_6, t: { V1(u8) V2(u8) #[tabled(order = 0)] V3(u8) }, headers: ["V3", "V1", "V2"], tests: V1(0) => ["", "+", ""], V2(0) => ["", "", "+"], V3(0) => ["+", "", ""],);
    test_enum!(order_7, t: { V1(u8) V2(u8) #[tabled(order = 1)] V3(u8) }, headers: ["V1", "V3", "V2"], tests: V1(0) => ["+", "", ""], V2(0) => ["", "", "+"], V3(0) => ["", "+", ""],);
    test_enum!(order_8, t: { V1(u8) V2(u8) #[tabled(order = 2)] V3(u8) }, headers: ["V1", "V2", "V3"], tests: V1(0) => ["+", "", ""], V2(0) => ["", "+", ""], V3(0) => ["", "", "+"],);
    test_enum!(order_9, t: { #[tabled(order = 2)] V1(u8) V2(u8) #[tabled(order = 0)] V3(u8) }, headers: ["V3", "V2", "V1"], tests: V1(0) => ["", "", "+"], V2(0) => ["", "+", ""], V3(0) => ["+", "", ""],);
    test_enum!(order_10, t: { #[tabled(order = 2)] V1(u8) V2(u8) #[tabled(order = 1)] V3(u8) }, headers: ["V2", "V3", "V1"], tests: V1(0) => ["", "", "+"], V2(0) => ["+", "", ""], V3(0) => ["", "+", ""],);
    test_enum!(order_11, t: { #[tabled(order = 2)] V1(u8) #[tabled(order = 2)] V2(u8) #[tabled(order = 1)] V3(u8) }, headers: ["V1", "V3", "V2"], tests: V1(0) => ["+", "", ""], V2(0) => ["", "", "+"], V3(0) => ["", "+", ""],);
    test_enum!(order_12, t: { #[tabled(order = 2)] V1(u8) #[tabled(order = 1)] V2(u8) #[tabled(order = 0)] V3(u8) }, headers: ["V3", "V2", "V1"], tests: V1(0) => ["", "", "+"], V2(0) => ["", "+", ""], V3(0) => ["+", "", ""],);
    test_enum!(order_13, t: { #[tabled(order = 0)] V1(u8) #[tabled(order = 0)] V2(u8) #[tabled(order = 0)] V3(u8) }, headers: ["V3", "V1", "V2"], tests: V1(0) => ["", "+", ""], V2(0) => ["", "", "+"], V3(0) => ["+", "", ""],);
    test_enum!(order_14, t: { #[tabled(order = 1)] V1(u8) #[tabled(order = 1)] V2(u8) #[tabled(order = 1)] V3(u8) }, headers: ["V1", "V3", "V2"], tests: V1(0) => ["+", "", ""], V2(0) => ["", "", "+"], V3(0) => ["", "+", ""],);
    test_enum!(order_15, t: { #[tabled(order = 2)] V1(u8) #[tabled(order = 2)] V2(u8) #[tabled(order = 2)] V3(u8) }, headers: ["V1", "V2", "V3"], tests: V1(0) => ["+", "", ""], V2(0) => ["", "+", ""], V3(0) => ["", "", "+"],);

    test_enum!(order_0_inlined, t: #[tabled(inline)] { #[tabled(order = 1)] V1(u8) V2(u8) V3(u8) }, headers: ["TestType"], tests: V1(0) => ["V1"], V2(0) => ["V2"], V3(0) => ["V3"],);
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

    test_struct!(empty, t: { } init: { } expected: [], []);
    test_struct!(general, t: { f1: u8, f2: sstr } init: { f1: 0, f2: "v2" } expected: ["f1", "f2"], ["0", "v2"]);
    test_struct!(rename, t: { #[tabled(rename = "field 1")] f1: u8, #[tabled(rename = "field 2")] f2: sstr } init: { f1: 0, f2: "v2" } expected: ["field 1", "field 2"], ["0", "v2"]);
    test_struct!(skip, t: { #[tabled(skip)] f1: u8, #[tabled(rename = "field 2", skip)] f2: sstr, f3: sstr } init: { f1: 0, f2: "v2", f3: "123" } expected: ["f3"], ["123"]);
    test_struct!(skip_true, t: { #[tabled(skip = true)] f1: u8, #[tabled(rename = "field 2", skip = true)] f2: sstr, f3: sstr } init: { f1: 0, f2: "v2", f3: "123" } expected: ["f3"], ["123"]);
    test_struct!(
        inline,
        t: {
            #[tabled(inline = true)]
            id: u8,
            name: sstr,
            #[tabled(inline)]
            ed: Education
        }
        pre: {
            #[derive(Tabled)]
            struct Education { uni: sstr, graduated: bool }
        }
        init: { id: 0, name: "Maxim", ed: Education { uni: "BNTU", graduated: true }}
        expected: ["u8", "name","uni","graduated"], ["0", "Maxim", "BNTU", "true"]
    );
    test_struct!(
        inline_with_prefix,
        t: {
            #[tabled(rename = "it's an ignored option", inline)]
            id: u8,
            name: sstr,
            #[tabled(inline("education::"))]
            ed: Education,
        }
        pre: {
            #[derive(Tabled)]
            struct Education { uni: sstr, graduated: bool }
        }
        init: { id: 0, name: "Maxim", ed: Education { uni: "BNTU", graduated: true }}
        expected: ["u8", "name","education::uni","education::graduated"], ["0", "Maxim", "BNTU", "true"]
    );
    test_struct!(
        display_with,
        t: {
            f1: u8,
            #[tabled(display_with = "display_option")]
            f2: Option<sstr>,
        }
        pre: {
            fn display_option(o: &Option<sstr>) -> String {
                match o {
                    Some(s) => format!("some {s}"),
                    None => "none".to_string(),
                }
            }
        }
        init: { f1: 0, f2: Some("v2") }
        expected: ["f1", "f2"], ["0", "some v2"]
    );
    test_struct!(
        display_with_args,
        t: {
            f1: u8,
            #[tabled(display_with("display_option", 1, 2, 3))]
            f2: Option<sstr>,
        }
        pre: {
            fn display_option(v1: usize, v2: usize, v3: usize) -> String {
                format!("{v1} {v2} {v3}")
            }
        }
        init: { f1: 0, f2: Some("v2") }
        expected: ["f1", "f2"], ["0", "1 2 3"]
    );
    test_struct!(
        display_with_self_static_method,
        t: {
            f1: u8,
            #[tabled(display_with = "Self::display_option")]
            f2: Option<sstr>,
        }
        pre: {
            impl TestType {
                fn display_option(o: &Option<sstr>) -> String {
                    match o {
                        Some(s) => format!("some {s}"),
                        None => "none".to_string(),
                    }
                }
            }
        }
        init: { f1: 0, f2: Some("v2") }
        expected: ["f1", "f2"], ["0", "some v2"]
    );
    test_struct!(
        display_with_self_static_method_2,
        t: {
            f1: u8,
            #[tabled(display_with("Self::display_option", self))]
            f2: Option<sstr>,
        }
        pre: {
            impl TestType {
                fn display_option(o: &TestType) -> String {
                    match o.f2 {
                        Some(s) => format!("some {s}"),
                        None => "none".to_string(),
                    }
                }
            }
        }
        init: { f1: 0, f2: Some("v2") }
        expected: ["f1", "f2"], ["0", "some v2"]
    );
    test_struct!(
        display_with_self_2_self_static_method_2,
        t: {
            f1: u8,
            #[tabled(display_with("Self::display_option", self))]
            f2: Option<sstr>,
        }
        pre: {
            impl TestType {
                fn display_option(&self) -> String {
                    match self.f2 {
                        Some(s) => format!("some {s}"),
                        None => "none".to_string(),
                    }
                }
            }
        }
        init: { f1: 0, f2: Some("v2") }
        expected: ["f1", "f2"], ["0", "some v2"]
    );
    test_struct!(
        display_with_self_2_self_static_method,
        t: {
            f1: u8,
            #[tabled(display_with("display_option", self))]
            f2: Option<sstr>,
        }
        pre: {
            fn display_option(o: &TestType) -> String {
                match o.f2 {
                    Some(s) => format!("some {s}"),
                    None => "none".to_string(),
                }
            }
        }
        init: { f1: 0, f2: Some("v2") }
        expected: ["f1", "f2"], ["0", "some v2"]
    );
    test_struct!(order_0, t: { #[tabled(order = 0)] f0: u8, f1: u8, f2: u8 } init: { f0: 0, f1: 1, f2: 2 } expected: ["f0", "f1", "f2"], ["0", "1", "2"]);
    test_struct!(order_1, t: { #[tabled(order = 1)] f0: u8, f1: u8, f2: u8 } init: { f0: 0, f1: 1, f2: 2 } expected: ["f1", "f0", "f2"], ["1", "0", "2"]);
    test_struct!(order_2, t: { #[tabled(order = 2)] f0: u8, f1: u8, f2: u8 } init: { f0: 0, f1: 1, f2: 2 } expected: ["f1", "f2", "f0"], ["1", "2", "0"]);
    test_struct!(order_3, t: { f0: u8, #[tabled(order = 0)] f1: u8, f2: u8 } init: { f0: 0, f1: 1, f2: 2 } expected: ["f1", "f0", "f2"], ["1", "0", "2"]);
    test_struct!(order_4, t: { f0: u8, #[tabled(order = 1)] f1: u8, f2: u8 } init: { f0: 0, f1: 1, f2: 2 } expected: ["f0", "f1", "f2"], ["0", "1", "2"]);
    test_struct!(order_5, t: { f0: u8, #[tabled(order = 2)] f1: u8, f2: u8 } init: { f0: 0, f1: 1, f2: 2 } expected: ["f0", "f2", "f1"], ["0", "2", "1"]);
    test_struct!(order_6, t: { f0: u8, f1: u8, #[tabled(order = 0)] f2: u8 } init: { f0: 0, f1: 1, f2: 2 } expected: ["f2", "f0", "f1"], ["2", "0", "1"]);
    test_struct!(order_7, t: { f0: u8, f1: u8, #[tabled(order = 1)] f2: u8 } init: { f0: 0, f1: 1, f2: 2 } expected: ["f0", "f2", "f1"], ["0", "2", "1"]);
    test_struct!(order_8, t: { f0: u8, f1: u8, #[tabled(order = 2)] f2: u8 } init: { f0: 0, f1: 1, f2: 2 } expected: ["f0", "f1", "f2"], ["0", "1", "2"]);
    test_struct!(order_9, t: { #[tabled(order = 2)] f0: u8, f1: u8, #[tabled(order = 0)] f2: u8 } init: { f0: 0, f1: 1, f2: 2 } expected: ["f2", "f1", "f0"], ["2", "1", "0"]);
    test_struct!(order_10, t: { #[tabled(order = 2)] f0: u8, #[tabled(order = 1)] f1: u8, f2: u8 } init: { f0: 0, f1: 1, f2: 2 } expected: ["f2", "f1", "f0"], ["2", "1", "0"]);
    test_struct!(order_11, t: { #[tabled(order = 2)] f0: u8, #[tabled(order = 2)] f1: u8, #[tabled(order = 1)] f2: u8 } init: { f0: 0, f1: 1, f2: 2 } expected: ["f0", "f2", "f1"], ["0", "2", "1"]);
    test_struct!(order_12, t: { #[tabled(order = 2)] f0: u8, #[tabled(order = 1)] f1: u8, #[tabled(order = 0)] f2: u8 } init: { f0: 0, f1: 1, f2: 2 } expected: ["f2", "f1", "f0"], ["2", "1", "0"]);

    test_struct!(
        rename_all,
        t: #[tabled(rename_all = "UPPERCASE")] { f1: u8, f2: sstr }
        init: { f1: 0, f2: "v2" }
        expected: ["F1", "F2"], ["0", "v2"]
    );
    test_struct!(
        rename_all_override_in_field_by_rename,
        t: #[tabled(rename_all = "UPPERCASE")] { #[tabled(rename = "213213")] f1: u8, f2: sstr }
        init: { f1: 0, f2: "v2" }
        expected: ["213213", "F2"], ["0", "v2"]
    );
    test_struct!(
        rename_all_override_in_field_by_rename_all,
        t: #[tabled(rename_all = "UPPERCASE")] { #[tabled(rename_all = "lowercase")] f1: u8, f2: sstr }
        init: { f1: 0, f2: "v2" }
        expected: ["f1", "F2"], ["0", "v2"]
    );
    test_struct!(
        rename_all_field,
        t: { #[tabled(rename_all = "lowercase")] f1: u8, #[tabled(rename_all = "UPPERCASE")] f2: sstr }
        init: { f1: 0, f2: "v2" }
        expected: ["f1", "F2"], ["0", "v2"]
    );
    test_struct!(
        rename_all_field_overridden_by_rename,
        t: { #[tabled(rename_all = "lowercase", rename = "Hello")] f1: u8, #[tabled(rename_all = "UPPERCASE")] f2: sstr }
        init: { f1: 0, f2: "v2" }
        expected: ["Hello", "F2"], ["0", "v2"]
    );

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

test_tuple!(skipped_fields_not_implement_display_tuple, t: { #[tabled(skip)] () sstr }, init: { () "123" }, expected: ["1"], ["123"],);
test_struct!(skipped_fields_not_implement_display_struct, t: { #[tabled(skip)] _unit: (), s: sstr } init: { _unit: (), s: "123" } expected: ["s"], ["123"],);
test_struct!(
    skipped_fields_not_implement_display_struct_in_inline,
    t: { s: sstr, #[tabled(inline)] f: S1 }
    pre: {
        #[derive(Tabled)]
        struct S1 {
            #[tabled(skip)]
            _unit: (),
            s: sstr,
        }
    }
    init: { s: "123", f: S1 { _unit: (), s: "..." } }
    expected: ["s", "s"], ["123", "..."],
);
test_enum!(
    skipped_fields_not_implement_display_enum,
    t: {
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
    },
    headers: ["A::name", "B::issue", "B::name", "C::0", "C::2", "D"],
    tests:
        A { name: "nrdxp" } => ["nrdxp", "", "", "", "", ""],
        B { _gem: (), issue: 32, name: "nrdxp" } => ["", "32", "nrdxp", "", "", ""],
        C(32, (), "nrdxp") => ["", "", "", "32", "nrdxp", ""],
        D => ["", "", "", "", "", "+"],
);

test_struct!(
    ignore_display_with_when_used_with_inline,
    t: { f1: sstr, f2: sstr, #[tabled(display_with = "print", inline)] f3: usize }
    init: { f1: "123", f2: "456", f3: 789 }
    expected: ["f1", "f2", "usize"], ["123", "456", "789"],
);
test_struct!(
    ignore_display_with_when_used_with_inline_2,
    t: { f1: sstr, f2: sstr, #[tabled(display_with = "print", )] #[tabled(inline)] f3: usize }
    init: { f1: "123", f2: "456", f3: 789 }
    expected: ["f1", "f2", "usize"], ["123", "456", "789"],
);
test_struct!(
    display_with_and_rename,
    t: { f1: sstr, f2: sstr, #[tabled(display_with = "print", rename = "asd")] f3: usize }
    pre: { #[allow(dead_code)] fn print<T>(_: T) -> String { String::new() } }
    init: { f1: "123", f2: "456", f3: 789 }
    expected: ["f1", "f2", "asd"], ["123", "456", ""],
);
test_struct!(
    display_with_and_rename_2,
    t: { f1: sstr, f2: sstr, #[tabled(display_with = "print")] #[tabled(rename = "asd")] f3: usize }
    pre: { #[allow(dead_code)] fn print<T>(_: T) -> String { String::new() } }
    init: { f1: "123", f2: "456", f3: 789 }
    expected: ["f1", "f2", "asd"], ["123", "456", ""],
);
test_struct!(
    display_with_and_rename_all,
    t: { f1: sstr, f2: sstr, #[tabled(display_with = "print", rename_all = "UPPERCASE")] f3: usize }
    pre: { #[allow(dead_code)] fn print<T>(_: T) -> String { String::new() } }
    init: { f1: "123", f2: "456", f3: 789 }
    expected: ["f1", "f2", "F3"], ["123", "456", ""],
);

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

#[test]
fn test_order_skip_usage() {
    #[derive(Tabled, Default)]
    pub struct Example {
        #[tabled(skip)]
        #[allow(dead_code)]
        id: usize,
        name: String,
        #[tabled(order = 0)]
        details: String,
    }

    #[derive(Tabled, Default)]
    pub struct Example2 {
        #[tabled(skip)]
        #[allow(dead_code)]
        id: usize,
        name: String,
        #[tabled(order = 1)]
        details: String,
    }

    assert_eq!(Example::headers(), vec!["details", "name"],);
    assert_eq!(Example::default().fields(), vec!["", ""]);
}

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

mod __ {
    #[test]
    fn dont_import_the_trait() {
        #[derive(tabled::Tabled)]
        struct __;
    }
}
