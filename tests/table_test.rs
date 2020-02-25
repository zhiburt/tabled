use tabled::{table, Tabled};

#[test]
fn table_vector_structures() {
    #[derive(Tabled)]
    struct St {
        f1: u8,
        f2: &'static str,
    }

    let st = vec![St { f1: 0, f2: "0" }, St { f1: 1, f2: "1" }];
    let expected = "+----+----+\n\
                    | f1 | f2 |\n\
                    +----+----+\n\
                    | 0  | 0  |\n\
                    +----+----+\n\
                    | 1  | 1  |\n\
                    +----+----+\n";

    let table = table(st);

    assert_eq!(expected, table);
}

#[test]
fn table_empty_vector_structures() {
    #[derive(Tabled)]
    struct St {
        f1: u8,
        f2: &'static str,
    }

    let st: Vec<St> = Vec::new();
    let expected = "+----+----+\n\
                    | f1 | f2 |\n\
                    +----+----+\n";

    let table = table(st);

    assert_eq!(expected, table);
}
