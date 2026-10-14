//! Thin wrapper delegates rendering/input and reports the real layout through an operation.
use accesskit::NodeId;
use iced::advanced::{
    Clipboard, Shell, Widget,
    layout::{self, Layout},
    mouse, overlay, renderer,
    widget::{Operation, Tree},
};
use iced::{Element, Event, Length, Rectangle, Size, Vector};

struct Semantic<'a> {
    id: NodeId,
    content: Element<'a, crate::Message>,
}
#[derive(Clone, Copy)]
struct Tag(NodeId);
pub fn wrap<'a>(
    id: NodeId,
    content: impl Into<Element<'a, crate::Message>>,
) -> Element<'a, crate::Message> {
    Element::new(Semantic {
        id,
        content: content.into(),
    })
}
impl Widget<crate::Message, iced::Theme, iced::Renderer> for Semantic<'_> {
    fn size(&self) -> Size<Length> {
        self.content.as_widget().size()
    }
    fn size_hint(&self) -> Size<Length> {
        self.content.as_widget().size_hint()
    }
    fn children(&self) -> Vec<Tree> {
        vec![Tree::new(&self.content)]
    }
    fn diff(&self, tree: &mut Tree) {
        tree.diff_children(std::slice::from_ref(&self.content));
    }
    fn layout(
        &mut self,
        tree: &mut Tree,
        renderer: &iced::Renderer,
        limits: &layout::Limits,
    ) -> layout::Node {
        self.content
            .as_widget_mut()
            .layout(&mut tree.children[0], renderer, limits)
    }
    fn draw(
        &self,
        tree: &Tree,
        renderer: &mut iced::Renderer,
        theme: &iced::Theme,
        style: &renderer::Style,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        viewport: &Rectangle,
    ) {
        self.content.as_widget().draw(
            &tree.children[0],
            renderer,
            theme,
            style,
            layout,
            cursor,
            viewport,
        );
    }
    fn operate(
        &mut self,
        tree: &mut Tree,
        layout: Layout<'_>,
        renderer: &iced::Renderer,
        operation: &mut dyn Operation,
    ) {
        operation.custom(None, layout.bounds(), &mut Tag(self.id));
        self.content
            .as_widget_mut()
            .operate(&mut tree.children[0], layout, renderer, operation);
    }
    fn update(
        &mut self,
        tree: &mut Tree,
        event: &Event,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        renderer: &iced::Renderer,
        clipboard: &mut dyn Clipboard,
        shell: &mut Shell<'_, crate::Message>,
        viewport: &Rectangle,
    ) {
        self.content.as_widget_mut().update(
            &mut tree.children[0],
            event,
            layout,
            cursor,
            renderer,
            clipboard,
            shell,
            viewport,
        );
    }
    fn mouse_interaction(
        &self,
        tree: &Tree,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        viewport: &Rectangle,
        renderer: &iced::Renderer,
    ) -> mouse::Interaction {
        self.content.as_widget().mouse_interaction(
            &tree.children[0],
            layout,
            cursor,
            viewport,
            renderer,
        )
    }
    fn overlay<'a>(
        &'a mut self,
        tree: &'a mut Tree,
        layout: Layout<'a>,
        renderer: &iced::Renderer,
        viewport: &Rectangle,
        translation: Vector,
    ) -> Option<overlay::Element<'a, crate::Message, iced::Theme, iced::Renderer>> {
        self.content.as_widget_mut().overlay(
            &mut tree.children[0],
            layout,
            renderer,
            viewport,
            translation,
        )
    }
}
#[derive(Default)]
pub struct Collect(crate::semantics::Bounds);
impl Operation<crate::semantics::Bounds> for Collect {
    fn traverse(&mut self, operate: &mut dyn FnMut(&mut dyn Operation<crate::semantics::Bounds>)) {
        operate(self);
    }
    fn custom(
        &mut self,
        _: Option<&iced::advanced::widget::Id>,
        bounds: Rectangle,
        state: &mut dyn std::any::Any,
    ) {
        if let Some(tag) = state.downcast_ref::<Tag>() {
            self.0.insert(tag.0, bounds);
        }
    }
    fn finish(&self) -> iced::advanced::widget::operation::Outcome<crate::semantics::Bounds> {
        iced::advanced::widget::operation::Outcome::Some(self.0.clone())
    }
}
