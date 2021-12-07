use tabled::{Style, Table, Tabled, join};

mod util;

#[test]
fn table_join() {
    #[derive(Tabled)]
    enum Domain {
        Security,
        Embeded,
        Frontend,
        Unknown,
    }

    #[derive(Tabled)]
    struct Developer(#[header("name")] &'static str);

    let data1 = vec![
        (Developer("Terri Kshlerin"), Domain::Embeded),
        (Developer("Catalina Dicki"), Domain::Security),
        (Developer("Jennie Schmeler"), Domain::Frontend),
        (Developer("Maxim Zhiburt"), Domain::Unknown),
    ];

    let table1 = Table::new(data1).with(Style::psql());

    let data2 = vec![
        (Developer("Kshlerin"), Domain::Embeded),
        (Developer("Dicki"), Domain::Security),
        (Developer("Zhiburt"), Domain::Unknown),
        (Developer("Schmeler"), Domain::Frontend),
    ];

    let table2 = Table::new(data2).with(Style::psql());
    let table3 = table1.with(join::Join::Horizontal(table2));

    assert_eq!(
        table3.to_string(),
        concat!(
            "      name       | Security | Embeded | Frontend | Unknown \n",
            "-----------------+----------+---------+----------+---------\n",
            " Terri Kshlerin  |          |    +    |          |         \n",
            " Catalina Dicki  |    +     |         |          |         \n",
            " Jennie Schmeler |          |         |    +     |         \n",
            "  Maxim Zhiburt  |          |         |          |    +    \n",
            "       Kshlerin  |          |    +    |          |         \n",
            "          Dicki  |    +     |         |          |         \n",
            "        Zhiburt  |          |         |          |    +    \n",
            "       Schmeler  |          |         |    +     |         \n",
        )
    );
}