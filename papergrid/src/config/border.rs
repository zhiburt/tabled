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
    pub const fn new(
        top: Option<T>,
        bottom: Option<T>,
        left: Option<T>,
        right: Option<T>,
        left_top_corner: Option<T>,
        left_bottom_corner: Option<T>,
        right_top_corner: Option<T>,
        right_bottom_corner: Option<T>,
    ) -> Self {
        Self {
            top,
            bottom,
            left,
            right,
            left_top_corner,
            left_bottom_corner,
            right_top_corner,
            right_bottom_corner,
        }
    }

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
        Self::new(
            Some(top),
            Some(bottom),
            Some(left),
            Some(right),
            Some(top_left),
            Some(bottom_left),
            Some(top_right),
            Some(bottom_right),
        )
    }

    /// This function constructs a cell borders with all sides being empty (set off).
    pub const fn empty() -> Self {
        Self::new(None, None, None, None, None, None, None, None)
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

    /// Checks whether all sides are equal to one another.
    pub fn is_same(&self) -> bool
    where
        T: PartialEq,
    {
        self.top == self.bottom
            && self.top == self.left
            && self.top == self.right
            && self.top == self.left_top_corner
            && self.top == self.right_top_corner
            && self.top == self.left_bottom_corner
            && self.top == self.right_bottom_corner
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

    /// Converts a border with a given function.
    pub fn map<F, T1>(self, f: F) -> Border<T1>
    where
        F: Fn(T) -> T1,
    {
        Border {
            top: self.top.map(&f),
            bottom: self.bottom.map(&f),
            left: self.left.map(&f),
            right: self.right.map(&f),
            left_top_corner: self.left_top_corner.map(&f),
            left_bottom_corner: self.left_bottom_corner.map(&f),
            right_top_corner: self.right_top_corner.map(&f),
            right_bottom_corner: self.right_bottom_corner.map(&f),
        }
    }
}

impl<T: Copy> Border<T> {
    /// This function constructs a cell borders with all sides's char set to a given character.
    ///
    /// It behaves like [`Border::full`] with the same character set to each side.
    pub const fn filled(c: T) -> Self {
        Self::full(c, c, c, c, c, c, c, c)
    }
}

impl<T: Copy> Border<&T> {
    /// Copies the underlying reference to a new border.
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
    /// Copies the underlying reference to a new border.
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

impl<T> Border<T> {
    /// Convert all values on the border into another ones.
    pub fn convert<B>(self) -> Border<B>
    where
        B: From<T>,
    {
        macro_rules! conv_opt {
            ($opt:expr) => {
                match $opt {
                    Some(opt) => Some(B::from(opt)),
                    None => None,
                }
            };
        }

        Border {
            top: conv_opt!(self.top),
            bottom: conv_opt!(self.bottom),
            left: conv_opt!(self.left),
            right: conv_opt!(self.right),
            left_bottom_corner: conv_opt!(self.left_bottom_corner),
            left_top_corner: conv_opt!(self.left_top_corner),
            right_bottom_corner: conv_opt!(self.right_bottom_corner),
            right_top_corner: conv_opt!(self.right_top_corner),
        }
    }
}
