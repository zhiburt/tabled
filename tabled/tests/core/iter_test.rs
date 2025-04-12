#![cfg(all(feature = "std", feature = "derive", feature = "assert"))]

use tabled::{
    assert::test_table,
    iter::LayoutIterator,
    settings::{style::BorderCorrection, Span, Style},
    Table, Tabled,
};

test_table!(
    push_record,
    {
        #[derive(Tabled)]
        struct Company<'a> {
            name: &'a str,
            street: &'a str,
            city: &'a str,
            zip_code: &'a str,
        }


        let companies = vec![
            Company { name: "INTEL CORP", city: "SANTA CLARA", street: "2200 MISSION COLLEGE BLVD, RNB-4-151", zip_code: "95054" },
            Company { name: "Apple Inc.", city: "CUPERTINO", street: "ONE APPLE PARK WAY", zip_code: "95014" },
        ];

        let mut table = Table::kv(&companies);

        for row in LayoutIterator::kv_batches::<Company>(&table) {
            table.modify((row, 1), Span::column(-1));
        }

        table.with(Style::modern());
        table.with(BorderCorrection);

        table
    },
    "┌─────────────────────────────────────────────────┐"
    "│ INTEL CORP                                      │"
    "├──────────┬──────────────────────────────────────┤"
    "│ street   │ 2200 MISSION COLLEGE BLVD, RNB-4-151 │"
    "├──────────┼──────────────────────────────────────┤"
    "│ city     │ SANTA CLARA                          │"
    "├──────────┼──────────────────────────────────────┤"
    "│ zip_code │ 95054                                │"
    "├──────────┴──────────────────────────────────────┤"
    "│ Apple Inc.                                      │"
    "├──────────┬──────────────────────────────────────┤"
    "│ street   │ ONE APPLE PARK WAY                   │"
    "├──────────┼──────────────────────────────────────┤"
    "│ city     │ CUPERTINO                            │"
    "├──────────┼──────────────────────────────────────┤"
    "│ zip_code │ 95014                                │"
    "└──────────┴──────────────────────────────────────┘"
);
