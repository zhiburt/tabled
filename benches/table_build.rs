use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use tabled::{TableIteratorExt, Tabled};

fn build_simple_table() {
    #[derive(Tabled)]
    struct Entry {
        field1: String,
        field2: usize,
        field3: i32,
    }

    impl Entry {
        fn new(field1: &str, field2: usize, field3: i32) -> Self {
            Self {
                field1: field1.to_string(),
                field2,
                field3,
            }
        }
    }

    let data = vec![
        Entry::new("This is a text 0", 0, 1),
        Entry::new("This is a text 1", 2, 3),
        Entry::new("This is a text 2", 4, 5),
    ];

    let _table = data.table().to_string();
}

/// Create a dynamic 10x10 Table with width 400 and unevenly distributed content.
/// On top of that, most of the columns have some kind of constraint.
fn build_huge_table() {
    #[derive(Tabled)]
    struct Entry {
        field0: usize,
        field1: usize,
        field2: usize,
        field3: usize,
        field4: usize,
        field5: usize,
        field6: usize,
        field7: usize,
        field8: usize,
        field9: usize,
    }

    impl Entry {
        #[allow(clippy::too_many_arguments)]
        fn new(
            field0: usize,
            field1: usize,
            field2: usize,
            field3: usize,
            field4: usize,
            field5: usize,
            field6: usize,
            field7: usize,
            field8: usize,
            field9: usize,
        ) -> Self {
            Self {
                field0,
                field1,
                field2,
                field3,
                field4,
                field5,
                field6,
                field7,
                field8,
                field9,
            }
        }
    }

    let mut data = Vec::new();

    for i in 1..=10 {
        let entry = Entry::new(
            0 % i,
            1 % i,
            2 % i,
            3 % i,
            4 % i,
            5 % i,
            6 % i,
            7 % i,
            8 % i,
            9 % i,
        );
        data.push(entry);
    }

    let _table = data.table().to_string();
}

pub fn build_tables(crit: &mut Criterion) {
    crit.bench_function("Simple table", |b| b.iter(build_simple_table));
    crit.bench_function("Huge table", |b| b.iter(build_huge_table));
}

fn build_table_with_rows(size: usize) {
    let data = vec![(0, "123", "234"); size];
    let _table = data.table();
}

pub fn build_table_with_raws(c: &mut Criterion) {
    let size: usize = 1024;

    c.bench_with_input(BenchmarkId::new("input_example", size), &size, |b, &s| {
        b.iter(|| build_table_with_rows(s));
    });
}

criterion_group!(benches, build_tables, build_table_with_raws);
criterion_main!(benches);
