use core::ops::{Deref, DerefMut};

use papergrid::config::compact::CompactConfig;
use papergrid::config::AlignmentVertical;

/// A [`Table`] configuration.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct CompactMultilineConfig {
    config: CompactConfig,
    alignment_vertical: AlignmentVertical,
}

impl CompactMultilineConfig {
    /// Create a new colored config.
    pub fn new(config: CompactConfig) -> Self {
        Self::from(config)
    }

    /// Set a horizontal alignment.
    pub const fn set_alignment_vertical(mut self, alignment: AlignmentVertical) -> Self {
        self.alignment_vertical = alignment;
        self
    }

    /// Get a alignment horizontal.
    pub const fn get_alignment_vertical(&self) -> AlignmentVertical {
        self.alignment_vertical
    }
}

impl From<CompactConfig> for CompactMultilineConfig {
    fn from(config: CompactConfig) -> Self {
        Self {
            config,
            alignment_vertical: AlignmentVertical::Top,
        }
    }
}

impl Deref for CompactMultilineConfig {
    type Target = CompactConfig;

    fn deref(&self) -> &Self::Target {
        &self.config
    }
}

impl DerefMut for CompactMultilineConfig {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.config
    }
}

impl AsRef<CompactConfig> for CompactMultilineConfig {
    fn as_ref(&self) -> &CompactConfig {
        &self.config
    }
}

#[cfg(feature = "std")]
impl From<CompactMultilineConfig> for crate::grid::config::SpannedConfig {
    fn from(compact: CompactMultilineConfig) -> Self {
        use crate::grid::color::{AnsiColor, StaticColor};
        use crate::grid::config::{Borders, Entity::Global, HorizontalLine, Indent, Offset, Sides};
        use papergrid::config::spanned::{ColoredIndent, ColoredMarginIndent};

        fn to_margin(
            pad: &Sides<Indent>,
            colors: Sides<StaticColor>,
        ) -> Sides<ColoredMarginIndent> {
            let colors = to_ansi_color(colors);
            Sides::new(
                ColoredMarginIndent::new(pad.left, Offset::Begin(0), Some(colors.left)),
                ColoredMarginIndent::new(pad.right, Offset::Begin(0), Some(colors.right)),
                ColoredMarginIndent::new(pad.top, Offset::Begin(0), Some(colors.top)),
                ColoredMarginIndent::new(pad.bottom, Offset::Begin(0), Some(colors.bottom)),
            )
        }

        fn to_padding(pad: &Sides<Indent>, colors: Sides<StaticColor>) -> Sides<ColoredIndent> {
            let colors = to_ansi_color(colors);
            Sides::new(
                ColoredIndent::new(pad.left, Some(colors.left)),
                ColoredIndent::new(pad.right, Some(colors.right)),
                ColoredIndent::new(pad.top, Some(colors.top)),
                ColoredIndent::new(pad.bottom, Some(colors.bottom)),
            )
        }

        fn to_ansi_color(b: Sides<StaticColor>) -> Sides<AnsiColor<'static>> {
            Sides::new(b.left.into(), b.right.into(), b.top.into(), b.bottom.into())
        }

        fn borders_static_color_to_ansi_color(
            b: Borders<StaticColor>,
        ) -> Borders<AnsiColor<'static>> {
            Borders {
                left: b.left.map(|c| c.into()),
                right: b.right.map(|c| c.into()),
                top: b.top.map(|c| c.into()),
                bottom: b.bottom.map(|c| c.into()),
                bottom_intersection: b.bottom_intersection.map(|c| c.into()),
                bottom_left: b.bottom_left.map(|c| c.into()),
                bottom_right: b.bottom_right.map(|c| c.into()),
                horizontal: b.horizontal.map(|c| c.into()),
                intersection: b.intersection.map(|c| c.into()),
                left_intersection: b.left_intersection.map(|c| c.into()),
                right_intersection: b.right_intersection.map(|c| c.into()),
                top_intersection: b.top_intersection.map(|c| c.into()),
                top_left: b.top_left.map(|c| c.into()),
                top_right: b.top_right.map(|c| c.into()),
                vertical: b.vertical.map(|c| c.into()),
            }
        }

        let mut cfg = crate::grid::config::SpannedConfig::default();

        let pad = to_padding(compact.get_padding(), compact.get_padding_color());
        cfg.set_padding(Global, pad);
        *cfg.get_margin_mut() = to_margin(compact.get_margin(), compact.get_margin_color());
        cfg.set_alignment_horizontal(Global, compact.get_alignment_horizontal());
        cfg.set_alignment_vertical(Global, compact.get_alignment_vertical());
        cfg.set_borders(*compact.get_borders());
        cfg.set_borders_color(borders_static_color_to_ansi_color(
            *compact.get_borders_color(),
        ));

        if let Some(line) = compact.get_first_horizontal_line() {
            cfg.insert_horizontal_line(
                1,
                HorizontalLine {
                    intersection: line.intersection,
                    left: line.connect1,
                    right: line.connect2,
                    main: Some(line.main),
                },
            );
        }

        cfg
    }
}
