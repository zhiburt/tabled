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

pub mod tabled_current {
    use tabled_current::builder::Builder;

    #[inline]
    pub fn build(columns: Vec<String>, data: Vec<Vec<String>>) -> String {
        let mut b = Builder::with_capacity(data.len() + 1, columns.len());
        b.push_record(columns);

        for row in data {
            b.push_record(row);
        }

        b.build().to_string()
    }
}

pub mod tabled_color_current {
    use tabled_color_current::builder::Builder;

    #[inline]
    pub fn build(columns: Vec<String>, data: Vec<Vec<String>>) -> String {
        let mut b = Builder::with_capacity(data.len() + 1, columns.len());
        b.push_record(columns);

        for row in data {
            b.push_record(row);
        }

        b.build().to_string()
    }
}

pub mod tabled_current_iter {
    use tabled_current::tables::IterTable;

    #[inline]
    pub fn build(columns: Vec<String>, data: Vec<Vec<String>>) -> String {
        let count_columns = columns.len();

        let mut data = data;
        data.insert(0, columns);

        IterTable::new(data).columns(count_columns).to_string()
    }
}

pub mod tabled_current_compact {
    use tabled_current::grid::config::{CompactConfig, Indent, Sides};
    use tabled_current::grid::dimension::CompactGridDimension;
    use tabled_current::grid::dimension::Estimate;
    use tabled_current::grid::records::IterRecords;
    use tabled_current::tables::CompactTable;

    #[inline]
    pub fn build(columns: Vec<String>, data: Vec<Vec<String>>) -> String {
        let count_columns = columns.len();

        let mut data = data;
        data.insert(0, columns);

        let mut dims = CompactGridDimension::default();
        let cfg = CompactConfig::default().set_padding(Sides::new(
            Indent::spaced(1),
            Indent::spaced(1),
            Indent::zero(),
            Indent::zero(),
        ));
        let records = IterRecords::new(&data, count_columns, Some(data.len()));
        dims.estimate(records, &cfg);

        CompactTable::with_dimension(data, dims)
            .columns(count_columns)
            .to_string()
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
