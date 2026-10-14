//! Experiment-specific mapping, not a second editable model or widget framework.
use crate::model::{Action as ModelAction, Feedback, Focus, Model};
use accesskit::{
    Action, ActionData, ActionRequest, Node, NodeId, Rect, Role, Toggled, TreeId, TreeInfo,
    TreeUpdate,
};
use std::collections::BTreeMap;

pub const ROOT: NodeId = NodeId(1);
pub const LABEL: NodeId = NodeId(2);
pub const INPUT: NodeId = NodeId(3);
pub const VALIDATE: NodeId = NodeId(4);
pub const MOTION: NodeId = NodeId(5);
pub const CLOSE: NodeId = NodeId(6);
pub const INPUT_ID: &str = "spike-input";

#[derive(Debug, Default)]
pub struct State {
    pub model: Model,
    pub error_epoch: u64,
}
impl State {
    pub fn error_id(&self) -> NodeId {
        NodeId(100 + self.error_epoch * 2)
    }
    pub fn dismiss_id(&self) -> NodeId {
        NodeId(101 + self.error_epoch * 2)
    }
    pub fn focus_id(&self) -> NodeId {
        match self.model.focus {
            Focus::Input => INPUT,
            Focus::Validate => VALIDATE,
            Focus::Motion => MOTION,
            Focus::DismissError => self.dismiss_id(),
            Focus::Close => CLOSE,
        }
    }
    pub fn apply(&mut self, action: ModelAction) -> bool {
        let had_error = self.model.feedback == Feedback::Empty;
        let changed = self.model.apply(action);
        if !had_error && self.model.feedback == Feedback::Empty {
            self.error_epoch += 1;
        }
        changed
    }
    pub fn accessible_action(&self, request: &ActionRequest) -> Option<ModelAction> {
        if self.model.closing || request.target_tree != TreeId::ROOT {
            return None;
        }
        let id = request.target_node;
        if request.action == Action::Focus {
            let focus = match id {
                INPUT => Focus::Input,
                VALIDATE => Focus::Validate,
                MOTION => Focus::Motion,
                CLOSE => Focus::Close,
                id if id == self.dismiss_id() && self.model.feedback == Feedback::Empty => {
                    Focus::DismissError
                }
                _ => return None,
            };
            return Some(ModelAction::Focus(focus));
        }
        match (id, request.action, &request.data) {
            (INPUT, Action::SetValue, Some(ActionData::Value(value))) if value.len() <= 4096 => {
                Some(ModelAction::Edit(value.to_string()))
            }
            (VALIDATE, Action::Click, _) => Some(ModelAction::Validate),
            (MOTION, Action::Click, _) => Some(ModelAction::ToggleMotion),
            (CLOSE, Action::Click, _) => Some(ModelAction::Close),
            (id, Action::Click, _)
                if id == self.dismiss_id() && self.model.feedback == Feedback::Empty =>
            {
                Some(ModelAction::DismissError)
            }
            _ => None,
        }
    }
}

pub type Bounds = BTreeMap<NodeId, iced::Rectangle>;

pub fn tree(state: &State, bounds: &Bounds, scale: f64) -> TreeUpdate {
    let mut nodes = Vec::new();
    let mut children = vec![LABEL, INPUT, VALIDATE, MOTION];
    let error = state.model.feedback == Feedback::Empty;
    if error {
        children.extend([state.error_id(), state.dismiss_id()]);
    }
    children.push(CLOSE);
    let mut root = Node::new(Role::Window);
    root.set_label("AccessKit iced spike");
    root.set_children(children);
    nodes.push((ROOT, root));
    let mut add = |id: NodeId, role, name: &str, clickable: bool| {
        let mut node = Node::new(role);
        node.set_label(name);
        node.set_author_id(format!("spike-{}", id.0));
        if role == Role::Label {
            // AccessKit's Windows Label name comes from its textual value.
            node.set_value(name);
        }
        if let Some(rect) = bounds.get(&id) {
            node.set_bounds(Rect::new(
                f64::from(rect.x) * scale,
                f64::from(rect.y) * scale,
                f64::from(rect.x + rect.width) * scale,
                f64::from(rect.y + rect.height) * scale,
            ));
        }
        if clickable {
            node.add_action(Action::Click);
            node.add_action(Action::Focus);
        }
        nodes.push((id, node));
    };
    add(LABEL, Role::Label, "Sample text", false);
    add(INPUT, Role::TextInput, "Sample text", false);
    add(VALIDATE, Role::Button, "Validate", true);
    add(MOTION, Role::CheckBox, "Prefer reduced motion", true);
    add(CLOSE, Role::Button, "Close", true);
    if error {
        // No Live or Text/selection patterns are advertised without validation.
        add(
            state.error_id(),
            Role::Label,
            "Enter some text before validating.",
            false,
        );
        add(state.dismiss_id(), Role::Button, "Dismiss error", true);
    }
    let input = &mut nodes.iter_mut().find(|(id, _)| *id == INPUT).unwrap().1;
    input.set_labelled_by(vec![LABEL]);
    input.set_value(state.model.value.clone());
    input.add_action(Action::SetValue);
    input.add_action(Action::Focus);
    let motion = &mut nodes.iter_mut().find(|(id, _)| *id == MOTION).unwrap().1;
    motion.set_toggled(if state.model.prefer_reduced_motion {
        Toggled::True
    } else {
        Toggled::False
    });
    TreeUpdate {
        nodes,
        tree: Some(TreeInfo::new(ROOT)),
        tree_id: TreeId::ROOT,
        focus: state.focus_id(),
    }
}

pub fn incremental(previous: &TreeUpdate, next: &TreeUpdate) -> TreeUpdate {
    let nodes = next
        .nodes
        .iter()
        .filter(|(id, node)| {
            previous
                .nodes
                .iter()
                .find(|(old, _)| old == id)
                .is_none_or(|(_, old)| old != node)
        })
        .cloned()
        .collect();
    TreeUpdate {
        nodes,
        tree: None,
        tree_id: TreeId::ROOT,
        focus: next.focus,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn click(id: NodeId) -> ActionRequest {
        ActionRequest {
            action: Action::Click,
            target_tree: TreeId::ROOT,
            target_node: id,
            data: None,
        }
    }
    #[test]
    fn removed_and_recreated_error_rejects_stale_action() {
        let mut state = State::default();
        state.apply(ModelAction::Validate);
        let old = state.dismiss_id();
        assert!(state.accessible_action(&click(old)).is_some());
        state.apply(ModelAction::DismissError);
        assert!(state.accessible_action(&click(old)).is_none());
        state.apply(ModelAction::Validate);
        assert_ne!(old, state.dismiss_id());
        assert!(state.accessible_action(&click(old)).is_none());
        state.apply(ModelAction::Close);
        assert!(state.accessible_action(&click(VALIDATE)).is_none());
    }
    #[test]
    fn delta_removes_children_without_resending_deleted_nodes() {
        let mut state = State::default();
        let bounds = Bounds::new();
        state.apply(ModelAction::Validate);
        let before = tree(&state, &bounds, 1.0);
        let removed = state.dismiss_id();
        state.apply(ModelAction::DismissError);
        let after = tree(&state, &bounds, 1.0);
        let delta = incremental(&before, &after);
        assert!(delta.nodes.iter().any(|(id, _)| *id == ROOT));
        assert!(!delta.nodes.iter().any(|(id, _)| *id == removed));
        assert!(incremental(&after, &after).nodes.is_empty());
    }
    #[test]
    fn state_value_toggle_and_scaled_layout_are_authoritative() {
        let mut state = State::default();
        state.apply(ModelAction::Edit("example".into()));
        state.apply(ModelAction::ToggleMotion);
        let bounds = Bounds::from([(
            INPUT,
            iced::Rectangle {
                x: 10.0,
                y: 20.0,
                width: 30.0,
                height: 40.0,
            },
        )]);
        let update = tree(&state, &bounds, 1.5);
        let node = &update.nodes.iter().find(|(id, _)| *id == INPUT).unwrap().1;
        assert_eq!(node.value(), Some("example"));
        assert_eq!(node.bounds(), Some(Rect::new(15.0, 30.0, 60.0, 90.0)));
        assert_eq!(
            update
                .nodes
                .iter()
                .find(|(id, _)| *id == MOTION)
                .unwrap()
                .1
                .toggled(),
            Some(Toggled::True)
        );
        assert!(!node.supports_action(Action::SetTextSelection));
    }
}
