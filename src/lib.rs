pub mod tabled {
    use tabled::builder::Builder;

    #[inline]
    pub fn build(columns: Vec<String>, data: Vec<Vec<String>>) -> String {
        let mut b = Builder::from(data);
        b.set_columns(columns);
        let table = b.build();

        table.to_string()
    }
}

pub mod tabled_color {
    use tabled_color::builder::Builder;

    #[inline]
    pub fn build(columns: Vec<String>, data: Vec<Vec<String>>) -> String {
        let mut b = Builder::from(data);
        b.set_columns(columns);
        let table = b.build();

        table.to_string()
    }
}

pub mod tabled_par {
    use tabled_par::builder::Builder;

    #[inline]
    pub fn build(columns: Vec<String>, data: Vec<Vec<String>>) -> String {
        let mut b = Builder::from(data);
        b.set_columns(columns);
        let table = b.build();

        table.to_string()
    }
}

pub mod nu_table {
    use std::collections::HashMap;

    use nu_protocol::Config;
    use nu_table::{draw_table, StyledString, Table, TableTheme, TextStyle};

    #[inline]
    pub fn build(columns: Vec<String>, data: Vec<Vec<String>>) -> String {
        let columns = columns
            .into_iter()
            .map(|c| StyledString::new(c, TextStyle::default()))
            .collect();

        let data = data
            .into_iter()
            .map(|row| {
                row.into_iter()
                    .map(|c| StyledString::new(c, TextStyle::default()))
                    .collect()
            })
            .collect();

        let table = Table::new(columns, data, TableTheme::basic());

        draw_table(&table, 1000000000, &HashMap::new(), &Config::default())
    }
}

pub mod cli_table {
    use cli_table::Table;

    #[inline]
    pub fn build(columns: Vec<String>, mut data: Vec<Vec<String>>) -> String {
        data.insert(0, columns);
        let table = <Vec<Vec<String>> as Table>::table(data);

        // here's a conversion and Vec<u8> cache which is something need to be aware of.
        table.display().unwrap().to_string()
    }
}

pub mod comfy_table {
    use comfy_table::{Row, Table};

    #[inline]
    pub fn build(columns: Vec<String>, mut data: Vec<Vec<String>>) -> String {
        data.insert(0, columns);

        let mut t = Table::new();

        for row in data {
            t.add_row(Row::from(row));
        }

        t.to_string()
    }
}

pub mod term_table {
    use term_table::{row::Row, Table};

    #[inline]
    pub fn build(columns: Vec<String>, mut data: Vec<Vec<String>>) -> String {
        data.insert(0, columns);

        let mut t = Table::new();
        for row in data {
            t.add_row(Row::new(row));
        }

        t.render()
    }
}

pub mod prettytable_rs {
    use prettytable::{Row, Table};

    #[inline]
    pub fn build(columns: Vec<String>, mut data: Vec<Vec<String>>) -> String {
        data.insert(0, columns);

        let mut t = Table::new();

        for row in data {
            t.add_row(Row::from(row));
        }

        t.to_string()
    }
}
