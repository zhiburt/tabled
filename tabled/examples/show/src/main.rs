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

use clap::Parser;
use crossterm::{
    cursor, queue,
    style::Stylize,
    terminal::{Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen},
};

use tabled::{
    settings::{
        object::{Columns, Object, Rows},
        style::{Border, BorderColor, LineText, Style},
        Alignment, Color, Disable, Format, Highlight, Margin, Panel, Width,
    },
    Table, Tabled,
};

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

#[derive(Parser)]
struct Config {
    /// Turn debugging information on
    #[clap(long, short)]
    pub debug: bool,
}

fn main() {
    let cfg = Config::parse();

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
        detached_action(|t| { t.with(Disable::row(Rows::new(1..))).with(Disable::column(Columns::new(1..))).with(Style::modern()); }),
        detached_action(|t| { t.with(Disable::row(Rows::new(1..))).with(Disable::column(Columns::new(2..))).with(Style::modern()); }),
        detached_action(|t| { t.with(Disable::row(Rows::new(1..))).with(Disable::column(Columns::new(3..))).with(Style::modern()); }),
        detached_action(|t| { t.with(Disable::row(Rows::new(1..))).with(Disable::column(Columns::new(4..))).with(Style::modern()); }),
        detached_action(|t| { t.with(Disable::row(Rows::new(1..))).with(Disable::column(Columns::new(5..))).with(Style::modern()); }),
    ];

    #[rustfmt::skip]
    let add_movies_actions: Vec<Action> = vec![
        detached_action(|t| { t.with(Disable::row(Rows::new(2..))).with(Style::modern()); }),
        detached_action(|t| { t.with(Disable::row(Rows::new(3..))).with(Style::modern()); }),
        detached_action(|t| { t.with(Disable::row(Rows::new(4..))).with(Style::modern()); }),
        detached_action(|t| { t.with(Disable::row(Rows::new(5..))).with(Style::modern()); }),
        detached_action(|t| { t.with(Disable::row(Rows::new(6..))).with(Style::modern()); }),
        detached_action(|t| { t.with(Disable::row(Rows::new(7..))).with(Style::modern()); }),
    ];

    #[rustfmt::skip]
    let add_summary_actions: Vec<Action> = vec![
        new_action(|m| {
            let mut b = Table::builder(m);
            b.push_record(["", "", "", "", ""]);
            
            let mut table = b.build();
            table.with(Style::modern());

            table
        }),
        action(|t| { t.modify(Rows::last().not(Columns::new(..2)).not(Columns::new(3..)), ">= $281,000,000"); }),
        action(|t| { t.modify(Rows::last().not(Columns::new(..3)).not(Columns::new(4..)), "$394,835,565"); }),
        action(|t| { t.modify(Rows::last().not(Columns::new(..4)).not(Columns::new(5..)), "$5,930,687,178"); }),
    ];

    #[rustfmt::skip]
    let formatting_actions: Vec<Action> = vec![
        action(|t| { t.modify(Columns::single(0), Alignment::right()); }),
        action(|t| { t.modify(Columns::single(1), Alignment::right()); }),
        action(|t| { t.modify(Columns::single(2), Alignment::right()); }),
        action(|t| { t.modify(Columns::single(3), Alignment::right()); }),
        action(|t| { t.modify(Columns::single(4), Alignment::right()); }),
        action(|t| { t.modify(Columns::single(5), Alignment::right()); }),
        //
        action(|t| { t.modify(Columns::single(0), Alignment::left()); }),
        action(|t| { t.modify(Columns::single(1), Alignment::left()); }),
        action(|t| { t.modify(Columns::single(2), Alignment::left()); }),
        action(|t| { t.modify(Columns::single(3), Alignment::left()); }),
        action(|t| { t.modify(Columns::single(4), Alignment::left()); }),
        action(|t| { t.modify(Columns::single(5), Alignment::left()); }),
    ];

    #[rustfmt::skip]
    let style_actions: Vec<Action> = vec![
        action(|t| { t.with(Style::extended()); } ),
        action(|t| { t.with(Style::ascii()); } ),
        action(|t| { t.with(Style::rounded()); } ),
        action(|t| { t.with(Style::psql()); } ),
        action(|t| { t.with(Style::markdown()); } ),
        action(|t| { t.with(Style::ascii_rounded()); } ),
        action(|t| { t.with(Style::blank()); } ),
    ];

    let border_colors_actions: Vec<Action> = vec![];

    #[rustfmt::skip]
    let panel_actions: Vec<Action> = vec![
        action(|t| {
            t.with(Panel::header("The Lord of the Rings"));
            t.modify(Rows::first(), Alignment::center());
        }),
        action(|t| {
            t.with(Highlight::new(Rows::single(2)).color(BorderColor::default().top(Color::FG_YELLOW)));
            t.with(Highlight::new(Rows::single(2)).border(Border::new().set_top('━')));
        }),
        action(|t| {
            t.with(Highlight::new(Rows::last()).color(BorderColor::default().top(Color::FG_YELLOW)));
            t.with(Highlight::new(Rows::last()).border(Border::new().set_top('━')));
        }),
        action(|t| {
            let color = Color::try_from(" ".black().on_yellow().to_string()).unwrap();
            t.with(LineText::new("Statistics", Rows::last()).color(color));
        }),
    ];

    #[rustfmt::skip]
    let colorization_actions: Vec<Action> = vec![
        action(|t| { t.modify(Rows::single(1).and(Rows::last()).intersect(Columns::single(0)), Format::content(|s| s.white().bold().to_string())); }),
        action(|t| { t.modify(Rows::single(1).and(Rows::last()).intersect(Columns::single(1)), Format::content(|s| s.white().bold().to_string())); }),
        action(|t| { t.modify(Rows::single(1).and(Rows::last()).intersect(Columns::single(2)), Format::content(|s| s.red().bold().to_string())); }),
        action(|t| { t.modify(Rows::single(1).and(Rows::last()).intersect(Columns::single(3)), Format::content(|s| s.green().bold().to_string())); }),
        action(|t| { t.modify(Rows::single(1).and(Rows::last()).intersect(Columns::single(4)), Format::content(|s| s.blue().bold().to_string())); }),
    ];

    #[rustfmt::skip]
    let resize_actions: Vec<Action> = vec![
        detached_action(|t| { t.with(Width::wrap(115).keep_words(true)); }),
        detached_action(|t| { t.with(Width::wrap(110).keep_words(true)); }),
        detached_action(|t| { t.with(Width::wrap(105).keep_words(true)); }),
        detached_action(|t| { t.with(Width::wrap(100).keep_words(true)); }),
        detached_action(|t| { t.with(Width::wrap(95).keep_words(true)); }),
        detached_action(|t| { t.with(Width::wrap(90).keep_words(true)); }),
        detached_action(|t| { t.with(Width::wrap(80).keep_words(true)); }),
        //
        detached_action(|t| { t.with(Width::wrap(90).keep_words(true)); }),
        detached_action(|t| { t.with(Width::wrap(95).keep_words(true)); }),
        detached_action(|t| { t.with(Width::wrap(100).keep_words(true)); }),
        detached_action(|t| { t.with(Width::wrap(105).keep_words(true)); }),
        detached_action(|t| { t.with(Width::wrap(110).keep_words(true)); }),
        detached_action(|t| { t.with(Width::wrap(115).keep_words(true)); }),
        detached_action(|t| { t.with(Width::wrap(120).keep_words(true)); }),
        //
        detached_action(|t| { t.with(Width::increase(125)); }),
        detached_action(|t| { t.with(Width::increase(130)); }),
        detached_action(|t| { t.with(Width::increase(135)); }),
        detached_action(|t| { t.with(Width::increase(140)); }),
        detached_action(|t| { t.with(Width::increase(145)); }),
        detached_action(|t| { t.with(Width::increase(150)); }),
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

fn action<F>(f: F) -> Action
where
    F: Fn(&mut Table) + 'static,
{
    Box::new(move |mut t, _, _| {
        f(&mut t);
        t
    })
}

fn detached_action<F>(f: F) -> Action
where
    F: Fn(&mut Table) + 'static,
{
    Box::new(move |mut table, _, ctx| {
        ctx.ignore_table = true;
        f(&mut table);
        table
    })
}

fn new_action<F>(f: F) -> Action
where
    F: Fn(Vec<Movie>) -> Table + 'static,
{
    Box::new(move |_, m, _| f(m.to_vec()))
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
        for mut frame in frames.into_iter() {
            frame.with(left_padding.clone());

            queue!(self.stdout, Clear(ClearType::All), cursor::MoveTo(0, 7)).unwrap();

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
