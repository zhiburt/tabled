//! This example demonstrates usage of [`ron_to_table::to_string`].

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

    println!("{}", ron_to_table::to_string(&scene));
}
