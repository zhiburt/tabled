use std::hint::black_box;

use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};

use tabled::{grid::util::string, settings::Color};

pub fn string_width(c: &mut Criterion) {
    let mut group = c.benchmark_group("string_width");

    for size in [1 << 8, 1 << 15, 1 << 22] {
        group.bench_with_input(
            BenchmarkId::from_parameter(format!("tabled-{size}")),
            &size,
            |b, &size| {
                let text = black_box(build_string(size));
                b.iter(|| black_box(string::get_string_width(&text)));
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
                b.iter(|| black_box(string::get_string_width(&text)));
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
                            .with(tabled::settings::Width::wrap(1))
                            .to_string(),
                    )
                });
            },
        );
    }

    group.finish();
}

fn build_string(size: usize) -> String {
    let mut buf = String::new();
    for i in 0..size {
        let s = (Color::FG_RED | Color::BG_BRIGHT_CYAN).colorize(i.to_string());
        buf.push_str(&s);
    }

    buf
}

fn build_string_multiline(size: usize) -> String {
    let mut buf = String::new();
    for i in 0..size {
        let s = (Color::FG_RED | Color::BG_BRIGHT_CYAN).colorize(i.to_string());
        buf.push_str(&s);
        buf.push('\n');
    }

    buf
}

criterion_group!(benches, string_width, string_width_multiline, wrap);
criterion_main!(benches);
