use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};

macro_rules! bench_lib {
    ($name:ident, $data_fn:expr, $({ $lib_name:expr, $lib_table:expr, $lib_print:expr, }),* $(,)?) => {
        pub fn $name(c: &mut Criterion) {
            let mut group = c.benchmark_group(stringify!($name));

            for size in [1, 4, 8, 64, 512, 1024] {
                let (columns, data) = $data_fn(size);

                $({
                    let mut table = $lib_table(columns.clone(), data.clone());
                    group.bench_with_input(BenchmarkId::new($lib_name, size), &size, |b, _size| {
                        b.iter(|| {
                            let ptr: *mut _ = &mut table;
                            black_box(ptr);

                            let _ = black_box($lib_print(&table));

                            let ptr: *mut _ = &mut table;
                            black_box(ptr);
                        });
                    });
                })*
            }

            group.finish();
        }
    };
    ($name:ident, $table:expr) => {
        table_bench! { $name, $table, }
    };
}

macro_rules! create_bench {
    ($name:ident, $table:expr) => {
        bench_lib!(
            $name,
            $table,
            { "tabled", benchs::tabled::build, benchs::tabled::print, },
            { "tabled_color", benchs::tabled_color::build, benchs::tabled_color::print, },
            { "nu-table", benchs::nu_table::build, benchs::nu_table::print, },
            { "cli_table", benchs::cli_table::build, benchs::cli_table::print, },
            { "comfy_table", benchs::comfy_table::build, benchs::comfy_table::print, },
            { "term_table", benchs::term_table::build, benchs::term_table::print, },
            { "prettytable_rs", benchs::prettytable_rs::build, benchs::prettytable_rs::print, },
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

create_bench!(test_colored_table, |size| {
    use owo_colors::OwoColorize;

    let header = "Hello World".red().on_blue().to_string();
    let cell = "He\nll\no W\nor\nld".green().on_blue().to_string();
    build_cost_table(size, header, cell)
});

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
    let columns = build_row(size, |n| format!("{}", n));

    let mut side = false;
    let mut data = Vec::with_capacity(size);
    for _ in 0..size {
        let mut row = build_row(size, |n| format!("{}", n));
        if side {
            row = row.into_iter().rev().collect();
            side = false;
        } else {
            side = true;
        }

        data.push(row);
    }

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
