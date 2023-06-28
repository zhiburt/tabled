use papergrid::{
    color::AnsiColor,
    config::{Entity, Sides},
};

use crate::{
    grid::{
        config::ColoredConfig,
        records::{ExactRecords, Records},
    },
    settings::{object::Object, Color, TableOption},
};

/// [`Colorization`] sets a color for the whole table data (so it's not include the borders).
///
/// You can colorize borders in a different round using [`BorderColor`] or [`RawStyle`]
///
/// # Examples
///
/// ```
/// use std::iter::FromIterator;
///
/// use tabled::builder::Builder;
/// use tabled::settings::{style::BorderColor, themes::Colorization, Color, Style};
///
/// let data = [["Hello", "World"], ["Hi", "World"], ["Halo", "World"]];
///
/// let color1 = Color::FG_BLACK | Color::BG_WHITE;
/// let color2 = Color::BG_BLACK | Color::FG_WHITE;
/// let color3 = Color::FG_RED | Color::BG_RED;
///
/// let mut table = Builder::from_iter(data).build();
/// table
///     .with(Colorization::chess(color1, color2))
///     .with(Style::modern())
///     .with(BorderColor::filled(color3));
///
/// println!("{table}");
/// ```
///
/// [`RawStyle`]: crate::settings::style::RawStyle
/// [`BorderColor`]: crate::settings::style::BorderColor
#[derive(Debug, Clone)]
pub struct Colorization {
    pattern: ColorizationPattern,
    colors: Vec<Color>,
}

#[derive(Debug, Clone)]
enum ColorizationPattern {
    Column,
    Row,
    ByRow,
    ByColumn,
    Chess,
}

impl Colorization {
    /// Creates a [`Colorization`] with a chess pattern.
    ///
    /// ```
    /// use std::iter::FromIterator;
    ///
    /// use tabled::builder::Builder;
    /// use tabled::settings::{themes::Colorization, Color, Style};
    ///
    /// let data = [["Hello", "World"], ["Hi", "World"], ["Halo", "World"]];
    ///
    /// let color1 = Color::FG_BLACK | Color::BG_WHITE;
    /// let color2 = Color::BG_BLACK | Color::FG_WHITE;
    ///
    /// let mut table = Builder::from_iter(data).build();
    /// table
    ///     .with(Colorization::chess(color1, color2))
    ///     .with(Style::empty());
    ///
    /// println!("{table}");
    /// ```
    pub fn chess(white: Color, black: Color) -> Self {
        Self::new(vec![white, black], ColorizationPattern::Chess)
    }

    /// Creates a [`Colorization`] with a target [`Object`].
    ///
    /// ```
    /// use std::iter::FromIterator;
    ///
    /// use tabled::builder::Builder;
    /// use tabled::settings::object::Rows;
    /// use tabled::settings::{themes::Colorization, Color, Style};
    ///
    /// let data = [["Hello", "World"], ["Hi", "World"], ["Halo", "World"]];
    ///
    /// let color1 = Color::FG_BLACK | Color::BG_WHITE;
    /// let color2 = Color::BG_BLACK | Color::FG_WHITE;
    ///
    /// let mut table = Builder::from_iter(data).build();
    /// table
    ///     .with(Colorization::exact([color1, color2], Rows::first()))
    ///     .with(Style::empty());
    ///
    /// println!("{table}");
    /// ```
    pub fn exact<I, O>(colors: I, target: O) -> ExactColorization<O>
    where
        I: IntoIterator,
        I::Item: Into<Color>,
    {
        let colors = colors.into_iter().map(Into::into).collect();
        ExactColorization::new(colors, target)
    }

    /// Creates a [`Colorization`] with a pattern which changes row by row.
    ///
    /// ```
    /// use std::iter::FromIterator;
    ///
    /// use tabled::builder::Builder;
    /// use tabled::settings::object::Rows;
    /// use tabled::settings::{themes::Colorization, Color, Style};
    ///
    /// let data = [["Hello", "World"], ["Hi", "World"], ["Halo", "World"]];
    ///
    /// let color1 = Color::FG_BLACK | Color::BG_WHITE;
    /// let color2 = Color::BG_BLACK | Color::FG_WHITE;
    ///
    /// let mut table = Builder::from_iter(data).build();
    /// table
    ///     .with(Colorization::rows([color1, color2]))
    ///     .with(Style::empty());
    ///
    /// println!("{table}");
    /// ```
    pub fn rows<I>(colors: I) -> Self
    where
        I: IntoIterator,
        I::Item: Into<Color>,
    {
        Self::new(colors, ColorizationPattern::Row)
    }

    /// Creates a [`Colorization`] with a pattern which changes column by column.
    ///
    /// ```
    /// use std::iter::FromIterator;
    ///
    /// use tabled::builder::Builder;
    /// use tabled::settings::object::Rows;
    /// use tabled::settings::{themes::Colorization, Color, Style};
    ///
    /// let data = [["Hello", "World"], ["Hi", "World"], ["Halo", "World"]];
    ///
    /// let color1 = Color::FG_BLACK | Color::BG_WHITE;
    /// let color2 = Color::BG_BLACK | Color::FG_WHITE;
    ///
    /// let mut table = Builder::from_iter(data).build();
    /// table
    ///     .with(Colorization::columns([color1, color2]))
    ///     .with(Style::empty());
    ///
    /// println!("{table}");
    /// ```
    pub fn columns<I>(colors: I) -> Self
    where
        I: IntoIterator,
        I::Item: Into<Color>,
    {
        Self::new(colors, ColorizationPattern::Column)
    }

    /// Creates a [`Colorization`] with a pattern which peaks cells one by one iterating over rows.
    ///
    /// ```
    /// use std::iter::FromIterator;
    ///
    /// use tabled::builder::Builder;
    /// use tabled::settings::object::Rows;
    /// use tabled::settings::{themes::Colorization, Color, Style};
    ///
    /// let data = [["Hello", "World"], ["Hi", "World"], ["Halo", "World"]];
    ///
    /// let color1 = Color::FG_BLACK | Color::BG_WHITE;
    /// let color2 = Color::BG_BLACK | Color::FG_WHITE;
    ///
    /// let mut table = Builder::from_iter(data).build();
    /// table
    ///     .with(Colorization::by_row([color1, color2]))
    ///     .with(Style::empty());
    ///
    /// println!("{table}");
    /// ```
    pub fn by_row<I>(colors: I) -> Self
    where
        I: IntoIterator,
        I::Item: Into<Color>,
    {
        Self::new(colors, ColorizationPattern::ByRow)
    }

    /// Creates a [`Colorization`] with a pattern which peaks cells one by one iterating over columns.
    ///
    /// ```
    /// use std::iter::FromIterator;
    ///
    /// use tabled::builder::Builder;
    /// use tabled::settings::object::Rows;
    /// use tabled::settings::{themes::Colorization, Color, Style};
    ///
    /// let data = [["Hello", "World"], ["Hi", "World"], ["Halo", "World"]];
    ///
    /// let color1 = Color::FG_BLACK | Color::BG_WHITE;
    /// let color2 = Color::BG_BLACK | Color::FG_WHITE;
    ///
    /// let mut table = Builder::from_iter(data).build();
    /// table
    ///     .with(Colorization::by_column([color1, color2]))
    ///     .with(Style::empty());
    ///
    /// println!("{table}");
    /// ```
    pub fn by_column<I>(colors: I) -> Self
    where
        I: IntoIterator,
        I::Item: Into<Color>,
    {
        Self::new(colors, ColorizationPattern::ByColumn)
    }

    fn new<I>(colors: I, pattern: ColorizationPattern) -> Self
    where
        I: IntoIterator,
        I::Item: Into<Color>,
    {
        let colors = colors.into_iter().map(Into::into).collect();
        Self { colors, pattern }
    }
}

impl<R, D> TableOption<R, D, ColoredConfig> for Colorization
where
    R: Records + ExactRecords,
{
    fn change(self, records: &mut R, cfg: &mut ColoredConfig, _: &mut D) {
        if self.colors.is_empty() {
            return;
        }

        let count_columns = records.count_columns();
        let count_rows = records.count_rows();

        match self.pattern {
            ColorizationPattern::Column => colorize_columns(&self.colors, count_columns, cfg),
            ColorizationPattern::Row => colorize_rows(&self.colors, count_rows, cfg),
            ColorizationPattern::ByRow => {
                colorize_by_row(&self.colors, count_rows, count_columns, cfg)
            }
            ColorizationPattern::ByColumn => {
                colorize_by_column(&self.colors, count_rows, count_columns, cfg)
            }
            ColorizationPattern::Chess => {
                colorize_diogonals(&self.colors, count_rows, count_columns, cfg)
            }
        }
    }
}

fn colorize_columns(colors: &[Color], count_columns: usize, cfg: &mut ColoredConfig) {
    for (col, color) in (0..count_columns).zip(colors.iter().cycle()) {
        colorize_entity(color, Entity::Column(col), cfg);
    }
}

fn colorize_rows(colors: &[Color], count_rows: usize, cfg: &mut ColoredConfig) {
    for (row, color) in (0..count_rows).zip(colors.iter().cycle()) {
        colorize_entity(color, Entity::Row(row), cfg);
    }
}

fn colorize_by_row(
    colors: &[Color],
    count_rows: usize,
    count_columns: usize,
    cfg: &mut ColoredConfig,
) {
    let mut color_peek = colors.iter().cycle();
    for row in 0..count_rows {
        for col in 0..count_columns {
            let color = color_peek.next().unwrap();
            colorize_entity(color, Entity::Cell(row, col), cfg);
        }
    }
}

fn colorize_by_column(
    colors: &[Color],
    count_rows: usize,
    count_columns: usize,
    cfg: &mut ColoredConfig,
) {
    let mut color_peek = colors.iter().cycle();
    for col in 0..count_columns {
        for row in 0..count_rows {
            let color = color_peek.next().unwrap();
            colorize_entity(color, Entity::Cell(row, col), cfg);
        }
    }
}

fn colorize_diogonals(
    colors: &[Color],
    count_rows: usize,
    count_columns: usize,
    cfg: &mut ColoredConfig,
) {
    let mut color_peek = colors.iter().cycle();
    for mut row in 0..count_rows {
        let color = color_peek.next().unwrap();
        for col in 0..count_columns {
            colorize_entity(color, Entity::Cell(row, col), cfg);

            row += 1;
            if row == count_rows {
                break;
            }
        }
    }

    let _ = color_peek.next().unwrap();

    for mut col in 1..count_columns {
        let color = color_peek.next().unwrap();
        for row in 0..count_rows {
            colorize_entity(color, Entity::Cell(row, col), cfg);

            col += 1;
            if col == count_columns {
                break;
            }
        }
    }
}

fn colorize_entity(color: &Color, pos: Entity, cfg: &mut ColoredConfig) {
    let ansi_color = AnsiColor::from(color.clone());
    let _ = cfg.set_color(pos, ansi_color.clone());
    cfg.set_justification_color(pos, Some(ansi_color.clone()));
    cfg.set_padding_color(
        pos,
        Sides::new(
            Some(ansi_color.clone()),
            Some(ansi_color.clone()),
            Some(ansi_color.clone()),
            Some(ansi_color),
        ),
    );
}

/// A colorization of a target [`Object`].
///
/// Can be created by [`Colorization::exact`].
#[derive(Debug, Clone)]
pub struct ExactColorization<O> {
    colors: Vec<Color>,
    target: O,
}

impl<O> ExactColorization<O> {
    fn new(colors: Vec<Color>, target: O) -> Self {
        Self { colors, target }
    }
}

impl<R, D, O> TableOption<R, D, ColoredConfig> for ExactColorization<O>
where
    O: Object<R>,
{
    fn change(self, records: &mut R, cfg: &mut ColoredConfig, _: &mut D) {
        if self.colors.is_empty() {
            return;
        }

        let mut color_peek = self.colors.iter().cycle();
        for pos in self.target.cells(records) {
            let color = color_peek.next().unwrap();
            colorize_entity(color, pos, cfg);
        }
    }
}
