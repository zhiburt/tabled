use criterion::{black_box, criterion_group, criterion_main, BatchSize, BenchmarkId, Criterion};

macro_rules! bench_lib {
    ($name:ident, $data_fn:expr, $({ $lib_name:expr, $lib_table:expr }),* $(,)?) => {
        pub fn $name(c: &mut Criterion) {
            let mut group = c.benchmark_group(stringify!($name));

            for size in [1, 8, 32, 128, 512] {
                #[allow(clippy::redundant_closure_call)]
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
            { "tabled_master", lib_comp::tabled_current::build },
            { "tabled_master_color", lib_comp::tabled_color::build },
            { "tabled", lib_comp::tabled::build },
            { "tabled_color", lib_comp::tabled_color_current::build },
            // { "current_iter", lib_comp::tabled_current_iter::build },
            // { "current_compact", lib_comp::tabled_current_compact::build },
            // { "cli_table", lib_comp::cli_table::build  },
            // { "comfy_table", lib_comp::comfy_table::build },
            // { "term_table", lib_comp::term_table::build  },
            // { "prettytable_rs", lib_comp::prettytable_rs::build },
        );
    };
}

create_bench!(test_empty_table, benching_empty_table);
create_bench!(test_small_table, benching_small_table);
create_bench!(test_general_table, benching_general_table);
create_bench!(test_small_multiline_table, benching_small_multiline_table);

criterion_group!(
    benches,
    test_empty_table,
    test_small_table,
    test_general_table,
    test_small_multiline_table,
);
criterion_main!(benches);

fn benching_empty_table(size: usize) -> (Vec<String>, Vec<Vec<String>>) {
    build_cost_table(size, "", "")
}

fn benching_small_table(size: usize) -> (Vec<String>, Vec<Vec<String>>) {
    build_cost_table(size, "Hello World", "Hi!")
}

fn benching_small_multiline_table(size: usize) -> (Vec<String>, Vec<Vec<String>>) {
    build_cost_table(size, "H\ne\nl\nlo\nWo\nr\nld", "Hello\n111\n111\ni\n!")
}

fn benching_general_table(size: usize) -> (Vec<String>, Vec<Vec<String>>) {
    let mut data = Vec::with_capacity(size);
    for i in 0..size {
        let mut row = build_row(size, |i| format!("{i}"));

        // just make things more complex
        if i % 2 == 0 {
            row.sort_by(|a, b| b.cmp(a));
        }

        data.push(row);
    }

    let columns = build_row(size, |i| format!("{i}"));

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
