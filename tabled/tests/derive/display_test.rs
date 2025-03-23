#![cfg(all(feature = "derive", feature = "assert"))]

use std::fmt::Display;

use tabled::{assert::test_table, Table, Tabled};

use super::{sstr, test_enum, test_struct, test_tuple};

test_tuple!(
    display_option,
    declare: { { u8 #[tabled(display = "display_option")] Option<sstr> } }
    init: {
        fn display_option(o: &Option<sstr>) -> String {
            match o {
                Some(s) => format!("some {s}"),
                None => "none".to_string(),
            }
        }
    }
    define: { 0 Some("v2") }
    assert_headers: { ["0", "1"] }
    assert_fields: { ["0", "some v2"] }
);

test_tuple!(
    display_option_args,
    declare: { { u8 #[tabled(display("display_option", 1, "234"))] Option<sstr> } }
    init: {
        fn display_option(_opt: &Option<sstr>, val: usize, text: &str) -> String {
            format!("some {val} {text}")
        }
    }
    define: { 0 Some("v2") }
    assert_headers: { ["0", "1"] }
    assert_fields: { ["0", "some 1 234"] }
);

test_tuple!(
    display_option_self,
    declare: { { u8 #[tabled(display = "Self::display_option")] Option<sstr> } }
    init: {
        impl Test {
            fn display_option(o: &Option<sstr>) -> String {
                match o {
                    Some(s) => format!("some {s}"),
                    None => "none".to_string(),
                }
            }
        }
    }
    define: { 0 Some("v2") }
    assert_headers: { ["0", "1"] }
    assert_fields: { ["0", "some v2"] }
);

test_tuple!(
    display_option_self_2,
    declare: { { u8 #[tabled(display("Self::display_option", self))] Option<sstr> } }
    init: {
        impl Test {
            fn display_option(_opt: &Option<sstr>, o: &Test) -> String {
                match o.1 {
                    Some(s) => format!("some {s}"),
                    None => "none".to_string(),
                }
            }
        }
    }
    define: { 0 Some("v2") }
    assert_headers: { ["0", "1"] }
    assert_fields: { ["0", "some v2"] }
);

test_tuple!(
    display_option_self_3,
    declare: { { u8 #[tabled(display("display_option", self))] Option<sstr> } }
    init: {
        fn display_option(_opt: &Option<sstr>, o: &Test) -> String {
            match o.1 {
                Some(s) => format!("some {s}"),
                None => "none".to_string(),
            }
        }
    }
    define: { 0 Some("v2") }
    assert_headers: { ["0", "1"] }
    assert_fields: { ["0", "some v2"] }
);

test_tuple!(
    display_option_self_4,
    declare: { { u8 #[tabled(display("display_option", self.0, self.0))] Option<sstr> } }
    init: {
        fn display_option(_opt: &Option<sstr>, o1: u8, o2: u8) -> String {
            format!("some {o1}.{o2}")
        }
    }
    define: { 0 Some("v2") }
    assert_headers: { ["0", "1"] }
    assert_fields: { ["0", "some 0.0"] }
);

test_enum!(
    enum_with_display,
    declare: {
        {
            #[tabled(inline)]
            A(
                #[tabled(display = "format::<4>")]
                sstr
            )
            B
        }
    }
    init: {
        fn format<const ID: usize>(_: sstr) -> String {
            ID.to_string()
        }
    }
    assert_headers: { ["0", "B"] }
    assert: {
        A("") => ["4", ""],
        B => ["", "+"],
    }
);

test_enum!(
    enum_with_display_self,
    declare: {
        {
            #[tabled(inline)]
            A(
                #[tabled(display("Self::format::<4>", self))]
                sstr
            )
            B
        }
    }
    init: {
        impl Test {
            fn format<const ID: usize>(_opt: &sstr, _: &Self) -> String {
                ID.to_string()
            }
        }
    }
    assert_headers: { ["0", "B"] }
    assert: {
        A("") => ["4", ""],
        B => ["", "+"],
    }
);

test_enum!(
    enum_with_display_self_complex_0,
    declare: {
        {
            #[tabled(inline)]
            A(
                #[tabled(display("Self::format::<4>", self, self.0))]
                sstr
            )
            B
        }
    }
    init: {
        impl Test {
            fn format<const ID: usize>(_opt: &sstr, _: &Self, _: sstr) -> String {
                ID.to_string()
            }
        }
    }
    assert_headers: { ["0", "B"] }
    assert: {
        A("") => ["4", ""],
        B => ["", "+"],
    }
);

test_enum!(
    enum_with_display_self_complex_1,
    declare: {
        {
            #[tabled(inline)]
            A{
                #[tabled(display("Self::format::<4>", self, self.asd))]
                asd: sstr
            }
            B
        }
    }
    init: {
        impl Test {
            fn format<const ID: usize>(_opt: &sstr, _: &Self, _: sstr) -> String {
                ID.to_string()
            }
        }
    }
    assert_headers: { ["asd", "B"] }
    assert: {
        A { asd: "" } => ["4", ""],
        B => ["", "+"],
    }
);

test_enum!(
    enum_display_with_variant,
    declare: {
        {
            #[tabled(display = "display_variant1")]
            AbsdEgh { a: u8, b: i32 }
            #[tabled(display = "display_variant2::<200>")]
            B(String)
            K
        }
    }
    init: {
        fn display_variant1(_: &Test) -> &'static str {
            "Hello World"
        }

        fn display_variant2<const VAL: usize>(_: &Test) -> String {
            format!("asd {VAL}")
        }
    }
    assert_headers: { ["AbsdEgh", "B", "K"] }
    assert: {
        AbsdEgh { a: 0, b: 0 }  => ["Hello World", "", ""],
        B(String::new()) => ["", "asd 200", ""],
        K => ["", "", "+"],
    }
);

test_enum!(
    enum_display_with_self_variant,
    declare: {
        {
            #[tabled(display("display_variant1", self))]
            AbsdEgh { a: u8, b: i32 }
            #[tabled(display("display_variant2::<200, _>", self))]
            B(String)
            K
        }
    }
    init: {
        fn display_variant1<D>(_: &Test, _: &D) -> &'static str {
            "Hello World"
        }

        fn display_variant2<const VAL: usize, D>(_: &Test, _: &D) -> String {
            format!("asd {VAL}")
        }
    }
    assert_headers: { ["AbsdEgh", "B", "K"] }
    assert: {
        AbsdEgh { a: 0, b: 0 }  => ["Hello World", "", ""],
        B(String::new()) => ["", "asd 200", ""],
        K => ["", "", "+"],
    }
);

test_enum!(
    enum_display_with_arguments,
    declare: {
        {
            #[tabled(display("display1", 1, 2, self))]
            AbsdEgh { a: u8, b: i32 }
            #[tabled(display("display2::<200>", "Hello World"))]
            B(String)
            #[tabled(display("display1", 100, 200, self))]
            K
        }
    }
    init: {
        fn display1<D>(_: &Test, val: usize, val2: usize, _: &D) -> String {
            format!("{val} {val2}")
        }

        fn display2<const VAL: usize>(_: &Test, val: &str) -> String {
            format!("asd {VAL} {val}")
        }
    }
    assert_headers: { ["AbsdEgh", "B", "K"] }
    assert: {
        AbsdEgh { a: 0, b: 0 }  => ["1 2", "", ""],
        B(String::new()) => ["", "asd 200 Hello World", ""],
        K => ["", "", "100 200"],
    }
);

test_struct!(
    struct_display_with,
    declare: {
        {
            f1: u8,
            #[tabled(display = "display_option")]
            f2: Option<sstr>,
        }
    }
    init: {
        fn display_option(o: &Option<sstr>) -> String {
            match o {
                Some(s) => format!("some {s}"),
                None => "none".to_string(),
            }
        }
    }
    define: { f1: 0, f2: Some("v2") }
    assert_headers: { ["f1", "f2"] }
    assert_fields: { ["0", "some v2"] }
);

test_struct!(
    struct_display_with_args,
    declare: {
        {
            f1: u8,
            #[tabled(display("display_option", 1, 2, 3))]
            f2: Option<sstr>,
        }
    }
    init: {
        fn display_option(_opt: &Option<sstr>, v1: usize, v2: usize, v3: usize) -> String {
            format!("{v1} {v2} {v3}")
        }
    }
    define: { f1: 0, f2: Some("v2") }
    assert_headers: { ["f1", "f2"] }
    assert_fields: { ["0", "1 2 3"] }
);

test_struct!(
    struct_display_with_args_using_self,
    declare: {
        {
            f1: u8,
            #[tabled(display("display_option", &self.f1, 2, 3))]
            f2: Option<sstr>,
        }
    }
    init: {
        fn display_option(_opt: &Option<sstr>, v1: &u8, v2: usize, v3: usize) -> String {
            format!("{v1} {v2} {v3}")
        }
    }
    define: { f1: 0, f2: Some("v2") }
    assert_headers: { ["f1", "f2"] }
    assert_fields: { ["0", "0 2 3"] }
);

test_struct!(
    struct_display_with_self_static_method,
    declare: {
        {
            f1: u8,
            #[tabled(display = "Self::display_option")]
            f2: Option<sstr>,
        }
    }
    init: {
        impl Test {
            fn display_option(o: &Option<sstr>) -> String {
                match o {
                    Some(s) => format!("some {s}"),
                    None => "none".to_string(),
                }
            }
        }
    }
    define: { f1: 0, f2: Some("v2") }
    assert_headers: { ["f1", "f2"] }
    assert_fields: { ["0", "some v2"] }
);

test_struct!(
    struct_display_with_self_static_method_2,
    declare: {
        {
            f1: u8,
            #[tabled(display("Self::display_option", self))]
            f2: Option<sstr>,
        }
    }
    init: {
        impl Test {
            fn display_option(_opt: &Option<sstr>, o: &Test) -> String {
                match o.f2 {
                    Some(s) => format!("some {s}"),
                    None => "none".to_string(),
                }
            }
        }
    }
    define: { f1: 0, f2: Some("v2") }
    assert_headers: { ["f1", "f2"] }
    assert_fields: { ["0", "some v2"] }
);

test_struct!(
    struct_display_with_self_2_self_static_method_2,
    declare: {
        {
            f1: u8,
            #[tabled(display("Self::display_option", self))]
            f2: Option<sstr>,
        }
    }
    init: {
        impl Test {
            fn display_option(_opt: &Option<sstr>, s: &Self) -> String {
                match s.f2 {
                    Some(s) => format!("some {s}"),
                    None => "none".to_string(),
                }
            }
        }
    }
    define: { f1: 0, f2: Some("v2") }
    assert_headers: { ["f1", "f2"] }
    assert_fields: { ["0", "some v2"] }
);

test_struct!(
    struct_display_with_self_2_self_static_method,
    declare: {
        {
            f1: u8,
            #[tabled(display("display_option", self))]
            f2: Option<sstr>,
        }
    }
    init: {
        fn display_option(_opt: &Option<sstr>, o: &Test) -> String {
            match o.f2 {
                Some(s) => format!("some {s}"),
                None => "none".to_string(),
            }
        }
    }
    define: { f1: 0, f2: Some("v2") }
    assert_headers: { ["f1", "f2"] }
    assert_fields: { ["0", "some v2"] }
);

test_struct!(
    struct_display_with_args_using_self_array_and_func,
    declare: {
        {
            #[tabled(skip)]
            f1: [u8; 4],
            #[tabled(display("display_option", &[self.f1[0], self.f1[1]], ToString::to_string(&self.f3.to_string())))]
            f2: Option<sstr>,
            f3: usize,
        }
    }
    init: {
        fn display_option(_opt: &Option<sstr>, v1: &[u8; 2], v4: String) -> String {
            format!("{} {} {v4}", v1[0], v1[1])
        }
    }
    define: { f1: [0, 1, 2, 3], f2: Some("v2"), f3: 100 }
    assert_headers: { ["f2", "f3"] }
    assert_fields: { ["0 1 100", "100"] }
);

#[test]
#[allow(dead_code)]
fn test_macros_in_display_with() {
    #[derive(Tabled)]
    #[tabled(rename_all = "camelCase")]
    struct Country {
        name: String,
        #[tabled(display("display_capital", format!(".{}", self.capital)))]
        capital: String,
        #[tabled(display("display_perimeter", self))]
        area_km2: f32,
        #[tabled(display = "str::to_lowercase")]
        national_currency: String,
        national_currency_short: String,
    }

    fn display_perimeter(_area: &f32, country: &Country) -> sstr {
        if country.area_km2 > 1_000_000.0 {
            "Very Big Land"
        } else {
            "Big Land"
        }
    }

    fn display_capital(_capital: &str, country: String) -> std::borrow::Cow<'static, str> {
        format!("{country}!").into()
    }
}

#[cfg(test)]
mod test_display_enum {
    use super::*;

    #[derive(Tabled)]
    pub enum User {
        #[tabled(display("some::bar::user_fmt"))]
        Public { id: usize },
        #[tabled(display("user_fmt"))]
        Private { id: usize },
    }

    fn user_fmt(_: &User) -> String {
        format!("...")
    }

    pub mod some {
        pub mod bar {
            pub fn user_fmt(_: &super::super::User) -> String {
                format!("111")
            }
        }
    }

    test_table!(
        test_display_enum,
        {
            let data = [
                User::Public { id: 0 },
                User::Private { id: 1 },
            ];

            Table::new(data)
        },
        "+--------+---------+"
        "| Public | Private |"
        "+--------+---------+"
        "| 111    |         |"
        "+--------+---------+"
        "|        | ...     |"
        "+--------+---------+"
    );
}

test_table!(
    test_display_type_0,
    {
        #[derive(Tabled)]
        #[tabled(display(Password, "password_fmt"))]
        struct User {
            id: usize,
            pass: Password,
            mirrow: Password,
        }

        struct Password([u8; 4]);

        fn password_fmt(p: &Password) -> String {
            p.0.iter().sum::<u8>().to_string()
        }

        let data = [
            User { id: 0, pass: Password([0, 1, 2, 3]), mirrow: Password([1, 1, 1, 1]) },
            User { id: 1, pass: Password([1, 1, 2, 3]), mirrow: Password([2, 2, 1, 1]) },
        ];

        Table::new(data)
    },
    "+----+------+--------+"
    "| id | pass | mirrow |"
    "+----+------+--------+"
    "| 0  | 6    | 4      |"
    "+----+------+--------+"
    "| 1  | 7    | 6      |"
    "+----+------+--------+"
);

test_table!(
    test_display_type_args,
    {
        #[derive(Tabled)]
        #[tabled(display(Password, "password_fmt", self, 0))]
        struct User {
            id: usize,
            pass: Password,
            mirrow: Password,
        }

        struct Password([u8; 4]);

        fn password_fmt(p: &Password, _: &User, _: usize) -> String {
            p.0.iter().sum::<u8>().to_string()
        }

        let data = [
            User {
                id: 0,
                pass: Password([0, 1, 2, 3]),
                mirrow: Password([1, 1, 1, 1]),
            },
            User {
                id: 1,
                pass: Password([1, 1, 2, 3]),
                mirrow: Password([2, 2, 1, 1]),
            },
        ];

        Table::new(data)
    },
    "+----+------+--------+"
    "| id | pass | mirrow |"
    "+----+------+--------+"
    "| 0  | 6    | 4      |"
    "+----+------+--------+"
    "| 1  | 7    | 6      |"
    "+----+------+--------+"
);

test_table!(
    test_display_type_generic,
    {
        #[derive(Tabled)]
        #[tabled(display(Password, "password_fmt"))]
        struct User {
            id: usize,
            pass: Password<usize>,
            mirrow: Password<u8>,
        }

        struct Password<T>([T; 4]);

        fn password_fmt<T>(p: &Password<T>) -> String where T: Display {
            p.0.iter().map(|s| s.to_string()).collect::<Vec<_>>().join("-")
        }

        let data = [
            User {
                id: 0,
                pass: Password([0, 1, 2, 3]),
                mirrow: Password([1, 1, 1, 1]),
            },
            User {
                id: 1,
                pass: Password([1, 1, 2, 3]),
                mirrow: Password([2, 2, 1, 1]),
            },
        ];

        Table::new(data)
    },
    "+----+---------+---------+"
    "| id | pass    | mirrow  |"
    "+----+---------+---------+"
    "| 0  | 0-1-2-3 | 1-1-1-1 |"
    "+----+---------+---------+"
    "| 1  | 1-1-2-3 | 2-2-1-1 |"
    "+----+---------+---------+"
);

test_table!(
    test_display_type_generic_use,
    {
        #[derive(Tabled)]
        #[tabled(display(Password<usize>, "password_usize_fmt"))]
        #[tabled(display(Password<u8>, "password_u8_fmt"))]
        struct User {
            id: usize,
            pass: Password<usize>,
            mirrow: Password<u8>,
        }

        struct Password<T>([T; 4]);

        fn password_usize_fmt(p: &Password<usize>) -> String {
            p.0.iter().sum::<usize>().to_string()
        }

        fn password_u8_fmt(p: &Password<u8>) -> String {
            p.0.iter().sum::<u8>().to_string()
        }

        let data = [
            User {
                id: 0,
                pass: Password([0, 1, 2, 3]),
                mirrow: Password([1, 1, 1, 1]),
            },
            User {
                id: 1,
                pass: Password([1, 1, 2, 3]),
                mirrow: Password([2, 2, 1, 1]),
            },
        ];

        Table::new(data)
    },
    "+----+------+--------+"
    "| id | pass | mirrow |"
    "+----+------+--------+"
    "| 0  | 6    | 4      |"
    "+----+------+--------+"
    "| 1  | 7    | 6      |"
    "+----+------+--------+"
);

test_struct!(
    display_with_and_rename,
    declare: {
        {
            f1: sstr,
            f2: sstr,
            #[tabled(display = "print", rename = "asd")]
            f3: usize,
        }
    }
    init: {
        fn print<T>(_: T) -> String { String::new() }
    }
    define: { f1: "123", f2: "456", f3: 789 }
    assert_headers: { ["f1", "f2", "asd"] }
    assert_fields: { ["123", "456", ""] }
);

test_struct!(
    display_with_and_rename_2,
    declare: {
        {
            f1: sstr,
            f2: sstr,
            #[tabled(display = "print")]
            #[tabled(rename = "asd")]
            f3: usize,
        }
    }
    init: {
        fn print<T>(_: T) -> String { String::new() }
    }
    define: { f1: "123", f2: "456", f3: 789 }
    assert_headers: { ["f1", "f2", "asd"] }
    assert_fields: { ["123", "456", ""] }
);

test_struct!(
    display_with_and_rename_all,
    declare: {
        {
            f1: sstr,
            f2: sstr,
            #[tabled(display = "print", rename_all = "UPPERCASE")]
            f3: usize,
        }
    }
    init: {
        fn print<T>(_: T) -> String { String::new() }
    }
    define: { f1: "123", f2: "456", f3: 789 }
    assert_headers: { ["f1", "f2", "F3"] }
    assert_fields: { ["123", "456", ""] }
);

// TODO: Shall we support a 'display_type' for #inlined structs??
// Seems like we have to?
// But there's a small issue currently - we treat enum variants as field currently in a parsing stage.
// And this particular case have no scense applied to struct field.
// Soooo
// Yes..
//
// test_table!(
//     test_display_type_enum,
//     {
//         #[derive(Tabled)]
//         enum User {
//             #[tabled(inline)]
//             #[tabled(display(Password<usize>, "password_usize_fmt"))]
//             #[tabled(display(Password<u8>, "password_u8_fmt"))]
//             Public {
//                 id: usize,
//                 pass: Password<usize>,
//                 mirrow: Password<u8>,
//             },
//             #[tabled(display("user_fmt"))]
//             Private {
//                 id: usize,
//             }
//         }

//         struct Password<T>([T; 4]);

//         fn password_usize_fmt(p: &Password<usize>) -> String {
//             p.0.iter().sum::<usize>().to_string()
//         }

//         fn password_u8_fmt(p: &Password<u8>) -> String {
//             p.0.iter().sum::<u8>().to_string()
//         }

//         fn user_fmt(p: &User) -> String {
//             format!("...")
//         }

//         let data = [
//             User::Public {
//                 id: 0,
//                 pass: Password([0, 1, 2, 3]),
//                 mirrow: Password([1, 1, 1, 1]),
//             },
//             User::Private {
//                 id: 1,
//             },
//         ];

//         Table::new(data)
//     },
//     "+----+------+--------+"
//     "| id | pass | mirrow |"
//     "+----+------+--------+"
//     "| 0  | 6    | 4      |"
//     "+----+------+--------+"
//     "| 1  | 7    | 6      |"
//     "+----+------+--------+"
// );
