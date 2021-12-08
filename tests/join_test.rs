use tabled::{Style, Table, Tabled, join};

mod util;

#[test]
fn table_join_vertical_top() {
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
        (Developer("Taro Yamada"), Domain::Embeded),
        (Developer("Hanako Sato"), Domain::Security),
        (Developer("Ken Nakamura"), Domain::Unknown),
        (Developer("Hiro Takahashi"), Domain::Frontend),
    ];

    let table2 = Table::new(data2).with(Style::psql());
    let table3 = table1.with(join::Join::VerticalTop(table2));

    assert_eq!(
        table3.to_string(),
        concat!(
            "      name       | Security | Embeded | Frontend | Unknown \n",
            "-----------------+----------+---------+----------+---------\n",
            "   Taro Yamada   |          |    +    |          |         \n",
            "   Hanako Sato   |    +     |         |          |         \n",
            "  Ken Nakamura   |          |         |          |    +    \n",
            " Hiro Takahashi  |          |         |    +     |         \n",
            "      name       | Security | Embeded | Frontend | Unknown \n",
            "-----------------+----------+---------+----------+---------\n",
            " Terri Kshlerin  |          |    +    |          |         \n",
            " Catalina Dicki  |    +     |         |          |         \n",
            " Jennie Schmeler |          |         |    +     |         \n",
            "  Maxim Zhiburt  |          |         |          |    +    \n",
        )
    );
}

#[test]
fn table_join_vertical_bottom() {
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
        (Developer("Taro Yamada"), Domain::Embeded),
        (Developer("Hanako Sato"), Domain::Security),
        (Developer("Ken Nakamura"), Domain::Unknown),
        (Developer("Hiro Takahashi"), Domain::Frontend),
    ];

    let table2 = Table::new(data2).with(Style::psql());
    let table3 = table1.with(join::Join::VerticalBottom(table2));

    assert_eq!(
        table3.to_string(),
        concat!(
            "      name       | Security | Embeded | Frontend | Unknown \n",
            "-----------------+----------+---------+----------+---------\n",
            " Terri Kshlerin  |          |    +    |          |         \n",
            " Catalina Dicki  |    +     |         |          |         \n",
            " Jennie Schmeler |          |         |    +     |         \n",
            "  Maxim Zhiburt  |          |         |          |    +    \n",
            "      name       | Security | Embeded | Frontend | Unknown \n",
            "-----------------+----------+---------+----------+---------\n",
            "   Taro Yamada   |          |    +    |          |         \n",
            "   Hanako Sato   |    +     |         |          |         \n",
            "  Ken Nakamura   |          |         |          |    +    \n",
            " Hiro Takahashi  |          |         |    +     |         \n",
        )
    );
}

#[test]
fn table_join_horizontal_left() {
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
        (Developer("Taro Yamada"), Domain::Embeded),
        (Developer("Hanako Sato"), Domain::Security),
        (Developer("Ken Nakamura"), Domain::Unknown),
        (Developer("Hiro Takahashi"), Domain::Frontend),
    ];

    let table2 = Table::new(data2).with(Style::psql());
    let table3 = table1.with(join::Join::HorizontalLeft(table2));

    assert_eq!(
        table3.to_string(),
        concat!(
            "      name      | Security | Embeded | Frontend | Unknown       name       | Security | Embeded | Frontend | Unknown \n",
            "----------------+----------+---------+----------+--------------------------+----------+---------+----------+---------\n",
            "  Taro Yamada   |          |    +    |          |          Terri Kshlerin  |          |    +    |          |         \n",
            "  Hanako Sato   |    +     |         |          |          Catalina Dicki  |    +     |         |          |         \n",
            "  Ken Nakamura  |          |         |          |    +     Jennie Schmeler |          |         |    +     |         \n",
            " Hiro Takahashi |          |         |    +     |           Maxim Zhiburt  |          |         |          |    +    \n",
        )
    );
}


#[test]
fn table_join_horizontal_right() {
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
        (Developer("Taro Yamada"), Domain::Embeded),
        (Developer("Hanako Sato"), Domain::Security),
        (Developer("Ken Nakamura"), Domain::Unknown),
        (Developer("Hiro Takahashi"), Domain::Frontend),
    ];

    let table2 = Table::new(data2).with(Style::psql());
    let table3 = table1.with(join::Join::HorizontalRight(table2));

    assert_eq!(
        table3.to_string(),
        concat!(
            "      name       | Security | Embeded | Frontend | Unknown       name      | Security | Embeded | Frontend | Unknown \n",
            "-----------------+----------+---------+----------+-------------------------+----------+---------+----------+---------\n",
            " Terri Kshlerin  |          |    +    |          |           Taro Yamada   |          |    +    |          |         \n",
            " Catalina Dicki  |    +     |         |          |           Hanako Sato   |    +     |         |          |         \n",
            " Jennie Schmeler |          |         |    +     |           Ken Nakamura  |          |         |          |    +    \n",
            "  Maxim Zhiburt  |          |         |          |    +     Hiro Takahashi |          |         |    +     |         \n",
        )
    );
}