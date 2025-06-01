use std::hint::black_box;

use criterion::{criterion_group, criterion_main, Criterion};

use tabled::{Table, Tabled};

macro_rules! table_bench {
    ($name:ident, $table:expr, $( $modificator:expr ),*) => {
        pub fn $name(c: &mut Criterion) {
            for size in [1, 4, 8, 64, 512, 1024] {
                let entry = $table;
                let data = vec![&entry; size];
                let table = Table::new(data);

                let id = format!("{}_to_string_{}", stringify!($name), size);
                c.bench_function(&id, |b| b.iter(|| black_box(&table).to_string()));
            }
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

table_bench!(big_table, [0; 100]);

criterion_group!(benches, small_table, big_table);
criterion_main!(benches);
