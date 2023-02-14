use static_table::static_table;

#[test]
fn static_table_with_padding() {
    let table = static_table!(
        [[1, 2, 123], [1, 2, 123], [1, 2, 123]],
        MARGIN = "2, 3, 3, 3"
    );
    let expected = concat!(
        "                   \n",
        "                   \n",
        "  +---+---+-----+  \n",
        "  | 1 | 2 | 123 |  \n",
        "  +---+---+-----+  \n",
        "  | 1 | 2 | 123 |  \n",
        "  +---+---+-----+  \n",
        "  | 1 | 2 | 123 |  \n",
        "  +---+---+-----+  \n",
        "                   \n",
        "                   "
    );
    assert_eq!(table, expected);
}
