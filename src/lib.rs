pub mod tabled {
    #[inline]
    pub fn build(columns: Vec<String>, data: Vec<Vec<String>>) -> tabled::Table {
        tabled::builder::Builder::from(data)
            .set_columns(columns)
            .build()
    }

    #[inline]
    pub fn print(table: &tabled::Table) -> String {
        table.to_string()
    }
}

pub mod tabled_color {
    pub fn build(columns: Vec<String>, data: Vec<Vec<String>>) -> tabled_color::Table {
        tabled_color::builder::Builder::from(data)
            .set_columns(columns)
            .build()
    }

    #[inline]
    pub fn print(table: &tabled_color::Table) -> String {
        table.to_string()
    }
}

pub mod nu_table {
    use std::collections::HashMap;

    pub type NuTableType = (
        nu_table::Table,
        HashMap<String, nu_ansi_term::Style>,
        nu_protocol::Config,
    );

    #[inline]
    pub fn build(columns: Vec<String>, data: Vec<Vec<String>>) -> NuTableType {
        let columns = columns
            .into_iter()
            .map(|c| nu_table::StyledString::new(c, nu_table::TextStyle::default()))
            .collect();

        let data = data
            .into_iter()
            .map(|row| {
                row.into_iter()
                    .map(|c| nu_table::StyledString::new(c, nu_table::TextStyle::default()))
                    .collect()
            })
            .collect();

        let table = nu_table::Table::new(columns, data, nu_table::TableTheme::basic());
        (table, HashMap::new(), nu_protocol::Config::default())
    }

    #[inline]
    pub fn print((table, color_hm, config): &NuTableType) -> String {
        nu_table::draw_table(table, 1000000000, color_hm, config)
    }
}

pub mod cli_table {
    #[inline]
    pub fn build(columns: Vec<String>, mut data: Vec<Vec<String>>) -> cli_table::TableStruct {
        data.insert(0, columns);
        <Vec<Vec<String>> as cli_table::Table>::table(data)
    }

    #[inline]
    pub fn print(table: &cli_table::TableStruct) -> String {
        // here's a conversion and Vec<u8> cache which is something need to be aware of.
        table.display().unwrap().to_string()
    }
}

pub mod comfy_table {
    #[inline]
    pub fn build(columns: Vec<String>, mut data: Vec<Vec<String>>) -> comfy_table::Table {
        data.insert(0, columns);

        let mut t = comfy_table::Table::new();

        for row in data {
            t.add_row(comfy_table::Row::from(row));
        }

        t
    }

    #[inline]
    pub fn print(table: &comfy_table::Table) -> String {
        table.to_string()
    }
}

pub mod term_table {
    #[inline]
    pub fn build(columns: Vec<String>, mut data: Vec<Vec<String>>) -> term_table::Table<'static> {
        data.insert(0, columns);

        let mut t = term_table::Table::new();

        for row in data {
            t.add_row(term_table::row::Row::new(row));
        }

        t
    }

    #[inline]
    pub fn print(table: &term_table::Table<'static>) -> String {
        table.render()
    }
}

pub mod prettytable_rs {
    #[inline]
    pub fn build(columns: Vec<String>, mut data: Vec<Vec<String>>) -> prettytable::Table {
        data.insert(0, columns);

        let mut t = prettytable::Table::new();

        for row in data {
            t.add_row(prettytable::Row::from(row));
        }

        t
    }

    #[inline]
    pub fn print(table: &prettytable::Table) -> String {
        table.to_string()
    }
}
