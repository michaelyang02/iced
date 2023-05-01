use std::slice;
use iced_style::container;
use iced_style::table::StyleSheet;
use crate::Element;


/// An [`Iterator`] for all rows (incl. header) of a [`Table`].
#[allow(missing_debug_implementations)]
pub enum Iter<'a, 'b, Message, Renderer>
    where
        Renderer: crate::Renderer,
        Renderer::Theme: StyleSheet + container::StyleSheet,
{
    Header(std::iter::Chain<std::iter::Once<&'b Element<'a, Message, Renderer>>, slice::Iter<'b, Element<'a, Message, Renderer>>>),
    Content(slice::Iter<'b, Element<'a, Message, Renderer>>)
}

impl<'a, 'b, Message, Renderer> Clone for Iter<'a, 'b, Message, Renderer> where
    Renderer: crate::Renderer,
    Renderer::Theme: StyleSheet + container::StyleSheet, {
    fn clone(&self) -> Self {
        match self {
            Iter::Header(iter) => Iter::Header(iter.clone()),
            Iter::Content(iter) => Iter::Content(iter.clone()),
        }
    }
}

impl<'a, 'b, Message, Renderer> Iterator for Iter<'a, 'b, Message, Renderer>
    where
        Renderer: crate::Renderer,
        Renderer::Theme: StyleSheet + container::StyleSheet,
{
    type Item = &'b Element<'a, Message, Renderer>;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            Iter::Header(iter) => iter.next(),
            Iter::Content(iter) => iter.next(),
        }
    }
}

/// A mutable [`Iterator`] for all rows (incl. header) of a [`Table`].
#[allow(missing_debug_implementations)]
pub enum IterMut<'a, 'b, Message, Renderer>
    where
        Renderer: crate::Renderer,
        Renderer::Theme: StyleSheet + container::StyleSheet,
{
    Header(std::iter::Chain<std::iter::Once<&'b mut Element<'a, Message, Renderer>>, slice::IterMut<'b, Element<'a, Message, Renderer>>>),
    Content(slice::IterMut<'b, Element<'a, Message, Renderer>>)
}

impl<'a, 'b, Message, Renderer> Iterator for IterMut<'a, 'b, Message, Renderer>
    where
        Renderer: crate::Renderer,
        Renderer::Theme: StyleSheet + container::StyleSheet,
{
    type Item = &'b mut Element<'a, Message, Renderer>;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            IterMut::Header(iter) => iter.next(),
            IterMut::Content(iter) => iter.next(),
        }
    }
}