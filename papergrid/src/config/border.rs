/// Border is a representation of a cells's borders (left, right, top, bottom, and the corners)
///
///
/// ```text
///                         top border
///                             |
///                             V
/// corner top left ------> +_______+  <---- corner top left
///                         |       |
/// left border ----------> |  cell |  <---- right border
///                         |       |
/// corner bottom right --> +_______+  <---- corner bottom right
///                             ^
///                             |
///                        bottom border
/// ```
#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, PartialOrd, Ord)]
pub struct Border<T> {
    /// A character for a top.
    pub top: Option<T>,
    /// A character for a bottom.
    pub bottom: Option<T>,
    /// A character for a left.
    pub left: Option<T>,
    /// A character for a right.
    pub right: Option<T>,
    /// A character for a left top corner.
    pub left_top_corner: Option<T>,
    /// A character for a left bottom corner.
    pub left_bottom_corner: Option<T>,
    /// A character for a right top corner.
    pub right_top_corner: Option<T>,
    /// A character for a right bottom corner.
    pub right_bottom_corner: Option<T>,
}

impl<T> Border<T> {
    /// This function constructs a cell borders with all sides set.
    #[allow(clippy::too_many_arguments)]
    pub const fn full(
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

    /// Checks whether any side is set.
    pub const fn is_empty(&self) -> bool {
        self.top.is_none()
            && self.left_top_corner.is_none()
            && self.right_top_corner.is_none()
            && self.bottom.is_none()
            && self.left_bottom_corner.is_none()
            && self.left_top_corner.is_none()
            && self.left.is_none()
            && self.right.is_none()
    }

    /// Verifies whether anything is set on the top.
    pub const fn has_top(&self) -> bool {
        self.top.is_some() || self.left_top_corner.is_some() || self.right_top_corner.is_some()
    }

    /// Verifies whether anything is set on the bottom.
    pub const fn has_bottom(&self) -> bool {
        self.bottom.is_some()
            || self.left_bottom_corner.is_some()
            || self.right_bottom_corner.is_some()
    }

    /// Verifies whether anything is set on the left.
    pub const fn has_left(&self) -> bool {
        self.left.is_some() || self.left_top_corner.is_some() || self.left_bottom_corner.is_some()
    }

    /// Verifies whether anything is set on the right.
    pub const fn has_right(&self) -> bool {
        self.right.is_some()
            || self.right_top_corner.is_some()
            || self.right_bottom_corner.is_some()
    }
}

impl<T: Copy> Border<T> {
    /// This function constructs a cell borders with all sides's char set to a given character.
    ///
    /// It behaves like [`Border::full`] with the same character set to each side.
    pub fn filled(c: T) -> Self {
        Self::full(c, c, c, c, c, c, c, c)
    }
}

impl<T: Copy> Border<&T> {
    /// This function constructs a cell borders with all sides's char set to a given character.
    ///
    /// It behaves like [`Border::full`] with the same character set to each side.
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

impl<T: Clone> Border<&T> {
    /// This function constructs a cell borders with all sides's char set to a given character.
    ///
    /// It behaves like [`Border::full`] with the same character set to each side.
    pub fn cloned(&self) -> Border<T> {
        Border {
            top: self.top.cloned(),
            bottom: self.bottom.cloned(),
            left: self.left.cloned(),
            right: self.right.cloned(),
            left_bottom_corner: self.left_bottom_corner.cloned(),
            left_top_corner: self.left_top_corner.cloned(),
            right_bottom_corner: self.right_bottom_corner.cloned(),
            right_top_corner: self.right_top_corner.cloned(),
        }
    }
}
