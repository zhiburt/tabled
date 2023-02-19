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
//!
//! Credit for data: https://www.boxofficemojo.com/

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
    settings::{
        alignment::Alignment,
        color::Color,
        disable::Disable,
        format::Format,
        highlight::Highlight,
        margin::Margin,
        object::{Columns, Object, Rows},
        panel::Panel,
        style::{Border, BorderColor, BorderText, Style},
        width::Width,
        Modify,
    },
    Table, Tabled,
};

mod config;

#[derive(Tabled, Debug, Clone)]
struct Movie {
    #[tabled(rename = "Release Date")]
    release_date: &'static str,
    #[tabled(rename = "Title")]
    title: &'static str,
    #[tabled(rename = "Directed by")]
    #[tabled(skip)]
    #[allow(dead_code)]
    director: &'static str,
    #[tabled(rename = "Budget")]
    budget: &'static str,
    #[tabled(rename = "Opening Weekend")]
    opening_weekend: &'static str,
    #[tabled(rename = "Box Office")]
    box_office: &'static str,
}

const MOVIES: &[Movie] = &[
    Movie {
        title: "The Lord of the Rings: The Fellowship of the Ring",
        release_date: "December 19, 2001",
        budget: "$93,000,000",
        opening_weekend: "$47,211,490",
        box_office: "$898,094,742",
        director: "Peter Jackson",
    },
    Movie {
        title: "The Lord of the Rings: The Two Towers",
        release_date: "December 18, 2002",
        budget: "$94,000,000",
        opening_weekend: "$62,007,528",
        box_office: "$947,896,241",
        director: "Peter Jackson",
    },
    Movie {
        title: "The Lord of the Rings: The Return of the King",
        release_date: "December 17, 2003",
        budget: "$94,000,000",
        opening_weekend: "$72,629,713",
        box_office: "$1,146,436,214",
        director: "Peter Jackson",
    },
    Movie {
        title: "The Hobbit: An Unexpected Journey",
        release_date: "December 14, 2012",
        budget: "unknown",
        opening_weekend: "$84,617,303",
        box_office: "$1,017,030,651",
        director: "Peter Jackson",
    },
    Movie {
        title: "The Hobbit: The Desolation of Smaug",
        release_date: "December 13, 2013",
        budget: "unknown",
        opening_weekend: "$73,645,197",
        box_office: "$959,027,992",
        director: "Peter Jackson",
    },
    Movie {
        title: "The Hobbit: The Battle of the Five Armies",
        release_date: "December 17, 2014",
        budget: "unknown",
        opening_weekend: "$54,724,334",
        box_office: "$962,201,338",
        director: "Peter Jackson",
    },
];

fn main() {
    let cfg = config::parse();

    run(MOVIES, cfg.debug);
}

fn run(movies: &[Movie], debug: bool) {
    let printer = PrinterCtrl::new();

    if debug {
        let mut p = printer.start_debug();
        print_movies(&mut p, movies);
        p.stop();
    } else {
        let mut p = printer.start();
        print_movies(&mut p, movies);
        p.stop();
    };
}

fn print_movies(p: &mut impl Printer, movies: &[Movie]) {
    #[rustfmt::skip]
    let create_titles_actions: Vec<Action> = vec![
        detached_action(|_, m| Table::new(m).with(Disable::row(Rows::new(1..))).with(Disable::column(Columns::new(1..))).with(Style::modern()).clone()),
        detached_action(|_, m| Table::new(m).with(Disable::row(Rows::new(1..))).with(Disable::column(Columns::new(2..))).with(Style::modern()).clone()),
        detached_action(|_, m| Table::new(m).with(Disable::row(Rows::new(1..))).with(Disable::column(Columns::new(3..))).with(Style::modern()).clone()),
        detached_action(|_, m| Table::new(m).with(Disable::row(Rows::new(1..))).with(Disable::column(Columns::new(4..))).with(Style::modern()).clone()),
        detached_action(|_, m| Table::new(m).with(Disable::row(Rows::new(1..))).with(Disable::column(Columns::new(5..))).with(Style::modern()).clone()),
    ];

    #[rustfmt::skip]
    let add_movies_actions: Vec<Action> = vec![
        detached_action(|_, m| Table::new(m).with(Disable::row(Rows::new(2..))).with(Style::modern()).clone()),
        detached_action(|_, m| Table::new(m).with(Disable::row(Rows::new(3..))).with(Style::modern()).clone()),
        detached_action(|_, m| Table::new(m).with(Disable::row(Rows::new(4..))).with(Style::modern()).clone()),
        detached_action(|_, m| Table::new(m).with(Disable::row(Rows::new(5..))).with(Style::modern()).clone()),
        detached_action(|_, m| Table::new(m).with(Disable::row(Rows::new(6..))).with(Style::modern()).clone()),
        detached_action(|_, m| Table::new(m).with(Disable::row(Rows::new(7..))).with(Style::modern()).clone()),
        detached_action(|_, m| Table::new(m).with(Disable::row(Rows::new(8..))).with(Style::modern()).clone()),
        detached_action(|_, m| Table::new(m).with(Disable::row(Rows::new(9..))).with(Style::modern()).clone()),
        detached_action(|_, m| Table::new(m).with(Disable::row(Rows::new(10..))).with(Style::modern()).clone()),
        detached_action(|_, m| Table::new(m).with(Disable::row(Rows::new(11..))).with(Style::modern()).clone()),
        detached_action(|_, m| Table::new(m).with(Disable::row(Rows::new(12..))).with(Style::modern()).clone()),
    ];

    #[rustfmt::skip]
    let add_summary_actions: Vec<Action> = vec![
        full_action(|_, m, _| {
            let mut table = Table::builder(m);
            table.push_record(["", "", "", "", ""]);
            
            table.build().with(Style::modern()).clone()
        }),
        action(|mut t| t.with(Modify::new(Rows::last().not(Columns::new(..2)).not(Columns::new(3..))).with(">= $281,000,000")).clone()),
        action(|mut t| t.with(Modify::new(Rows::last().not(Columns::new(..3)).not(Columns::new(4..))).with("$394,835,565")).clone()),
        action(|mut t| t.with(Modify::new(Rows::last().not(Columns::new(..4)).not(Columns::new(5..))).with("$5,930,687,178")).clone()),
    ];

    #[rustfmt::skip]
    let formatting_actions: Vec<Action> = vec![
        action(|mut t| t.with(Modify::new(Columns::single(0)).with(Alignment::right())).clone()),
        action(|mut t| t.with(Modify::new(Columns::single(1)).with(Alignment::right())).clone()),
        action(|mut t| t.with(Modify::new(Columns::single(2)).with(Alignment::right())).clone()),
        action(|mut t| t.with(Modify::new(Columns::single(3)).with(Alignment::right())).clone()),
        action(|mut t| t.with(Modify::new(Columns::single(4)).with(Alignment::right())).clone()),
        action(|mut t| t.with(Modify::new(Columns::single(5)).with(Alignment::right())).clone()),
        //
        action(|mut t| t.with(Modify::new(Columns::single(0)).with(Alignment::left())).clone()),
        action(|mut t| t.with(Modify::new(Columns::single(1)).with(Alignment::left())).clone()),
        action(|mut t| t.with(Modify::new(Columns::single(2)).with(Alignment::left())).clone()),
        action(|mut t| t.with(Modify::new(Columns::single(3)).with(Alignment::left())).clone()),
        action(|mut t| t.with(Modify::new(Columns::single(4)).with(Alignment::left())).clone()),
        action(|mut t| t.with(Modify::new(Columns::single(5)).with(Alignment::left())).clone()),
    ];

    #[rustfmt::skip]
    let style_actions: Vec<Action> = vec![
        action(|mut t| t.with(Style::extended()).clone()),
        action(|mut t| t.with(Style::ascii()).clone()),
        action(|mut t| t.with(Style::rounded()).clone()),
        action(|mut t| t.with(Style::psql()).clone()),
        action(|mut t| t.with(Style::markdown()).clone()),
        action(|mut t| t.with(Style::ascii_rounded()).clone()),
        action(|mut t| t.with(Style::blank()).clone()),
    ];

    let border_colors_actions: Vec<Action> = vec![];

    #[rustfmt::skip]
    let panel_actions: Vec<Action> = vec![
        action(|mut t| t.with(Panel::header("The Lord of the Rings")).with(Modify::new(Rows::first()).with(Alignment::center())).clone()),
        action(|mut t| t
            .with(Highlight::colored(Rows::single(2), BorderColor::default().top(Color::FG_YELLOW)))
            .with(Highlight::new(Rows::single(2), Border::default().top('━')))
            .clone()),
        action(|mut t| t
            .with(Highlight::colored(Rows::last(), BorderColor::default().top(Color::FG_YELLOW)))
            .with(Highlight::new(Rows::last(), Border::default().top('━')))
            .clone()),
        full_action(|mut t, m, _| {
            let c = "━".yellow();
            let statistics_text = format!("{}{}{}", c, c, "Statistics".black().on_yellow());
            t.with(BorderText::new(m.len()+2, statistics_text)).clone()
        }),
    ];

    #[rustfmt::skip]
    let colorization_actions: Vec<Action> = vec![
        action(|mut t| t.with(Modify::new(Rows::single(1).and(Rows::last()).not(Columns::new(..0)).not(Columns::new(1..))).with(Format::content(|s| s.white().bold().to_string()))).clone()),
        action(|mut t| t.with(Modify::new(Rows::single(1).and(Rows::last()).not(Columns::new(..1)).not(Columns::new(2..))).with(Format::content(|s| s.white().bold().to_string()))).clone()),
        action(|mut t| t.with(Modify::new(Rows::single(1).and(Rows::last()).not(Columns::new(..2)).not(Columns::new(3..))).with(Format::content(|s| s.red().bold().to_string()))).clone()),
        action(|mut t| t.with(Modify::new(Rows::single(1).and(Rows::last()).not(Columns::new(..3)).not(Columns::new(4..))).with(Format::content(|s| s.green().bold().to_string()))).clone()),
        action(|mut t| t.with(Modify::new(Rows::single(1).and(Rows::last()).not(Columns::new(..4)).not(Columns::new(5..))).with(Format::content(|s| s.blue().bold().to_string()))).clone()),
    ];

    #[rustfmt::skip]
    let resize_actions: Vec<Action> = vec![
        detached_action(|mut t, _| t.with(Width::wrap(115).keep_words()).clone()),
        detached_action(|mut t, _| t.with(Width::wrap(110).keep_words()).clone()),
        detached_action(|mut t, _| t.with(Width::wrap(105).keep_words()).clone()),
        detached_action(|mut t, _| t.with(Width::wrap(100).keep_words()).clone()),
        detached_action(|mut t, _| t.with(Width::wrap(95).keep_words()).clone()),
        detached_action(|mut t, _| t.with(Width::wrap(90).keep_words()).clone()),
        detached_action(|mut t, _| t.with(Width::wrap(80).keep_words()).clone()),
        //
        detached_action(|mut t, _| t.with(Width::wrap(90).keep_words()).clone()),
        detached_action(|mut t, _| t.with(Width::wrap(95).keep_words()).clone()),
        detached_action(|mut t, _| t.with(Width::wrap(100).keep_words()).clone()),
        detached_action(|mut t, _| t.with(Width::wrap(105).keep_words()).clone()),
        detached_action(|mut t, _| t.with(Width::wrap(110).keep_words()).clone()),
        detached_action(|mut t, _| t.with(Width::wrap(115).keep_words()).clone()),
        detached_action(|mut t, _| t.with(Width::wrap(120).keep_words()).clone()),
        //
        detached_action(|mut t, _| t.with(Width::increase(125)).clone()),
        detached_action(|mut t, _| t.with(Width::increase(130)).clone()),
        detached_action(|mut t, _| t.with(Width::increase(135)).clone()),
        detached_action(|mut t, _| t.with(Width::increase(140)).clone()),
        detached_action(|mut t, _| t.with(Width::increase(145)).clone()),
        detached_action(|mut t, _| t.with(Width::increase(150)).clone()),
    ];

    let mut runner = Runner::new(movies);
    p.print(450, runner.build_frames(create_titles_actions));
    p.print(200, runner.build_frames(add_movies_actions));
    p.print(450, runner.build_frames(add_summary_actions));
    p.print(350, runner.build_frames(formatting_actions));
    p.print(650, runner.build_frames(style_actions));
    p.print(500, runner.build_frames(border_colors_actions));
    p.print(600, runner.build_frames(panel_actions));
    p.print(400, runner.build_frames(colorization_actions));
    p.print(190, runner.build_frames(resize_actions));
}

struct Runner<'a> {
    last_table: Table,
    movies: &'a [Movie],
}

type Action = Box<dyn Fn(Table, &[Movie], &mut Context) -> Table>;

struct Context {
    ignore_table: bool,
}

fn detached_action<F>(f: F) -> Action
where
    F: Fn(Table, Vec<Movie>) -> Table + 'static,
{
    Box::new(move |table, m, ctx| {
        ctx.ignore_table = true;
        f(table, m.to_vec())
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
            last_table: Table::new(movies.to_vec()),
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
        let left_padding = Margin::new(10, 0, 0, 0);
        for (_i, mut frame) in frames.into_iter().enumerate() {
            frame.with(left_padding.clone());

            queue!(self.stdout, Clear(ClearType::All), cursor::MoveTo(0, 7)).unwrap();

            // writeln!(&mut self.stdout, "i={}", _i).unwrap();

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
            writeln!(self.stdout, "{frame}").unwrap();
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
