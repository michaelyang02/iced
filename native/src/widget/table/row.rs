use iced_core::{Alignment, Length, Padding, Point, Rectangle, Size};
use iced_style::table::StyleSheet;

use crate::layout::flex::Axis;
use crate::layout::{flex, Limits, Node};
use crate::renderer::Style;
use crate::widget::{Operation, Tree};
use crate::{event, overlay, Clipboard, Element, Event, Layout, Shell, Widget};

mod empty {
    use super::*;

    pub(super) struct Empty {}

    impl<Message, Renderer> Widget<Message, Renderer> for Empty
    where
        Renderer: crate::Renderer,
    {
        fn width(&self) -> Length {
            Length::Shrink
        }

        fn height(&self) -> Length {
            Length::Shrink
        }

        fn layout(&self, _renderer: &Renderer, _limits: &Limits) -> Node {
            Node::new(Size::ZERO)
        }

        fn draw(
            &self,
            _tree: &Tree,
            _renderer: &mut Renderer,
            _theme: &Renderer::Theme,
            _style: &Style,
            _layout: Layout<'_>,
            _cursor_position: Point,
            _viewport: &Rectangle,
        ) {
        }
    }

    impl<Message, Renderer> From<Empty> for Element<'_, Message, Renderer>
    where
        Renderer: crate::Renderer,
    {
        fn from(empty: Empty) -> Self {
            Self::new(empty)
        }
    }
}

/// A [`Row`] of a [`Table`] widget.
#[allow(missing_debug_implementations)]
pub struct Row<'a, Message, Renderer>
where
    Renderer: crate::Renderer,
    Renderer::Theme: StyleSheet,
{
    /// The cells of a [`Row`].
    pub(super) cells: Vec<Element<'a, Message, Renderer>>,
    /// The height of a [`Row`].
    pub(super) height: f32,
}

impl<'a, Message, Renderer> Row<'a, Message, Renderer>
where
    Renderer: crate::Renderer,
    Renderer::Theme: StyleSheet,
{
    /// Creates a new [`Table`] row with the given `cells`,
    /// where [`None`] denotes an empty cell, and `height`.
    pub fn new(
        cells: Vec<Option<Element<'a, Message, Renderer>>>,
        height: f32,
    ) -> Self {
        Self {
            cells: cells
                .into_iter()
                .map(|c| c.unwrap_or(Element::from(empty::Empty {})))
                .collect(),
            height,
        }
    }
}

mod private {
    use super::*;
    use iced_core::mouse::Interaction;

    impl<'a, Message, Renderer> Widget<Message, Renderer>
        for Row<'a, Message, Renderer>
    where
        Renderer: crate::Renderer,
        Renderer::Theme: StyleSheet,
    {
        fn width(&self) -> Length {
            Length::Fill
        }

        fn height(&self) -> Length {
            Length::Fixed(self.height)
        }

        fn layout(&self, renderer: &Renderer, limits: &Limits) -> Node {
            let limits = limits.width(self.width()).height(self.height());
            flex::resolve(
                Axis::Horizontal,
                renderer,
                &limits,
                Padding::ZERO,
                0.0,
                Alignment::Start,
                &self.cells,
            )
        }

        fn draw(
            &self,
            tree: &Tree,
            renderer: &mut Renderer,
            theme: &Renderer::Theme,
            style: &Style,
            layout: Layout<'_>,
            cursor_position: Point,
            viewport: &Rectangle,
        ) {
            for ((cell, state), layout) in
                self.cells.iter().zip(&tree.children).zip(layout.children())
            {
                cell.as_widget().draw(
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
            self.cells.iter().map(Tree::new).collect()
        }

        fn diff(&self, tree: &mut Tree) {
            tree.diff_children(&self.cells)
        }

        fn operate(
            &self,
            tree: &mut Tree,
            layout: Layout<'_>,
            renderer: &Renderer,
            operation: &mut dyn Operation<Message>,
        ) {
            operation.container(None, &mut |operation| {
                self.cells
                    .iter()
                    .zip(&mut tree.children)
                    .zip(layout.children())
                    .for_each(|((cell, state), layout)| {
                        cell.as_widget()
                            .operate(state, layout, renderer, operation);
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
            self.cells
                .iter_mut()
                .zip(&mut tree.children)
                .zip(layout.children())
                .map(|((cell, state), layout)| {
                    cell.as_widget_mut().on_event(
                        state,
                        event.clone(),
                        layout,
                        cursor_position,
                        renderer,
                        clipboard,
                        shell,
                    )
                })
                .fold(event::Status::Ignored, event::Status::merge)
        }

        fn mouse_interaction(
            &self,
            tree: &Tree,
            layout: Layout<'_>,
            cursor_position: Point,
            viewport: &Rectangle,
            renderer: &Renderer,
        ) -> Interaction {
            self.cells
                .iter()
                .zip(&tree.children)
                .zip(layout.children())
                .map(|((cell, state), layout)| {
                    cell.as_widget().mouse_interaction(
                        state,
                        layout,
                        cursor_position,
                        viewport,
                        renderer,
                    )
                })
                .max()
                .unwrap_or_default()
        }

        fn overlay<'b>(
            &'b mut self,
            tree: &'b mut Tree,
            layout: Layout<'_>,
            renderer: &Renderer,
        ) -> Option<overlay::Element<'b, Message, Renderer>> {
            overlay::from_children(&mut self.cells, tree, layout, renderer)
        }
    }

    impl<'a, Message, Renderer> From<Row<'a, Message, Renderer>>
        for Element<'a, Message, Renderer>
    where
        Message: 'a,
        Renderer: crate::Renderer + 'a,
        Renderer::Theme: StyleSheet,
    {
        fn from(row: Row<'a, Message, Renderer>) -> Self {
            Self::new(row)
        }
    }
}
