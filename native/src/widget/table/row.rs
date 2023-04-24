use iced_core::{Alignment, Length, Padding, Point, Rectangle};
use iced_style::table::StyleSheet;

use crate::layout::flex::Axis;
use crate::layout::{flex, Limits, Node};
use crate::renderer::Style;
use crate::widget::{Operation, Tree};
use crate::{event, overlay, Clipboard, Element, Event, Layout, Shell, Widget};

pub(super) struct Row<'a, Message, Renderer>(
    pub(super) Vec<Element<'a, Message, Renderer>>,
    pub(super) f32,
)
where
    Renderer: crate::Renderer,
    Renderer::Theme: StyleSheet;

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
        Length::Fixed(self.1)
    }

    fn layout(&self, renderer: &Renderer, limits: &Limits) -> Node {
        flex::resolve(
            Axis::Horizontal,
            renderer,
            limits,
            Padding::ZERO,
            0.0,
            Alignment::Start,
            &self.0,
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
            self.0.iter().zip(&tree.children).zip(layout.children())
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
        self.0.iter().map(Tree::new).collect()
    }

    fn diff(&self, tree: &mut Tree) {
        tree.diff_children(&self.0)
    }

    fn operate(
        &self,
        tree: &mut Tree,
        layout: Layout<'_>,
        renderer: &Renderer,
        operation: &mut dyn Operation<Message>,
    ) {
        operation.container(None, &mut |operation| {
            self.0
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
        self.0
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

    fn overlay<'b>(
        &'b mut self,
        tree: &'b mut Tree,
        layout: Layout<'_>,
        renderer: &Renderer,
    ) -> Option<overlay::Element<'b, Message, Renderer>> {
        overlay::from_children(&mut self.0, tree, layout, renderer)
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
