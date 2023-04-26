
/// The [`Length`] of a [`Column`] width, without the [`Length::Shrink`] variant.
#[derive(Debug, Copy, Clone)]
pub enum Length {
    /// Equivalent to [`Length::Fill`].
    Fill,
    /// Equivalent to [`Length::FillPortion`].
    FillPortion(u16),
    /// Equivalent to [`Length::Fixed`].
    Fixed(f32),
}

impl From<Length> for iced_core::Length {
    fn from(value: Length) -> Self {
        match value {
            Length::Fill => Self::Fill,
            Length::FillPortion(p) => Self::FillPortion(p),
            Length::Fixed(w) => Self::Fixed(w),
        }
    }
}