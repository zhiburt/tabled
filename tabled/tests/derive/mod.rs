mod derive_test;
mod display_test;
mod format_test;
mod inline_test;
mod map_test;
mod order_test;
mod rename_all_test;
mod rename_test;
mod skip_test;

// https://users.rust-lang.org/t/create-a-struct-from-macro-rules/19829
#[allow(unused_macros)]
macro_rules! test_tuple {
    (
        $test_name:ident,
        declare: {
            $(#[$struct_attr:meta])*
            { $( $(#[$attr:meta])* $ty:ty)* }
        }
        init: {
            $($init_block:stmt)*
        }
        define: {
            $($init:expr)*
        }
        assert_headers: { $headers:expr }
        assert_fields: { $fields:expr }
    ) => {

        #[test]
        fn $test_name() {
            $($init_block)*

            #[derive(Tabled)]
            $(#[$struct_attr])*
            struct Test(
                $( $(#[$attr])* $ty, )*
            );

            let value = Test($($init,)*);

            let fields: Vec<&'static str> = $fields.to_vec();
            let headers: Vec<&'static str> = $headers.to_vec();

            assert_eq!(value.fields(), fields);
            assert_eq!(Test::headers(), headers);
            assert_eq!(<Test as Tabled>::LENGTH, headers.len());
            assert_eq!(<Test as Tabled>::LENGTH, fields.len());
        }

    };
}

#[allow(unused_macros)]
macro_rules! test_enum {
    (
        $test_name:ident,
        declare: {
            $(#[$struct_attr:meta])*
            {
                $(
                    $(#[$var_attr:meta])*
                    $var:ident
                    $({
                        $( $(#[$attr:meta])* $field:ident: $ty:ty),* $(,)?
                    })?
                    $((
                        $( $(#[$attr2:meta])* $ty2:ty),* $(,)?
                    ))?
                )*
            }
        }
        init: {
            $($init_block:stmt)*
        }
        assert_headers: {
            $headers:expr
        }
        assert: {
            $($init:expr => $expected:expr,)*
        }
    ) => {

        #[test]
        fn $test_name() {
            $($init_block)*

            #[derive(Tabled)]
            $(#[$struct_attr])*
            enum Test {
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
            assert_eq!(Test::headers(), headers);
            assert_eq!(<Test as Tabled>::LENGTH, headers.len());

            {
                use Test::*;
                $(
                    let variant = $init;
                    let fields: Vec<&'static str> = $expected.to_vec();
                    assert_eq!(variant.fields(), fields);
                )*
            }
        }

    };
}

#[allow(unused_macros)]
macro_rules! test_struct {
    (
        $test_name:ident,
        declare: {
            $(#[$struct_attr:meta])*
            {
                $(
                    $(#[$attr:meta])* $field:ident: $ty:ty
                ),* $(,)?
            }
        }
        init: { $($init_block:stmt)* }
        define: { $( $val_field:ident: $val:expr),* }
        assert_headers: { $headers:expr }
        assert_fields: { $fields:expr }
    ) => {

        #[test]
        fn $test_name() {
            $($init_block)*

            #[derive(Tabled)]
            $(#[$struct_attr])*
            struct Test {
                $(
                    $(#[$attr])*
                    $field: $ty,
                )*
            }

            let value = Test {
                $($val_field: $val,)*
            };

            let fields: Vec<&'static str> = $fields.to_vec();
            let headers: Vec<&'static str> = $headers.to_vec();
            assert_eq!(Test::headers(), headers);
            assert_eq!(value.fields(), fields);
            assert_eq!(<Test as Tabled>::LENGTH, headers.len());
            assert_eq!(<Test as Tabled>::LENGTH, fields.len());
        }
    };

}

#[allow(non_camel_case_types)]
#[allow(dead_code)]
pub(crate) type sstr = &'static str;

#[allow(unused_imports)]
pub(crate) use test_enum;
#[allow(unused_imports)]
pub(crate) use test_struct;
#[allow(unused_imports)]
pub(crate) use test_tuple;
