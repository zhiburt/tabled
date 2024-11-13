use std::{thread::sleep, time::Duration};

use tabled::{
    settings::{
        object::{ObjectIterator, Rows},
        style::BorderColor,
        themes::Colorization,
        Color, Style,
    },
    Table,
};
use tabled_derive::Tabled;

#[derive(Tabled, Clone, Debug)]
struct Item {
    name: String,
    #[tabled(format("{}", self.category.join(",")))]
    category: Vec<String>,
    value: f64,
}

impl Item {
    fn new(name: &str, category: &[&str], value: f64) -> Self {
        Self {
            name: name.to_owned(),
            category: category.iter().map(ToString::to_string).collect(),
            value,
        }
    }
}

fn main() {
    let mut p = Pager::default();
    p.append(|t| {
        t.with(Style::blank());
    });
    p.append(|t| {
        t.with(Colorization::rows([
            Color::rgb_bg(0, 0, 0) | Color::rgb_fg(255, 255, 255),
            Color::rgb_bg(255, 255, 255) | Color::rgb_fg(0, 0, 0),
        ]));
    });
    p.append(|t| {
        t.with(Colorization::exact([Color::UNDERLINE], Rows::first()));
    });
    p.append(|t| {
        t.modify(
            Rows::new(1..).step_by(2),
            BorderColor::new().left(Color::rgb_bg(255, 255, 255)),
        )
        .modify(
            Rows::new(2..).step_by(2),
            BorderColor::new().left(Color::rgb_bg(0, 0, 0)),
        );
    });
    p.append(|t| {
        t.with(Colorization::exact(
            [
                Color::rgb_bg(0, 0, 0) | Color::rgb_fg(255, 255, 255),
                Color::rgb_bg(255, 255, 255) | Color::rgb_fg(0, 0, 0),
            ],
            Rows::new(1..),
        ))
        .modify(
            Rows::new(1..).step_by(2),
            BorderColor::new().left(Color::rgb_bg(0, 0, 0)),
        )
        .modify(
            Rows::new(2..).step_by(2),
            BorderColor::new().left(Color::rgb_bg(255, 255, 255)),
        );
    });
    p.append(|t| {
        t.with(Colorization::exact(
            [
                Color::rgb_bg(128, 128, 255) | Color::rgb_fg(0, 0, 0),
                Color::rgb_bg(200, 100, 150) | Color::rgb_fg(0, 0, 0),
            ],
            Rows::new(1..),
        ))
        .modify(
            Rows::new(1..).step_by(2),
            BorderColor::new().left(Color::rgb_bg(128, 128, 255)),
        )
        .modify(
            Rows::new(2..).step_by(2),
            BorderColor::new().left(Color::rgb_bg(200, 100, 150)),
        );
    });

    let data = [
        Item::new("Light Bulb", &["Household"], 3.67),
        Item::new("Toothbrush", &["Household", "Bathroom"], 3.67),
        Item::new("Tire", &["Vehicle"], 299.0),
    ];

    let table = Table::new(data);

    p.render(table);
}

type Step = Box<dyn Fn(&mut Table) -> u64>;

#[derive(Default)]
struct Pager {
    pages: Vec<Step>,
}

impl Pager {
    fn append<F>(&mut self, f: F)
    where
        F: Fn(&mut Table) + 'static,
    {
        self.append_timed(f, 400);
    }

    fn append_timed<F>(&mut self, f: F, time_ms: u64)
    where
        F: Fn(&mut Table) + 'static,
    {
        self.pages.push(step(f, time_ms));
    }

    fn render(&self, mut table: Table) {
        run_steps(&mut table, &self.pages)
    }
}

fn step<F>(f: F, delay_ms: u64) -> Step
where
    F: Fn(&mut Table) + 'static,
{
    Box::new(move |t| {
        (f)(t);
        delay_ms
    })
}

fn run_steps(table: &mut Table, steps: &[Step]) {
    const CLEAR: &str = "\u{1b}[2J";

    let mut t: u64;
    for step in steps {
        println!("{}", CLEAR);
        t = (step)(table);
        println!("{}", table);
        sleep(Duration::from_millis(t));
    }
}
