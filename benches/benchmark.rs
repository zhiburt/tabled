use criterion::{black_box, criterion_group, criterion_main, BatchSize, BenchmarkId, Criterion};

macro_rules! bench_lib {
    ($name:ident, $data_fn:expr, $({ $lib_name:expr, $lib_table:expr }),* $(,)?) => {
        pub fn $name(c: &mut Criterion) {
            let mut group = c.benchmark_group(stringify!($name));

            for size in [1, 8, 32, 128, 512] {
                let (columns, data) = $data_fn(size);

                $({
                    group.bench_with_input(BenchmarkId::new($lib_name, size), &size, |b, _size| {
                        b.iter_batched(
                            || (columns.clone(), data.clone()),
                            |(columns, data)| { let _ = black_box($lib_table(columns, data)); },
                            BatchSize::SmallInput
                        );
                    });
                })*
            }

            group.finish();
        }
    };
}

macro_rules! create_bench {
    ($name:ident, $table:expr) => {
        bench_lib!(
            $name,
            $table,
            { "tabled", benchs::tabled::build },
            { "tabled_color", benchs::tabled_color::build },
            { "tabled_par", benchs::tabled_par::build },
            { "cli_table", benchs::cli_table::build  },
            { "comfy_table", benchs::comfy_table::build },
            { "term_table", benchs::term_table::build  },
            // { "tabled_master", benchs::tabled_master::build },
            // { "nu-table", benchs::nu_table::build  },
            // { "prettytable_rs", benchs::prettytable_rs::build },
        );
    };
}

create_bench!(test_empty_table, |size| build_cost_table(size, "", ""));

create_bench!(test_const_table, |size| build_cost_table(
    size,
    "Hello World",
    "Hi!"
));

create_bench!(test_dynamic_table, build_dynamic_table);

create_bench!(test_multiline_table, |size| build_cost_table(
    size,
    "H\ne\nl\nlo\nWo\nr\nld",
    "Hello\n111\n111\ni\n!"
));

criterion_group!(
    benches,
    test_empty_table,
    test_const_table,
    test_dynamic_table,
    test_multiline_table,
);
criterion_main!(benches);

fn build_cost_table<H, R>(size: usize, header: H, record: R) -> (Vec<String>, Vec<Vec<String>>)
where
    H: Into<String>,
    R: Into<String>,
{
    (
        vec![header.into(); size],
        vec![vec![record.into(); size]; size],
    )
}

fn build_dynamic_table(size: usize) -> (Vec<String>, Vec<Vec<String>>) {
    let mut data = Vec::with_capacity(size);
    for i in 0..size {
        let mut row = build_row(size, |n| format!("{}", n));

        // just make things more complex
        if i % 2 == 0 {
            row.sort_by(|a, b| b.cmp(a));
        }

        data.push(row);
    }

    let columns = build_row(size, |n| format!("{}", n));

    (columns, data)
}

fn build_row(size: usize, f: impl Fn(usize) -> String) -> Vec<String> {
    let mut row = Vec::with_capacity(size);
    for i in 0..size {
        let s = f(i);
        row.push(s);
    }

    row
}
