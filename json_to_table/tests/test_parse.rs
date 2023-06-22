use json_to_table::parse;
use serde_json::json;
use tabled::settings::Style;

#[test]
fn parse_json_1_test() {
    let value = json!(
        {"widget": {
            "debug": "on",
            "window": {
                "height": 500
            },
            "image": {
                "src": "Images/Sun.png",
                "alignment": "center"
            },
            "text": {
                "data": "Click Here",
            }
        }}
    );

    let table = parse(&value).with(Style::extended()).to_string();

    assert_eq!(
        table,
        concat!(
            "╔════════╦═══════════════════════════════════════════════════════════════════════════════════════════════════════════════════════════╗\n",
            "║ widget ║ {\"debug\":\"on\",\"image\":{\"alignment\":\"center\",\"src\":\"Images/Sun.png\"},\"text\":{\"data\":\"Click Here\"},\"window\":{\"height\":500}} ║\n",
            "╚════════╩═══════════════════════════════════════════════════════════════════════════════════════════════════════════════════════════╝",
        )
    );
}

#[test]
fn parse_json_2_test() {
    let value = json!(
        {
            "glossary": {
                "title": "e",
                "G": {
                    "title": "S",
                    "GlossList": {
                        "GlossEntry": {
                            "ID": "SGML",
                            "GlossDef": {
                                "GlossSeeAlso": ["GML", "XML"]
                            },
                            "GlossSee": "markup"
                        }
                    }
                }
            }
        }
    );

    let table = parse(&value).with(Style::modern()).to_string();

    assert_eq!(
        table,
        concat!(
            "┌──────────┬────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────┐\n",
            "│ glossary │ {\"G\":{\"GlossList\":{\"GlossEntry\":{\"GlossDef\":{\"GlossSeeAlso\":[\"GML\",\"XML\"]},\"GlossSee\":\"markup\",\"ID\":\"SGML\"}},\"title\":\"S\"},\"title\":\"e\"} │\n",
            "└──────────┴────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────┘",
        )
    );
}

#[test]
fn general_json_3_test() {
    let value = json!(
        {
            "header": "SVG Viewer",
            "items": [
                {"id": "Open"},
                {"id": "OpenNew", "label": "Open New"},
                null,
                {"id": "Help"},
                {"id": "About", "label": "About Adobe CVG Viewer..."}
            ]
        }
    );

    let table = parse(&value).with(Style::modern()).to_string();

    assert_eq!(
        table,
        concat!(
            "┌────────┬───────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────┐\n",
            "│ header │ SVG Viewer                                                                                                                │\n",
            "├────────┼───────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────┤\n",
            "│ items  │ [{\"id\":\"Open\"},{\"id\":\"OpenNew\",\"label\":\"Open New\"},null,{\"id\":\"Help\"},{\"id\":\"About\",\"label\":\"About Adobe CVG Viewer...\"}] │\n",
            "└────────┴───────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────┘",
        )
    );
}

#[test]
fn parse_map_empty_entity_plain_0() {
    let value = json!(
        {
            "field1" : {},
            "field2": "Value",
            "field3" : {},
            "field4": 3,
            "field5" : {}
        }
    );

    let table = parse(&value).with(Style::modern()).to_string();

    assert_eq!(
        table,
        concat!(
            "┌────────┬───────┐\n",
            "│ field1 │ {}    │\n",
            "├────────┼───────┤\n",
            "│ field2 │ Value │\n",
            "├────────┼───────┤\n",
            "│ field3 │ {}    │\n",
            "├────────┼───────┤\n",
            "│ field4 │ 3     │\n",
            "├────────┼───────┤\n",
            "│ field5 │ {}    │\n",
            "└────────┴───────┘",
        )
    );
}

#[test]
fn parse_list_empty_entity_plain_0() {
    let value = json!([{}, "field1", {}, "field2", {}]);

    let table = parse(&value).with(Style::modern()).to_string();

    assert_eq!(
        table,
        concat!(
            "┌────┬────────┬────┬────────┬────┐\n",
            "│ {} │ field1 │ {} │ field2 │ {} │\n",
            "└────┴────────┴────┴────────┴────┘",
        )
    );
}

#[test]
fn parse_list_empty_entity_plain_1() {
    let value = json!([[], "field1", [], "field2", []]);

    let table = parse(&value).with(Style::modern()).to_string();

    assert_eq!(
        table,
        concat!(
            "┌────┬────────┬────┬────────┬────┐\n",
            "│ [] │ field1 │ [] │ field2 │ [] │\n",
            "└────┴────────┴────┴────────┴────┘",
        )
    );
}

#[test]
fn parse_list_empty_entity_plain_2() {
    let value = json!([{}, {}, {}]);

    let table = parse(&value).with(Style::modern()).to_string();

    assert_eq!(
        table,
        concat!(
            "┌────┬────┬────┐\n",
            "│ {} │ {} │ {} │\n",
            "└────┴────┴────┘",
        )
    );
}
