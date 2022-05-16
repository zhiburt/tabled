//! A example mimics to a degree https://spectreconsole.net/
//! 
//! A recording is done via https://asciinema.org/
//!
//! ```bash
//! cargo build --release
//! asciinema rec -c ../../target/release/show
//! ```
//!
//! A convertation to gif is done via https://dstein64.github.io/gifcast/

use std::{io::stdout, time::Duration};

use crossterm::{
    cursor::{self, Hide, MoveTo},
    execute,
    style::Stylize,
    terminal::{Clear, ClearType},
};
use tabled::{
    object::{Columns, Object, Rows},
    style::{Border, BorderText, Symbol},
    Alignment, Disable, Header, Highlight, Margin, MaxWidth, MinWidth, Modify, Style, Table,
    Tabled,
};

#[derive(Tabled)]
struct Movie {
    #[tabled(rename = "Release Date")]
    release_date: &'static str,
    #[tabled(rename = "Title")]
    title: &'static str,
    #[tabled(rename = "Budget")]
    budget: &'static str,
    #[tabled(rename = "Opening Weekend")]
    opening_weekend: &'static str,
    #[tabled(rename = "Box Office")]
    box_office: &'static str,
}

fn main() {
    // Credit for data: https://www.boxofficemojo.com/
    let movies = [
        Movie {
            title: "Star Wars: Episode IV — A New Hope",
            release_date: "May 25, 1977",
            budget: "$11,000,000",
            opening_weekend: "$1,554,475",
            box_office: "$775,398,007",
        },
        Movie {
            title: "Star Wars: Episode V — The Empire Strikes Back",
            release_date: "May 21, 1980",
            budget: "$18,000,000",
            opening_weekend: "$4,910,483",
            box_office: "$538,375,067",
        },
        Movie {
            title: "Star Wars: Episode VI — Return of the Jedi",
            release_date: "May 25, 1983",
            budget: "$32,500,000",
            opening_weekend: "$23,019,618",
            box_office: "$475,106,177",
        },
        Movie {
            title: "Star Wars: Episode I — The Phantom Menace",
            release_date: "May 19, 1999",
            budget: "$115,000,000",
            opening_weekend: "$64,820,970",
            box_office: "$1,027,082,707",
        },
        Movie {
            title: "Star Wars: Episode II — Attack of the Clones",
            release_date: "May 16, 2002",
            budget: "$115,000,000",
            opening_weekend: "$80,027,814",
            box_office: "$653,779,970",
        },
        Movie {
            title: "Star Wars: Episode III — Revenge of the Sith",
            release_date: "May 18, 2005",
            budget: "$113,000,000",
            opening_weekend: "$108,435,841",
            box_office: "$868,390,560",
        },
        Movie {
            title: "Star Wars: The Force Awakens",
            release_date: "December 16, 2015",
            budget: "$245,000,000",
            opening_weekend: "$247,966,675",
            box_office: "$2,069,521,700",
        },
        Movie {
            title: "Rogue One: A Star Wars Story",
            release_date: "December 14, 2016",
            budget: "$200,000,000",
            opening_weekend: "$155,081,681",
            box_office: "$1,056,057,720",
        },
        Movie {
            title: "Star Wars: The Last Jedi",
            release_date: "December 13, 2017",
            budget: "$317,000,000",
            opening_weekend: "$220,009,584",
            box_office: "$1,332,698,830",
        },
        Movie {
            title: "Solo: A Star Wars Story",
            release_date: "May 23, 2018",
            budget: "$275,000,000",
            opening_weekend: "$84,420,489",
            box_office: "$392,924,807",
        },
        Movie {
            title: "Star Wars: The Rise of Skywalker",
            release_date: "December 18, 2019",
            budget: "$275,000,000",
            opening_weekend: "$177,383,864",
            box_office: "$1,074,149,279",
        },
    ];

    run(&movies);
}

fn run(movies: &[Movie]) {
    #[rustfmt::skip]
    let create_titles_actions: Vec<Box<dyn Fn(&[Movie]) -> Table>> = vec![
        Box::new(|m| Table::new(m).with(Disable::Row(1..)).with(Disable::Column(1..)).with(Style::modern())),
        Box::new(|m| Table::new(m).with(Disable::Row(1..)).with(Disable::Column(2..)).with(Style::modern())),
        Box::new(|m| Table::new(m).with(Disable::Row(1..)).with(Disable::Column(3..)).with(Style::modern())),
        Box::new(|m| Table::new(m).with(Disable::Row(1..)).with(Disable::Column(4..)).with(Style::modern())),
        Box::new(|m| Table::new(m).with(Disable::Row(1..)).with(Disable::Column(5..)).with(Style::modern())),
    ];

    #[rustfmt::skip]
    let add_movies_actions: Vec<Box<dyn Fn(&[Movie]) -> Table>> = vec![
        Box::new(|m| Table::new(m).with(Disable::Row(2..)).with(Style::modern())),
        Box::new(|m| Table::new(m).with(Disable::Row(3..)).with(Style::modern())),
        Box::new(|m| Table::new(m).with(Disable::Row(4..)).with(Style::modern())),
        Box::new(|m| Table::new(m).with(Disable::Row(5..)).with(Style::modern())),
        Box::new(|m| Table::new(m).with(Disable::Row(6..)).with(Style::modern())),
        Box::new(|m| Table::new(m).with(Disable::Row(7..)).with(Style::modern())),
        Box::new(|m| Table::new(m).with(Disable::Row(8..)).with(Style::modern())),
        Box::new(|m| Table::new(m).with(Disable::Row(9..)).with(Style::modern())),
        Box::new(|m| Table::new(m).with(Disable::Row(10..)).with(Style::modern())),
        Box::new(|m| Table::new(m).with(Disable::Row(11..)).with(Style::modern())),
        Box::new(|m| Table::new(m).with(Disable::Row(12..)).with(Style::modern())),
    ];

    #[rustfmt::skip]
    let add_summary_actions: Vec<Box<dyn Fn(&[Movie]) -> Table>> = vec![
        Box::new(|m| Table::builder(m).add_record(["", "", "", "", ""]).build().with(Style::modern())),
        Box::new(|m| Table::builder(m).add_record(["", "", "$1,716,500,000", "", ""]).build().with(Style::modern())),
        Box::new(|m| Table::builder(m).add_record(["", "", "$1,716,500,000", "$1,190,650,976", ""]).build().with(Style::modern())),
        Box::new(|m| Table::builder(m).add_record(["", "", "$1,716,500,000", "$1,190,650,976", "$10,263,484,824"]).build().with(Style::modern())),
    ];

    let full_table: &'static dyn Fn(&[Movie]) -> Table = &|m: &[Movie]| {
        Table::builder(m)
            .add_record([
                "",
                "",
                "$1,716,500,000",
                "$1,190,650,976",
                "$10,263,484,824",
            ])
            .build()
            .with(Style::modern())
    };

    #[rustfmt::skip]
    let formatting_actions: Vec<Box<dyn Fn(&[Movie]) -> Table>> = vec![
        Box::new(|m| full_table(m).with(Modify::new(Columns::new(..1)).with(Alignment::right()))),
        Box::new(|m| full_table(m).with(Modify::new(Columns::new(..2)).with(Alignment::right()))),
        Box::new(|m| full_table(m).with(Modify::new(Columns::new(..3)).with(Alignment::right()))),
        Box::new(|m| full_table(m).with(Modify::new(Columns::new(..4)).with(Alignment::right()))),
        Box::new(|m| full_table(m).with(Modify::new(Columns::new(..5)).with(Alignment::right()))),
        Box::new(|m| full_table(m).with(Modify::new(Columns::new(..6)).with(Alignment::right()))),
        //
        Box::new(|m| full_table(m).with(Modify::new(Columns::new(..6)).with(Alignment::right())).with(Modify::new(Columns::new(..1)).with(Alignment::left()))),
        Box::new(|m| full_table(m).with(Modify::new(Columns::new(..6)).with(Alignment::right())).with(Modify::new(Columns::new(..2)).with(Alignment::left()))),
        Box::new(|m| full_table(m).with(Modify::new(Columns::new(..6)).with(Alignment::right())).with(Modify::new(Columns::new(..3)).with(Alignment::left()))),
        Box::new(|m| full_table(m).with(Modify::new(Columns::new(..6)).with(Alignment::right())).with(Modify::new(Columns::new(..4)).with(Alignment::left()))),
        Box::new(|m| full_table(m).with(Modify::new(Columns::new(..6)).with(Alignment::right())).with(Modify::new(Columns::new(..5)).with(Alignment::left()))),
        Box::new(|m| full_table(m).with(Modify::new(Columns::new(..6)).with(Alignment::right())).with(Modify::new(Columns::new(..6)).with(Alignment::left()))),

    ];

    let full_table: &'static dyn Fn(&[Movie]) -> Table = &|m: &[Movie]| {
        Table::builder(m)
            .add_record([
                "",
                "",
                "$1,716,500,000",
                "$1,190,650,976",
                "$10,263,484,824",
            ])
            .build()
            .with(Style::modern())
            .with(Modify::new(Columns::new(..6)).with(Alignment::right()))
            .with(Modify::new(Columns::new(..6)).with(Alignment::left()))
    };

    #[rustfmt::skip]
    let style_actions: Vec<Box<dyn Fn(&[Movie]) -> Table>> = vec![
        Box::new(|m| full_table(m).with(Style::extended())),
        Box::new(|m| full_table(m).with(Style::ascii())),
        Box::new(|m| full_table(m).with(Style::rounded())),
        Box::new(|m| full_table(m).with(Style::psql())),
        Box::new(|m| full_table(m).with(Style::github_markdown())),
        Box::new(|m| full_table(m).with(Style::blank())),
    ];

    let full_table: &'static dyn Fn(&[Movie]) -> Table = &|m: &[Movie]| {
        Table::builder(m)
            .add_record([
                "",
                "",
                "$1,716,500,000",
                "$1,190,650,976",
                "$10,263,484,824",
            ])
            .build()
            .with(Style::blank())
            .with(Modify::new(Columns::new(..6)).with(Alignment::right()))
            .with(Modify::new(Columns::new(..6)).with(Alignment::left()))
    };

    let line_s: &'static dyn Fn() -> Symbol = &|| Symbol::ansi("━".yellow().to_string()).unwrap();

    #[rustfmt::skip]
    let border_colors_actions: Vec<Box<dyn Fn(&[Movie]) -> Table>> = vec![
        Box::new(|m| full_table(m).with(Highlight::new(Rows::single(0), Border::default().bottom(line_s())))),
        Box::new(|m| full_table(m).with(Highlight::new(Rows::single(0), Border::default().bottom(line_s())))
                                    .with(Highlight::new(Rows::last(), Border::default().top(line_s())))),
    ];

    let full_table: &'static dyn Fn(&[Movie]) -> Table = &|m: &[Movie]| {
        let line_s: &'static dyn Fn() -> Symbol =
            &|| Symbol::ansi("━".yellow().to_string()).unwrap();
        Table::builder(m)
            .add_record([
                "",
                "",
                "$1,716,500,000",
                "$1,190,650,976",
                "$10,263,484,824",
            ])
            .build()
            .with(Style::blank())
            .with(Modify::new(Columns::new(..6)).with(Alignment::right()))
            .with(Modify::new(Columns::new(..6)).with(Alignment::left()))
            .with(Highlight::new(
                Rows::single(0),
                Border::default().bottom(line_s()),
            ))
            .with(Highlight::new(
                Rows::last(),
                Border::default().top(line_s()),
            ))
    };

    #[rustfmt::skip]
    let panel_actions: Vec<Box<dyn Fn(&[Movie]) -> Table>> = vec![
        Box::new(|m| full_table(m).with(Header("Star Wars Movies")).with(Modify::new(Rows::single(0)).with(Alignment::center()))),
        Box::new(|m| {
            let c = "━".yellow();
            let statistics_text = format!("{}{}{}", c, c, "Statistics".black().on_yellow());
            full_table(m)
                .with(Header("Star Wars Movies"))
                .with(Modify::new(Rows::single(0)).with(Alignment::center()))
                .with(BorderText::new(m.len()+2, statistics_text))
        }),
        Box::new(|m| {
            let c = "━".yellow();
            let statistics_text = format!("{}{}{}", c, c, "Statistics".black().on_yellow());
            full_table(m)
                .with(Header("Star Wars Movies"))
                .with(Modify::new(Rows::single(0)).with(Alignment::center()))
                .with(BorderText::new(m.len()+2, statistics_text))
                .with(Modify::new(Rows::single(1).and(Rows::last()).not(Columns::new(..0)).not(Columns::new(1..))).with(|s: &str| s.white().bold().to_string()))
        }),
        Box::new(|m| {
            let c = "━".yellow();
            let statistics_text = format!("{}{}{}", c, c, "Statistics".black().on_yellow());
            full_table(m)
                .with(Header("Star Wars Movies"))
                .with(Modify::new(Rows::single(0)).with(Alignment::center()))
                .with(BorderText::new(m.len()+2, statistics_text))
                .with(Modify::new(Rows::single(1).and(Rows::last()).not(Columns::new(..0)).not(Columns::new(1..))).with(|s: &str| s.white().bold().to_string()))
                .with(Modify::new(Rows::single(1).and(Rows::last()).not(Columns::new(..1)).not(Columns::new(2..))).with(|s: &str| s.white().bold().to_string()))
        }),
        Box::new(|m| {
            let c = "━".yellow();
            let statistics_text = format!("{}{}{}", c, c, "Statistics".black().on_yellow());
            full_table(m)
                .with(Header("Star Wars Movies"))
                .with(Modify::new(Rows::single(0)).with(Alignment::center()))
                .with(BorderText::new(m.len()+2, statistics_text))
                .with(Modify::new(Rows::single(1).and(Rows::last()).not(Columns::new(..0)).not(Columns::new(1..))).with(|s: &str| s.white().bold().to_string()))
                .with(Modify::new(Rows::single(1).and(Rows::last()).not(Columns::new(..1)).not(Columns::new(2..))).with(|s: &str| s.white().bold().to_string()))
                .with(Modify::new(Rows::single(1).and(Rows::last()).not(Columns::new(..2)).not(Columns::new(3..))).with(|s: &str| s.red().bold().to_string()))
        }),
        Box::new(|m| {
            let c = "━".yellow();
            let statistics_text = format!("{}{}{}", c, c, "Statistics".black().on_yellow());
            full_table(m)
                .with(Header("Star Wars Movies"))
                .with(Modify::new(Rows::single(0)).with(Alignment::center()))
                .with(BorderText::new(m.len()+2, statistics_text))
                .with(Modify::new(Rows::single(1).and(Rows::last()).not(Columns::new(..0)).not(Columns::new(1..))).with(|s: &str| s.white().bold().to_string()))
                .with(Modify::new(Rows::single(1).and(Rows::last()).not(Columns::new(..1)).not(Columns::new(2..))).with(|s: &str| s.white().bold().to_string()))
                .with(Modify::new(Rows::single(1).and(Rows::last()).not(Columns::new(..2)).not(Columns::new(3..))).with(|s: &str| s.red().bold().to_string()))
                .with(Modify::new(Rows::single(1).and(Rows::last()).not(Columns::new(..3)).not(Columns::new(4..))).with(|s: &str| s.green().bold().to_string()))
        }),
        Box::new(|m| {
            let c = "━".yellow();
            let statistics_text = format!("{}{}{}", c, c, "Statistics".black().on_yellow());
            full_table(m)
                .with(Header("Star Wars Movies"))
                .with(Modify::new(Rows::single(0)).with(Alignment::center()))
                .with(BorderText::new(m.len()+2, statistics_text))
                .with(Modify::new(Rows::single(1).and(Rows::last()).not(Columns::new(..0)).not(Columns::new(1..))).with(|s: &str| s.white().bold().to_string()))
                .with(Modify::new(Rows::single(1).and(Rows::last()).not(Columns::new(..1)).not(Columns::new(2..))).with(|s: &str| s.white().bold().to_string()))
                .with(Modify::new(Rows::single(1).and(Rows::last()).not(Columns::new(..2)).not(Columns::new(3..))).with(|s: &str| s.red().bold().to_string()))
                .with(Modify::new(Rows::single(1).and(Rows::last()).not(Columns::new(..3)).not(Columns::new(4..))).with(|s: &str| s.green().bold().to_string()))
                .with(Modify::new(Rows::single(1).and(Rows::last()).not(Columns::new(..4)).not(Columns::new(5..))).with(|s: &str| s.blue().bold().to_string()))
        }),
    ];

    let full_table: &'static dyn Fn(&[Movie]) -> Table = &|m: &[Movie]| {
        let line_s: &'static dyn Fn() -> Symbol =
            &|| Symbol::ansi("━".yellow().to_string()).unwrap();

        let c = "━".yellow();
        let statistics_text = format!("{}{}{}", c, c, "Statistics".black().on_yellow());

        Table::builder(m)
            .add_record([
                "",
                "",
                "$1,716,500,000",
                "$1,190,650,976",
                "$10,263,484,824",
            ])
            .build()
            .with(Style::blank())
            .with(Modify::new(Columns::new(..6)).with(Alignment::right()))
            .with(Modify::new(Columns::new(..6)).with(Alignment::left()))
            .with(Highlight::new(
                Rows::single(0),
                Border::default().bottom(line_s()),
            ))
            .with(Highlight::new(
                Rows::last(),
                Border::default().top(line_s()),
            ))
            .with(Header("Star Wars Movies"))
            .with(Modify::new(Rows::single(0)).with(Alignment::center()))
            .with(BorderText::new(m.len() + 2, statistics_text))
            .with(
                Modify::new(
                    Rows::single(1)
                        .and(Rows::last())
                        .not(Columns::new(..0))
                        .not(Columns::new(1..)),
                )
                .with(|s: &str| s.white().bold().to_string()),
            )
            .with(
                Modify::new(
                    Rows::single(1)
                        .and(Rows::last())
                        .not(Columns::new(..1))
                        .not(Columns::new(2..)),
                )
                .with(|s: &str| s.white().bold().to_string()),
            )
            .with(
                Modify::new(
                    Rows::single(1)
                        .and(Rows::last())
                        .not(Columns::new(..2))
                        .not(Columns::new(3..)),
                )
                .with(|s: &str| s.red().bold().to_string()),
            )
            .with(
                Modify::new(
                    Rows::single(1)
                        .and(Rows::last())
                        .not(Columns::new(..3))
                        .not(Columns::new(4..)),
                )
                .with(|s: &str| s.green().bold().to_string()),
            )
            .with(
                Modify::new(
                    Rows::single(1)
                        .and(Rows::last())
                        .not(Columns::new(..4))
                        .not(Columns::new(5..)),
                )
                .with(|s: &str| s.blue().bold().to_string()),
            )
    };

    #[rustfmt::skip]
    let resize_actions: Vec<Box<dyn Fn(&[Movie]) -> Table>> = vec![
        Box::new(|m| full_table(m).with(MaxWidth::wrapping(120).keep_words())),
        Box::new(|m| full_table(m).with(MaxWidth::wrapping(115).keep_words())),
        Box::new(|m| full_table(m).with(MaxWidth::wrapping(110).keep_words())),
        Box::new(|m| full_table(m).with(MaxWidth::wrapping(105).keep_words())),
        Box::new(|m| full_table(m).with(MaxWidth::wrapping(100).keep_words())),
        Box::new(|m| full_table(m).with(MaxWidth::wrapping(95).keep_words())),
        Box::new(|m| full_table(m).with(MaxWidth::wrapping(90).keep_words())),
        Box::new(|m| full_table(m).with(MaxWidth::wrapping(80).keep_words())),
        //
        Box::new(|m| full_table(m).with(MaxWidth::wrapping(80).keep_words())),
        Box::new(|m| full_table(m).with(MaxWidth::wrapping(90).keep_words())),
        Box::new(|m| full_table(m).with(MaxWidth::wrapping(95).keep_words())),
        Box::new(|m| full_table(m).with(MaxWidth::wrapping(100).keep_words())),
        Box::new(|m| full_table(m).with(MaxWidth::wrapping(105).keep_words())),
        Box::new(|m| full_table(m).with(MaxWidth::wrapping(110).keep_words())),
        Box::new(|m| full_table(m).with(MaxWidth::wrapping(115).keep_words())),
        Box::new(|m| full_table(m).with(MaxWidth::wrapping(120).keep_words())),
        //
        Box::new(|m| full_table(m).with(MinWidth::new(125))),
        Box::new(|m| full_table(m).with(MinWidth::new(130))),
        Box::new(|m| full_table(m).with(MinWidth::new(135))),
        Box::new(|m| full_table(m).with(MinWidth::new(140))),
        Box::new(|m| full_table(m).with(MinWidth::new(145))),
        Box::new(|m| full_table(m).with(MinWidth::new(150))),
    ];

    run_actions(movies, create_titles_actions, 450);
    run_actions(movies, add_movies_actions, 200);
    run_actions(movies, add_summary_actions, 450);
    run_actions(movies, formatting_actions, 350);
    run_actions(movies, style_actions, 650);
    run_actions(movies, border_colors_actions, 500);
    run_actions(movies, panel_actions, 600);
    run_actions(movies, resize_actions, 190);
}

fn run_actions(movies: &[Movie], actions: Vec<Box<dyn Fn(&[Movie]) -> Table>>, timeout_ms: u64) {
    let cursor_pos_init = cursor::position().unwrap();

    println!("{:?}", cursor_pos_init);

    let mut stdout = stdout();

    execute!(stdout, Hide).unwrap();

    for action in actions {
        execute!(stdout, Clear(ClearType::All)).unwrap();

        execute!(stdout, MoveTo(0, 3)).unwrap();

        let table = (action)(movies);
        let table = table.with(Margin::new(20, 0, 0, 0));
        println!("{}", table);

        execute!(stdout, MoveTo(0, 3)).unwrap();
        // execute!(stdout, MoveToRow(cursor_pos_init.0)).unwrap();

        std::thread::sleep(Duration::from_millis(timeout_ms));
    }
}
