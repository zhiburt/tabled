use std::collections::HashMap;

use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use nu_table::{StyledString, TextStyle, Theme};

pub fn draw(c: &mut Criterion) {
    let mut group = c.benchmark_group(stringify!($name));
    for size in [1, 4, 8, 64, 512, 1024] {
        let headers = vec!["Hello".to_owned(); size];
        let data = vec![vec!["Hello".to_owned(); size]; size];

        let colors = HashMap::new();
        let termwidth = 1000;
        let config = nu_protocol::Config::default();
        let nutable = nu_shell_build(&headers, &data);

        group.bench_with_input(BenchmarkId::new("nu-table", size), &size, |b, &_| {
            b.iter(|| {
                let _ = black_box(nu_table::draw_table(&nutable, termwidth, &colors, &config));
            });
        });

        let tabled = tabled_build(&headers, &data);

        group.bench_with_input(BenchmarkId::new("tabled", size), &size, |b, &_| {
            b.iter(|| {
                let _ = black_box(tabled.to_string());
            });
        });
    }
    group.finish();
}

fn nu_shell_build(headers: &[String], content: &[Vec<String>]) -> nu_table::Table {
    let headers = headers
        .iter()
        .map(|s| StyledString::new(s.to_owned(), TextStyle::default()))
        .collect::<Vec<_>>();

    let data = content
        .iter()
        .map(|line| {
            line.iter()
                .map(|s| StyledString::new(s.to_owned(), TextStyle::default()))
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    nu_table::Table::new(headers, data, Theme::basic())
}

fn tabled_build(headers: &[String], content: &[Vec<String>]) -> tabled::Table {
    let headers = headers.to_vec();
    let data = content.to_vec();

    tabled::builder::Builder::from(data)
        .set_columns(headers)
        .build()
}

// macro_rules! table_bench {
//     ($name:ident, $table:expr, $( $modificator:expr ),*) => {
//         pub fn $name(c: &mut Criterion) {
//             let mut group = c.benchmark_group(stringify!($name));
//             for size in [1, 4, 8, 64, 512, 1024] {
//                 group.bench_with_input(BenchmarkId::from_parameter(size), &size, |b, &size| {
//                     b.iter(|| {
//                         let entry = $table;
//                         let data = vec![entry; size];

//                         #[allow(unused_mut)]
//                         let mut table = black_box(data.table());

//                         $(table = table.with($modificator);)*

//                         let _ = black_box(table.to_string());
//                     });
//                 });
//             }
//             group.finish();
//         }
//     };
//     ($name:ident, $table:expr) => {
//         table_bench! { $name, $table, }
//     };
// }

// table_bench!(small_table, {
//     #[derive(Tabled, Clone)]
//     struct Entry {
//         field1: String,
//         field2: usize,
//         field3: i32,
//     }

//     Entry {
//         field1: "This is a text 0".to_string(),
//         field2: 0,
//         field3: 1,
//     }
// });

// table_bench!(
//     small_table_stylish,
//     [0; 3],
//     Style::modern(),
//     Modify::new(Segment::all())
//         .with(Alignment::left())
//         .with(Alignment::top())
//         .with(Padding::new(1, 1, 0, 2))
// );

criterion_group!(benches, draw);
criterion_main!(benches);

// criterion_group!(benches, small_table, big_table, small_table_stylish);
// criterion_main!(benches);
