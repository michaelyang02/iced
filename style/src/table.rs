//! Change the appearance of a table.
use iced_core::{Background, Color};

/// The appearance of a table.
#[derive(Debug, Clone, Copy)]
pub struct Appearance {
    /// The background [`Color`] of the table.
    pub background: Background,

    /// The vertical border width between columns of the table.
    pub vertical_border_width: f32,
    /// The vertical border [`Color`] between columns of the table.
    pub vertical_border_color: Color,

    /// The horizontal border width between rows of the table.
    pub horizontal_border_width: f32,
    /// The horizontal border [`Color`] between rows of the table.
    pub horizontal_border_color: Color,

    /// The outside border radius of the table.
    pub border_radius: f32,
    /// The outside border width of the table.
    pub border_width: f32,
    /// The outside border [`Color`] of the table.
    pub border_color: Color,
}

/// A set of rules that dictate the style of a table.
pub trait StyleSheet {
    /// The supported style of the [`StyleSheet`].
    type Style: Default;

    /// Produces the active [`Appearance`] of a table.
    fn active(&self, style: &Self::Style) -> Appearance;

    /// Produces the background [`Color`] of the header of a table.
    fn header_background(&self, style: &Self::Style) -> Background {
        self.active(style).background
    }

    /// Produces the background [`Color`] of a striped row of a table.
    fn striped_background(&self, style: &Self::Style) -> Background {
        self.active(style).background
    }

    /// Produces the background [`Color`] of a selected row of a table.
    fn selected_background(&self, style: &Self::Style) -> Background {
        self.active(style).background
    }
}
