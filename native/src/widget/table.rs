//! Display content in a table.
use std::borrow::Cow;

pub use column::Column as Column;
use iced_core::{Alignment, Padding, Point, Rectangle, Size};
use iced_style::container;
pub use iced_style::table::{Appearance, StyleSheet};
pub use length::Length as Length;
pub use row::Row as Row;

use crate::{
    Clipboard, Element, event, Event, keyboard, Layout, overlay, renderer,
    Shell, Widget,
};
use crate::layout::{flex, Limits, Node};
use crate::widget::{Container, Operation, Tree};

mod column;
mod length;
mod row;
mod iter;

/// A [`Widget`] that displays its content in the form of a table.
#[allow(missing_debug_implementations)]
pub struct Table<'a, Message, Renderer>
    where
        Renderer: crate::Renderer,
        Renderer::Theme: StyleSheet + container::StyleSheet,
{
    columns: Vec<Column>,
    rows: Vec<Element<'a, Message, Renderer>>,

    fill_factor: u16,
    padding: Padding,

    selected: Option<Selected<'a, Message>>,

    header: Option<Element<'a, Message, Renderer>>,

    style: <Renderer::Theme as StyleSheet>::Style,
}

impl<'a, Message, Renderer> Default for Table<'a, Message, Renderer>
    where
        Message: 'a,
        Renderer: crate::Renderer + 'a,
        Renderer::Theme: StyleSheet + container::StyleSheet,
{
    fn default() -> Self {
        Self::try_new(Vec::new(), Vec::new()).unwrap()
    }
}

impl<'a, Message, Renderer> Table<'a, Message, Renderer>
    where
        Renderer: crate::Renderer,
        Renderer::Theme: StyleSheet + container::StyleSheet,
{
    /// Tries to create a new [`Table`] with the given list of [`Column`] and [`Row`].
    ///
    /// If the number of ([`Element`], height) pair in each row is equal to the number of [`Column`],
    /// return [`Ok(Table)`].
    ///
    /// Otherwise, return [`Err(usize)`] where the error value is the number of [`Column`].
    pub fn try_new(
        columns: Vec<Column>,
        rows: Vec<Row<'a, Message, Renderer>>,
    ) -> Result<Self, usize>
        where
            Message: 'a,
            Renderer: 'a,
    {
        Ok(Self {
            rows: {
                rows.into_iter()
                    .map(|Row { cells, height }| {
                        if cells.len() != columns.len() {
                            Err(columns.len())
                        } else {
                            Ok(Self::row(cells, height, &columns).into())
                        }
                    })
                    .collect::<Result<Vec<_>, _>>()?
            },
            columns,
            fill_factor: 1,
            header: None,
            padding: Padding::ZERO,
            selected: None,
            style: Default::default(),
        })
    }

    /// Sets the width [`Length`] of a [`Table`] to [`Length::FillPortion(fill_factor)`].
    ///
    /// This is applicable only when at least one [`Column`] of the [`Table`] does
    /// not have a [`Length::Fixed`] width. Otherwise, an exact [`Length::Fixed`] width
    /// will be used for the [`Table`] width in layout.
    ///
    ///
    /// The default fill factor for a [`Table`] is 1.
    pub fn fill_factor(mut self, fill_factor: u16) -> Self {
        self.fill_factor = fill_factor;
        self
    }

    /// Sets the amount of [`Padding`] around the [`Table`] content.
    pub fn padding(mut self, padding: impl Into<Padding>) -> Self {
        self.padding = padding.into();
        self
    }

    /// Tries to set the behaviour when the list of selected [`Row`] of the [`Table`] is changed.
    ///
    /// * `selected_rows` - a [`bool`] slice corresponding to whether each row is selected
    /// * `on_selected` - the message to produce given the changed list of selected [`Row`]
    ///
    /// If the length of `selected_rows` is equal to the number of [`Row`] of the [`Table`],
    /// return [`Ok(Table)`].
    ///
    /// Otherwise, return [`Err(usize)`], where the error value is the number of [`Row`] of the [`Table`].
    pub fn try_selected(
        mut self,
        selected_rows: &'a [bool],
        on_selected: impl Fn(Vec<bool>) -> Message + 'a,
    ) -> Result<Self, usize> {
        if selected_rows.len() != self.rows.len() {
            Err(self.rows.len())
        } else {
            self.selected = Some(Selected {
                selected_rows: Cow::Borrowed(selected_rows),
                on_selected: Box::new(on_selected),
            });
            Ok(self)
        }
    }

    /// Tries to set the header of the [`Table`].
    ///
    /// If the number of [`Element`] in the header [`Row`] is equal to the number of [`Column`]
    /// of the [`Table`], return [`Ok(Table)`].
    ///
    /// Otherwise, return [`Err(usize)`], where the error value is the number of [`Column`] of the [`Table`].
    pub fn try_header(
        self,
        header: Row<'a, Message, Renderer>,
    ) -> Result<Table<'a, Message, Renderer>, usize>
        where
            Message: 'a,
            Renderer: 'a,
    {
        Ok(Table {
            fill_factor: self.fill_factor,
            header: {
                if header.cells.len() != self.columns.len() {
                    return Err(self.columns.len());
                } else {
                    Some(Self::row(header.cells, header.height, &self.columns).into())
                }
            },
            columns: self.columns,
            rows: self.rows,
            padding: self.padding,
            selected: self.selected,
            style: self.style,
        })
    }

    /// Sets the style of the [`Table`].
    pub fn style(
        mut self,
        style: impl Into<<Renderer::Theme as StyleSheet>::Style>,
    ) -> Self {
        self.style = style.into();
        self
    }
}

impl<'a, Message, Renderer> Table<'a, Message, Renderer>
    where
        Renderer: crate::Renderer,
        Renderer::Theme: StyleSheet + container::StyleSheet,
{
    fn row<'b, E>(
        row: Vec<E>,
        height: f32,
        columns: &'_ [Column],
    ) -> Row<'b, Message, Renderer>
        where
            E: Into<Element<'b, Message, Renderer>>,
            Message: 'b,
            Renderer: 'b,
    {
        Row {
            cells: row.into_iter()
                .zip(columns.iter())
                .map(|(e,
                          c)| {
                    Container::new(e)
                        .width(iced_core::Length::from(c.width))
                        .height(iced_core::Length::Fixed(height))
                        .padding(c.cell_padding)
                        .align_x(c.alignment.0)
                        .align_y(c.alignment.1)
                        .into()
                })
                .collect(),
            height,
        }
    }

    fn len(&self) -> usize {
        self.rows.len() + self.header.is_some() as usize
    }
}

impl<'a, 'b, Message: 'a, Renderer: 'a> IntoIterator for &'b Table<'a, Message, Renderer>
    where
        Renderer: crate::Renderer,
        Renderer::Theme: StyleSheet + container::StyleSheet,
{
    type Item = &'b Element<'a, Message, Renderer>;
    type IntoIter = iter::Iter<'a, 'b, Message, Renderer>;

    fn into_iter(self) -> Self::IntoIter {
        match &self.header {
            Some(header) => iter::Iter::Header(std::iter::once(header).chain(self.rows.iter())),
            None => iter::Iter::Content(self.rows.iter()),
        }
    }
}

impl<'a, 'b, Message: 'a, Renderer: 'a> IntoIterator for &'b mut Table<'a, Message, Renderer>
    where
        Renderer: crate::Renderer,
        Renderer::Theme: StyleSheet + container::StyleSheet,
{
    type Item = &'b mut Element<'a, Message, Renderer>;
    type IntoIter = iter::IterMut<'a, 'b, Message, Renderer>;

    fn into_iter(self) -> Self::IntoIter {
        match &mut self.header {
            Some(header) => iter::IterMut::Header(std::iter::once(header).chain(self.rows.iter_mut())),
            None => iter::IterMut::Content(self.rows.iter_mut()),
        }
    }
}


impl<'a, Message: 'a, Renderer: 'a> Widget<Message, Renderer>
for Table<'a, Message, Renderer>
    where
        Renderer: crate::Renderer,
        Renderer::Theme: StyleSheet + container::StyleSheet,
{
    fn width(&self) -> iced_core::Length {
        if self
            .columns
            .iter()
            .map(|c| c.width)
            .any(|w| !matches!(w, Length::Fixed(_)))
        {
            iced_core::Length::FillPortion(self.fill_factor)
        } else {
            iced_core::Length::Shrink
        }
    }

    fn height(&self) -> iced_core::Length {
        iced_core::Length::Shrink
    }

    fn layout(&self, renderer: &Renderer, limits: &Limits) -> Node {
        let limits = limits
            .width(self.width())
            .height(self.height());

        flex::resolve_iter(
            flex::Axis::Vertical,
            renderer,
            &limits,
            self.padding,
            0.0,
            Alignment::Start,
            self,
            self.len() + self.header.is_some() as usize,
        )
    }

    fn draw(
        &self,
        tree: &Tree,
        renderer: &mut Renderer,
        theme: &Renderer::Theme,
        style: &renderer::Style,
        layout: Layout<'_>,
        cursor_position: Point,
        viewport: &Rectangle,
    ) {
        for ((row, state), layout) in
        self.into_iter().zip(&tree.children).zip(layout.children())
        {
            row.as_widget().draw(
                state,
                renderer,
                theme,
                style,
                layout,
                cursor_position,
                viewport,
            );
        }
    }

    fn children(&self) -> Vec<Tree> {
        self.into_iter().map(Tree::new).collect()
    }

    fn diff(&self, tree: &mut Tree) {
        tree.diff_children_iter(
            self,
            |tree, element| tree.diff(element.as_widget()),
            |element| Tree::new(element.as_widget()),
        );
    }

    fn operate(
        &self,
        tree: &mut Tree,
        layout: Layout<'_>,
        renderer: &Renderer,
        operation: &mut dyn Operation<Message>,
    ) {
        operation.container(None, &mut |operation| {
            self.into_iter()
                .zip(&mut tree.children)
                .zip(layout.children())
                .for_each(|((row, state), layout)| {
                    row.as_widget().operate(state, layout, renderer, operation);
                });
        });
    }

    fn on_event(
        &mut self,
        tree: &mut Tree,
        event: Event,
        layout: Layout<'_>,
        cursor_position: Point,
        renderer: &Renderer,
        clipboard: &mut dyn Clipboard,
        shell: &mut Shell<'_, Message>,
    ) -> event::Status {
        let table_status = update(
            event.clone(),
            layout,
            cursor_position,
            shell,
            self.selected.as_ref().map(|s| s.on_selected.as_ref()),
            || tree.state.downcast_mut::<State>(),
        );

        self.into_iter()
            .zip(&mut tree.children)
            .zip(layout.children())
            .map(|((row, state), layout)| {
                row.as_widget_mut().on_event(
                    state,
                    event.clone(),
                    layout,
                    cursor_position,
                    renderer,
                    clipboard,
                    shell,
                )
            })
            .fold(table_status, event::Status::merge)
    }

    fn overlay<'b>(
        &'b mut self,
        state: &'b mut Tree,
        layout: Layout<'_>,
        renderer: &Renderer,
    ) -> Option<overlay::Element<'b, Message, Renderer>> {
        overlay::from_children_iter(self, state, layout, renderer)
    }
}

impl<'a, Message, Renderer> From<Table<'a, Message, Renderer>>
for Element<'a, Message, Renderer>
    where
        Message: 'a,
        Renderer: crate::Renderer + 'a,
        Renderer::Theme: StyleSheet + container::StyleSheet,
{
    fn from(table: Table<'a, Message, Renderer>) -> Self {
        Self::new(table)
    }
}

struct Selected<'a, Message> {
    selected_rows: Cow<'a, [bool]>,
    on_selected: Box<dyn Fn(Vec<bool>) -> Message + 'a>,
}

/// The local state of a [`Table`].
#[derive(Debug, Copy, Clone, Default)]
pub struct State {
    keyboard_modifiers: keyboard::Modifiers,
}

impl State {
    /// Creates a new [`State`].
    pub fn new() -> State {
        State::default()
    }
}

/// Processes the given [`Event`] and updates the [`State`] of a [`Table`]
/// accordingly.
pub fn update<'a, Message>(
    event: Event,
    layout: Layout<'_>,
    cursor_position: Point,
    shell: &mut Shell<'_, Message>,
    on_selected: Option<&(dyn Fn(Vec<bool>) -> Message + 'a)>,
    state: impl FnOnce() -> &'a mut State,
) -> event::Status {
    event::Status::Ignored
}
