use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use tabled::{Alignment, Full, Modify, Padding, Style, TableIteratorExt, Tabled};

macro_rules! table_bench {
    ($name:ident, $table:expr, $( $modificator:expr ),*) => {
        pub fn $name(c: &mut Criterion) {
            let mut group = c.benchmark_group(stringify!($name));
            for size in [1, 4, 8, 64, 512, 1024] {
                group.bench_with_input(BenchmarkId::from_parameter(size), &size, |b, &size| {
                    b.iter(|| {
                        let entry = $table;
                        let data = vec![entry; size];

                        #[allow(unused_mut)]
                        let mut table = black_box(data.table());

                        $(table = table.with($modificator);)*

                        let _ = black_box(table.to_string());
                    });
                });
            }
            group.finish();
        }
    };
    ($name:ident, $table:expr) => {
        table_bench! { $name, $table, }
    };
}

table_bench!(small_table, {
    #[derive(Tabled, Clone)]
    struct Entry {
        field1: String,
        field2: usize,
        field3: i32,
    }

    Entry {
        field1: "This is a text 0".to_string(),
        field2: 0,
        field3: 1,
    }
});

table_bench!(
    small_table_stylish,
    [0; 3],
    Style::modern(),
    Modify::new(Full)
        .with(Alignment::left())
        .with(Alignment::top())
        .with(Padding::new(1, 1, 0, 2))
);

table_bench!(big_table, { [0; 16] });

criterion_group!(benches, small_table, big_table, small_table_stylish);
criterion_main!(benches);
