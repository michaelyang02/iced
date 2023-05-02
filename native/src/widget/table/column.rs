use iced_core::alignment::{Horizontal, Vertical};
use iced_core::Padding;

use super::length::Length;

/// A [`Column`] of a [`Table`] widget.
#[derive(Debug, Copy, Clone)]
pub struct Column {
    /// The width of a [`Column`].
    pub width: Length,
    /// The [`Horizontal`] and [`Vertical`] alignments of the content of each cell in a [`Column`].
    pub alignment: (Horizontal, Vertical),
    /// The [`Padding`] around the content of each cell in a [`Column`].
    pub cell_padding: Padding,
}