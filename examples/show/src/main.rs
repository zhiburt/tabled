//! A example mimics to a degree https://spectreconsole.net/
//!
//! A recording is done via https://asciinema.org/
//!
//! ```bash
//! cargo build --release
//! asciinema rec -c ../../target/release/show
//! ```
//!
//! A conversion to gif is done via https://dstein64.github.io/gifcast/

// todo: add --frames argument

use std::{
    io::{stdout, Stdout, StdoutLock, Write},
    time::Duration,
};

use crossterm::{
    cursor, queue,
    style::Stylize,
    terminal::{Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen},
};
use tabled::{
    object::{Columns, Object, Rows},
    style::{Border, BorderText, Symbol},
    Alignment, Disable, Header, Highlight, Margin, Modify, Style, Table, Tabled, Width,
};

mod config;

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
    let cfg = config::parse();

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

    run(&movies, cfg.debug);
}

fn run(movies: &[Movie], debug: bool) {
    #[rustfmt::skip]
    let create_titles_actions: Vec<Action> = vec![
        detached_action(|_, m| Table::new(m).with(Disable::Row(1..)).with(Disable::Column(1..)).with(Style::modern())),
        detached_action(|_, m| Table::new(m).with(Disable::Row(1..)).with(Disable::Column(2..)).with(Style::modern())),
        detached_action(|_, m| Table::new(m).with(Disable::Row(1..)).with(Disable::Column(3..)).with(Style::modern())),
        detached_action(|_, m| Table::new(m).with(Disable::Row(1..)).with(Disable::Column(4..)).with(Style::modern())),
        detached_action(|_, m| Table::new(m).with(Disable::Row(1..)).with(Disable::Column(5..)).with(Style::modern())),
    ];

    #[rustfmt::skip]
    let add_movies_actions: Vec<Action> = vec![
        detached_action(|_, m| Table::new(m).with(Disable::Row(2..)).with(Style::modern())),
        detached_action(|_, m| Table::new(m).with(Disable::Row(3..)).with(Style::modern())),
        detached_action(|_, m| Table::new(m).with(Disable::Row(4..)).with(Style::modern())),
        detached_action(|_, m| Table::new(m).with(Disable::Row(5..)).with(Style::modern())),
        detached_action(|_, m| Table::new(m).with(Disable::Row(6..)).with(Style::modern())),
        detached_action(|_, m| Table::new(m).with(Disable::Row(7..)).with(Style::modern())),
        detached_action(|_, m| Table::new(m).with(Disable::Row(8..)).with(Style::modern())),
        detached_action(|_, m| Table::new(m).with(Disable::Row(9..)).with(Style::modern())),
        detached_action(|_, m| Table::new(m).with(Disable::Row(10..)).with(Style::modern())),
        detached_action(|_, m| Table::new(m).with(Disable::Row(11..)).with(Style::modern())),
        detached_action(|_, m| Table::new(m).with(Disable::Row(12..)).with(Style::modern())),
    ];

    #[rustfmt::skip]
    let add_summary_actions: Vec<Action> = vec![
        full_action(|_, m, _| Table::builder(m).add_record(["", "", "", "", ""]).build().with(Style::modern())),
        action(|t| t.with(Modify::new(Rows::last().not(Columns::new(..2)).not(Columns::new(3..))).with(|_: &str| String::from("$1,716,500,000")))),
        action(|t| t.with(Modify::new(Rows::last().not(Columns::new(..3)).not(Columns::new(4..))).with(|_: &str| String::from("$1,190,650,976")))),
        action(|t| t.with(Modify::new(Rows::last().not(Columns::new(..4)).not(Columns::new(5..))).with(|_: &str| String::from("$10,263,484,824")))),
    ];

    #[rustfmt::skip]
    let formatting_actions: Vec<Action> = vec![
        action(|t| t.with(Modify::new(Columns::single(0)).with(Alignment::right()))),
        action(|t| t.with(Modify::new(Columns::single(1)).with(Alignment::right()))),
        action(|t| t.with(Modify::new(Columns::single(2)).with(Alignment::right()))),
        action(|t| t.with(Modify::new(Columns::single(3)).with(Alignment::right()))),
        action(|t| t.with(Modify::new(Columns::single(4)).with(Alignment::right()))),
        action(|t| t.with(Modify::new(Columns::single(5)).with(Alignment::right()))),
        //
        action(|t| t.with(Modify::new(Columns::single(0)).with(Alignment::left()))),
        action(|t| t.with(Modify::new(Columns::single(1)).with(Alignment::left()))),
        action(|t| t.with(Modify::new(Columns::single(2)).with(Alignment::left()))),
        action(|t| t.with(Modify::new(Columns::single(3)).with(Alignment::left()))),
        action(|t| t.with(Modify::new(Columns::single(4)).with(Alignment::left()))),
        action(|t| t.with(Modify::new(Columns::single(5)).with(Alignment::left()))),
    ];

    #[rustfmt::skip]
    let style_actions: Vec<Action> = vec![
        action(|t| t.with(Style::extended())),
        action(|t| t.with(Style::ascii())),
        action(|t| t.with(Style::rounded())),
        action(|t| t.with(Style::psql())),
        action(|t| t.with(Style::github_markdown())),
        action(|t| t.with(Style::blank())),
    ];

    #[rustfmt::skip]
    let border_colors_actions: Vec<Action> = vec![
        action(|t| t.with(Highlight::new(Rows::first(), Border::default().bottom(Symbol::ansi("━".yellow().to_string()).unwrap())))),
        action(|t| t.with(Highlight::new(Rows::last(), Border::default().top(Symbol::ansi("━".yellow().to_string()).unwrap())))),
    ];

    #[rustfmt::skip]
    let panel_actions: Vec<Action> = vec![
        action(|t| t.with(Header("Star Wars Movies")).with(Modify::new(Rows::first()).with(Alignment::center()))),
        full_action(|t, m, _| {
            let c = "━".yellow();
            let statistics_text = format!("{}{}{}", c, c, "Statistics".black().on_yellow());
            t.with(BorderText::new(m.len()+2, statistics_text))
        }),
        action(|t| t.with(Modify::new(Rows::single(1).and(Rows::last()).not(Columns::new(..0)).not(Columns::new(1..))).with(|s: &str| s.white().bold().to_string()))),
        action(|t| t.with(Modify::new(Rows::single(1).and(Rows::last()).not(Columns::new(..1)).not(Columns::new(2..))).with(|s: &str| s.white().bold().to_string()))),
        action(|t| t.with(Modify::new(Rows::single(1).and(Rows::last()).not(Columns::new(..2)).not(Columns::new(3..))).with(|s: &str| s.red().bold().to_string()))),
        action(|t| t.with(Modify::new(Rows::single(1).and(Rows::last()).not(Columns::new(..3)).not(Columns::new(4..))).with(|s: &str| s.green().bold().to_string()))),
        action(|t| t.with(Modify::new(Rows::single(1).and(Rows::last()).not(Columns::new(..4)).not(Columns::new(5..))).with(|s: &str| s.blue().bold().to_string()))),
    ];

    #[rustfmt::skip]
    let resize_actions: Vec<Action> = vec![
        detached_action(|t, _| t.with(Width::wrap(115).keep_words())),
        detached_action(|t, _| t.with(Width::wrap(110).keep_words())),
        detached_action(|t, _| t.with(Width::wrap(105).keep_words())),
        detached_action(|t, _| t.with(Width::wrap(100).keep_words())),
        detached_action(|t, _| t.with(Width::wrap(95).keep_words())),
        detached_action(|t, _| t.with(Width::wrap(90).keep_words())),
        detached_action(|t, _| t.with(Width::wrap(80).keep_words())),
        //
        detached_action(|t, _| t.with(Width::wrap(80).keep_words())),
        detached_action(|t, _| t.with(Width::wrap(90).keep_words())),
        detached_action(|t, _| t.with(Width::wrap(95).keep_words())),
        detached_action(|t, _| t.with(Width::wrap(100).keep_words())),
        detached_action(|t, _| t.with(Width::wrap(105).keep_words())),
        detached_action(|t, _| t.with(Width::wrap(110).keep_words())),
        detached_action(|t, _| t.with(Width::wrap(115).keep_words())),
        detached_action(|t, _| t.with(Width::wrap(120).keep_words())),
        //
        detached_action(|t, _| t.with(Width::increase(125))),
        detached_action(|t, _| t.with(Width::increase(130))),
        detached_action(|t, _| t.with(Width::increase(135))),
        detached_action(|t, _| t.with(Width::increase(140))),
        detached_action(|t, _| t.with(Width::increase(145))),
        detached_action(|t, _| t.with(Width::increase(150))),
    ];

    let mut runner = Runner::new(movies);

    let printer = PrinterCtrl::new();

    // fixme: I didn't figured out how to make it work with Box<dyn Printer>.
    if debug {
        let mut p = printer.start_debug();

        p.print(450, runner.build_frames(create_titles_actions));
        p.print(200, runner.build_frames(add_movies_actions));
        p.print(450, runner.build_frames(add_summary_actions));
        p.print(350, runner.build_frames(formatting_actions));
        p.print(650, runner.build_frames(style_actions));
        p.print(500, runner.build_frames(border_colors_actions));
        p.print(600, runner.build_frames(panel_actions));
        p.print(190, runner.build_frames(resize_actions));

        p.stop();
    } else {
        let mut p = printer.start();

        p.print(450, runner.build_frames(create_titles_actions));
        p.print(200, runner.build_frames(add_movies_actions));
        p.print(450, runner.build_frames(add_summary_actions));
        p.print(350, runner.build_frames(formatting_actions));
        p.print(650, runner.build_frames(style_actions));
        p.print(500, runner.build_frames(border_colors_actions));
        p.print(600, runner.build_frames(panel_actions));
        p.print(190, runner.build_frames(resize_actions));

        p.stop();
    };
}

struct Runner<'a> {
    last_table: Table,
    movies: &'a [Movie],
}

type Action = Box<dyn Fn(Table, &[Movie], &mut Context) -> Table>;

struct Context {
    ignore_table: bool,
}

fn detached_action<F: Fn(Table, &[Movie]) -> Table + 'static>(f: F) -> Action {
    Box::new(move |table, m, ctx| {
        ctx.ignore_table = true;
        f(table, m)
    })
}

fn full_action<F: Fn(Table, &[Movie], &mut Context) -> Table + 'static>(f: F) -> Action {
    Box::new(move |t, m, ctx| f(t, m, ctx))
}

fn action<F: Fn(Table) -> Table + 'static>(f: F) -> Action {
    Box::new(move |t, _, _| f(t))
}

impl<'a> Runner<'a> {
    fn new(movies: &'a [Movie]) -> Self {
        Self {
            movies,
            last_table: Table::new(movies),
        }
    }

    fn build_frames(&mut self, actions: Vec<Action>) -> Vec<Table> {
        let mut frames = Vec::new();
        for action in actions {
            let table = self.build_frame(action);
            frames.push(table);
        }

        frames
    }

    fn build_frame(&mut self, action: Action) -> Table {
        let movies = &self.movies;
        let last_table = self.last_table.clone();

        let mut ctx = Context {
            ignore_table: false,
        };

        let table = (action)(last_table, movies, &mut ctx);

        if !ctx.ignore_table {
            self.last_table = table.clone();
        }

        table
    }
}

struct PrinterCtrl {
    stdout: Stdout,
}

impl PrinterCtrl {
    fn new() -> Self {
        PrinterCtrl { stdout: stdout() }
    }

    fn start(&self) -> BasicPrinter<'_> {
        BasicPrinter::start(self)
    }

    fn start_debug(&self) -> DebugPrinter<'_> {
        DebugPrinter::start(self)
    }
}

struct BasicPrinter<'a> {
    stdout: StdoutLock<'a>,
}

impl<'a> BasicPrinter<'a> {
    fn start(ctrl: &'a PrinterCtrl) -> Self {
        let mut stdout = ctrl.stdout.lock();

        queue!(stdout, EnterAlternateScreen, cursor::Hide).unwrap();
        stdout.flush().unwrap();

        Self { stdout }
    }
}

impl Printer for BasicPrinter<'_> {
    fn print<I>(&mut self, timeout_ms: u64, frames: I)
    where
        I: IntoIterator<Item = Table>,
    {
        let left_padding = |t: Table| t.with(Margin::new(20, 0, 0, 0));

        for frame in frames {
            let frame = left_padding(frame);

            queue!(self.stdout, Clear(ClearType::All), cursor::MoveTo(0, 3)).unwrap();

            self.stdout.write_all(frame.to_string().as_bytes()).unwrap();
            self.stdout.flush().unwrap();

            std::thread::sleep(Duration::from_millis(timeout_ms));
        }
    }

    fn stop(&mut self) {
        let stdout = &mut self.stdout;
        queue!(stdout, LeaveAlternateScreen, cursor::Show).unwrap();
        stdout.flush().unwrap();
    }
}

struct DebugPrinter<'a> {
    i: usize,
    stdout: StdoutLock<'a>,
}

impl<'a> DebugPrinter<'a> {
    fn start(ctrl: &'a PrinterCtrl) -> Self {
        let mut stdout = ctrl.stdout.lock();
        stdout.flush().unwrap();

        Self { stdout, i: 1 }
    }
}

impl Printer for DebugPrinter<'_> {
    fn print<I>(&mut self, _: u64, frames: I)
    where
        I: IntoIterator<Item = Table>,
    {
        for frame in frames {
            writeln!(self.stdout, "FRAME={}", self.i).unwrap();
            writeln!(self.stdout, "{}", frame).unwrap();
            self.stdout.flush().unwrap();
            self.i += 1;
        }
    }

    fn stop(&mut self) {
        let stdout = &mut self.stdout;
        stdout.flush().unwrap();
    }
}

trait Printer {
    fn print<I>(&mut self, _: u64, frames: I)
    where
        I: IntoIterator<Item = Table>;
    fn stop(&mut self) {}
}
