use static_table::static_table;

use tabled::assert::test_table;

test_table!(
    static_table_with_padding,
    static_table!(
        [[1, 2, 123], [1, 2, 123], [1, 2, 123]],
        PADDING = "2, 3, 4, 5"
    ),
    "+------+------+--------+"
    "|      |      |        |"
    "|      |      |        |"
    "|      |      |        |"
    "|      |      |        |"
    "|  1   |  2   |  123   |"
    "|      |      |        |"
    "|      |      |        |"
    "|      |      |        |"
    "|      |      |        |"
    "|      |      |        |"
    "+------+------+--------+"
    "|      |      |        |"
    "|      |      |        |"
    "|      |      |        |"
    "|      |      |        |"
    "|  1   |  2   |  123   |"
    "|      |      |        |"
    "|      |      |        |"
    "|      |      |        |"
    "|      |      |        |"
    "|      |      |        |"
    "+------+------+--------+"
    "|      |      |        |"
    "|      |      |        |"
    "|      |      |        |"
    "|      |      |        |"
    "|  1   |  2   |  123   |"
    "|      |      |        |"
    "|      |      |        |"
    "|      |      |        |"
    "|      |      |        |"
    "|      |      |        |"
    "+------+------+--------+"
);
