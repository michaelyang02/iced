//! Display content in a table.
use std::borrow::Cow;

use iced_core::{Alignment, Length, Padding, Point, Rectangle};
use iced_core::alignment::{Horizontal, Vertical};
pub use iced_style::table::{Appearance, StyleSheet};
use row::Row;

use crate::{
    Clipboard, Element, event, Event, keyboard, Layout, overlay, renderer,
    Shell, Widget,
};
use crate::layout::{flex, Limits, Node};
use crate::widget::{Container, Operation, Tree};

mod row;

/// The [`Length`] of a [`Column`] width, without the [`Length::Shrink`] variant.
#[derive(Debug, Copy, Clone)]
pub enum TableLength {
    /// Equivalent to [`Length::Fill`].
    Fill,
    /// Equivalent to [`Length::FillPortion`].
    FillPortion(u16),
    /// Equivalent to [`Length::Fixed`].
    Fixed(f32),
}

impl From<TableLength> for Length {
    fn from(value: TableLength) -> Self {
        match value {
            TableLength::Fill => Self::Fill,
            TableLength::FillPortion(p) => Self::FillPortion(p),
            TableLength::Fixed(w) => Self::Fixed(w),
        }
    }
}

/// A [`Column`] of a [`Table`] widget.
#[derive(Debug, Copy, Clone)]
pub struct Column {
    /// The width of a [`Column`].
    pub width: TableLength,
    /// The [`Horizontal`] and [`Vertical`] alignment of the content of each cell in a [`Column`].
    pub alignment: (Horizontal, Vertical),
    /// The [`Padding`] around the content of each cell in a [`Column`].
    pub cell_padding: Padding,
}

struct Selected<'a, Message> {
    selected_rows: Cow<'a, [bool]>,
    on_selected: Box<dyn Fn(Vec<bool>) -> Message + 'a>,
}

/// A [`Widget`] that displays its content in the form of a table.
#[allow(missing_debug_implementations)]
pub struct Table<'a, Message, Renderer>
    where
        Renderer: crate::Renderer,
        Renderer::Theme: StyleSheet,
{
    columns: Vec<Column>,
    rows: Vec<Element<'a, Message, Renderer>>,

    fill_factor: u16,
    table_padding: Padding,

    selected: Option<Selected<'a, Message>>,

    header: Option<Element<'a, Message, Renderer>>,

    style: <Renderer::Theme as StyleSheet>::Style,
}

impl<'a, Message, Renderer> Default for Table<'a, Message, Renderer>
    where
        Message: 'a,
        Renderer: crate::Renderer + 'a,
        Renderer::Theme: StyleSheet,
{
    fn default() -> Self {
        Self::try_new(Vec::new(), Vec::new()).unwrap()
    }
}

impl<'a, Message, Renderer> Table<'a, Message, Renderer>
    where
        Renderer: crate::Renderer,
        Renderer::Theme: StyleSheet,
{
    /// Tries to create a new [`Table`] with the given list of [`Column`] and [`Row`].
    ///
    /// If the number of ([`Element`], height) pair in each row is equal to the number of [`Column`],
    /// return [`Ok(Table`].
    ///
    /// Otherwise, return [`Err(usize`] where the error value is the number of [`Column`].
    pub fn try_new(
        columns: Vec<Column>,
        rows: Vec<(Vec<Element<'a, Message, Renderer>>, f32)>,
    ) -> Result<Self, usize>
        where
            Message: 'a,
            Renderer: 'a,
    {
        Ok(Self {
            rows: {
                rows.into_iter()
                    .map(|(row, height)| {
                        if row.len() != columns.len() {
                            Err(columns.len())
                        } else {
                            Ok(Self::row(row, height, &columns).into())
                        }
                    })
                    .collect::<Result<Vec<_>, _>>()?
            },
            columns,
            fill_factor: 1,
            header: None,
            table_padding: Padding::ZERO,
            selected: None,
            style: Default::default(),
        })
    }

    /// Sets the width [`Length`] of a [`Table`] to [`Length::FillPortion(fill_factor)`].
    ///
    /// This is applicable only when at least one [`Column`] of the [`Table`] does
    /// not have a [`TableLength::Fixed`] width. Otherwise, an exact [`Length::Fixed`] width
    /// will be used for the [`Table`] width in layout.
    ///
    ///
    /// The default fill factor for a [`Table`] is 1.
    pub fn fill_factor(mut self, fill_factor: u16) -> Self {
        self.fill_factor = fill_factor;
        self
    }

    /// Sets the amount of [`Padding`] around the [`Table`] content.
    pub fn table_padding(mut self, padding: impl Into<Padding>) -> Self {
        self.table_padding = padding.into();
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
    /// If the number of [`Element`] in the header is equal to the number of [`Column`]
    /// of the [`Table`], return [`Ok(Table)`].
    ///
    /// Otherwise, return [`Err(usize)`], where the error value is the number of [`Column`] of the [`Table`].
    pub fn try_header<E>(
        self,
        header: (Vec<E>, f32),
    ) -> Result<Table<'a, Message, Renderer>, usize>
        where
            E: Into<Element<'a, Message, Renderer>>,
            Message: 'a,
            Renderer: 'a,
    {
        Ok(Table {
            fill_factor: self.fill_factor,
            header: {
                if header.0.len() != self.columns.len() {
                    return Err(self.columns.len());
                } else {
                    Some(Self::row(header.0, header.1, &self.columns).into())
                }
            },
            columns: self.columns,
            rows: self.rows,
            table_padding: self.table_padding,
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
        Renderer::Theme: StyleSheet,
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
        Row(
            row.into_iter()
                .zip(columns.iter())
                .map(|(e, c)| {
                    Container::new(e)
                        .width(Length::from(c.width))
                        .height(Length::Fixed(height))
                        .padding(c.cell_padding)
                        .align_x(c.alignment.0)
                        .align_y(c.alignment.1)
                        .into()
                })
                .collect(),
            height,
        )
    }
}

impl<'a, Message, Renderer> Widget<Message, Renderer>
for Table<'a, Message, Renderer>
    where
        Renderer: crate::Renderer,
        Renderer::Theme: StyleSheet,
{
    fn width(&self) -> Length {
        if self
            .columns
            .iter()
            .map(|c| c.width)
            .any(|w| !matches!(w, TableLength::Fixed(_)))
        {
            Length::FillPortion(self.fill_factor)
        } else {
            Length::Shrink
        }
    }

    fn height(&self) -> Length {
        Length::Shrink
    }

    fn layout(&self, renderer: &Renderer, limits: &Limits) -> Node {
        let limits = limits
            .pad(self.table_padding)
            .width(self.width())
            .height(self.height());
        flex::resolve(
            flex::Axis::Vertical,
            renderer,
            &limits,
            Padding::ZERO,
            0.0,
            Alignment::Start,
            &self.rows,
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
        self.rows.iter().zip(&tree.children).zip(layout.children())
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
        self.rows.iter().map(Tree::new).collect()
    }

    fn diff(&self, tree: &mut Tree) {
        tree.diff_children(&self.rows)
    }

    fn operate(
        &self,
        tree: &mut Tree,
        layout: Layout<'_>,
        renderer: &Renderer,
        operation: &mut dyn Operation<Message>,
    ) {
        operation.container(None, &mut |operation| {
            self.rows
                .iter()
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

        self.rows
            .iter_mut()
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
        overlay::from_children(&mut self.rows, state, layout, renderer)
    }
}

impl<'a, Message, Renderer> From<Table<'a, Message, Renderer>>
for Element<'a, Message, Renderer>
    where
        Message: 'a,
        Renderer: crate::Renderer + 'a,
        Renderer::Theme: StyleSheet,
{
    fn from(table: Table<'a, Message, Renderer>) -> Self {
        Self::new(table)
    }
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
