use tabled::assert::test_table;

test_table!(
    test_0,
    csv_to_table::from_reader(csv1()).unwrap(),
    "+------+---------+----+-------------+--------------------+-----+----------------------------------------+-----------------------+---------+---------------------+"
    "| 2021 | Level 1 | AA | Agriculture | Dollars (millions) | H10 | Indirect taxes                         | Financial performance | 489     | ANZSIC06 division A |"
    "+------+---------+----+-------------+--------------------+-----+----------------------------------------+-----------------------+---------+---------------------+"
    "| 2021 | Level 1 | AA | Agriculture | Dollars (millions) | H11 | Depreciation                           | Financial performance | 2,318   | ANZSIC06 division A |"
    "+------+---------+----+-------------+--------------------+-----+----------------------------------------+-----------------------+---------+---------------------+"
    "| 2021 | Level 1 | AA | Agriculture | Dollars (millions) | H12 | Salaries and wages paid                | Financial performance | 6,202   | ANZSIC06 division A |"
    "+------+---------+----+-------------+--------------------+-----+----------------------------------------+-----------------------+---------+---------------------+"
    "| 2021 | Level 1 | AA | Agriculture | Dollars (millions) | H13 | Redundancy and severance               | Financial performance | 1       | ANZSIC06 division A |"
    "+------+---------+----+-------------+--------------------+-----+----------------------------------------+-----------------------+---------+---------------------+"
    "| 2021 | Level 1 | AA | Agriculture | Dollars (millions) | H19 | Purchases and other operating expenses | Financial performance | 29,002  | ANZSIC06 division A |"
    "+------+---------+----+-------------+--------------------+-----+----------------------------------------+-----------------------+---------+---------------------+"
    "| 2021 | Level 1 | AA | Agriculture | Dollars (millions) | H20 | Non-operating expenses                 | Financial performance | 275     | ANZSIC06 division A |"
    "+------+---------+----+-------------+--------------------+-----+----------------------------------------+-----------------------+---------+---------------------+"
    "| 2021 | Level 1 | AA | Agriculture | Dollars (millions) | H21 | Opening stocks                         | Financial performance | 14,215  | ANZSIC06 division A |"
    "+------+---------+----+-------------+--------------------+-----+----------------------------------------+-----------------------+---------+---------------------+"
    "| 2021 | Level 1 | AA | Agriculture | Dollars (millions) | H22 | Closing stocks                         | Financial performance | 14,215  | ANZSIC06 division A |"
    "+------+---------+----+-------------+--------------------+-----+----------------------------------------+-----------------------+---------+---------------------+"
    "| 2021 | Level 1 | AA | Agriculture | Dollars (millions) | H23 | Surplus before income tax              | Financial performance | 8,034   | ANZSIC06 division A |"
    "+------+---------+----+-------------+--------------------+-----+----------------------------------------+-----------------------+---------+---------------------+"
    "| 2021 | Level 1 | AA | Agriculture | Dollars (millions) | H24 | Total assets                           | Financial position    | 190,239 | ANZSIC06 division A |"
    "+------+---------+----+-------------+--------------------+-----+----------------------------------------+-----------------------+---------+---------------------+"
    "| 2021 | Level 1 | AA | Agriculture | Dollars (millions) | H25 | Current assets                         | Financial position    | 35,038  | ANZSIC06 division A |"
    "+------+---------+----+-------------+--------------------+-----+----------------------------------------+-----------------------+---------+---------------------+"
    "| 2021 | Level 1 | AA | Agriculture | Dollars (millions) | H26 | Fixed tangible assets                  | Financial position    | 108,875 | ANZSIC06 division A |"
    "+------+---------+----+-------------+--------------------+-----+----------------------------------------+-----------------------+---------+---------------------+"
    "| 2021 | Level 1 | AA | Agriculture | Dollars (millions) | H29 | Other assets                           | Financial position    | 46,325  | ANZSIC06 division A |"
    "+------+---------+----+-------------+--------------------+-----+----------------------------------------+-----------------------+---------+---------------------+"
    "| 2021 | Level 1 | AA | Agriculture | Dollars (millions) | H30 | Total equity and liabilities           | Financial position    | 190,239 | ANZSIC06 division A |"
    "+------+---------+----+-------------+--------------------+-----+----------------------------------------+-----------------------+---------+---------------------+"
    "| 2021 | Level 1 | AA | Agriculture | Dollars (millions) | H31 | Shareholders funds or owners equity    | Financial position    | 91,748  | ANZSIC06 division A |"
    "+------+---------+----+-------------+--------------------+-----+----------------------------------------+-----------------------+---------+---------------------+"
    "| 2021 | Level 1 | AA | Agriculture | Dollars (millions) | H32 | Current liabilities                    | Financial position    | 33,218  | ANZSIC06 division A |"
    "+------+---------+----+-------------+--------------------+-----+----------------------------------------+-----------------------+---------+---------------------+"
    "| 2021 | Level 1 | AA | Agriculture | Dollars (millions) | H33 | Other liabilities                      | Financial position    | 65,274  | ANZSIC06 division A |"
    "+------+---------+----+-------------+--------------------+-----+----------------------------------------+-----------------------+---------+---------------------+"
    "| 2021 | Level 1 | AA | Agriculture | Dollars            | H34 | Total income per employee count        | Financial ratios      | 419,000 | ANZSIC06 division A |"
    "+------+---------+----+-------------+--------------------+-----+----------------------------------------+-----------------------+---------+---------------------+"
    "| 2021 | Level 1 | AA | Agriculture | Dollars            | H35 | Surplus per employee count             | Financial ratios      | 69,100  | ANZSIC06 division A |"
    "+------+---------+----+-------------+--------------------+-----+----------------------------------------+-----------------------+---------+---------------------+"
    "| 2021 | Level 1 | AA | Agriculture | Percentage         | H36 | Current ratio                          | Financial ratios      | 105     | ANZSIC06 division A |"
    "+------+---------+----+-------------+--------------------+-----+----------------------------------------+-----------------------+---------+---------------------+"
);

fn csv1() -> &'static [u8] {
    static DATA: &'_ str = r#"
2021,Level 1,AA,"Agriculture",Dollars (millions),H10,Indirect taxes,Financial performance,489,ANZSIC06 division A
2021,Level 1,AA,"Agriculture",Dollars (millions),H11,Depreciation,Financial performance,"2,318",ANZSIC06 division A
2021,Level 1,AA,"Agriculture",Dollars (millions),H12,Salaries and wages paid,Financial performance,"6,202",ANZSIC06 division A
2021,Level 1,AA,"Agriculture",Dollars (millions),H13,Redundancy and severance,Financial performance,1,ANZSIC06 division A
2021,Level 1,AA,"Agriculture",Dollars (millions),H19,Purchases and other operating expenses,Financial performance,"29,002",ANZSIC06 division A
2021,Level 1,AA,"Agriculture",Dollars (millions),H20,Non-operating expenses,Financial performance,275,ANZSIC06 division A
2021,Level 1,AA,"Agriculture",Dollars (millions),H21,Opening stocks,Financial performance,"14,215",ANZSIC06 division A
2021,Level 1,AA,"Agriculture",Dollars (millions),H22,Closing stocks,Financial performance,"14,215",ANZSIC06 division A
2021,Level 1,AA,"Agriculture",Dollars (millions),H23,Surplus before income tax,Financial performance,"8,034",ANZSIC06 division A
2021,Level 1,AA,"Agriculture",Dollars (millions),H24,Total assets,Financial position,"190,239",ANZSIC06 division A
2021,Level 1,AA,"Agriculture",Dollars (millions),H25,Current assets,Financial position,"35,038",ANZSIC06 division A
2021,Level 1,AA,"Agriculture",Dollars (millions),H26,Fixed tangible assets,Financial position,"108,875",ANZSIC06 division A
2021,Level 1,AA,"Agriculture",Dollars (millions),H29,Other assets,Financial position,"46,325",ANZSIC06 division A
2021,Level 1,AA,"Agriculture",Dollars (millions),H30,Total equity and liabilities,Financial position,"190,239",ANZSIC06 division A
2021,Level 1,AA,"Agriculture",Dollars (millions),H31,Shareholders funds or owners equity,Financial position,"91,748",ANZSIC06 division A
2021,Level 1,AA,"Agriculture",Dollars (millions),H32,Current liabilities,Financial position,"33,218",ANZSIC06 division A
2021,Level 1,AA,"Agriculture",Dollars (millions),H33,Other liabilities,Financial position,"65,274",ANZSIC06 division A
2021,Level 1,AA,"Agriculture",Dollars,H34,Total income per employee count,Financial ratios,"419,000",ANZSIC06 division A
2021,Level 1,AA,"Agriculture",Dollars,H35,Surplus per employee count,Financial ratios,"69,100",ANZSIC06 division A
2021,Level 1,AA,"Agriculture",Percentage,H36,Current ratio,Financial ratios,105,ANZSIC06 division A
"#;

    DATA.trim().as_bytes()
}
