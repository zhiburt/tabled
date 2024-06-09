use testing_table::test_table;

test_table!(
    test_iter,
    csv_to_table::iter::from_reader(csv1()),
    "+------+-----------------------------+----------------------+--------------------+---------------+-----------------------------------+-----------------------+---------+"
    "| Year | Industry_aggregation_NZSIOC | Industry_name_NZSIOC | Units              | Variable_code | Variable_name                     | Variable_category     | Value   |"
    "+------+-----------------------------+----------------------+--------------------+---------------+-----------------------------------+-----------------------+---------+"
    "| 2021 | Level 1                     | All industries       | Dollars (millions) | H01           | Total income                      | Financial performance | 757,504 |"
    "+------+-----------------------------+----------------------+--------------------+---------------+-----------------------------------+-----------------------+---------+"
    "| 2021 | Level 1                     | All industries       | Dollars (millions) | H05           | Interest, dividends and donations | Financial performance | 49,593  |"
    "+------+-----------------------------+----------------------+--------------------+---------------+-----------------------------------+-----------------------+---------+"
    "| 2021 | Level 1                     | All industries       | Dollars (millions) | H07           | Non-operating income              | Financial performance | 33,020  |"
    "+------+-----------------------------+----------------------+--------------------+---------------+-----------------------------------+-----------------------+---------+"
    "| 2021 | Level 1                     | All industries       | Dollars (millions) | H08           | Total expenditure                 | Financial performance | 654,404 |"
    "+------+-----------------------------+----------------------+--------------------+---------------+-----------------------------------+-----------------------+---------+"
    "| 2021 | Level 1                     | All industries       | Dollars (millions) | H09           | Interest and donations            | Financial performance | 26,138  |"
    "+------+-----------------------------+----------------------+--------------------+---------------+-----------------------------------+-----------------------+---------+"
);

test_table!(
    test_iter_width,
    csv_to_table::iter::from_reader(csv1()).width(2),
    "+----+----+----+----+----+----+----+----+"
    "| Ye | In | In | Un | Va | Va | Va | Va |"
    "+----+----+----+----+----+----+----+----+"
    "| 20 | Le | Al | Do | H0 | To | Fi | 75 |"
    "+----+----+----+----+----+----+----+----+"
    "| 20 | Le | Al | Do | H0 | In | Fi | 49 |"
    "+----+----+----+----+----+----+----+----+"
    "| 20 | Le | Al | Do | H0 | No | Fi | 33 |"
    "+----+----+----+----+----+----+----+----+"
    "| 20 | Le | Al | Do | H0 | To | Fi | 65 |"
    "+----+----+----+----+----+----+----+----+"
    "| 20 | Le | Al | Do | H0 | In | Fi | 26 |"
    "+----+----+----+----+----+----+----+----+"
);

test_table!(
    test_iter_width_zero,
    csv_to_table::iter::from_reader(csv1()).width(0),
    "+--+--+--+--+--+--+--+--+"
    "|  |  |  |  |  |  |  |  |"
    "+--+--+--+--+--+--+--+--+"
    "|  |  |  |  |  |  |  |  |"
    "+--+--+--+--+--+--+--+--+"
    "|  |  |  |  |  |  |  |  |"
    "+--+--+--+--+--+--+--+--+"
    "|  |  |  |  |  |  |  |  |"
    "+--+--+--+--+--+--+--+--+"
    "|  |  |  |  |  |  |  |  |"
    "+--+--+--+--+--+--+--+--+"
    "|  |  |  |  |  |  |  |  |"
    "+--+--+--+--+--+--+--+--+"
);

test_table!(
    test_iter_width_and_cols,
    csv_to_table::iter::from_reader(csv1()).width(2).columns(5),
    "+----+----+----+----+----+"
    "| Ye | In | In | Un | Va |"
    "+----+----+----+----+----+"
    "| 20 | Le | Al | Do | H0 |"
    "+----+----+----+----+----+"
    "| 20 | Le | Al | Do | H0 |"
    "+----+----+----+----+----+"
    "| 20 | Le | Al | Do | H0 |"
    "+----+----+----+----+----+"
    "| 20 | Le | Al | Do | H0 |"
    "+----+----+----+----+----+"
    "| 20 | Le | Al | Do | H0 |"
    "+----+----+----+----+----+"
);

test_table!(
    test_iter_cols,
    csv_to_table::iter::from_reader(csv1()).columns(5),
    "+------+-----------------------------+----------------------+--------------------+---------------+"
    "| Year | Industry_aggregation_NZSIOC | Industry_name_NZSIOC | Units              | Variable_code |"
    "+------+-----------------------------+----------------------+--------------------+---------------+"
    "| 2021 | Level 1                     | All industries       | Dollars (millions) | H01           |"
    "+------+-----------------------------+----------------------+--------------------+---------------+"
    "| 2021 | Level 1                     | All industries       | Dollars (millions) | H05           |"
    "+------+-----------------------------+----------------------+--------------------+---------------+"
    "| 2021 | Level 1                     | All industries       | Dollars (millions) | H07           |"
    "+------+-----------------------------+----------------------+--------------------+---------------+"
    "| 2021 | Level 1                     | All industries       | Dollars (millions) | H08           |"
    "+------+-----------------------------+----------------------+--------------------+---------------+"
    "| 2021 | Level 1                     | All industries       | Dollars (millions) | H09           |"
    "+------+-----------------------------+----------------------+--------------------+---------------+"
);

test_table!(
    test_iter_height,
    csv_to_table::iter::from_reader(csv1()).height(2),
    "+------+-----------------------------+----------------------+--------------------+---------------+-----------------------------------+-----------------------+---------+"
    "| Year | Industry_aggregation_NZSIOC | Industry_name_NZSIOC | Units              | Variable_code | Variable_name                     | Variable_category     | Value   |"
    "|      |                             |                      |                    |               |                                   |                       |         |"
    "+------+-----------------------------+----------------------+--------------------+---------------+-----------------------------------+-----------------------+---------+"
    "| 2021 | Level 1                     | All industries       | Dollars (millions) | H01           | Total income                      | Financial performance | 757,504 |"
    "|      |                             |                      |                    |               |                                   |                       |         |"
    "+------+-----------------------------+----------------------+--------------------+---------------+-----------------------------------+-----------------------+---------+"
    "| 2021 | Level 1                     | All industries       | Dollars (millions) | H05           | Interest, dividends and donations | Financial performance | 49,593  |"
    "|      |                             |                      |                    |               |                                   |                       |         |"
    "+------+-----------------------------+----------------------+--------------------+---------------+-----------------------------------+-----------------------+---------+"
    "| 2021 | Level 1                     | All industries       | Dollars (millions) | H07           | Non-operating income              | Financial performance | 33,020  |"
    "|      |                             |                      |                    |               |                                   |                       |         |"
    "+------+-----------------------------+----------------------+--------------------+---------------+-----------------------------------+-----------------------+---------+"
    "| 2021 | Level 1                     | All industries       | Dollars (millions) | H08           | Total expenditure                 | Financial performance | 654,404 |"
    "|      |                             |                      |                    |               |                                   |                       |         |"
    "+------+-----------------------------+----------------------+--------------------+---------------+-----------------------------------+-----------------------+---------+"
    "| 2021 | Level 1                     | All industries       | Dollars (millions) | H09           | Interest and donations            | Financial performance | 26,138  |"
    "|      |                             |                      |                    |               |                                   |                       |         |"
    "+------+-----------------------------+----------------------+--------------------+---------------+-----------------------------------+-----------------------+---------+"
);

test_table!(
    test_iter_rows,
    csv_to_table::iter::from_reader(csv1()).rows(2),
    "+------+-----------------------------+----------------------+--------------------+---------------+-----------------------------------+-----------------------+---------+"
    "| Year | Industry_aggregation_NZSIOC | Industry_name_NZSIOC | Units              | Variable_code | Variable_name                     | Variable_category     | Value   |"
    "+------+-----------------------------+----------------------+--------------------+---------------+-----------------------------------+-----------------------+---------+"
    "| 2021 | Level 1                     | All industries       | Dollars (millions) | H01           | Total income                      | Financial performance | 757,504 |"
    "+------+-----------------------------+----------------------+--------------------+---------------+-----------------------------------+-----------------------+---------+"
);

test_table!(
    test_iter_rows_cols,
    csv_to_table::iter::from_reader(csv1()).rows(2).columns(2),
    "+------+-----------------------------+"
    "| Year | Industry_aggregation_NZSIOC |"
    "+------+-----------------------------+"
    "| 2021 | Level 1                     |"
    "+------+-----------------------------+"
);

test_table!(
    test_iter_rows_cols_zero,
    csv_to_table::iter::from_reader(csv1()).rows(0).columns(0),
    ""
);

test_table!(
    test_iter_rows_zero,
    csv_to_table::iter::from_reader(csv1()).rows(0),
    ""
);

test_table!(
    test_iter_1,
    csv_to_table::iter::from_reader(csv2()),
    "+------+-----------------------------+----------------------+--------------------+---------------+-----------------------------------+-----------------------+---------+"
    "| Year | Industry_aggregation_NZSIOC | Industry_name_NZSIOC | Units              | Variable_code | Variable_name                     | Variable_category     | Value   |"
    "+------+-----------------------------+----------------------+--------------------+---------------+-----------------------------------+-----------------------+---------+"
    "| 2021 | Level 2                     | Healthcare           | Dollars (millions) | H01           | Total income                      | Financial performance | 757,504 |"
    "+------+-----------------------------+----------------------+--------------------+---------------+-----------------------------------+-----------------------+---------+"
    "| 2021 | Level 1                     | All industries       | Dollars            | H05           | Interest, dividends and donations | Financial performance | 49,593  |"
    "+------+-----------------------------+----------------------+--------------------+---------------+-----------------------------------+-----------------------+---------+"
    "| 2021 | Level 4                     | All industries       | Yean               | H07           | Non-operating income              | Financial performance | 33,020  |"
    "+------+-----------------------------+----------------------+--------------------+---------------+-----------------------------------+-----------------------+---------+"
    "| 2021 | Level 1                     | All industries       | Rubble             | H08           | Total expenditure                 | Financial performance | 654,404 |"
    "+------+-----------------------------+----------------------+--------------------+---------------+-----------------------------------+-----------------------+---------+"
    "| 2021 | Level 1                     | Machinery            | Dollars            | H09           | Interest and donations            | Financial performance | 26,138  |"
    "+------+-----------------------------+----------------------+--------------------+---------------+-----------------------------------+-----------------------+---------+"
);

test_table!(
    test_iter_sniff,
    csv_to_table::iter::from_reader(csv2()).sniff(1),
    "+------+-----------------------------+----------------------+-------+---------------+---------------+-------------------+-------+"
    "| Year | Industry_aggregation_NZSIOC | Industry_name_NZSIOC | Units | Variable_code | Variable_name | Variable_category | Value |"
    "+------+-----------------------------+----------------------+-------+---------------+---------------+-------------------+-------+"
    "| 2021 | Level 2                     | Healthcare           | Dolla | H01           | Total income  | Financial perform | 757,5 |"
    "+------+-----------------------------+----------------------+-------+---------------+---------------+-------------------+-------+"
    "| 2021 | Level 1                     | All industries       | Dolla | H05           | Interest, div | Financial perform | 49,59 |"
    "+------+-----------------------------+----------------------+-------+---------------+---------------+-------------------+-------+"
    "| 2021 | Level 4                     | All industries       | Yean  | H07           | Non-operating | Financial perform | 33,02 |"
    "+------+-----------------------------+----------------------+-------+---------------+---------------+-------------------+-------+"
    "| 2021 | Level 1                     | All industries       | Rubbl | H08           | Total expendi | Financial perform | 654,4 |"
    "+------+-----------------------------+----------------------+-------+---------------+---------------+-------------------+-------+"
    "| 2021 | Level 1                     | Machinery            | Dolla | H09           | Interest and  | Financial perform | 26,13 |"
    "+------+-----------------------------+----------------------+-------+---------------+---------------+-------------------+-------+"
);

test_table!(
    test_iter_sniff_zero,
    csv_to_table::iter::from_reader(csv2()).sniff(0),
    ""
);

test_table!(
    test_iter_sniff_zero_cols,
    csv_to_table::iter::from_reader(csv2()).sniff(0).columns(3),
    "+--+--+--+"
    "|  |  |  |"
    "+--+--+--+"
    "|  |  |  |"
    "+--+--+--+"
    "|  |  |  |"
    "+--+--+--+"
    "|  |  |  |"
    "+--+--+--+"
    "|  |  |  |"
    "+--+--+--+"
    "|  |  |  |"
    "+--+--+--+"
);

fn csv1() -> &'static [u8] {
    static DATA: &'_ str = r#"
Year,Industry_aggregation_NZSIOC,Industry_name_NZSIOC,Units,Variable_code,Variable_name,Variable_category,Value
2021,Level 1,All industries,Dollars (millions),H01,Total income,Financial performance,"757,504"
2021,Level 1,All industries,Dollars (millions),H04,"Sales, government funding, grants and subsidies"
2021,Level 1,All industries,Dollars (millions),H05,"Interest, dividends and donations",Financial performance,"49,593"
2021,Level 1,All industries,Dollars (millions),H07,Non-operating income,Financial performance,"33,020"
2021,Level 1,All industries,Dollars (millions),H08,Total expenditure,Financial performance,"654,404"
2021,Level 1,All industries,Dollars (millions),H09,Interest and donations,Financial performance,"26,138"
"#;

    DATA.trim().as_bytes()
}

fn csv2() -> &'static [u8] {
    static DATA: &'_ str = r#"
Year,Industry_aggregation_NZSIOC,Industry_name_NZSIOC,Units,Variable_code,Variable_name,Variable_category,Value
2021,Level 2,Healthcare,Dollars (millions),H01,Total income,Financial performance,"757,504"
2021,Level 3,All industries,Dollars (millions),H04,"Sales, government funding, grants and subsidies"
2021,Level 1,All industries,Dollars,H05,"Interest, dividends and donations",Financial performance,"49,593"
2021,Level 4,All industries,Yean,H07,Non-operating income,Financial performance,"33,020"
2021,Level 1,All industries,Rubble,H08,Total expenditure,Financial performance,"654,404"
2021,Level 1,Machinery,Dollars,H09,Interest and donations,Financial performance,"26,138"
"#;

    DATA.trim().as_bytes()
}
