use crate::Message;
use iced::advanced::input_method::InputMethod;
use iced::advanced::{
    Clipboard, Shell, Widget,
    layout::{self, Layout},
    mouse, overlay, renderer,
    widget::{Operation, Tree},
};
use iced::widget::text_input::{self, Value, cursor};
use iced::{Element, Event, Length, Rectangle, Size, Vector};
use std::{
    collections::BTreeMap,
    fs::{File, OpenOptions},
    io::{self, Write},
    sync::{Arc, Mutex},
};
type InputState = text_input::State<<iced::Renderer as iced::advanced::text::Renderer>::Paragraph>;
pub fn quote(s: &str) -> String {
    let mut out = String::from("\"");
    for c in s.chars() {
        match c {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            c if c.is_control() => out.push_str(&format!("\\u{:04x}", c as u32)),
            c => out.push(c),
        }
    }
    out.push('"');
    out
}
pub fn offsets(value: &str, index: usize) -> Option<(usize, usize)> {
    let v = Value::new(value);
    if index > v.len() {
        return None;
    }
    let prefix = v.until(index).to_string();
    Some((prefix.len(), prefix.encode_utf16().count()))
}
pub fn endpoints(state: cursor::State) -> (usize, usize, &'static str) {
    match state {
        cursor::State::Index(i) => (i, i, "collapsed"),
        cursor::State::Selection { start, end } => {
            (start, end, if end > start { "forward" } else { "reverse" })
        }
    }
}
fn rect(r: Rectangle) -> String {
    format!("[{}, {}, {}, {}]", r.x, r.y, r.width, r.height)
}
fn ime(i: &InputMethod) -> String {
    match i {
        InputMethod::Disabled => "{\"enabled\":false}".into(),
        InputMethod::Enabled {
            cursor,
            purpose,
            preedit,
        } => format!(
            "{{\"enabled\":true,\"cursor\":{},\"purpose\":{},\"preedit\":{}}}",
            rect(*cursor),
            quote(&format!("{purpose:?}")),
            preedit.as_ref().map_or("null".into(), |p| format!(
                "{{\"content\":{},\"selection_utf8_bytes\":{},\"text_size\":{}}}",
                quote(&p.content),
                p.selection
                    .as_ref()
                    .map_or("null".into(), |r| format!("[{},{}]", r.start, r.end)),
                quote(&format!("{:?}", p.text_size))
            ))
        ),
    }
}
/// Only caller-issued model/event epochs, never an alleged iced layout version.
#[derive(Default)]
pub struct EpochGate {
    current: u64,
    closed: bool,
}
impl EpochGate {
    pub fn advance(&mut self) -> Option<u64> {
        if self.closed {
            return None;
        }
        self.current = self.current.checked_add(1)?;
        Some(self.current)
    }
    pub fn accepts(&self, epoch: u64) -> bool {
        !self.closed && epoch == self.current
    }
    pub fn close(&mut self) {
        self.closed = true
    }
}
pub struct Logger {
    file: File,
    last: BTreeMap<usize, (u64, String)>,
    rows: usize,
    gate: EpochGate,
    sequence: u64,
    last_observation: Option<(u64, u64)>,
    started: std::time::Instant,
}
impl Logger {
    pub fn new(path: &str) -> io::Result<Self> {
        Ok(Self {
            file: OpenOptions::new().write(true).create_new(true).open(path)?,
            last: BTreeMap::new(),
            rows: 0,
            gate: EpochGate::default(),
            sequence: 0,
            last_observation: None,
            started: std::time::Instant::now(),
        })
    }
    pub fn event(&mut self, label: &str, epoch: u64) {
        let generated = self.gate.advance().expect("log epoch exhausted");
        assert_eq!(generated, epoch);
        writeln!(
            self.file,
            "{{\"kind\":\"app_event\",\"epoch\":{epoch},\"event\":{}}}",
            quote(label)
        )
        .unwrap();
        self.file.flush().unwrap();
    }
    fn emit(&mut self, id: usize, epoch: u64, body: String) {
        if self.rows >= 2048 {
            return;
        }
        if self
            .last
            .get(&id)
            .is_some_and(|(e, b)| *e == epoch && b == &body)
        {
            return;
        }
        self.last.insert(id, (epoch, body.clone()));
        self.rows += 1;
        let seq = self.sequence;
        self.sequence = seq.checked_add(1).expect("observation sequence exhausted");
        self.last_observation = Some((seq, epoch));
        writeln!(self.file,"{{\"kind\":\"observation\",\"seq\":{seq},\"elapsed_micros\":{},\"input\":{},\"model_epoch\":{epoch},\"accepted_model_epoch\":{},\"layout_generation\":\"UNAVAILABLE\",\"coordinate_space\":\"iced logical window layout; screen origin UNAVAILABLE\",{body}}}",self.started.elapsed().as_micros(),quote(if id==0{"A"}else{"B"}),self.gate.accepts(epoch)).unwrap();
        self.file.flush().unwrap();
    }
    pub fn replay(&mut self) {
        if let Some((seq, epoch)) = self.last_observation {
            writeln!(self.file,"{{\"kind\":\"deliberate_stale_delivery_probe\",\"original_seq\":{seq},\"original_epoch\":{epoch},\"accepted_model_epoch\":{}}}",self.gate.accepts(epoch)).unwrap();
            self.file.flush().unwrap();
        }
    }
}
impl Drop for Logger {
    fn drop(&mut self) {
        self.gate.close();
        let _ = writeln!(
            self.file,
            "{{\"kind\":\"logger_closed\",\"rows\":{}}}",
            self.rows
        );
    }
}
pub struct Observer<'a> {
    id: usize,
    epoch: u64,
    value: String,
    log: Arc<Mutex<Logger>>,
    child: Element<'a, Message>,
}
impl<'a> Observer<'a> {
    pub fn wrap(
        id: usize,
        epoch: u64,
        value: String,
        log: Arc<Mutex<Logger>>,
        child: impl Into<Element<'a, Message>>,
    ) -> Element<'a, Message> {
        Element::new(Self {
            id,
            epoch,
            value,
            log,
            child: child.into(),
        })
    }
}
impl Widget<Message, iced::Theme, iced::Renderer> for Observer<'_> {
    fn size(&self) -> Size<Length> {
        self.child.as_widget().size()
    }
    fn size_hint(&self) -> Size<Length> {
        self.child.as_widget().size_hint()
    }
    fn children(&self) -> Vec<Tree> {
        vec![Tree::new(&self.child)]
    }
    fn diff(&self, tree: &mut Tree) {
        tree.diff_children(std::slice::from_ref(&self.child))
    }
    fn layout(
        &mut self,
        tree: &mut Tree,
        renderer: &iced::Renderer,
        limits: &layout::Limits,
    ) -> layout::Node {
        self.child
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
        self.child.as_widget().draw(
            &tree.children[0],
            renderer,
            theme,
            style,
            layout,
            cursor,
            viewport,
        )
    }
    fn operate(
        &mut self,
        tree: &mut Tree,
        layout: Layout<'_>,
        renderer: &iced::Renderer,
        op: &mut dyn Operation,
    ) {
        self.child
            .as_widget_mut()
            .operate(&mut tree.children[0], layout, renderer, op)
    }
    fn update(
        &mut self,
        tree: &mut Tree,
        event: &Event,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        renderer: &iced::Renderer,
        clipboard: &mut dyn Clipboard,
        shell: &mut Shell<'_, Message>,
        viewport: &Rectangle,
    ) {
        let before = shell.input_method().clone();
        self.child.as_widget_mut().update(
            &mut tree.children[0],
            event,
            layout,
            cursor,
            renderer,
            clipboard,
            shell,
            viewport,
        );
        let after = shell.input_method();
        let focused = tree.children[0]
            .state
            .downcast_ref::<InputState>()
            .is_focused();
        let (anchor, active, direction) = endpoints(
            tree.children[0]
                .state
                .downcast_ref::<InputState>()
                .cursor()
                .state(&Value::new(&self.value)),
        );
        let redraw = matches!(
            event,
            Event::Window(iced::window::Event::RedrawRequested(_))
        );
        let attribution = if matches!(before, InputMethod::Disabled)
            && matches!(after, InputMethod::Enabled { .. })
        {
            "CHILD_ENABLED"
        } else if matches!(before, InputMethod::Enabled { .. }) {
            "AMBIGUOUS_INHERITED_SHARED_SHELL"
        } else {
            "NO_IME_OUTPUT"
        };
        let body = format!(
            "\"phase\":{},\"event\":{},\"document_value\":{},\"value_lag_possible\":{},\"focused\":{focused},\"anchor_graphemes\":{anchor},\"active_graphemes\":{active},\"direction\":{},\"anchor_utf8_utf16\":{},\"active_utf8_utf16\":{},\"widget_bounds\":{},\"text_child_bounds\":{},\"viewport\":{},\"ime_before\":{},\"ime_after\":{},\"attribution\":{}",
            quote(if redraw {
                "after_child_redraw"
            } else {
                "after_child_update"
            }),
            quote(&if redraw {
                "Window(RedrawRequested)".to_owned()
            } else {
                format!("{event:?}")
            }),
            quote(&self.value),
            !redraw,
            quote(direction),
            offsets(&self.value, anchor).map_or("null".into(), |(a, b)| format!("[{a},{b}]")),
            offsets(&self.value, active).map_or("null".into(), |(a, b)| format!("[{a},{b}]")),
            rect(layout.bounds()),
            layout
                .children()
                .next()
                .map_or("null".into(), |l| rect(l.bounds())),
            rect(*viewport),
            ime(&before),
            ime(after),
            quote(attribution)
        );
        self.log.lock().unwrap().emit(self.id, self.epoch, body);
    }
    fn mouse_interaction(
        &self,
        tree: &Tree,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        viewport: &Rectangle,
        renderer: &iced::Renderer,
    ) -> mouse::Interaction {
        self.child.as_widget().mouse_interaction(
            &tree.children[0],
            layout,
            cursor,
            viewport,
            renderer,
        )
    }
    fn overlay<'b>(
        &'b mut self,
        tree: &'b mut Tree,
        layout: Layout<'b>,
        renderer: &iced::Renderer,
        viewport: &Rectangle,
        translation: Vector,
    ) -> Option<overlay::Element<'b, Message, iced::Theme, iced::Renderer>> {
        self.child.as_widget_mut().overlay(
            &mut tree.children[0],
            layout,
            renderer,
            viewport,
            translation,
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn preserve_direction() {
        assert_eq!(
            endpoints(cursor::State::Selection { start: 4, end: 1 }),
            (4, 1, "reverse")
        );
        assert_eq!(
            endpoints(cursor::State::Selection { start: 1, end: 4 }),
            (1, 4, "forward")
        );
    }
    #[test]
    fn checked_unicode_units() {
        let s = "aé😀e\u{301}👩\u{200d}💻";
        assert_eq!(offsets(s, 5), Some((21, 11)));
        assert_eq!(offsets(s, 6), None);
        assert_eq!(offsets("", 0), Some((0, 0)));
        assert_eq!(offsets("", 1), None);
    }
    #[test]
    fn stale_closed_and_exhausted_epochs() {
        let mut gate = EpochGate::default();
        assert_eq!(gate.advance(), Some(1));
        assert!(gate.accepts(1));
        assert_eq!(gate.advance(), Some(2));
        assert!(!gate.accepts(1));
        gate.close();
        assert!(!gate.accepts(2));
        assert_eq!(gate.advance(), None);
        let mut max = EpochGate {
            current: u64::MAX,
            closed: false,
        };
        assert_eq!(max.advance(), None);
        assert!(max.accepts(u64::MAX));
    }
    #[test]
    fn json_strings_are_escaped() {
        assert_eq!(quote("a\n\"\\"), "\"a\\u000a\\\"\\\\\"");
    }
}
