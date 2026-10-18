//! Preserve native text editing while applying the launch key policy before
//! TextInput's submit callback (which otherwise also accepts modifiers/repeat).
use iced::advanced::{
    Clipboard, Layout, Shell, Widget, layout, mouse, renderer,
    widget::{Tree, tree},
};
use iced::{Element, Event, Length, Rectangle, Renderer, Size, Theme, keyboard, widget::TextInput};

pub fn guarded<'a, Message: Clone + 'a>(input: TextInput<'a, Message>) -> Element<'a, Message> {
    Element::new(Search(input))
}

struct Search<'a, Message>(TextInput<'a, Message>);
impl<Message: Clone> Widget<Message, Theme, Renderer> for Search<'_, Message> {
    fn size(&self) -> Size<Length> {
        Widget::size(&self.0)
    }
    fn tag(&self) -> tree::Tag {
        Widget::tag(&self.0)
    }
    fn state(&self) -> tree::State {
        Widget::state(&self.0)
    }
    fn layout(
        &mut self,
        tree: &mut Tree,
        renderer: &Renderer,
        limits: &layout::Limits,
    ) -> layout::Node {
        Widget::layout(&mut self.0, tree, renderer, limits)
    }
    fn operate(
        &mut self,
        tree: &mut Tree,
        layout: Layout<'_>,
        renderer: &Renderer,
        operation: &mut dyn iced::advanced::widget::Operation,
    ) {
        Widget::operate(&mut self.0, tree, layout, renderer, operation);
    }
    fn update(
        &mut self,
        tree: &mut Tree,
        event: &Event,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        renderer: &Renderer,
        clipboard: &mut dyn Clipboard,
        shell: &mut Shell<'_, Message>,
        viewport: &Rectangle,
    ) {
        if let Event::Keyboard(keyboard::Event::KeyPressed {
            key: keyboard::Key::Named(keyboard::key::Named::Enter),
            modifiers,
            repeat,
            ..
        }) = event
            && (*repeat || modifiers.control() || modifiers.alt() || modifiers.logo())
        {
            shell.capture_event();
            return;
        }
        Widget::update(
            &mut self.0,
            tree,
            event,
            layout,
            cursor,
            renderer,
            clipboard,
            shell,
            viewport,
        );
    }
    fn draw(
        &self,
        tree: &Tree,
        renderer: &mut Renderer,
        theme: &Theme,
        style: &renderer::Style,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        viewport: &Rectangle,
    ) {
        Widget::draw(
            &self.0, tree, renderer, theme, style, layout, cursor, viewport,
        );
    }
    fn mouse_interaction(
        &self,
        tree: &Tree,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        viewport: &Rectangle,
        renderer: &Renderer,
    ) -> mouse::Interaction {
        Widget::mouse_interaction(&self.0, tree, layout, cursor, viewport, renderer)
    }
}
