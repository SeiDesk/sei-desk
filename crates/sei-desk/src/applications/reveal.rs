//! Reveal local keyboard focus using measured widget bounds, not guessed rows.
use iced::{
    Rectangle, Vector,
    advanced::widget::{
        Id, Operation,
        operation::{Outcome, Scrollable, TextInput},
    },
};
pub fn query(target: &'static str) -> impl Operation<f32> {
    Reveal {
        target: target.into(),
        root: None,
        target_bounds: None,
    }
}
struct Reveal {
    target: Id,
    root: Option<(Rectangle, f32)>,
    target_bounds: Option<Rectangle>,
}
impl Operation<f32> for Reveal {
    fn traverse(&mut self, operate: &mut dyn FnMut(&mut dyn Operation<f32>)) {
        operate(self);
    }
    fn container(&mut self, id: Option<&Id>, bounds: Rectangle) {
        if id == Some(&self.target) {
            self.target_bounds = Some(bounds);
        }
    }
    fn text_input(&mut self, id: Option<&Id>, bounds: Rectangle, _state: &mut dyn TextInput) {
        self.container(id, bounds);
    }
    fn scrollable(
        &mut self,
        id: Option<&Id>,
        bounds: Rectangle,
        _content: Rectangle,
        translation: Vector,
        _state: &mut dyn Scrollable,
    ) {
        if id == Some(&Id::from("dashboard-content")) {
            self.root = Some((bounds, translation.y));
        }
    }
    fn finish(&self) -> Outcome<f32> {
        match (self.root, self.target_bounds) {
            (Some((root, offset)), Some(target)) => Outcome::Some(required(root, target, offset)),
            _ => Outcome::None,
        }
    }
}
fn required(root: Rectangle, target: Rectangle, offset: f32) -> f32 {
    let top = target.y - offset;
    if top < root.y {
        (target.y - root.y - 8.0).max(0.0)
    } else if top + target.height > root.y + root.height {
        (target.y + target.height - root.y - root.height + 8.0).max(0.0)
    } else {
        offset
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn actual_bounds_reveal_only_when_needed() {
        let root = Rectangle {
            x: 200.0,
            y: 80.0,
            width: 500.0,
            height: 440.0,
        };
        let target = Rectangle {
            x: 210.0,
            y: 600.0,
            width: 480.0,
            height: 220.0,
        };
        assert_eq!(required(root, target, 0.0), 308.0);
        assert_eq!(required(root, target, 308.0), 308.0);
        assert_eq!(
            required(
                root,
                Rectangle {
                    y: 180.0,
                    height: 40.0,
                    ..target
                },
                308.0
            ),
            92.0
        );
    }
}
