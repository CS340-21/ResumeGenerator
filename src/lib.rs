mod html;
pub use html::HTML;
mod resume;
pub use resume::*;
mod theme;
pub use theme::Theme;

/// Use named colors instead of RGB so that theme can control actual color values
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Color {
    Red,
    Pink,
    Orange,
    Yellow,
    Green,
    Blue,
    Violet,
    Brown,
    Black,
    White,
    Grey,
    DefaultTitle,
    DefaultSectionTitle,
    DefaultSubtitle,
    DefaultForeground,
    DefaultBackground,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum HorizontalAlignment {
    Center,
    Left,
    Right,
    SameAsParent,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum VerticalAlignment {
    Center,
    Top,
    Bottom,
    SameAsParent,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Direction {
    Left,
    Right,
    Up,
    Down,
}
