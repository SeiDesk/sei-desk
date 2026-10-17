//! Lightweight iced advanced widget using only the already-enabled quad renderer.
//! Discrete paired bars show real endpoint times, without interpolating gaps.
use crate::traffic::{History, positions};
use iced::{
    Color, Element, Length, Rectangle, Size, Theme,
    advanced::{self, Widget, layout, mouse, renderer, widget::Tree},
};
use std::time::Instant;
pub const RECEIVE: Color = iced::color!(0x44638A);
pub const TRANSMIT: Color = iced::color!(0x725477);
struct Chart<'a> {
    history: &'a History,
    now: Instant,
}
pub fn chart<'a, Message: 'a>(history: &'a History, now: Instant) -> Element<'a, Message> {
    Element::new(Chart { history, now })
}
impl<Message> Widget<Message, Theme, iced::Renderer> for Chart<'_> {
    fn size(&self) -> Size<Length> {
        Size::new(Length::Fill, Length::Fixed(88.0))
    }
    fn layout(
        &mut self,
        _: &mut Tree,
        _: &iced::Renderer,
        limits: &layout::Limits,
    ) -> layout::Node {
        layout::atomic(limits, Length::Fill, Length::Fixed(88.0))
    }
    fn draw(
        &self,
        _: &Tree,
        renderer: &mut iced::Renderer,
        _: &Theme,
        _: &renderer::Style,
        layout: advanced::Layout<'_>,
        _: mouse::Cursor,
        viewport: &Rectangle,
    ) {
        use advanced::Renderer;
        let bounds = layout.bounds();
        let Some(clip) = bounds.intersection(viewport) else {
            return;
        };
        if bounds.width < 8.0 || bounds.height < 2.0 {
            return;
        }
        let positions = positions(self.history, self.now);
        renderer.with_layer(clip, |renderer| {
            let bottom = bounds.y + bounds.height - 1.0;
            renderer.fill_quad(
                renderer::Quad {
                    bounds: Rectangle {
                        x: bounds.x,
                        y: bottom,
                        width: bounds.width,
                        height: 1.0,
                    },
                    ..Default::default()
                },
                iced::color!(0xB7C6DA),
            );
            for (x, rx, tx) in positions {
                let center = bounds.x + 4.0 + x * (bounds.width - 8.0);
                for (offset, value, color) in [(-3.0, rx, RECEIVE), (0.5, tx, TRANSMIT)] {
                    // A true zero is a one-pixel mark on the zero baseline.
                    let height = (value * (bounds.height - 2.0)).max(1.0);
                    renderer.fill_quad(
                        renderer::Quad {
                            bounds: Rectangle {
                                x: center + offset,
                                y: bottom - height,
                                width: 2.5,
                                height,
                            },
                            ..Default::default()
                        },
                        color,
                    );
                }
            }
        });
    }
}
