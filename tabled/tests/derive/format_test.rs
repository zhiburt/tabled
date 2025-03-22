#![cfg(feature = "derive")]

use tabled::Tabled;

use super::{sstr, test_enum, test_struct, test_tuple};

test_tuple!(
    tuple_format_1,
    declare: { { u8 #[tabled(format = "foo {}")] sstr } }
    init: {}
    define: { 0 "v2" }
    assert_headers: { ["0", "1"] }
    assert_fields: { ["0", "foo v2"] }
);

test_tuple!(
    tuple_format_2,
    declare: { { u8 #[tabled(format = "foo {:?}")] sstr } }
    init: {}
    define: { 0 "v2" }
    assert_headers: { ["0", "1"] }
    assert_fields: { ["0", "foo \"v2\""] }
);

// todo : self represents the tuple here. It should be the sstr element instead.
test_tuple!(
    tuple_format_3,
    declare: {
        #[allow(dead_code)]
        #[derive(Debug)]
        {
            u8
            #[tabled(format("foo {} {:?}", 2, self))]
            String
        }
    }
    init: {}
    define: { 0 String::from("string") }
    assert_headers: { ["0", "1"] }
    assert_fields: { ["0", "foo 2 Test(0, \"string\")"] }
);

test_enum!(
    enum_format,
    declare: {
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
    }
    init: {}
    assert_headers: { ["a", "b", "B", "C", "K"] }
    assert: {
        AbsdEgh { a: 0, b: 1 }  => ["0-1", "1", "", "", ""],
        B(String::new()) => ["",  "", "4 s", "", ""],
        C(String::new()) => ["", "",  "", "4 ss", ""],
        K => ["", "", "",  "", "k."],
    }
);

test_enum!(
    enum_format_complex,
    declare: {
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
    }
    init: {
        fn foo(a: usize) -> String {
            if a > 100 {
                String::from(">100")
            } else {
                String::from("<100")
            }
        }
    }
    assert_headers: { ["a", "b", "B", "C", "K"] }
    assert: {
        AbsdEgh { a: 0, b: 1, c: vec![1, 2, 3] }  => ["<100-<100-3", "1", "", "", ""],
        B(String::new()) => ["",  "", "<100 s", "", ""],
        C(String::new(), vec![1, 2, 3]) => ["", "",  "", "<100 ss 3", ""],
        K => ["", "", "",  "", "k."],
    }
);

test_struct!(
    struct_format,
    declare: {
        {
            #[tabled(format = "{} cc")]
            f1: u8,
            f2: u8,
        }
    }
    init: {}
    define: { f1: 0, f2: 0 }
    assert_headers: { ["f1", "f2"] }
    assert_fields: { ["0 cc", "0"] }
);

test_struct!(
    struct_format_with_args,
    declare: {
        {
            #[tabled(format("{}/{} cc/kg", self.f1, self.f2))]
            f1: u8,
            f2: u8,
        }
    }
    init: {}
    define: { f1: 1, f2: 2 }
    assert_headers: { ["f1", "f2"] }
    assert_fields: { ["1/2 cc/kg", "2"] }
);

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
