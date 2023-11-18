//! This example demonstrates [`Locator`] usage to colorize certain cells.

use tabled::{
    settings::{location::Locator, style::Style, Color},
    Table, Tabled,
};

#[derive(Tabled)]
struct Job {
    title: String,
    #[tabled(display_with = "JobStatus::as_string")]
    status: JobStatus,
}

impl Job {
    fn new(title: &str, status: JobStatus) -> Self {
        Self {
            title: title.to_string(),
            status,
        }
    }
}

enum JobStatus {
    Open,
    Closed,
}

impl JobStatus {
    fn as_string(&self) -> &'static str {
        match self {
            JobStatus::Open => "open",
            JobStatus::Closed => "closed",
        }
    }
}

fn main() {
    let data = vec![
        Job::new("C Developer", JobStatus::Open),
        Job::new("Rust Developer", JobStatus::Closed),
        Job::new("Kernel Developer", JobStatus::Open),
    ];

    let mut table = Table::new(data);
    table
        .with(Style::empty())
        .modify(Locator::content("open"), Color::BG_WHITE | Color::FG_BLACK)
        .modify(
            Locator::content("closed"),
            Color::BG_GREEN | Color::FG_BLACK,
        );

    println!("{table}");
}
