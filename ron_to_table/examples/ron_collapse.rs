//! This example demonstrates usage of [`ron_to_table::RonTable`].

use ron_to_table::RonTable;
use tabled::settings::Style;

fn main() {
    let scene = ron::from_str(
        r#"
        Scene(
            materials: {
                "metal": (reflectivity: 1.0),
                "plastic": (reflectivity: 0.5),
            },
            entities: [
                (name: "hero", material: "metal"),
                (name: "monster", material: "plastic"),
            ],
        )
    "#,
    )
    .unwrap();

    let table = RonTable::default()
        .collapse()
        .with(Style::extended())
        .build(&scene);

    println!("{table}");
}
