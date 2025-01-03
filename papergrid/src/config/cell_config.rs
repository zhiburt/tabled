use crate::{
    ansi::ANSIBuf,
    config::{AlignmentHorizontal, AlignmentVertical, Formatting, Indent, Sides},
};

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct CellConfig {
    pub padding: Sides<Indent>,
    pub padding_color: Sides<Option<ANSIBuf>>,
    pub alignment_horizontal: AlignmentHorizontal,
    pub alignment_vertical: AlignmentVertical,
    pub formatting: Formatting,
    pub justification: char,
    pub justification_color: Option<ANSIBuf>,
}

impl CellConfig {
    pub fn new(
        padding: Sides<Indent>,
        padding_color: Sides<Option<ANSIBuf>>,
        alignment_horizontal: AlignmentHorizontal,
        alignment_vertical: AlignmentVertical,
        formatting: Formatting,
        justification: char,
        justification_color: Option<ANSIBuf>,
    ) -> Self {
        Self {
            padding,
            padding_color,
            alignment_horizontal,
            alignment_vertical,
            formatting,
            justification,
            justification_color,
        }
    }
}

impl Default for CellConfig {
    fn default() -> Self {
        Self {
            padding: Sides::default(),
            padding_color: Sides::default(),
            alignment_horizontal: AlignmentHorizontal::Left,
            alignment_vertical: AlignmentVertical::Top,
            formatting: Formatting::default(),
            justification: ' ',
            justification_color: None,
        }
    }
}
