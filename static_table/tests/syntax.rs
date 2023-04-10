use static_table::static_table;

#[test]
fn static_table() {
    static_table!([[1, 2, 123], [1, 2, 123], [1, 2, 123]]);
}

#[test]
fn static_table_trailing_comma() {
    static_table!([[1, 2, 123], [1, 2, 123], [1, 2, 123],]);
}

#[test]
fn static_table_trailing_comma_inside() {
    static_table!([[1, 2, 123,], [1, 2, 123,], [1, 2, 123,]]);
}

#[test]
fn static_table_empty() {
    static_table!([]);
}

#[test]
fn static_table_with_init_columns() {
    static_table!([[1; 3], [1; 5], [1, 2, 123]]);
}

#[test]
fn static_table_vspan() {
    static_table!([
        [{
            1;
            3
        }],
        [1, 2, 123],
        [1, 2, 123]
    ]);
}

#[test]
fn static_table_vspan_const() {
    static_table!([
        [{
            1;
            3
        }; 5],
        [1, 2, 123],
        [1, 2, 123]
    ]);
}

#[test]
fn static_table_hspan() {
    static_table!([[{ "Hello World" }, 2, 3], [{}, 2, 123], [1, 2, 123]]);
}

#[test]
fn static_table_layer_0() {
    static_table!([
        [{ "Hello World", 2, 3 }],
        [{ 1, 2 }, 4],
        [1, { 2 }, 123]
    ]);
}

#[test]
fn static_table_style() {
    static_table!([[1, 2, 123], [1, 2, 123], [1, 2, 123]], THEME = "MODERN");
}

// #[test(compail_fail)]
// fn static_table_verbatim() {
//     static_table!([[asd], [zxc], [1, 2, 123]]);
// }

// #[test]
// fn static_table_with_init_rows() {
//     static_table::static_table!([[1, 1, 3]; 3]);
// }

// #[test]
// fn static_table_style() {
//     static_table::static_table!([[1, 2, 123], [1, 2, 123], [1, 2, 123],], Style::modern());
// }

// #[test]
// fn static_table_span___7() {
//     static_table::static_table!([
//         [{ 1, 2, 3 }],
//         [{{ 1, 2, 3 }}],
//         [{{{ 1, 2, 3 }}}],
//     ]);
// }

// #[test]
// fn static_table_span___7() {
//     static_table::static_table!([
//         [{ 112 }, 1, {112; 2}],
//         [{     }, {{{1, 2, 3}}}],
//         [{{23; 2}}, 1,   2, 3],
//     ]);
// }

// #[test]
// fn static_table_span___7() {
//     static_table::static_table!([
//         [{ 112 }, 1, {112; 2}],
//         [{     }, 1,   2, 3],
//         [{{23; 2}}, 1,   2, 3],
//     ]);
// }

// #[test]
// fn static_table_span___5() {
//     static_table::static_table!([
//         [{112; 2}, 1, {112; 2}],
//         [{      }, 1,   2, 3],
//         [{23; 3}, 1,   2, 3],
//     ]);
// }

// #[test]
// fn static_table_span___5() {
//     static_table::static_table!([
//         [{112; 2}, 1, {112; 2}],
//         [{      }, 1,   2, 3],
//         [{{23; 2}}, 1,   2, 3],
//     ]);
// }

// #[test]
// fn static_table_span___4() {
//     static_table::static_table!([
//         [{112; 2}, 1, {112; 2}],
//         [{      }, 1,   2, 3],
//         [{      }, 1,   2, 3],
//     ]);
// }

// #[test]
// fn static_table_span___3() {
//     static_table::static_table!([
//         [{112; hspan=3, vspan=2, group=1}, 1, {112; hspan=2}],
//         [                                  1, 2, 3],
//         [                                  1 2 3],
//     ]);
// }

// #[test]
// fn static_table_span___3() {
//     static_table::static_table!([
//         [
//             {
//                 112 | 3;
//                 2
//             },
//             1,
//             {
//                 2;
//                 2
//             }
//         ],
//         [{
//             {
//                 123;
//                 3
//             }
//         }],
//         [1, {
//             11;
//             2
//         }],
//     ]);
// }

// #[test]
// fn static_table_span___3() {
//     static_table::static_table!([
//         [{ 112 | 3 }, 1, {
//             2;
//             2
//         }],
//         [{
//             {
//                 123;
//                 3
//             }
//         }],
//         [1, {
//             11;
//             2
//         }],
//     ]);
// }

// #[test]
// fn static_table_span___3() {
//     static_table::static_table!([
//         [1, {
//             2;
//             2
//         }],
//         [{
//             {
//                 123;
//                 3
//             }
//         }],
//         [1, {
//             11;
//             2
//         }],
//     ]);
// }

// #[test]
// fn static_table_span___3() {
//     static_table::static_table!([
//         [1, {2, 123}],
//         [{{1, 2, 123}}],
//         [1, {2, 123}],
//     ]);
// }

// // #[test]
// // fn static_table_span___1() {
// //     static_table::static_table!([
// //         [1, [2], 123],
// //         [1, [2], 123],
// //         _
// //         [1, [2], 123],
// //         [1, [2], 123],
// //     ]);
// // }

// // #[test]
// // fn static_table_span___1() {
// //     static_table::static_table!([
// //         [1, {2, 123}],
// //         [{1, 2, 123}],
// //         [1, {2, 123}],
// //     ]);
// // }

// // #[test]
// // fn static_table_span___() {
// //     static_table::static_table!([
// //         [1, |2, 123],
// //         [1, |2, 123],
// //         [1, |2, 123],
// //     ]);
// // }

// // #[test]
// // fn static_table_row() {
// //     // we must support the
// //     static_table::static_table!([
// //         1, 2, [123, 2, 3]
// //     ]);
// // }
