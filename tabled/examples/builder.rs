use tabled::{builder::Builder, settings::Style};

fn main() {
    let oceans = "Atlantic, Pacific, Indian, Southern, Arctic";

    let mut builder = Builder::default();

    for (i, ocean) in oceans.split(", ").enumerate() {
        builder.push_record([i.to_string(), ocean.to_string()]);
    }

    builder.insert_record(0, ["#", "Ocean"]);

    let mut table = builder.build();
    table.with(Style::markdown().remove_horizontals());

    println!("{table}");
}
