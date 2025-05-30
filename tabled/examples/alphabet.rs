use tabled::{
    grid::{
        config::ColoredConfig,
        records::vec_records::{Text, VecRecords},
    },
    settings::{Color, TableOption},
    Table,
};

struct LookupMARKUP {
    pattern: String,
}

impl<D> TableOption<VecRecords<Text<String>>, ColoredConfig, D> for LookupMARKUP {
    fn change(self, records: &mut VecRecords<Text<String>>, cfg: &mut ColoredConfig, _: &mut D) {
        for (row, cells) in records.iter().enumerate() {
            for (col, text) in cells.iter().enumerate() {
                if text.as_ref().contains(&self.pattern) {
                    cfg.set_color((row, col).into(), Color::BG_BLUE.into());
                }
            }
        }
    }
}

fn main() {
    let data = vec![
        ("number", "name"),
        ("285-324-7322", "Rosalia"),
        ("564.549.6468", "Mary"),
    ];

    let mut table = Table::builder(data).build();
    table.with(LookupMARKUP {
        pattern: String::from("7322"),
    });

    println!("{table}");
}
