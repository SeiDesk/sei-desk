//! API capability probes only: no window, UIA provider, layout or text geometry.
use iced::advanced::{text, widget::Tree};
use iced::widget::text_input::{self, Value, cursor};
type InputState = text_input::State<<iced::Renderer as text::Renderer>::Paragraph>;

pub fn read_from_known_input_tree(tree: &Tree, actual_value: &str) -> cursor::State {
    // Only valid for a Tree owned by this exact TextInput type, not arbitrary widgets.
    tree.state
        .downcast_ref::<InputState>()
        .cursor()
        .state(&Value::new(actual_value))
}

#[test]
fn public_state_exposes_caret_and_directional_selection() {
    let mut state = InputState::new();
    let value = Value::new("abcXd");
    state.move_cursor_to(3);
    assert_eq!(state.cursor().state(&value), cursor::State::Index(3));
    state.select_range(4, 1);
    assert_eq!(
        state.cursor().state(&value),
        cursor::State::Selection { start: 4, end: 1 }
    );
    assert_eq!(state.cursor().selection(&value), Some((1, 4)));
    assert_eq!(value.select(1, 4).to_string(), "bcX");
}

#[test]
fn pinned_value_graphemes_map_to_utf8_and_utf16_boundaries() {
    let value = Value::new("aé😀e\u{301}👩\u{200d}💻");
    assert_eq!(value.len(), 5);
    let boundaries: Vec<_> = (0..=value.len())
        .map(|index| {
            let prefix = value.until(index).to_string();
            (prefix.len(), prefix.encode_utf16().count())
        })
        .collect();
    assert_eq!(
        boundaries,
        vec![(0, 0), (1, 1), (3, 2), (7, 4), (10, 6), (21, 11)]
    );
}

#[test]
fn cursor_getter_clamps_to_current_value_not_uia_validation() {
    let mut state = InputState::new();
    state.move_cursor_to_end();
    assert_eq!(
        state.cursor().state(&Value::new("")),
        cursor::State::Index(0)
    );
    state.select_range(usize::MAX, 1);
    assert_eq!(
        state.cursor().state(&Value::new("abc")),
        cursor::State::Selection { start: 3, end: 1 }
    );
    assert_eq!(
        state.cursor().state(&Value::new("")),
        cursor::State::Index(0)
    );
}
