#![cfg(feature = "derive")]

use tabled::Tabled;

use super::{test_enum, test_struct, test_tuple};

test_tuple!(tuple_order_0,  declare: { { #[tabled(order = 0)] u8 u8 u8} }                                           init: {} define: { 0 1 2 } assert_headers: { ["0", "1", "2"] } assert_fields: { ["0", "1", "2"] });
test_tuple!(tuple_order_1,  declare: { { #[tabled(order = 1)] u8 u8 u8} }                                           init: {} define: { 0 1 2 } assert_headers: { ["1", "0", "2"] } assert_fields: { ["1", "0", "2"] });
test_tuple!(tuple_order_2,  declare: { { #[tabled(order = 2)] u8 u8 u8} }                                           init: {} define: { 0 1 2 } assert_headers: { ["1", "2", "0"] } assert_fields: { ["1", "2", "0"] });
test_tuple!(tuple_order_3,  declare: { { u8 #[tabled(order = 0)] u8 u8} }                                           init: {} define: { 0 1 2 } assert_headers: { ["1", "0", "2"] } assert_fields: { ["1", "0", "2"] });
test_tuple!(tuple_order_4,  declare: { { u8 #[tabled(order = 1)] u8 u8} }                                           init: {} define: { 0 1 2 } assert_headers: { ["0", "1", "2"] } assert_fields: { ["0", "1", "2"] });
test_tuple!(tuple_order_5,  declare: { { u8 #[tabled(order = 2)] u8 u8} }                                           init: {} define: { 0 1 2 } assert_headers: { ["0", "2", "1"] } assert_fields: { ["0", "2", "1"] });
test_tuple!(tuple_order_6,  declare: { { u8 u8 #[tabled(order = 0)] u8} }                                           init: {} define: { 0 1 2 } assert_headers: { ["2", "0", "1"] } assert_fields: { ["2", "0", "1"] });
test_tuple!(tuple_order_7,  declare: { { u8 u8 #[tabled(order = 1)] u8} }                                           init: {} define: { 0 1 2 } assert_headers: { ["0", "2", "1"] } assert_fields: { ["0", "2", "1"] });
test_tuple!(tuple_order_8,  declare: { { u8 u8 #[tabled(order = 2)] u8} }                                           init: {} define: { 0 1 2 } assert_headers: { ["0", "1", "2"] } assert_fields: { ["0", "1", "2"] });
test_tuple!(tuple_order_9,  declare: { { #[tabled(order = 2)] u8 u8 #[tabled(order = 0)] u8} }                      init: {} define: { 0 1 2 } assert_headers: { ["2", "1", "0"] } assert_fields: { ["2", "1", "0"] });
test_tuple!(tuple_order_10, declare: { { #[tabled(order = 2)] u8 #[tabled(order = 1)] u8 u8} }                      init: {} define: { 0 1 2 } assert_headers: { ["2", "1", "0"] } assert_fields: { ["2", "1", "0"] });
test_tuple!(tuple_order_11, declare: { { #[tabled(order = 2)] u8 #[tabled(order = 2)] u8 #[tabled(order = 1)] u8} } init: {} define: { 0 1 2 } assert_headers: { ["0", "2", "1"] } assert_fields: { ["0", "2", "1"] });
test_tuple!(tuple_order_12, declare: { { #[tabled(order = 2)] u8 #[tabled(order = 2)] u8 #[tabled(order = 2)] u8} } init: {} define: { 0 1 2 } assert_headers: { ["0", "1", "2"] } assert_fields: { ["0", "1", "2"] });
test_tuple!(tuple_order_13, declare: { { #[tabled(order = 1)] u8 #[tabled(order = 1)] u8 #[tabled(order = 1)] u8} } init: {} define: { 0 1 2 } assert_headers: { ["0", "2", "1"] } assert_fields: { ["0", "2", "1"] });
test_tuple!(tuple_order_14, declare: { { #[tabled(order = 2)] u8 #[tabled(order = 1)] u8 #[tabled(order = 0)] u8} } init: {} define: { 0 1 2 } assert_headers: { ["2", "1", "0"] } assert_fields: { ["2", "1", "0"] });

test_enum!(enum_order_0,  declare: { { #[tabled(order = 0)] V1(u8) V2(u8) V3(u8) } }                                           init: {} assert_headers: { ["V1", "V2", "V3"] } assert: { V1(0) => ["+", "", ""], V2(0) => ["", "+", ""], V3(0) => ["", "", "+"],});
test_enum!(enum_order_1,  declare: { { #[tabled(order = 1)] V1(u8) V2(u8) V3(u8) } }                                           init: {} assert_headers: { ["V2", "V1", "V3"] } assert: { V1(0) => ["", "+", ""], V2(0) => ["+", "", ""], V3(0) => ["", "", "+"],});
test_enum!(enum_order_2,  declare: { { #[tabled(order = 2)] V1(u8) V2(u8) V3(u8) } }                                           init: {} assert_headers: { ["V2", "V3", "V1"] } assert: { V1(0) => ["", "", "+"], V2(0) => ["+", "", ""], V3(0) => ["", "+", ""],});
test_enum!(enum_order_3,  declare: { { V1(u8) #[tabled(order = 0)] V2(u8) V3(u8) } }                                           init: {} assert_headers: { ["V2", "V1", "V3"] } assert: { V1(0) => ["", "+", ""], V2(0) => ["+", "", ""], V3(0) => ["", "", "+"],});
test_enum!(enum_order_4,  declare: { { V1(u8) #[tabled(order = 1)] V2(u8) V3(u8) } }                                           init: {} assert_headers: { ["V1", "V2", "V3"] } assert: { V1(0) => ["+", "", ""], V2(0) => ["", "+", ""], V3(0) => ["", "", "+"],});
test_enum!(enum_order_5,  declare: { { V1(u8) #[tabled(order = 2)] V2(u8) V3(u8) } }                                           init: {} assert_headers: { ["V1", "V3", "V2"] } assert: { V1(0) => ["+", "", ""], V2(0) => ["", "", "+"], V3(0) => ["", "+", ""],});
test_enum!(enum_order_6,  declare: { { V1(u8) V2(u8) #[tabled(order = 0)] V3(u8) } }                                           init: {} assert_headers: { ["V3", "V1", "V2"] } assert: { V1(0) => ["", "+", ""], V2(0) => ["", "", "+"], V3(0) => ["+", "", ""],});
test_enum!(enum_order_7,  declare: { { V1(u8) V2(u8) #[tabled(order = 1)] V3(u8) } }                                           init: {} assert_headers: { ["V1", "V3", "V2"] } assert: { V1(0) => ["+", "", ""], V2(0) => ["", "", "+"], V3(0) => ["", "+", ""],});
test_enum!(enum_order_8,  declare: { { V1(u8) V2(u8) #[tabled(order = 2)] V3(u8) } }                                           init: {} assert_headers: { ["V1", "V2", "V3"] } assert: { V1(0) => ["+", "", ""], V2(0) => ["", "+", ""], V3(0) => ["", "", "+"],});
test_enum!(enum_order_9,  declare: { { #[tabled(order = 2)] V1(u8) V2(u8) #[tabled(order = 0)] V3(u8) } }                      init: {} assert_headers: { ["V3", "V2", "V1"] } assert: { V1(0) => ["", "", "+"], V2(0) => ["", "+", ""], V3(0) => ["+", "", ""],});
test_enum!(enum_order_10, declare: { { #[tabled(order = 2)] V1(u8) V2(u8) #[tabled(order = 1)] V3(u8) } }                      init: {} assert_headers: { ["V2", "V3", "V1"] } assert: { V1(0) => ["", "", "+"], V2(0) => ["+", "", ""], V3(0) => ["", "+", ""],});
test_enum!(enum_order_11, declare: { { #[tabled(order = 2)] V1(u8) #[tabled(order = 2)] V2(u8) #[tabled(order = 1)] V3(u8) } } init: {} assert_headers: { ["V1", "V3", "V2"] } assert: { V1(0) => ["+", "", ""], V2(0) => ["", "", "+"], V3(0) => ["", "+", ""],});
test_enum!(enum_order_12, declare: { { #[tabled(order = 2)] V1(u8) #[tabled(order = 1)] V2(u8) #[tabled(order = 0)] V3(u8) } } init: {} assert_headers: { ["V3", "V2", "V1"] } assert: { V1(0) => ["", "", "+"], V2(0) => ["", "+", ""], V3(0) => ["+", "", ""],});
test_enum!(enum_order_13, declare: { { #[tabled(order = 0)] V1(u8) #[tabled(order = 0)] V2(u8) #[tabled(order = 0)] V3(u8) } } init: {} assert_headers: { ["V3", "V1", "V2"] } assert: { V1(0) => ["", "+", ""], V2(0) => ["", "", "+"], V3(0) => ["+", "", ""],});
test_enum!(enum_order_14, declare: { { #[tabled(order = 1)] V1(u8) #[tabled(order = 1)] V2(u8) #[tabled(order = 1)] V3(u8) } } init: {} assert_headers: { ["V1", "V3", "V2"] } assert: { V1(0) => ["+", "", ""], V2(0) => ["", "", "+"], V3(0) => ["", "+", ""],});
test_enum!(enum_order_15, declare: { { #[tabled(order = 2)] V1(u8) #[tabled(order = 2)] V2(u8) #[tabled(order = 2)] V3(u8) } } init: {} assert_headers: { ["V1", "V2", "V3"] } assert: { V1(0) => ["+", "", ""], V2(0) => ["", "+", ""], V3(0) => ["", "", "+"],});

test_struct!(struct_order_0,  declare: { { #[tabled(order = 0)] f0: u8, f1: u8, f2: u8 } }                                           init: {} define: { f0: 0, f1: 1, f2: 2 } assert_headers: { ["f0", "f1", "f2"] } assert_fields: { ["0", "1", "2"] } );
test_struct!(struct_order_1,  declare: { { #[tabled(order = 1)] f0: u8, f1: u8, f2: u8 } }                                           init: {} define: { f0: 0, f1: 1, f2: 2 } assert_headers: { ["f1", "f0", "f2"] } assert_fields: { ["1", "0", "2"] } );
test_struct!(struct_order_2,  declare: { { #[tabled(order = 2)] f0: u8, f1: u8, f2: u8 } }                                           init: {} define: { f0: 0, f1: 1, f2: 2 } assert_headers: { ["f1", "f2", "f0"] } assert_fields: { ["1", "2", "0"] } );
test_struct!(struct_order_3,  declare: { { f0: u8, #[tabled(order = 0)] f1: u8, f2: u8 } }                                           init: {} define: { f0: 0, f1: 1, f2: 2 } assert_headers: { ["f1", "f0", "f2"] } assert_fields: { ["1", "0", "2"] } );
test_struct!(struct_order_4,  declare: { { f0: u8, #[tabled(order = 1)] f1: u8, f2: u8 } }                                           init: {} define: { f0: 0, f1: 1, f2: 2 } assert_headers: { ["f0", "f1", "f2"] } assert_fields: { ["0", "1", "2"] } );
test_struct!(struct_order_5,  declare: { { f0: u8, #[tabled(order = 2)] f1: u8, f2: u8 } }                                           init: {} define: { f0: 0, f1: 1, f2: 2 } assert_headers: { ["f0", "f2", "f1"] } assert_fields: { ["0", "2", "1"] } );
test_struct!(struct_order_6,  declare: { { f0: u8, f1: u8, #[tabled(order = 0)] f2: u8 } }                                           init: {} define: { f0: 0, f1: 1, f2: 2 } assert_headers: { ["f2", "f0", "f1"] } assert_fields: { ["2", "0", "1"] } );
test_struct!(struct_order_7,  declare: { { f0: u8, f1: u8, #[tabled(order = 1)] f2: u8 } }                                           init: {} define: { f0: 0, f1: 1, f2: 2 } assert_headers: { ["f0", "f2", "f1"] } assert_fields: { ["0", "2", "1"] } );
test_struct!(struct_order_8,  declare: { { f0: u8, f1: u8, #[tabled(order = 2)] f2: u8 } }                                           init: {} define: { f0: 0, f1: 1, f2: 2 } assert_headers: { ["f0", "f1", "f2"] } assert_fields: { ["0", "1", "2"] } );
test_struct!(struct_order_9,  declare: { { #[tabled(order = 2)] f0: u8, f1: u8, #[tabled(order = 0)] f2: u8 } }                      init: {} define: { f0: 0, f1: 1, f2: 2 } assert_headers: { ["f2", "f1", "f0"] } assert_fields: { ["2", "1", "0"] } );
test_struct!(struct_order_10, declare: { { #[tabled(order = 2)] f0: u8, #[tabled(order = 1)] f1: u8, f2: u8 } }                      init: {} define: { f0: 0, f1: 1, f2: 2 } assert_headers: { ["f2", "f1", "f0"] } assert_fields: { ["2", "1", "0"] } );
test_struct!(struct_order_11, declare: { { #[tabled(order = 2)] f0: u8, #[tabled(order = 2)] f1: u8, #[tabled(order = 1)] f2: u8 } } init: {} define: { f0: 0, f1: 1, f2: 2 } assert_headers: { ["f0", "f2", "f1"] } assert_fields: { ["0", "2", "1"] } );
test_struct!(struct_order_12, declare: { { #[tabled(order = 2)] f0: u8, #[tabled(order = 1)] f1: u8, #[tabled(order = 0)] f2: u8 } } init: {} define: { f0: 0, f1: 1, f2: 2 } assert_headers: { ["f2", "f1", "f0"] } assert_fields: { ["2", "1", "0"] } );

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
