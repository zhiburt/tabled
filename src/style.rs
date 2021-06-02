use papergrid::{Border, Grid};

use crate::TableOption;

pub enum Style {
    Default,
    Psql,
    GithubMarkdown,
    Pseudo,
    PseudoClean,
    NoBorder,
}

impl Style {
    fn make(&self, border: &mut Border, row: usize, count_rows: usize) {
        match self {
            Style::Default => (),
            Style::NoBorder => Self::noborder_style(border),
            Style::GithubMarkdown => Self::github_markdown_style(border, row),
            Style::Pseudo => Self::pseudo_style(border, row, count_rows),
            Style::PseudoClean => Self::pseudo_clean_style(border, row, count_rows),
            Style::Psql => Self::psql_style(border, row),
        }
    }

    fn noborder_style(border: &mut Border) {
        border.empty().inner(Some(' '), None, None);
    }

    fn psql_style(border: &mut Border, row: usize) {
        if row == 0 {
            border
                .empty()
                .bottom('-', '+', None, None)
                .inner(Some('|'), None, None);
        } else {
            border.empty().inner(Some('|'), None, None);
        }
    }

    fn github_markdown_style(border: &mut Border, row: usize) {
        if row == 0 {
            border.empty().bottom('-', '+', Some('|'), Some('|')).inner(
                Some('|'),
                Some('|'),
                Some('|'),
            );
        } else {
            border.empty().inner(Some('|'), Some('|'), Some('|'));
        }
    }

    fn pseudo_style(border: &mut Border, row: usize, count_rows: usize) {
        if row == 0 {
            border
                .empty()
                .top('─', '┬', Some('┌'), Some('┐'))
                .bottom('─', '┼', Some('├'), Some('┤'))
                .inner(Some('│'), Some('│'), Some('│'));
        } else if row == count_rows - 1 {
            border.empty().bottom('─', '┴', Some('└'), Some('┘')).inner(
                Some('│'),
                Some('│'),
                Some('│'),
            );
        } else {
            border.empty().bottom('─', '┼', Some('├'), Some('┤')).inner(
                Some('│'),
                Some('│'),
                Some('│'),
            );
        }
    }

    fn pseudo_clean_style(border: &mut Border, row: usize, count_rows: usize) {
        if row == 0 {
            border
                .empty()
                .top('─', '┬', Some('┌'), Some('┐'))
                .bottom('─', '┼', Some('├'), Some('┤'))
                .inner(Some('│'), Some('│'), Some('│'));
        } else if row == count_rows - 1 {
            border.empty().bottom('─', '┴', Some('└'), Some('┘')).inner(
                Some('│'),
                Some('│'),
                Some('│'),
            );
        } else {
            border.empty().inner(Some('│'), Some('│'), Some('│'));
        }
    }
}

impl TableOption for Style {
    fn change(&self, grid: &mut Grid) {
        let count_rows = grid.count_rows();
        for row in 0..count_rows {
            let border = grid.get_border_mut(row);
            self.make(border, row, count_rows);
        }
    }
}
