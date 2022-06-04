use tabled::Table;

use crate::util::{create_vector, static_table};

mod util;

#[test]
fn builder_index() {
    let table = Table::builder(create_vector::<3, 2>()).index().build();

    assert_eq!(
        table.to_string(),
        static_table!(
            "+---+---+----------+----------+"
            "|   | N | column 0 | column 1 |"
            "+---+---+----------+----------+"
            "| 0 | 0 |   0-0    |   0-1    |"
            "+---+---+----------+----------+"
            "| 1 | 1 |   1-0    |   1-1    |"
            "+---+---+----------+----------+"
            "| 2 | 2 |   2-0    |   2-1    |"
            "+---+---+----------+----------+"
        )
    );
}

#[test]
fn builder_index_transpose() {
    let table = Table::builder(create_vector::<4, 2>())
        .index()
        .transpose()
        .build();

    assert_eq!(
        table.to_string(),
        static_table!(
            "+----------+-----+-----+-----+-----+"
            "|          |  0  |  1  |  2  |  3  |"
            "+----------+-----+-----+-----+-----+"
            "|    N     |  0  |  1  |  2  |  3  |"
            "+----------+-----+-----+-----+-----+"
            "| column 0 | 0-0 | 1-0 | 2-0 | 3-0 |"
            "+----------+-----+-----+-----+-----+"
            "| column 1 | 0-1 | 1-1 | 2-1 | 3-1 |"
            "+----------+-----+-----+-----+-----+"
        )
    );
}

#[test]
fn builder_index_transpose_transpose() {
    let data = create_vector::<4, 2>();
    let builder = Table::builder(&data).index();

    let orig_table = builder.clone().build().to_string();
    let two_times_transposed_table = builder.transpose().transpose().build().to_string();

    assert_eq!(orig_table, two_times_transposed_table,);
}

#[test]
fn builder_index_no_name_transpose_transpose() {
    let data = create_vector::<4, 2>();
    let builder = Table::builder(&data).index().set_name(None);

    let orig_table = builder.clone().build().to_string();
    let two_times_transposed_table = builder.transpose().transpose().build().to_string();

    assert_eq!(orig_table, two_times_transposed_table,);
}

#[test]
fn builder_index_0() {
    let table = Table::builder(create_vector::<4, 2>())
        .index()
        .set_index(0)
        .build();

    assert_eq!(
        table.to_string(),
        static_table!(
            "+---+----------+----------+"
            "|   | column 0 | column 1 |"
            "+---+----------+----------+"
            "| N |          |          |"
            "+---+----------+----------+"
            "| 0 |   0-0    |   0-1    |"
            "+---+----------+----------+"
            "| 1 |   1-0    |   1-1    |"
            "+---+----------+----------+"
            "| 2 |   2-0    |   2-1    |"
            "+---+----------+----------+"
            "| 3 |   3-0    |   3-1    |"
            "+---+----------+----------+"
        )
    );
}

#[test]
fn builder_index_0_no_name() {
    let table = Table::builder(create_vector::<4, 2>())
        .index()
        .set_index(0)
        .set_name(None)
        .build();

    assert_eq!(
        table.to_string(),
        static_table!(
            "+---+----------+----------+"
            "|   | column 0 | column 1 |"
            "+---+----------+----------+"
            "| 0 |   0-0    |   0-1    |"
            "+---+----------+----------+"
            "| 1 |   1-0    |   1-1    |"
            "+---+----------+----------+"
            "| 2 |   2-0    |   2-1    |"
            "+---+----------+----------+"
            "| 3 |   3-0    |   3-1    |"
            "+---+----------+----------+"
        )
    );
}

#[test]
fn builder_index_0_name() {
    let table = Table::builder(create_vector::<4, 2>())
        .index()
        .set_index(0)
        .set_name(Some("Hello World".to_owned()))
        .build();

    assert_eq!(
        table.to_string(),
        static_table!(
            "+-------------+----------+----------+"
            "|             | column 0 | column 1 |"
            "+-------------+----------+----------+"
            "| Hello World |          |          |"
            "+-------------+----------+----------+"
            "|      0      |   0-0    |   0-1    |"
            "+-------------+----------+----------+"
            "|      1      |   1-0    |   1-1    |"
            "+-------------+----------+----------+"
            "|      2      |   2-0    |   2-1    |"
            "+-------------+----------+----------+"
            "|      3      |   3-0    |   3-1    |"
            "+-------------+----------+----------+"
        )
    );
}

#[test]
fn builder_index_0_name_transpose() {
    let table = Table::builder(create_vector::<4, 2>())
        .index()
        .set_index(0)
        .set_name(Some("Hello World".to_owned()))
        .transpose()
        .build();

    assert_eq!(
        table.to_string(),
        static_table!(
            "+-------------+-----+-----+-----+-----+"
            "| Hello World |  0  |  1  |  2  |  3  |"
            "+-------------+-----+-----+-----+-----+"
            "|  column 0   | 0-0 | 1-0 | 2-0 | 3-0 |"
            "+-------------+-----+-----+-----+-----+"
            "|  column 1   | 0-1 | 1-1 | 2-1 | 3-1 |"
            "+-------------+-----+-----+-----+-----+"
        )
    );
}

#[test]
fn builder_index_with_no_columns() {
    let table = tabled::builder::Builder::default()
        .add_record(["1", "2", "3"])
        .add_record(["a", "b", "c"])
        .add_record(["d", "e", "f"])
        .index()
        .build();

    assert_eq!(
        table.to_string(),
        static_table!(
            "+---+---+---+---+"
            "|   | 0 | 1 | 2 |"
            "+---+---+---+---+"
            "| 0 | 1 | 2 | 3 |"
            "+---+---+---+---+"
            "| 1 | a | b | c |"
            "+---+---+---+---+"
            "| 2 | d | e | f |"
            "+---+---+---+---+"
        )
    );
}

#[test]
fn builder_index_with_no_columns_and_name() {
    let table = tabled::builder::Builder::default()
        .add_record(["1", "2", "3"])
        .add_record(["a", "b", "c"])
        .add_record(["d", "e", "f"])
        .index()
        .set_name(Some("Hello World".to_owned()))
        .build();

    assert_eq!(
        table.to_string(),
        static_table!(
            "+-------------+---+---+---+"
            "|             | 0 | 1 | 2 |"
            "+-------------+---+---+---+"
            "| Hello World |   |   |   |"
            "+-------------+---+---+---+"
            "|      0      | 1 | 2 | 3 |"
            "+-------------+---+---+---+"
            "|      1      | a | b | c |"
            "+-------------+---+---+---+"
            "|      2      | d | e | f |"
            "+-------------+---+---+---+"
        )
    );
}

#[test]
fn builder_index_with_no_columns_transpose() {
    let table = tabled::builder::Builder::default()
        .add_record(["1", "2", "3"])
        .add_record(["a", "b", "c"])
        .add_record(["d", "e", "f"])
        .index()
        .transpose()
        .build();

    assert_eq!(
        table.to_string(),
        static_table!(
            "+---+---+---+---+"
            "|   | 0 | 1 | 2 |"
            "+---+---+---+---+"
            "| 0 | 1 | a | d |"
            "+---+---+---+---+"
            "| 1 | 2 | b | e |"
            "+---+---+---+---+"
            "| 2 | 3 | c | f |"
            "+---+---+---+---+"
        )
    );
}

#[test]
fn builder_index_empty() {
    let table = tabled::builder::Builder::default().index().build();

    assert_eq!(table.to_string(), "");
}

#[test]
fn builder_index_transpose_empty() {
    let table = tabled::builder::Builder::default()
        .index()
        .transpose()
        .build();

    assert_eq!(table.to_string(), "");
}

#[test]
fn builder_index_invalid_dosnt_panic() {
    let table = tabled::builder::Builder::default()
        .index()
        .set_index(100)
        .build();

    assert_eq!(table.to_string(), "");
}

#[test]
fn builder_index_name_doesnt_shown_when_empty() {
    let table = tabled::builder::Builder::default()
        .index()
        .set_name(Some("Hello World".to_owned()))
        .build();

    assert_eq!(table.to_string(), "");
}
