/// Border is a representation of a cells's borders (left, right, top, bottom, and the corners)
#[derive(Debug, Clone, Default, Eq, PartialEq)]
pub struct Border<T = char> {
    pub top: Option<T>,
    pub bottom: Option<T>,
    pub left: Option<T>,
    pub right: Option<T>,
    pub left_top_corner: Option<T>,
    pub left_bottom_corner: Option<T>,
    pub right_top_corner: Option<T>,
    pub right_bottom_corner: Option<T>,
}

impl<T> Border<T> {
    /// This function constructs a cell borders with all sides set.
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        top: T,
        bottom: T,
        left: T,
        right: T,
        top_left: T,
        top_right: T,
        bottom_left: T,
        bottom_right: T,
    ) -> Self {
        Self {
            top: Some(top),
            bottom: Some(bottom),
            right: Some(right),
            right_top_corner: Some(top_right),
            right_bottom_corner: Some(bottom_right),
            left: Some(left),
            left_bottom_corner: Some(bottom_left),
            left_top_corner: Some(top_left),
        }
    }
}

impl<T: Copy> Border<T> {
    /// This function constructs a cell borders with all sides's char set to a given character.
    /// It behaives like [`Border::new`] with the same character set to each side.
    pub fn filled(c: T) -> Self {
        Self::new(c, c, c, c, c, c, c, c)
    }
}

impl<T: Copy> Border<&T> {
    /// This function constructs a cell borders with all sides's char set to a given character.
    /// It behaives like [`Border::new`] with the same character set to each side.
    pub fn copied(&self) -> Border<T> {
        Border {
            top: self.top.copied(),
            bottom: self.bottom.copied(),
            left: self.left.copied(),
            right: self.right.copied(),
            left_bottom_corner: self.left_bottom_corner.copied(),
            left_top_corner: self.left_top_corner.copied(),
            right_bottom_corner: self.right_bottom_corner.copied(),
            right_top_corner: self.right_top_corner.copied(),
        }
    }
}
