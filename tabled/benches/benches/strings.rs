use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};

pub fn string_width(c: &mut Criterion) {
    let mut group = c.benchmark_group("string_width");
    for size in [1 << 8, 1 << 15, 1 << 22] {
        group.bench_with_input(
            BenchmarkId::from_parameter(format!("tabled-{size}")),
            &size,
            |b, &size| {
                let text = black_box(build_string(size));
                b.iter(|| black_box(tabled::grid::util::string::string_width(&text)));
            },
        );

        group.bench_with_input(
            BenchmarkId::from_parameter(format!("tabled_master-{size}")),
            &size,
            |b, &size| {
                let text = black_box(build_string(size));
                b.iter(|| black_box(tabled_master::papergrid::util::string_width(&text)));
            },
        );
    }
    group.finish();
}

pub fn string_width_multiline(c: &mut Criterion) {
    let mut group = c.benchmark_group("string_width_multiline");
    for size in [1 << 8, 1 << 15, 1 << 22] {
        group.bench_with_input(
            BenchmarkId::from_parameter(format!("tabled-{size}")),
            &size,
            |b, &size| {
                let text = black_box(build_string_multiline(size));
                b.iter(|| black_box(tabled::grid::util::string::string_width(&text)));
            },
        );

        group.bench_with_input(
            BenchmarkId::from_parameter(format!("tabled_master-{size}")),
            &size,
            |b, &size| {
                let text = black_box(build_string_multiline(size));
                b.iter(|| black_box(tabled_master::papergrid::util::string_width(&text)));
            },
        );
    }
    group.finish();
}

pub fn wrap(c: &mut Criterion) {
    let mut group = c.benchmark_group("wrap");
    for size in [1 << 8, 1 << 15, 1 << 22] {
        group.bench_with_input(
            BenchmarkId::from_parameter(format!("tabled-{size}")),
            &size,
            |b, &size| {
                let text = black_box(build_string(size));
                b.iter(|| {
                    black_box(
                        tabled::Table::new(&[&text])
                            .with(tabled::settings::width::Width::wrap(1))
                            .to_string(),
                    )
                });
            },
        );

        group.bench_with_input(
            BenchmarkId::from_parameter(format!("tabled_master-{size}")),
            &size,
            |b, &size| {
                let text = black_box(build_string(size));
                b.iter(|| {
                    black_box(
                        tabled_master::Table::new([&text])
                            .with(tabled_master::Width::wrap(1))
                            .to_string(),
                    )
                });
            },
        );
    }
    group.finish();
}

fn build_string(size: usize) -> String {
    use owo_colors::OwoColorize;
    use std::fmt::Write;

    let mut buf = String::new();
    for i in 0..size {
        writeln!(buf, "{}", i.red().on_bright_purple()).unwrap();
    }

    buf
}

fn build_string_multiline(size: usize) -> String {
    use owo_colors::OwoColorize;
    use std::fmt::Write;

    let mut buf = String::new();
    for i in 0..size {
        writeln!(buf, "{}\n", i.red().on_bright_purple()).unwrap();
    }

    buf
}

criterion_group!(benches, string_width, string_width_multiline, wrap);
criterion_main!(benches);
