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
    name: &'static str,
    category: &'static str,
    value: f64,
}

const CLEAR: &str = "\u{1b}[2J";

type Step = Box<dyn Fn(&mut Table) -> (u64, &mut Table)>;

const ITEM_LIST: &[Item] = &[
    Item {
        name: "Light Bulb",
        category: "Household",
        value: 3.67,
    },
    Item {
        name: "Toothbrush",
        category: "Bathroom",
        value: 0.99,
    },
    Item {
        name: "Tire",
        category: "Vehicle",
        value: 230.0,
    },
];

const TIME: u64 = 400;

fn main() {
    let mut table = Table::new(ITEM_LIST);
    let steps: Vec<Step> = vec![
        Box::new(|t: &mut Table| (TIME, t.with(Style::blank()))),
        Box::new(|t: &mut Table| {
            (
                TIME,
                t.with(Colorization::rows([
                    Color::rgb_bg(0, 0, 0) | Color::rgb_fg(255, 255, 255),
                    Color::rgb_bg(255, 255, 255) | Color::rgb_fg(0, 0, 0),
                ])),
            )
        }),
        Box::new(|t: &mut Table| {
            (
                TIME,
                t.with(Colorization::exact([Color::UNDERLINE], Rows::first())),
            )
        }),
        Box::new(|t: &mut Table| {
            (
                TIME,
                t.modify(
                    Rows::new(1..).step_by(2),
                    BorderColor::new().left(Color::rgb_bg(255, 255, 255)),
                )
                .modify(
                    Rows::new(2..).step_by(2),
                    BorderColor::new().left(Color::rgb_bg(0, 0, 0)),
                ),
            )
        }),
        Box::new(|t: &mut Table| {
            (
                TIME,
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
                ),
            )
        }),
        Box::new(|t: &mut Table| {
            (
                300,
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
                ),
            )
        }),
    ];

    run_steps(&mut table, &steps);
}

fn run_steps(initial_table: &mut Table, steps: &[Step]) {
    let mut t: u64;
    let mut table = initial_table;
    for step in steps {
        println!("{}", CLEAR);
        println!("{}", table);
        (t, table) = step(table);
        sleep(Duration::from_millis(t));
    }

    println!("{}", CLEAR);
    println!("{}", table);
    sleep(Duration::from_millis(1000));
}
