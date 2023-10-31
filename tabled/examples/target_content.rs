//! This example demonstrates using the [`Color`] [setting](tabled::settings) to
//! stylize text, backgrounds, and borders.
//!
//! * 🚩 This example requires the `color` feature.
//!
//! * Note how [`Format::content()`] is used to break out [`CellOption`]
//! specifications. This is helpful for organizing extensive [`Table`] configurations.

use tabled::{
    settings::{location::ByContent, style::Style, Color, Modify},
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

    let color_open_jobs = Color::BG_WHITE | Color::FG_BLACK;
    let color_closed_jobs = Color::BG_GREEN | Color::FG_BLACK;

    let mut table = Table::new(data);
    table
        .with(Style::empty())
        .with(Modify::list(ByContent::new("open"), color_open_jobs))
        .with(Modify::list(ByContent::new("closed"), color_closed_jobs));

    println!("{table}");
}
