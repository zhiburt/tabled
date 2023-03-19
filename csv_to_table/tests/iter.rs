static CSV_DATA_1: &'_ str = r#"
Year,Industry_aggregation_NZSIOC,Industry_name_NZSIOC,Units,Variable_code,Variable_name,Variable_category,Value
2021,Level 1,All industries,Dollars (millions),H01,Total income,Financial performance,"757,504"
2021,Level 1,All industries,Dollars (millions),H04,"Sales, government funding, grants and subsidies"
2021,Level 1,All industries,Dollars (millions),H05,"Interest, dividends and donations",Financial performance,"49,593"
2021,Level 1,All industries,Dollars (millions),H07,Non-operating income,Financial performance,"33,020"
2021,Level 1,All industries,Dollars (millions),H08,Total expenditure,Financial performance,"654,404"
2021,Level 1,All industries,Dollars (millions),H09,Interest and donations,Financial performance,"26,138"
"#;

static CSV_DATA_2: &'_ str = r#"
Year,Industry_aggregation_NZSIOC,Industry_name_NZSIOC,Units,Variable_code,Variable_name,Variable_category,Value
2021,Level 2,Healthcare,Dollars (millions),H01,Total income,Financial performance,"757,504"
2021,Level 3,All industries,Dollars (millions),H04,"Sales, government funding, grants and subsidies"
2021,Level 1,All industries,Dollars,H05,"Interest, dividends and donations",Financial performance,"49,593"
2021,Level 4,All industries,Yean,H07,Non-operating income,Financial performance,"33,020"
2021,Level 1,All industries,Rubble,H08,Total expenditure,Financial performance,"654,404"
2021,Level 1,Machinary,Dollars,H09,Interest and donations,Financial performance,"26,138"
"#;

#[test]
fn test_iter() {
    let table = csv_to_table::iter::from_reader(CSV_DATA_1.trim().as_bytes());
    let table = table.to_string();

    assert_eq!(
        table,
        concat!(
            "+------+-----------------------------+----------------------+--------------------+---------------+-----------------------------------+-----------------------+---------+\n",
            "| Year | Industry_aggregation_NZSIOC | Industry_name_NZSIOC | Units              | Variable_code | Variable_name                     | Variable_category     | Value   |\n",
            "+------+-----------------------------+----------------------+--------------------+---------------+-----------------------------------+-----------------------+---------+\n",
            "| 2021 | Level 1                     | All industries       | Dollars (millions) | H01           | Total income                      | Financial performance | 757,504 |\n",
            "+------+-----------------------------+----------------------+--------------------+---------------+-----------------------------------+-----------------------+---------+\n",
            "| 2021 | Level 1                     | All industries       | Dollars (millions) | H05           | Interest, dividends and donations | Financial performance | 49,593  |\n",
            "+------+-----------------------------+----------------------+--------------------+---------------+-----------------------------------+-----------------------+---------+\n",
            "| 2021 | Level 1                     | All industries       | Dollars (millions) | H07           | Non-operating income              | Financial performance | 33,020  |\n",
            "+------+-----------------------------+----------------------+--------------------+---------------+-----------------------------------+-----------------------+---------+\n",
            "| 2021 | Level 1                     | All industries       | Dollars (millions) | H08           | Total expenditure                 | Financial performance | 654,404 |\n",
            "+------+-----------------------------+----------------------+--------------------+---------------+-----------------------------------+-----------------------+---------+\n",
            "| 2021 | Level 1                     | All industries       | Dollars (millions) | H09           | Interest and donations            | Financial performance | 26,138  |\n",
            "+------+-----------------------------+----------------------+--------------------+---------------+-----------------------------------+-----------------------+---------+",
        )
    )
}

#[test]
fn test_iter_width() {
    let table = csv_to_table::iter::from_reader(CSV_DATA_1.trim().as_bytes()).width(2);

    let table = table.to_string();

    assert_eq!(
        table,
        concat!(
            "+----+----+----+----+----+----+----+----+\n",
            "| Ye | In | In | Un | Va | Va | Va | Va |\n",
            "+----+----+----+----+----+----+----+----+\n",
            "| 20 | Le | Al | Do | H0 | To | Fi | 75 |\n",
            "+----+----+----+----+----+----+----+----+\n",
            "| 20 | Le | Al | Do | H0 | In | Fi | 49 |\n",
            "+----+----+----+----+----+----+----+----+\n",
            "| 20 | Le | Al | Do | H0 | No | Fi | 33 |\n",
            "+----+----+----+----+----+----+----+----+\n",
            "| 20 | Le | Al | Do | H0 | To | Fi | 65 |\n",
            "+----+----+----+----+----+----+----+----+\n",
            "| 20 | Le | Al | Do | H0 | In | Fi | 26 |\n",
            "+----+----+----+----+----+----+----+----+",
        )
    )
}

#[test]
fn test_iter_width_zero() {
    let table = csv_to_table::iter::from_reader(CSV_DATA_1.trim().as_bytes()).width(0);

    let table = table.to_string();

    assert_eq!(
        table,
        concat!(
            "+--+--+--+--+--+--+--+--+\n",
            "|  |  |  |  |  |  |  |  |\n",
            "+--+--+--+--+--+--+--+--+\n",
            "|  |  |  |  |  |  |  |  |\n",
            "+--+--+--+--+--+--+--+--+\n",
            "|  |  |  |  |  |  |  |  |\n",
            "+--+--+--+--+--+--+--+--+\n",
            "|  |  |  |  |  |  |  |  |\n",
            "+--+--+--+--+--+--+--+--+\n",
            "|  |  |  |  |  |  |  |  |\n",
            "+--+--+--+--+--+--+--+--+\n",
            "|  |  |  |  |  |  |  |  |\n",
            "+--+--+--+--+--+--+--+--+",
        )
    )
}

#[test]
fn test_iter_width_and_cols() {
    let table = csv_to_table::iter::from_reader(CSV_DATA_1.trim().as_bytes())
        .width(2)
        .columns(5);

    let table = table.to_string();

    assert_eq!(
        table,
        concat!(
            "+----+----+----+----+----+\n",
            "| Ye | In | In | Un | Va |\n",
            "+----+----+----+----+----+\n",
            "| 20 | Le | Al | Do | H0 |\n",
            "+----+----+----+----+----+\n",
            "| 20 | Le | Al | Do | H0 |\n",
            "+----+----+----+----+----+\n",
            "| 20 | Le | Al | Do | H0 |\n",
            "+----+----+----+----+----+\n",
            "| 20 | Le | Al | Do | H0 |\n",
            "+----+----+----+----+----+\n",
            "| 20 | Le | Al | Do | H0 |\n",
            "+----+----+----+----+----+",
        )
    )
}

#[test]
fn test_iter_cols() {
    let table = csv_to_table::iter::from_reader(CSV_DATA_1.trim().as_bytes()).columns(5);

    let table = table.to_string();

    assert_eq!(
        table,
        concat!(
            "+------+-----------------------------+----------------------+--------------------+---------------+\n",
            "| Year | Industry_aggregation_NZSIOC | Industry_name_NZSIOC | Units              | Variable_code |\n",
            "+------+-----------------------------+----------------------+--------------------+---------------+\n",
            "| 2021 | Level 1                     | All industries       | Dollars (millions) | H01           |\n",
            "+------+-----------------------------+----------------------+--------------------+---------------+\n",
            "| 2021 | Level 1                     | All industries       | Dollars (millions) | H05           |\n",
            "+------+-----------------------------+----------------------+--------------------+---------------+\n",
            "| 2021 | Level 1                     | All industries       | Dollars (millions) | H07           |\n",
            "+------+-----------------------------+----------------------+--------------------+---------------+\n",
            "| 2021 | Level 1                     | All industries       | Dollars (millions) | H08           |\n",
            "+------+-----------------------------+----------------------+--------------------+---------------+\n",
            "| 2021 | Level 1                     | All industries       | Dollars (millions) | H09           |\n",
            "+------+-----------------------------+----------------------+--------------------+---------------+",
        )
    )
}

#[test]
fn test_iter_height() {
    let table = csv_to_table::iter::from_reader(CSV_DATA_1.trim().as_bytes()).height(2);

    let table = table.to_string();

    assert_eq!(
        table,
        concat!(
            "+------+-----------------------------+----------------------+--------------------+---------------+-----------------------------------+-----------------------+---------+\n",
            "| Year | Industry_aggregation_NZSIOC | Industry_name_NZSIOC | Units              | Variable_code | Variable_name                     | Variable_category     | Value   |\n",
            "|      |                             |                      |                    |               |                                   |                       |         |\n",
            "+------+-----------------------------+----------------------+--------------------+---------------+-----------------------------------+-----------------------+---------+\n",
            "| 2021 | Level 1                     | All industries       | Dollars (millions) | H01           | Total income                      | Financial performance | 757,504 |\n",
            "|      |                             |                      |                    |               |                                   |                       |         |\n",
            "+------+-----------------------------+----------------------+--------------------+---------------+-----------------------------------+-----------------------+---------+\n",
            "| 2021 | Level 1                     | All industries       | Dollars (millions) | H05           | Interest, dividends and donations | Financial performance | 49,593  |\n",
            "|      |                             |                      |                    |               |                                   |                       |         |\n",
            "+------+-----------------------------+----------------------+--------------------+---------------+-----------------------------------+-----------------------+---------+\n",
            "| 2021 | Level 1                     | All industries       | Dollars (millions) | H07           | Non-operating income              | Financial performance | 33,020  |\n",
            "|      |                             |                      |                    |               |                                   |                       |         |\n",
            "+------+-----------------------------+----------------------+--------------------+---------------+-----------------------------------+-----------------------+---------+\n",
            "| 2021 | Level 1                     | All industries       | Dollars (millions) | H08           | Total expenditure                 | Financial performance | 654,404 |\n",
            "|      |                             |                      |                    |               |                                   |                       |         |\n",
            "+------+-----------------------------+----------------------+--------------------+---------------+-----------------------------------+-----------------------+---------+\n",
            "| 2021 | Level 1                     | All industries       | Dollars (millions) | H09           | Interest and donations            | Financial performance | 26,138  |\n",
            "|      |                             |                      |                    |               |                                   |                       |         |\n",
            "+------+-----------------------------+----------------------+--------------------+---------------+-----------------------------------+-----------------------+---------+",
        )
    )
}

#[test]
fn test_iter_rows() {
    let table = csv_to_table::iter::from_reader(CSV_DATA_1.trim().as_bytes()).rows(2);

    let table = table.to_string();

    assert_eq!(
        table,
        concat!(
            "+------+-----------------------------+----------------------+--------------------+---------------+-----------------------------------+-----------------------+---------+\n",
            "| Year | Industry_aggregation_NZSIOC | Industry_name_NZSIOC | Units              | Variable_code | Variable_name                     | Variable_category     | Value   |\n",
            "+------+-----------------------------+----------------------+--------------------+---------------+-----------------------------------+-----------------------+---------+\n",
            "| 2021 | Level 1                     | All industries       | Dollars (millions) | H01           | Total income                      | Financial performance | 757,504 |\n",
            "+------+-----------------------------+----------------------+--------------------+---------------+-----------------------------------+-----------------------+---------+",
        )
    )
}

#[test]
fn test_iter_rows_cols() {
    let table = csv_to_table::iter::from_reader(CSV_DATA_1.trim().as_bytes())
        .rows(2)
        .columns(2);

    let table = table.to_string();

    assert_eq!(
        table,
        concat!(
            "+------+-----------------------------+\n",
            "| Year | Industry_aggregation_NZSIOC |\n",
            "+------+-----------------------------+\n",
            "| 2021 | Level 1                     |\n",
            "+------+-----------------------------+",
        )
    )
}

#[test]
fn test_iter_rows_cols_zero() {
    let table = csv_to_table::iter::from_reader(CSV_DATA_1.trim().as_bytes())
        .rows(0)
        .columns(0);

    let table = table.to_string();

    assert_eq!(table, "")
}

#[test]
fn test_iter_rows_zero() {
    let table = csv_to_table::iter::from_reader(CSV_DATA_1.trim().as_bytes()).rows(0);

    let table = table.to_string();

    assert_eq!(table, "")
}

#[test]
fn test_iter_1() {
    let table = csv_to_table::iter::from_reader(CSV_DATA_2.trim().as_bytes());

    let table = table.to_string();

    assert_eq!(
        table,
        concat!(
            "+------+-----------------------------+----------------------+--------------------+---------------+-----------------------------------+-----------------------+---------+\n",
            "| Year | Industry_aggregation_NZSIOC | Industry_name_NZSIOC | Units              | Variable_code | Variable_name                     | Variable_category     | Value   |\n",
            "+------+-----------------------------+----------------------+--------------------+---------------+-----------------------------------+-----------------------+---------+\n",
            "| 2021 | Level 2                     | Healthcare           | Dollars (millions) | H01           | Total income                      | Financial performance | 757,504 |\n",
            "+------+-----------------------------+----------------------+--------------------+---------------+-----------------------------------+-----------------------+---------+\n",
            "| 2021 | Level 1                     | All industries       | Dollars            | H05           | Interest, dividends and donations | Financial performance | 49,593  |\n",
            "+------+-----------------------------+----------------------+--------------------+---------------+-----------------------------------+-----------------------+---------+\n",
            "| 2021 | Level 4                     | All industries       | Yean               | H07           | Non-operating income              | Financial performance | 33,020  |\n",
            "+------+-----------------------------+----------------------+--------------------+---------------+-----------------------------------+-----------------------+---------+\n",
            "| 2021 | Level 1                     | All industries       | Rubble             | H08           | Total expenditure                 | Financial performance | 654,404 |\n",
            "+------+-----------------------------+----------------------+--------------------+---------------+-----------------------------------+-----------------------+---------+\n",
            "| 2021 | Level 1                     | Machinary            | Dollars            | H09           | Interest and donations            | Financial performance | 26,138  |\n",
            "+------+-----------------------------+----------------------+--------------------+---------------+-----------------------------------+-----------------------+---------+",
        )
    );
}

#[test]
fn test_iter_sniff() {
    let table = csv_to_table::iter::from_reader(CSV_DATA_2.trim().as_bytes()).sniff(1);

    let table = table.to_string();

    assert_eq!(
        table,
        concat!(
            "+------+-----------------------------+----------------------+-------+---------------+---------------+-------------------+-------+\n",
            "| Year | Industry_aggregation_NZSIOC | Industry_name_NZSIOC | Units | Variable_code | Variable_name | Variable_category | Value |\n",
            "+------+-----------------------------+----------------------+-------+---------------+---------------+-------------------+-------+\n",
            "| 2021 | Level 2                     | Healthcare           | Dolla | H01           | Total income  | Financial perform | 757,5 |\n",
            "+------+-----------------------------+----------------------+-------+---------------+---------------+-------------------+-------+\n",
            "| 2021 | Level 1                     | All industries       | Dolla | H05           | Interest, div | Financial perform | 49,59 |\n",
            "+------+-----------------------------+----------------------+-------+---------------+---------------+-------------------+-------+\n",
            "| 2021 | Level 4                     | All industries       | Yean  | H07           | Non-operating | Financial perform | 33,02 |\n",
            "+------+-----------------------------+----------------------+-------+---------------+---------------+-------------------+-------+\n",
            "| 2021 | Level 1                     | All industries       | Rubbl | H08           | Total expendi | Financial perform | 654,4 |\n",
            "+------+-----------------------------+----------------------+-------+---------------+---------------+-------------------+-------+\n",
            "| 2021 | Level 1                     | Machinary            | Dolla | H09           | Interest and  | Financial perform | 26,13 |\n",
            "+------+-----------------------------+----------------------+-------+---------------+---------------+-------------------+-------+",
        )
    );
}

#[test]
fn test_iter_sniff_zero() {
    let table = csv_to_table::iter::from_reader(CSV_DATA_2.trim().as_bytes()).sniff(0);

    let table = table.to_string();

    assert_eq!(table, "");
}

#[test]
fn test_iter_sniff_zero_cols() {
    let table = csv_to_table::iter::from_reader(CSV_DATA_2.trim().as_bytes())
        .sniff(0)
        .columns(3);

    let table = table.to_string();

    assert_eq!(
        table,
        concat!(
            "+--+--+--+\n",
            "|  |  |  |\n",
            "+--+--+--+\n",
            "|  |  |  |\n",
            "+--+--+--+\n",
            "|  |  |  |\n",
            "+--+--+--+\n",
            "|  |  |  |\n",
            "+--+--+--+\n",
            "|  |  |  |\n",
            "+--+--+--+\n",
            "|  |  |  |\n",
            "+--+--+--+",
        )
    );
}
