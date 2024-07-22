#![cfg(feature = "derive")]
#![cfg(feature = "std")]

use tabled::Tabled;

// https://users.rust-lang.org/t/create-a-struct-from-macro-rules/19829
macro_rules! test_tuple {
    (
        $test_name:ident,
        { $(#[$struct_attr:meta])* { $( $(#[$attr:meta])* $ty:ty)* } },
        { $($init:expr)* },
        { $headers:expr, $fields:expr } $(,)?
        $(pre: { $($init_block:stmt)* })?
    ) => {
        #[test]
        fn $test_name() {
            $($($init_block)*)?

            #[derive(Tabled)]
            $(#[$struct_attr])*
            #[allow(dead_code)]
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
        { $(#[$struct_attr:meta])* { $( $(#[$var_attr:meta])*  $var:ident $({ $( $(#[$attr:meta])* $field:ident: $ty:ty),* $(,)? })? $(( $( $(#[$attr2:meta])* $ty2:ty),* $(,)? ))? )* } },
        { $($init_block:stmt)* },
        { $headers:expr },
        { $($init:expr => $expected:expr,)* }
    ) => {
        #[allow(dead_code, unused_imports)]
        #[test]
        fn $test_name() {
            $($init_block)*

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
        { $(#[$struct_attr:meta])* { $( $(#[$attr:meta])* $field:ident: $ty:ty),* $(,)? } } $(,)?
        { $($init_block:stmt)* } $(,)?
        { $( $val_field:ident: $val:expr),* } $(,)?
        { $headers:expr, $fields:expr $(,)? }  $(,)?
    ) => {

        #[allow(dead_code, unused_imports)]
        #[test]
        fn $test_name() {
            $($init_block)*

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

    test_tuple!(basic, { { u8 sstr } }, { 0 "v2" }, { ["0", "1"], ["0", "v2"] } );
    test_tuple!(empty, { { } }, { }, { [], [] });

    test_tuple!(rename, { { u8 #[tabled(rename = "field 2")] sstr } }, { 0 "123" }, { ["0", "field 2"], ["0", "123"] });

    test_tuple!(skip_0, { { #[tabled(skip)] u8 #[tabled(rename = "field 2", skip)] sstr sstr } },           { 0 "v2" "123" }, { ["2"], ["123"] });
    test_tuple!(skip_1, { { #[tabled(skip)] u8 #[tabled(skip)] #[tabled(rename = "field 2")] sstr sstr } }, { 0 "v2" "123" }, { ["2"], ["123"] });

    test_tuple!(order_0,  { { #[tabled(order = 0)] u8 u8 u8} },                                           { 0 1 2 }, { ["0", "1", "2"], ["0", "1", "2"] });
    test_tuple!(order_1,  { { #[tabled(order = 1)] u8 u8 u8} },                                           { 0 1 2 }, { ["1", "0", "2"], ["1", "0", "2"] });
    test_tuple!(order_2,  { { #[tabled(order = 2)] u8 u8 u8} },                                           { 0 1 2 }, { ["1", "2", "0"], ["1", "2", "0"] });
    test_tuple!(order_3,  { { u8 #[tabled(order = 0)] u8 u8} },                                           { 0 1 2 }, { ["1", "0", "2"], ["1", "0", "2"] });
    test_tuple!(order_4,  { { u8 #[tabled(order = 1)] u8 u8} },                                           { 0 1 2 }, { ["0", "1", "2"], ["0", "1", "2"] });
    test_tuple!(order_5,  { { u8 #[tabled(order = 2)] u8 u8} },                                           { 0 1 2 }, { ["0", "2", "1"], ["0", "2", "1"] });
    test_tuple!(order_6,  { { u8 u8 #[tabled(order = 0)] u8} },                                           { 0 1 2 }, { ["2", "0", "1"], ["2", "0", "1"] });
    test_tuple!(order_7,  { { u8 u8 #[tabled(order = 1)] u8} },                                           { 0 1 2 }, { ["0", "2", "1"], ["0", "2", "1"] });
    test_tuple!(order_8,  { { u8 u8 #[tabled(order = 2)] u8} },                                           { 0 1 2 }, { ["0", "1", "2"], ["0", "1", "2"] });
    test_tuple!(order_9,  { { #[tabled(order = 2)] u8 u8 #[tabled(order = 0)] u8} },                      { 0 1 2 }, { ["2", "1", "0"], ["2", "1", "0"] });
    test_tuple!(order_10, { { #[tabled(order = 2)] u8 #[tabled(order = 1)] u8 u8} },                      { 0 1 2 }, { ["2", "1", "0"], ["2", "1", "0"] });
    test_tuple!(order_11, { { #[tabled(order = 2)] u8 #[tabled(order = 2)] u8 #[tabled(order = 1)] u8} }, { 0 1 2 }, { ["0", "2", "1"], ["0", "2", "1"] });
    test_tuple!(order_12, { { #[tabled(order = 2)] u8 #[tabled(order = 2)] u8 #[tabled(order = 2)] u8} }, { 0 1 2 }, { ["0", "1", "2"], ["0", "1", "2"] });
    test_tuple!(order_13, { { #[tabled(order = 1)] u8 #[tabled(order = 1)] u8 #[tabled(order = 1)] u8} }, { 0 1 2 }, { ["0", "2", "1"], ["0", "2", "1"] });
    test_tuple!(order_14, { { #[tabled(order = 2)] u8 #[tabled(order = 1)] u8 #[tabled(order = 0)] u8} }, { 0 1 2 }, { ["2", "1", "0"], ["2", "1", "0"] });

    test_tuple!(rename_all, { #[tabled(rename_all = "UPPERCASE")] { u8 sstr } }, { 0 "123" }, { ["0", "1"], ["0", "123"] });

    test_tuple!(rename_all_field, { { u8 #[tabled(rename_all = "UPPERCASE")] sstr } },                                                  { 0 "123" }, { ["0", "1"], ["0", "123"] });
    test_tuple!(rename_all_field_with_rename_0, { { u8 #[tabled(rename_all = "UPPERCASE", rename = "Something")] sstr } },              { 0 "123" }, { ["0", "Something"], ["0", "123"] });
    test_tuple!(rename_all_field_with_rename_1, { { u8 #[tabled(rename = "Something")] #[tabled(rename_all = "UPPERCASE")] sstr } },    { 0 "123" }, { ["0", "Something"], ["0", "123"] });

    test_tuple!(
        display_option,
        { { u8 #[tabled(display_with = "display_option")] Option<sstr> } },
        { 0 Some("v2") },
        { ["0", "1"], ["0", "some v2"] },
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
        { { u8 #[tabled(display_with("display_option", 1, "234"))] Option<sstr> } },
        { 0 Some("v2") },
        { ["0", "1"], ["0", "some 1 234"] },
        pre: {
            fn display_option(val: usize, text: &str) -> String {
                format!("some {val} {text}")
            }
        }
    );

    test_tuple!(
        display_option_self,
        { { u8 #[tabled(display_with = "Self::display_option")] Option<sstr> } },
        { 0 Some("v2") },
        { ["0", "1"], ["0", "some v2"] },
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
        { { u8 #[tabled(display_with("Self::display_option", self))] Option<sstr> } },
        { 0 Some("v2") },
        { ["0", "1"], ["0", "some v2"] },
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
        { { u8 #[tabled(display_with("display_option", self))] Option<sstr> } },
        { 0 Some("v2") },
        { ["0", "1"], ["0", "some v2"] },
        pre: {
            fn display_option(o: &TestType) -> String {
                match o.1 {
                    Some(s) => format!("some {s}"),
                    None => "none".to_string(),
                }
            }
        }
    );

    test_tuple!(
        display_option_self_4,
        { { u8 #[tabled(display_with("display_option", self.0, self.0))] Option<sstr> } },
        { 0 Some("v2") },
        { ["0", "1"], ["0", "some 0.0"] },
        pre: {
            fn display_option(o1: u8, o2: u8) -> String {
                format!("some {o1}.{o2}")
            }
        }
    );

    test_tuple!(format_1, { { u8 #[tabled(format = "foo {}")] sstr } },                                         { 0 "v2" },                     { ["0", "1"], ["0", "foo v2"] });
    test_tuple!(format_2, { { u8 #[tabled(format = "foo {:?}")] sstr } },                                       { 0 "v2" },                     { ["0", "1"], ["0", "foo \"v2\""] });
    // todo : self represents the tuple here. It should be the sstr element instead.
    test_tuple!(format_3, { #[derive(Debug)] { u8 #[tabled(format("foo {} {:?}", 2, self))] String } },         { 0 String::from("string") },   { ["0", "1"], ["0", "foo 2 TestType(0, \"string\")"] });

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
        { { Security Embedded Frontend Unknown } },
        {},
        { ["Security", "Embedded", "Frontend", "Unknown"] },
        {
            Security => ["+", "", "", ""],
            Embedded => ["", "+", "", ""],
            Frontend => ["", "", "+", ""],
            Unknown =>  ["", "", "", "+"],
        }
    );

    test_enum!(
        diverse,
        { { A { a: u8, b: i32 } B(sstr) K } },
        {},
        { ["A", "B", "K"] },
        {
            A { a: 1, b: 2 } => ["+", "", ""],
            B("") => ["", "+",  ""],
            K => ["", "", "+"],
        }
    );

    test_enum!(
        rename_variant,
        { { #[tabled(rename = "Variant 1")] A { a: u8, b: i32 } #[tabled(rename = "Variant 2")] B(sstr) K } },
        {},
        { ["Variant 1", "Variant 2", "K"] },
        {
            A { a: 1, b: 2 } => ["+", "", ""],
            B("") => ["", "+",  ""],
            K => ["", "", "+"],
        }
    );

    test_enum!(
        skip_variant,
        { { A { a: u8, b: i32 } #[tabled(skip)] B(sstr) K } },
        {},
        { ["A", "K"] },
        {
            A { a: 1, b: 2 } => ["+", ""],
            B("") => ["", ""],
            K => ["", "+"],
        }
    );

    test_enum!(
        inline_variant,
        {
            {
            #[tabled(inline("Auto::"))] Auto { #[tabled(rename = "mod")] model: sstr, engine: sstr }
            #[tabled(inline)] Bikecycle( #[tabled(rename = "name")] sstr, #[tabled(inline)] Bike )
            Skateboard
            }
        },
        {
            #[derive(Tabled)]
            struct Bike { brand: sstr, price: f32 }
        },
        { ["Auto::mod", "Auto::engine", "name", "brand", "price", "Skateboard"] },
        {
            Skateboard => ["", "", "", "", "", "+"],
            Auto { model: "Mini", engine: "v8" } => ["Mini", "v8", "", "", "", ""],
            Bikecycle("A bike", Bike { brand: "Canyon", price: 2000.0 })=> ["", "", "A bike", "Canyon", "2000", ""],
        }
    );

    test_enum!(
        inline_field_with_display_function,
        {
            {
                #[tabled(inline("backend::"))]
                Backend {
                    #[tabled(display_with = "display", rename = "name")]
                    value: sstr
                }
                Frontend
            }
        },
        {
            fn display(_: sstr) -> String {
                "asd".to_string()
            }
        },
        { ["backend::name", "Frontend"] },
        {
            Backend { value: "123" } => ["asd", ""],
            Frontend => ["", "+"],
        }
    );

    test_enum!(
        inline_field_with_display_self_function,
        {
            {
                #[tabled(inline("backend::"))]
                Backend {
                    #[tabled()]
                    #[tabled(display_with("display", self), rename = "name")]
                    value: sstr
                }
                Frontend
            }
        },
        {
            fn display<T>(_: &T) -> String {
                "asd".to_string()
            }
        },
        { ["backend::name", "Frontend"] },
        {
            Backend { value: "123" } => ["asd", ""],
            Frontend => ["", "+"],
        }
    );

    test_enum!(
        with_display,
        {
            {
                #[tabled(inline)]
                A(
                    #[tabled(display_with = "format::<4>")]
                    sstr
                )
                B
            }
        },
        {
            fn format<const ID: usize>(_: sstr) -> String {
                ID.to_string()
            }
        },
        { ["0", "B"] },
        {
            A("") => ["4", ""],
            B => ["", "+"],
        }
    );

    test_enum!(
        with_display_self,
        {
            {
                #[tabled(inline)]
                A(
                    #[tabled(display_with("Self::format::<4>", self))]
                    sstr
                )
                B
            }
        },
        {
            impl TestType {
                fn format<const ID: usize>(&self) -> String {
                    ID.to_string()
                }
            }
        },
        { ["0", "B"] },
        {
            A("") => ["4", ""],
            B => ["", "+"],
        }
    );

    test_enum!(
        with_display_self_complex_0,
        {
            {
                #[tabled(inline)]
                A(
                    #[tabled(display_with("Self::format::<4>", self, self.0))]
                    sstr
                )
                B
            }
        },
        {
            impl TestType {
                fn format<const ID: usize>(&self, _: sstr) -> String {
                    ID.to_string()
                }
            }
        },
        { ["0", "B"] },
        {
            A("") => ["4", ""],
            B => ["", "+"],
        }
    );

    test_enum!(
        with_display_self_complex_1,
        {
            {
                #[tabled(inline)]
                A{
                    #[tabled(display_with("Self::format::<4>", self, self.asd))]
                    asd: sstr
                }
                B
            }
        },
        {
            impl TestType {
                fn format<const ID: usize>(&self, _: sstr) -> String {
                    ID.to_string()
                }
            }
        },
        { ["asd", "B"] },
        {
            A { asd: "" } => ["4", ""],
            B => ["", "+"],
        }
    );

    test_enum!(
        rename_all_variant,
        {
            {
                #[tabled(rename_all = "snake_case")]
                VariantName1 { a: u8, b: i32 }
                #[tabled(rename_all = "UPPERCASE")]
                VariantName2(String)
                K
            }
        },
        {},
        { ["variant_name1", "VARIANTNAME2", "K"] },
        {}
    );

    test_enum!(
        rename_all_enum,
        {
            #[tabled(rename_all = "snake_case")]
            {
                VariantName1 { a: u8, b: i32 }
                VariantName2(String)
                K
            }
        },
        {},
        { ["variant_name1", "variant_name2", "k"] },
        {}
    );

    test_enum!(
        rename_all_enum_inherited_inside_struct_enum,
        {
            #[tabled(rename_all = "snake_case")]
            {
                #[tabled(inline)]
                VariantName1 { some_field_1: u8, some_field_2: i32 }
                VariantName2(String)
                K
            }
        },
        {},
        { ["some_field_1", "some_field_2", "variant_name2", "k"] },
        {}
    );

    test_enum!(
        rename_all_enum_inherited_inside_struct_override_by_rename_enum,
        {
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
                K
            }
        },
        {},
        { ["f1", "f2", "variant_name2", "k"] },
        {}
    );

    test_enum!(
        rename_all_enum_inherited_inside_struct_override_by_rename_all_enum,
        {
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
                K
            }
        },
        {},
        { ["SOMEFIELD1", "someField2", "variant_name2", "k"] },
        {}
    );

    test_enum!(
        rename_all_variant_inherited_inside_struct_enum,
        {
            #[tabled(rename_all = "snake_case")]
            {
                #[tabled(inline)]
                #[tabled(rename_all = "snake_case")]
                VariantName1 {
                    some_field_1: u8,
                    some_field_2: i32,
                }
                VariantName2(String)
                K
            }
        },
        {},
        { ["some_field_1", "some_field_2", "variant_name2", "k"] },
        {}
    );

    test_enum!(
        rename_all_variant_inherited_inside_struct_enum_overridden_by_rename,
        {
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
                K
            }
        },
        {},
        { ["f1", "f2", "variant_name2", "k"] },
        {}
    );

    test_enum!(
        rename_all_variant_inherited_inside_struct_override_by_rename_all_enum,
        {
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
                K
            }
        },
        {},
        { ["SOMEFIELD1", "someField2", "variant_name2", "k"] },
        {}
    );

    test_enum!(
        inline_enum_as_whole,
        {
            #[tabled(inline)] {
                AbsdEgh { a: u8, b: i32 }
                B(String)
                K
            }
        },
        {},
        { ["TestType"] },
        {
            AbsdEgh { a: 0, b: 0 }  => ["AbsdEgh"],
            B(String::new()) => ["B"],
            K => ["K"],
        }
    );

    test_enum!(
        inline_enum_as_whole_and_rename,
        {
            #[tabled(inline, rename_all = "snake_case")]
            {
                AbsdEgh { a: u8, b: i32 }
                B(String)
                K
            }
        },
        {},
        { ["TestType"] },
        {
            AbsdEgh { a: 0, b: 0 }  => ["absd_egh"],
            B(String::new()) => ["b"],
            K => ["k"],
        }
    );

    test_enum!(
        inline_enum_as_whole_and_rename_inner,
        {
            #[tabled(inline)]
            {
                #[tabled(rename_all = "snake_case")]
                AbsdEgh { a: u8, b: i32 }
                #[tabled(rename_all = "lowercase")]
                B(String)
                K
            }
        },
        {},
        { ["TestType"] },
        {
            AbsdEgh { a: 0, b: 0 }  => ["absd_egh"],
            B(String::new()) => ["b"],
            K => ["K"],
        }
    );

    test_enum!(
        inline_enum_name,
        {
            #[tabled(inline("A struct name"))]
            {
                AbsdEgh { a: u8, b: i32 }
                B(String)
                K
            }
        },
        {},
        { ["A struct name"] },
        {
            AbsdEgh { a: 0, b: 0 }  => ["AbsdEgh"],
            B(String::new()) => ["B"],
            K => ["K"],
        }
    );

    test_enum!(
        enum_display_with_variant,
        {
            {
                #[tabled(display_with = "display_variant1")]
                AbsdEgh { a: u8, b: i32 }
                #[tabled(display_with = "display_variant2::<200>")]
                B(String)
                #[tabled(display_with = "some::bar::display_variant1")]
                K
            }
        },
        {
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
        },
        { ["AbsdEgh", "B", "K"] },
        {
            AbsdEgh { a: 0, b: 0 }  => ["Hello World", "", ""],
            B(String::new()) => ["", "asd 200", ""],
            K => ["", "", "Hello World 123"],
        }
    );

    test_enum!(
        enum_display_with_self_variant,
        {
            {
                #[tabled(display_with("display_variant1", self))]
                AbsdEgh { a: u8, b: i32 }
                #[tabled(display_with("display_variant2::<200, _>", self))]
                B(String)
                #[tabled(display_with("some::bar::display_variant1", self))]
                K
            }
        },
        {
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
        },
        { ["AbsdEgh", "B", "K"] },
        {
            AbsdEgh { a: 0, b: 0 }  => ["Hello World", "", ""],
            B(String::new()) => ["", "asd 200", ""],
            K => ["", "", "Hello World 123"],
        }
    );

    test_enum!(
        enum_display_with_arguments,
        {
            {
                #[tabled(display_with("display1", 1, 2, self))]
                AbsdEgh { a: u8, b: i32 }
                #[tabled(display_with("display2::<200>", "Hello World"))]
                B(String)
                #[tabled(display_with("display1", 100, 200, self))]
                K
            }
        },
        {
            fn display1<D>(val: usize, val2: usize, _: &D) -> String {
                format!("{val} {val2}")
            }

            fn display2<const VAL: usize>(val: &str) -> String {
                format!("asd {VAL} {val}")
            }
        },
        { ["AbsdEgh", "B", "K"] },
        {
            AbsdEgh { a: 0, b: 0 }  => ["1 2", "", ""],
            B(String::new()) => ["", "asd 200 Hello World", ""],
            K => ["", "", "100 200"],
        }
    );

    test_enum!(order_0,  { { #[tabled(order = 0)] V1(u8) V2(u8) V3(u8) } },                                               {}, { ["V1", "V2", "V3"] }, { V1(0) => ["+", "", ""], V2(0) => ["", "+", ""], V3(0) => ["", "", "+"],});
    test_enum!(order_1,  { { #[tabled(order = 1)] V1(u8) V2(u8) V3(u8) } },                                               {}, { ["V2", "V1", "V3"] }, { V1(0) => ["", "+", ""], V2(0) => ["+", "", ""], V3(0) => ["", "", "+"],});
    test_enum!(order_2,  { { #[tabled(order = 2)] V1(u8) V2(u8) V3(u8) } },                                               {}, { ["V2", "V3", "V1"] }, { V1(0) => ["", "", "+"], V2(0) => ["+", "", ""], V3(0) => ["", "+", ""],});
    test_enum!(order_3,  { { V1(u8) #[tabled(order = 0)] V2(u8) V3(u8) } },                                               {}, { ["V2", "V1", "V3"] }, { V1(0) => ["", "+", ""], V2(0) => ["+", "", ""], V3(0) => ["", "", "+"],});
    test_enum!(order_4,  { { V1(u8) #[tabled(order = 1)] V2(u8) V3(u8) } },                                               {}, { ["V1", "V2", "V3"] }, { V1(0) => ["+", "", ""], V2(0) => ["", "+", ""], V3(0) => ["", "", "+"],});
    test_enum!(order_5,  { { V1(u8) #[tabled(order = 2)] V2(u8) V3(u8) } },                                               {}, { ["V1", "V3", "V2"] }, { V1(0) => ["+", "", ""], V2(0) => ["", "", "+"], V3(0) => ["", "+", ""],});
    test_enum!(order_6,  { { V1(u8) V2(u8) #[tabled(order = 0)] V3(u8) } },                                               {}, { ["V3", "V1", "V2"] }, { V1(0) => ["", "+", ""], V2(0) => ["", "", "+"], V3(0) => ["+", "", ""],});
    test_enum!(order_7,  { { V1(u8) V2(u8) #[tabled(order = 1)] V3(u8) } },                                               {}, { ["V1", "V3", "V2"] }, { V1(0) => ["+", "", ""], V2(0) => ["", "", "+"], V3(0) => ["", "+", ""],});
    test_enum!(order_8,  { { V1(u8) V2(u8) #[tabled(order = 2)] V3(u8) } },                                               {}, { ["V1", "V2", "V3"] }, { V1(0) => ["+", "", ""], V2(0) => ["", "+", ""], V3(0) => ["", "", "+"],});
    test_enum!(order_9,  { { #[tabled(order = 2)] V1(u8) V2(u8) #[tabled(order = 0)] V3(u8) } },                          {}, { ["V3", "V2", "V1"] }, { V1(0) => ["", "", "+"], V2(0) => ["", "+", ""], V3(0) => ["+", "", ""],});
    test_enum!(order_10, { { #[tabled(order = 2)] V1(u8) V2(u8) #[tabled(order = 1)] V3(u8) } },                          {}, { ["V2", "V3", "V1"] }, { V1(0) => ["", "", "+"], V2(0) => ["+", "", ""], V3(0) => ["", "+", ""],});
    test_enum!(order_11, { { #[tabled(order = 2)] V1(u8) #[tabled(order = 2)] V2(u8) #[tabled(order = 1)] V3(u8) } },     {}, { ["V1", "V3", "V2"] }, { V1(0) => ["+", "", ""], V2(0) => ["", "", "+"], V3(0) => ["", "+", ""],});
    test_enum!(order_12, { { #[tabled(order = 2)] V1(u8) #[tabled(order = 1)] V2(u8) #[tabled(order = 0)] V3(u8) } },     {}, { ["V3", "V2", "V1"] }, { V1(0) => ["", "", "+"], V2(0) => ["", "+", ""], V3(0) => ["+", "", ""],});
    test_enum!(order_13, { { #[tabled(order = 0)] V1(u8) #[tabled(order = 0)] V2(u8) #[tabled(order = 0)] V3(u8) } },     {}, { ["V3", "V1", "V2"] }, { V1(0) => ["", "+", ""], V2(0) => ["", "", "+"], V3(0) => ["+", "", ""],});
    test_enum!(order_14, { { #[tabled(order = 1)] V1(u8) #[tabled(order = 1)] V2(u8) #[tabled(order = 1)] V3(u8) } },     {}, { ["V1", "V3", "V2"] }, { V1(0) => ["+", "", ""], V2(0) => ["", "", "+"], V3(0) => ["", "+", ""],});
    test_enum!(order_15, { { #[tabled(order = 2)] V1(u8) #[tabled(order = 2)] V2(u8) #[tabled(order = 2)] V3(u8) } },     {}, { ["V1", "V2", "V3"] }, { V1(0) => ["+", "", ""], V2(0) => ["", "+", ""], V3(0) => ["", "", "+"],});

    test_enum!(order_0_inlined, { #[tabled(inline)] { #[tabled(order = 1)] V1(u8) V2(u8) V3(u8) } }, {}, { ["TestType"] }, { V1(0) => ["V1"], V2(0) => ["V2"], V3(0) => ["V3"], });

    test_enum!(
        format,
        {
            {
                #[tabled(inline)]
                AbsdEgh {
                    #[tabled(format("{}-{}", self.a, self.b))]
                    a: u8,
                    b: i32
                }
                #[tabled(format("{} s", 4))]
                B(String)
                #[tabled(inline)]
                C(
                    #[tabled(rename = "C", format("{} ss", 4))]
                    String
                )
                #[tabled(format = "k.")]
                K
            }
        },
        {},
        { ["a", "b", "B", "C", "K"] },
        {
            AbsdEgh { a: 0, b: 1 }  => ["0-1", "1", "", "", ""],
            B(String::new()) => ["",  "", "4 s", "", ""],
            C(String::new()) => ["", "",  "", "4 ss", ""],
            K => ["", "", "",  "", "k."],
        }
    );

    test_enum!(
        format_complex,
        {
            {
                #[tabled(inline)]
                AbsdEgh {
                    #[tabled(format("{}-{}-{}", foo(*self.a as usize), foo(*self.b as usize), self.c[2]))]
                    a: u8,
                    b: i32,
                    #[tabled(skip)]
                    c: Vec<usize>,
                }
                #[tabled(format("{} s", foo(4)))]
                B(String)
                #[tabled(inline)]
                C(
                    #[tabled(rename = "C", format("{} ss {}", foo(4), self.1[2]))]
                    String,
                    #[tabled(skip)]
                    Vec<usize>,
                )
                #[tabled(format = "k.")]
                K
            }
        },
        {
            fn foo(a: usize) -> String {
                if a > 100 {
                    String::from(">100")
                } else {
                    String::from("<100")
                }
            }
        },
        { ["a", "b", "B", "C", "K"] },
        {
            AbsdEgh { a: 0, b: 1, c: vec![1, 2, 3] }  => ["<100-<100-3", "1", "", "", ""],
            B(String::new()) => ["",  "", "<100 s", "", ""],
            C(String::new(), vec![1, 2, 3]) => ["", "",  "", "<100 ss 3", ""],
            K => ["", "", "",  "", "k."],
        }
    );
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

    test_struct!(empty, { {} } {} { } { [], [] });
    test_struct!(
        general,
        {
            {
                f1: u8,
                f2: sstr,
            }
        }
        {}
        { f1: 0, f2: "v2" }
        {
            ["f1", "f2"],
            ["0", "v2"],
        }
    );
    test_struct!(
        rename,
        {
            {
                #[tabled(rename = "field 1")]
                f1: u8,
                #[tabled(rename = "field 2")]
                f2: sstr,
            }
        }
        {}
        { f1: 0, f2: "v2" }
        {
            ["field 1", "field 2"],
            ["0", "v2"],
        }
    );
    test_struct!(
        skip,
        {
            {
                #[tabled(skip)]
                f1: u8,
                #[tabled(rename = "field 2", skip)]
                f2: sstr,
                f3: sstr,
            }
        }
        {}
        { f1: 0, f2: "v2", f3: "123" }
        {
            ["f3"],
            ["123"],
        }
    );
    test_struct!(
        skip_true,
        {
            {
                #[tabled(skip = true)]
                f1: u8,
                #[tabled(rename = "field 2", skip = true)]
                f2: sstr,
                f3: sstr,
            }
        }
        {}
        { f1: 0, f2: "v2", f3: "123" }
        {
            ["f3"],
            ["123"],
        }
    );
    test_struct!(
        inline,
        {
            {
                #[tabled(inline = true)]
                id: u8,
                name: sstr,
                #[tabled(inline)]
                ed: Education
            }
        }
        {
            #[derive(Tabled)]
            struct Education { uni: sstr, graduated: bool }
        }
        {
            id: 0, name: "Maxim", ed: Education { uni: "BNTU", graduated: true }
        }
        {
            ["u8", "name","uni","graduated"],
            ["0", "Maxim", "BNTU", "true"]
        }
    );
    test_struct!(
        inline_with_prefix,
        {
            {
                #[tabled(rename = "it's an ignored option", inline)]
                id: u8,
                name: sstr,
                #[tabled(inline("education::"))]
                ed: Education,
            }
        }
        {
            #[derive(Tabled)]
            struct Education { uni: sstr, graduated: bool }
        }
        {
            id: 0, name: "Maxim", ed: Education { uni: "BNTU", graduated: true }}
        {
            ["u8", "name","education::uni","education::graduated"],
            ["0", "Maxim", "BNTU", "true"]
        }
    );
    test_struct!(
        display_with,
        {
            {
                f1: u8,
                #[tabled(display_with = "display_option")]
                f2: Option<sstr>,
            }
        }
        {
            fn display_option(o: &Option<sstr>) -> String {
                match o {
                    Some(s) => format!("some {s}"),
                    None => "none".to_string(),
                }
            }
        }
        { f1: 0, f2: Some("v2") }
        { ["f1", "f2"], ["0", "some v2"] }
    );
    test_struct!(
        display_with_args,
        {
            {
                f1: u8,
                #[tabled(display_with("display_option", 1, 2, 3))]
                f2: Option<sstr>,
            }
        }
        {
            fn display_option(v1: usize, v2: usize, v3: usize) -> String {
                format!("{v1} {v2} {v3}")
            }
        }
        { f1: 0, f2: Some("v2") }
        { ["f1", "f2"], ["0", "1 2 3"] }
    );
    test_struct!(
        display_with_args_using_self,
        {
            {
                f1: u8,
                #[tabled(display_with("display_option", &self.f1, 2, 3))]
                f2: Option<sstr>,
            }
        }
        {
            fn display_option(v1: &u8, v2: usize, v3: usize) -> String {
                format!("{v1} {v2} {v3}")
            }
        }
        { f1: 0, f2: Some("v2") }
        { ["f1", "f2"], ["0", "0 2 3"] }
    );
    test_struct!(
        display_with_self_static_method,
        {
            {
                f1: u8,
                #[tabled(display_with = "Self::display_option")]
                f2: Option<sstr>,
            }
        }
        {
            impl TestType {
                fn display_option(o: &Option<sstr>) -> String {
                    match o {
                        Some(s) => format!("some {s}"),
                        None => "none".to_string(),
                    }
                }
            }
        }
        { f1: 0, f2: Some("v2") }
        { ["f1", "f2"], ["0", "some v2"] }
    );
    test_struct!(
        display_with_self_static_method_2,
        {
            {
                f1: u8,
                #[tabled(display_with("Self::display_option", self))]
                f2: Option<sstr>,
            }
        }
        {
            impl TestType {
                fn display_option(o: &TestType) -> String {
                    match o.f2 {
                        Some(s) => format!("some {s}"),
                        None => "none".to_string(),
                    }
                }
            }
        }
        { f1: 0, f2: Some("v2") }
        { ["f1", "f2"], ["0", "some v2"] }
    );
    test_struct!(
        display_with_self_2_self_static_method_2,
        {
            {
                f1: u8,
                #[tabled(display_with("Self::display_option", self))]
                f2: Option<sstr>,
            }
        }
        {
            impl TestType {
                fn display_option(&self) -> String {
                    match self.f2 {
                        Some(s) => format!("some {s}"),
                        None => "none".to_string(),
                    }
                }
            }
        }
        { f1: 0, f2: Some("v2") }
        { ["f1", "f2"], ["0", "some v2"] }
    );
    test_struct!(
        display_with_self_2_self_static_method,
        {
            {
                f1: u8,
                #[tabled(display_with("display_option", self))]
                f2: Option<sstr>,
            }
        }
        {
            fn display_option(o: &TestType) -> String {
                match o.f2 {
                    Some(s) => format!("some {s}"),
                    None => "none".to_string(),
                }
            }
        }
        { f1: 0, f2: Some("v2") }
        { ["f1", "f2"], ["0", "some v2"] }
    );
    test_struct!(
        display_with_args_using_self_array_and_func,
        {
            {
                #[tabled(skip)]
                f1: [u8; 4],
                #[tabled(display_with("display_option", &[self.f1[0], self.f1[1]], ToString::to_string(&self.f3.to_string())))]
                f2: Option<sstr>,
                f3: usize,
            }
        }
        {
            fn display_option(v1: &[u8; 2], v4: String) -> String {
                format!("{} {} {v4}", v1[0], v1[1])
            }
        }
        { f1: [0, 1, 2, 3], f2: Some("v2"), f3: 100 }
        { ["f2", "f3"], ["0 1 100", "100"] }
    );
    test_struct!(order_0,  { { #[tabled(order = 0)] f0: u8, f1: u8, f2: u8 } }                                           {} { f0: 0, f1: 1, f2: 2 } { ["f0", "f1", "f2"], ["0", "1", "2"] });
    test_struct!(order_1,  { { #[tabled(order = 1)] f0: u8, f1: u8, f2: u8 } }                                           {} { f0: 0, f1: 1, f2: 2 } { ["f1", "f0", "f2"], ["1", "0", "2"] });
    test_struct!(order_2,  { { #[tabled(order = 2)] f0: u8, f1: u8, f2: u8 } }                                           {} { f0: 0, f1: 1, f2: 2 } { ["f1", "f2", "f0"], ["1", "2", "0"] });
    test_struct!(order_3,  { { f0: u8, #[tabled(order = 0)] f1: u8, f2: u8 } }                                           {} { f0: 0, f1: 1, f2: 2 } { ["f1", "f0", "f2"], ["1", "0", "2"] });
    test_struct!(order_4,  { { f0: u8, #[tabled(order = 1)] f1: u8, f2: u8 } }                                           {} { f0: 0, f1: 1, f2: 2 } { ["f0", "f1", "f2"], ["0", "1", "2"] });
    test_struct!(order_5,  { { f0: u8, #[tabled(order = 2)] f1: u8, f2: u8 } }                                           {} { f0: 0, f1: 1, f2: 2 } { ["f0", "f2", "f1"], ["0", "2", "1"] });
    test_struct!(order_6,  { { f0: u8, f1: u8, #[tabled(order = 0)] f2: u8 } }                                           {} { f0: 0, f1: 1, f2: 2 } { ["f2", "f0", "f1"], ["2", "0", "1"] });
    test_struct!(order_7,  { { f0: u8, f1: u8, #[tabled(order = 1)] f2: u8 } }                                           {} { f0: 0, f1: 1, f2: 2 } { ["f0", "f2", "f1"], ["0", "2", "1"] });
    test_struct!(order_8,  { { f0: u8, f1: u8, #[tabled(order = 2)] f2: u8 } }                                           {} { f0: 0, f1: 1, f2: 2 } { ["f0", "f1", "f2"], ["0", "1", "2"] });
    test_struct!(order_9,  { { #[tabled(order = 2)] f0: u8, f1: u8, #[tabled(order = 0)] f2: u8 } }                      {} { f0: 0, f1: 1, f2: 2 } { ["f2", "f1", "f0"], ["2", "1", "0"] });
    test_struct!(order_10, { { #[tabled(order = 2)] f0: u8, #[tabled(order = 1)] f1: u8, f2: u8 } }                      {} { f0: 0, f1: 1, f2: 2 } { ["f2", "f1", "f0"], ["2", "1", "0"] });
    test_struct!(order_11, { { #[tabled(order = 2)] f0: u8, #[tabled(order = 2)] f1: u8, #[tabled(order = 1)] f2: u8 } } {} { f0: 0, f1: 1, f2: 2 } { ["f0", "f2", "f1"], ["0", "2", "1"] });
    test_struct!(order_12, { { #[tabled(order = 2)] f0: u8, #[tabled(order = 1)] f1: u8, #[tabled(order = 0)] f2: u8 } } {} { f0: 0, f1: 1, f2: 2 } { ["f2", "f1", "f0"], ["2", "1", "0"] });

    test_struct!(
        rename_all,
        { #[tabled(rename_all = "UPPERCASE")] { f1: u8, f2: sstr } }
        {}
        { f1: 0, f2: "v2" }
        { ["F1", "F2"], ["0", "v2"] }
    );
    test_struct!(
        rename_all_override_in_field_by_rename,
        { #[tabled(rename_all = "UPPERCASE")] { #[tabled(rename = "213213")] f1: u8, f2: sstr } }
        {}
        { f1: 0, f2: "v2" }
        { ["213213", "F2"], ["0", "v2"] }
    );
    test_struct!(
        rename_all_override_in_field_by_rename_all,
        { #[tabled(rename_all = "UPPERCASE")] { #[tabled(rename_all = "lowercase")] f1: u8, f2: sstr } }
        {}
        { f1: 0, f2: "v2" }
        { ["f1", "F2"], ["0", "v2"] }
    );
    test_struct!(
        rename_all_field,
        { { #[tabled(rename_all = "lowercase")] f1: u8, #[tabled(rename_all = "UPPERCASE")] f2: sstr } }
        {}
        { f1: 0, f2: "v2" }
        { ["f1", "F2"], ["0", "v2"] }
    );
    test_struct!(
        rename_all_field_overridden_by_rename,
        { { #[tabled(rename_all = "lowercase", rename = "Hello")] f1: u8, #[tabled(rename_all = "UPPERCASE")] f2: sstr } }
        {}
        { f1: 0, f2: "v2" }
        { ["Hello", "F2"], ["0", "v2"] }
    );

    test_struct!(
        format,
        {
            {
                #[tabled(format = "{} cc")]
                f1: u8,
                f2: u8,
            }
        }
        {}
        { f1: 0, f2: 0 }
        { ["f1", "f2"], ["0 cc", "0"] }
    );

    test_struct!(
        format_with_args,
        {
            {
                #[tabled(format("{}/{} cc/kg", self.f1, self.f2))]
                f1: u8,
                f2: u8,
            }
        }
        {}
        { f1: 1, f2: 2 }
        { ["f1", "f2"], ["1/2 cc/kg", "2"] }
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

test_tuple!(skipped_fields_not_implement_display_tuple, { { #[tabled(skip)] () sstr } }, { () "123" }, { ["1"], ["123"] });
test_struct!(skipped_fields_not_implement_display_struct, { { #[tabled(skip)] _unit: (), s: sstr } } {} { _unit: (), s: "123" } { ["s"], ["123"] });
test_struct!(
    skipped_fields_not_implement_display_struct_in_inline,
    { { s: sstr, #[tabled(inline)] f: S1 } }
    {
        #[derive(Tabled)]
        struct S1 {
            #[tabled(skip)]
            _unit: (),
            s: sstr,
        }
    }
    { s: "123", f: S1 { _unit: (), s: "..." } }
    { ["s", "s"], ["123", "..."] },
);
test_enum!(
    skipped_fields_not_implement_display_enum,
    {
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
    },
    {},
    { ["A::name", "B::issue", "B::name", "C::0", "C::2", "D"] },
    {
        A { name: "nrdxp" } => ["nrdxp", "", "", "", "", ""],
        B { _gem: (), issue: 32, name: "nrdxp" } => ["", "32", "nrdxp", "", "", ""],
        C(32, (), "nrdxp") => ["", "", "", "32", "nrdxp", ""],
        D => ["", "", "", "", "", "+"],
    }
);

test_struct!(
    ignore_display_with_when_used_with_inline,
    { { f1: sstr, f2: sstr, #[tabled(display_with = "print", inline)] f3: usize } }
    {}
    { f1: "123", f2: "456", f3: 789 }
    { ["f1", "f2", "usize"], ["123", "456", "789"], }
);
test_struct!(
    ignore_display_with_when_used_with_inline_2,
    { { f1: sstr, f2: sstr, #[tabled(display_with = "print", )] #[tabled(inline)] f3: usize } }
    {}
    { f1: "123", f2: "456", f3: 789 }
    { ["f1", "f2", "usize"], ["123", "456", "789"], }
);
test_struct!(
    display_with_and_rename,
    { { f1: sstr, f2: sstr, #[tabled(display_with = "print", rename = "asd")] f3: usize } }
    { #[allow(dead_code)] fn print<T>(_: T) -> String { String::new() } }
    { f1: "123", f2: "456", f3: 789 }
    { ["f1", "f2", "asd"], ["123", "456", ""], }
);
test_struct!(
    display_with_and_rename_2,
    { { f1: sstr, f2: sstr, #[tabled(display_with = "print")] #[tabled(rename = "asd")] f3: usize } }
    { #[allow(dead_code)] fn print<T>(_: T) -> String { String::new() } }
    { f1: "123", f2: "456", f3: 789 }
    { ["f1", "f2", "asd"], ["123", "456", ""], }
);
test_struct!(
    display_with_and_rename_all,
    { { f1: sstr, f2: sstr, #[tabled(display_with = "print", rename_all = "UPPERCASE")] f3: usize } }
    { #[allow(dead_code)] fn print<T>(_: T) -> String { String::new() } }
    { f1: "123", f2: "456", f3: 789 }
    { ["f1", "f2", "F3"], ["123", "456", ""], }
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

#[test]
fn test_display_with_2() {
    #[derive(tabled::Tabled)]
    #[allow(dead_code)]
    struct Struct<'a> {
        #[tabled(display_with("std::path::Path::display"))]
        path: &'a std::path::Path,
    }
}

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

#[test]
#[allow(dead_code)]
fn test_macros_in_display_with() {
    #[derive(Tabled)]
    #[tabled(rename_all = "camelCase")]
    struct Country {
        name: String,
        #[tabled(display_with("display_capital", format!(".{}", self.capital)))]
        capital: String,
        #[tabled(display_with("display_perimeter", self))]
        area_km2: f32,
        #[tabled(display_with = "str::to_lowercase")]
        national_currency: String,
        national_currency_short: String,
    }

    fn display_perimeter(country: &Country) -> std::borrow::Cow<'_, str> {
        if country.area_km2 > 1_000_000.0 {
            "Very Big Land".into()
        } else {
            "Big Land".into()
        }
    }

    fn display_capital(country: String) -> std::borrow::Cow<'static, str> {
        format!("{country}!").into()
    }
}

#[test]
#[allow(dead_code)]
fn test_macros_in_format() {
    #[derive(Tabled)]
    struct Country {
        name: String,
        #[tabled(format("{}", format!(".{}", self.capital)))]
        capital: String,
        #[tabled(format("{}", self.field1[0]))]
        field1: [u8; 4],
    }
}

#[test]
#[allow(dead_code)]
fn test_enum_format_1() {
    #[derive(Tabled)]
    struct User {
        #[tabled(format("{}.{}.{}.{}", self.ip[0], self.ip[1], self.ip[2], self.ip[3]))]
        ip: [u8; 4],
        #[tabled(inline)]
        password: Password,
    }

    #[derive(Tabled)]
    enum Password {
        #[tabled(inline)]
        Mask {
            #[tabled(format("t={}/{}", self.text, self.factor))]
            text: String,
            #[tabled(skip)]
            factor: usize,
        },
        #[tabled(inline)]
        Plain(#[tabled(rename = "password")] String),
    }
}

mod __ {
    #[test]
    fn dont_import_the_trait() {
        #[derive(tabled::Tabled)]
        struct __;
    }
}
