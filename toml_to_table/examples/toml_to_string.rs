//! This example demonstrates usage of [`toml_to_table::to_string`].

fn main() {
    let data = r#"
        [materials]
        metal = { reflectivity = 1.0 }
        plastic = { reflectivity = 0.5 }
        
        [[entities]]
        name = "hero"
        material = "metal"
        
        [[entities]]
        name = "monster"
        material = "plastic"
    "#;

    let scene = toml::from_str(data).unwrap();
    let table = toml_to_table::to_string(&scene);

    println!("{table}");
}
