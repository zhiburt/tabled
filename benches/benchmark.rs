use std::collections::HashMap;

use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};

macro_rules! bench_lib {
    ($name:ident, $data_fn:expr, $({ $lib_name:expr, $lib_table:expr, $lib_print:expr, }),* $(,)?) => {
        pub fn $name(c: &mut Criterion) {
            let mut group = c.benchmark_group(stringify!($name));

            for size in [1, 4, 8, 64, 512, 1024] {
                let (columns, data) = $data_fn(size);

                $({
                    let table = $lib_table(columns.clone(), data.clone());
                    group.bench_with_input(BenchmarkId::new($lib_name, size), &size, |b, _size| {
                        b.iter(|| {
                            let _ = black_box({
                                $lib_print(&table);
                            });
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

bench_lib!(
    test_empty_table,
    |size| build_cost_table(size, "", ""),
    { "tabled", build_tabled, print_tabled, },
    { "nu-table", build_nu_table, print_nu_table, },
    { "cli_table", build_cli_table, print_cli_table, },
    { "comfy_table", build_comfy_table, print_comfy_table, },
    { "term_table", build_term_table, print_term_table, },
    { "prettytable_rs", build_prettytable_rs, print_prettytable_rs, },
);

bench_lib!(
    test_const_table,
    |size| build_cost_table(size, "Hello World", "Hi!"),
    { "tabled", build_tabled, print_tabled, },
    { "nu-table", build_nu_table, print_nu_table, },
    { "cli_table", build_cli_table, print_cli_table, },
    { "comfy_table", build_comfy_table, print_comfy_table, },
    { "term_table", build_term_table, print_term_table, },
    { "prettytable_rs", build_prettytable_rs, print_prettytable_rs, },
);

bench_lib!(
    test_dynamic_table,
    build_dynamic_table,
    { "tabled", build_tabled, print_tabled, },
    { "nu-table", build_nu_table, print_nu_table, },
    { "cli_table", build_cli_table, print_cli_table, },
    { "comfy_table", build_comfy_table, print_comfy_table, },
    { "term_table", build_term_table, print_term_table, },
    { "prettytable_rs", build_prettytable_rs, print_prettytable_rs, },
);

bench_lib!(
    test_multiline_table,
    |size| build_cost_table(size, "Hello\nWorld", "H\n11111\n\n\ni!"),
    { "tabled", build_tabled, print_tabled, },
    { "nu-table", build_nu_table, print_nu_table, },
    { "cli_table", build_cli_table, print_cli_table, },
    { "comfy_table", build_comfy_table, print_comfy_table, },
    { "term_table", build_term_table, print_term_table, },
    { "prettytable_rs", build_prettytable_rs, print_prettytable_rs, },
);

criterion_group!(
    benches,
    test_empty_table,
    test_const_table,
    test_dynamic_table,
    test_multiline_table
);
criterion_main!(benches);

fn build_cost_table(size: usize, s: &str, s1: &str) -> (Vec<String>, Vec<Vec<String>>) {
    (
        vec![s.to_owned(); size],
        vec![vec![s1.to_owned(); size]; size],
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

fn build_tabled(columns: Vec<String>, data: Vec<Vec<String>>) -> tabled::Table {
    tabled::builder::Builder::from(data)
        .set_columns(columns)
        .build()
}

type NuTableType = (
    nu_table::Table,
    HashMap<String, nu_ansi_term::Style>,
    nu_protocol::Config,
);

fn build_nu_table(columns: Vec<String>, data: Vec<Vec<String>>) -> NuTableType {
    let columns = columns
        .into_iter()
        .map(|c| nu_table::StyledString::new(c, nu_table::TextStyle::default()))
        .collect();

    let data = data
        .into_iter()
        .map(|row| {
            row.into_iter()
                .map(|c| nu_table::StyledString::new(c, nu_table::TextStyle::default()))
                .collect()
        })
        .collect();

    let table = nu_table::Table::new(columns, data, nu_table::TableTheme::basic());
    (table, HashMap::new(), nu_protocol::Config::default())
}

#[inline]
fn print_tabled(table: &tabled::Table) -> String {
    table.to_string()
}

#[inline]
fn print_nu_table((table, color_hm, config): &NuTableType) -> String {
    nu_table::draw_table(table, 1000000000, color_hm, config)
}

fn build_cli_table(columns: Vec<String>, mut data: Vec<Vec<String>>) -> cli_table::TableStruct {
    data.insert(0, columns);
    <Vec<Vec<String>> as cli_table::Table>::table(data)
}

#[inline]
fn print_cli_table(table: &cli_table::TableStruct) -> String {
    // here's a conversion and Vec<u8> cache which is something need to be aware of.
    table.display().unwrap().to_string()
}

fn build_comfy_table(columns: Vec<String>, mut data: Vec<Vec<String>>) -> comfy_table::Table {
    data.insert(0, columns);

    let mut t = comfy_table::Table::new();

    for row in data {
        t.add_row(comfy_table::Row::from(row));
    }

    t
}

#[inline]
fn print_comfy_table(table: &comfy_table::Table) -> String {
    table.to_string()
}

fn build_term_table(
    columns: Vec<String>,
    mut data: Vec<Vec<String>>,
) -> term_table::Table<'static> {
    data.insert(0, columns);

    let mut t = term_table::Table::new();

    for row in data {
        t.add_row(term_table::row::Row::new(row));
    }

    t
}

#[inline]
fn print_term_table(table: &term_table::Table<'static>) -> String {
    table.render()
}

fn build_prettytable_rs(columns: Vec<String>, mut data: Vec<Vec<String>>) -> prettytable::Table {
    data.insert(0, columns);

    let mut t = prettytable::Table::new();

    for row in data {
        t.add_row(prettytable::Row::from(row));
    }

    t
}

#[inline]
fn print_prettytable_rs(table: &prettytable::Table) -> String {
    table.to_string()
}
