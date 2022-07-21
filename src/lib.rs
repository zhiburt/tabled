pub mod tabled {
    use tabled::{builder::Builder, object::Segment, Alignment, Modify, Table};

    #[inline]
    pub fn build(columns: Vec<String>, data: Vec<Vec<String>>) -> Table {
        let mut b = Builder::from(data);
        b.set_columns(columns);
        b.build()
            .with(Modify::new(Segment::all()).with(Alignment::left()))
    }

    #[inline]
    pub fn print(table: &Table) -> String {
        table.to_string()
    }
}

pub mod tabled_color {
    use tabled_color::{builder::Builder, object::Segment, Alignment, Modify, Table};

    pub fn build(columns: Vec<String>, data: Vec<Vec<String>>) -> Table {
        let mut b = Builder::from(data);
        b.set_columns(columns);
        b.build()
            .with(Modify::new(Segment::all()).with(Alignment::left()))
    }

    #[inline]
    pub fn print(table: &Table) -> String {
        table.to_string()
    }
}

pub mod nu_table {
    use std::collections::HashMap;

    use nu_ansi_term::Style;
    use nu_protocol::Config;
    use nu_table::{draw_table, StyledString, Table, TableTheme, TextStyle};

    pub type NuTableType = (Table, HashMap<String, Style>, Config);

    #[inline]
    pub fn build(columns: Vec<String>, data: Vec<Vec<String>>) -> NuTableType {
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
        (table, HashMap::new(), Config::default())
    }

    #[inline]
    pub fn print((table, color_hm, config): &NuTableType) -> String {
        draw_table(table, 1000000000, color_hm, config)
    }
}

pub mod cli_table {
    use cli_table::{Table, TableStruct};

    #[inline]
    pub fn build(columns: Vec<String>, mut data: Vec<Vec<String>>) -> TableStruct {
        data.insert(0, columns);
        <Vec<Vec<String>> as Table>::table(data)
    }

    #[inline]
    pub fn print(table: &TableStruct) -> String {
        // here's a conversion and Vec<u8> cache which is something need to be aware of.
        table.display().unwrap().to_string()
    }
}

pub mod comfy_table {
    use comfy_table::{Row, Table};

    #[inline]
    pub fn build(columns: Vec<String>, mut data: Vec<Vec<String>>) -> Table {
        data.insert(0, columns);

        let mut t = Table::new();

        for row in data {
            t.add_row(Row::from(row));
        }

        t
    }

    #[inline]
    pub fn print(table: &Table) -> String {
        table.to_string()
    }
}

pub mod term_table {
    use term_table::{row::Row, Table};

    #[inline]
    pub fn build(columns: Vec<String>, mut data: Vec<Vec<String>>) -> Table<'static> {
        data.insert(0, columns);

        let mut t = Table::new();
        for row in data {
            t.add_row(Row::new(row));
        }

        t
    }

    #[inline]
    pub fn print(table: &Table<'static>) -> String {
        table.render()
    }
}

pub mod prettytable_rs {
    use prettytable::{Row, Table};

    #[inline]
    pub fn build(columns: Vec<String>, mut data: Vec<Vec<String>>) -> Table {
        data.insert(0, columns);

        let mut t = Table::new();

        for row in data {
            t.add_row(Row::from(row));
        }

        t
    }

    #[inline]
    pub fn print(table: &Table) -> String {
        table.to_string()
    }
}
