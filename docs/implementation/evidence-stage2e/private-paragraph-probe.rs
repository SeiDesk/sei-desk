use iced::advanced::text;
use iced::widget::text_input;
type InputState = text_input::State<<iced::Renderer as text::Renderer>::Paragraph>;
pub fn shaped_paragraph(state: &InputState) {
    let _ = state.value.raw();
}
