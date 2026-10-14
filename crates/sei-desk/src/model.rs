//! Pure validation-window state. No iced or Win32 types or side effects.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Focus {
    Input,
    Validate,
    Motion,
    DismissError,
    Close,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Feedback {
    Idle,
    Empty,
    Valid,
}

#[derive(Debug, Clone)]
pub enum Action {
    Edit(String),
    Validate,
    ToggleMotion,
    DismissError,
    Focus(Focus),
    Traverse { reverse: bool },
    Activate,
    Escape,
    Close,
    Composition(bool),
}

#[derive(Debug)]
pub struct Model {
    pub value: String,
    pub feedback: Feedback,
    pub focus: Focus,
    pub prefer_reduced_motion: bool,
    pub closing: bool,
    pub composing: bool,
}
impl Default for Model {
    fn default() -> Self {
        Self {
            value: String::new(),
            feedback: Feedback::Idle,
            focus: Focus::Input,
            prefer_reduced_motion: false,
            closing: false,
            composing: false,
        }
    }
}
impl Model {
    pub fn effective_reduced_motion(&self, system_reduce: Option<bool>) -> bool {
        self.prefer_reduced_motion || system_reduce.unwrap_or(true)
    }
    pub fn apply(&mut self, action: Action) -> bool {
        if self.closing
            || (self.composing
                && matches!(
                    action,
                    Action::Validate | Action::Traverse { .. } | Action::Activate | Action::Escape
                ))
        {
            return false;
        }
        match action {
            Action::Edit(value) => {
                self.value = value;
                self.feedback = Feedback::Idle;
                self.focus = Focus::Input;
            }
            Action::Validate => {
                self.feedback = if self.value.trim().is_empty() {
                    Feedback::Empty
                } else {
                    Feedback::Valid
                };
            }
            Action::ToggleMotion => self.prefer_reduced_motion = !self.prefer_reduced_motion,
            Action::DismissError => {
                self.feedback = Feedback::Idle;
                if self.focus == Focus::DismissError {
                    self.focus = Focus::Input;
                }
            }
            Action::Focus(target) => {
                self.focus = if target == Focus::DismissError && self.feedback != Feedback::Empty {
                    Focus::Input
                } else {
                    target
                };
            }
            Action::Traverse { reverse } => {
                let mut order = vec![Focus::Input, Focus::Validate, Focus::Motion];
                if self.feedback == Feedback::Empty {
                    order.push(Focus::DismissError);
                }
                order.push(Focus::Close);
                let index = order
                    .iter()
                    .position(|target| *target == self.focus)
                    .unwrap_or(0);
                self.focus = order[if reverse {
                    (index + order.len() - 1) % order.len()
                } else {
                    (index + 1) % order.len()
                }];
            }
            Action::Activate => {
                self.apply(match self.focus {
                    Focus::Input | Focus::Validate => Action::Validate,
                    Focus::Motion => Action::ToggleMotion,
                    Focus::DismissError => Action::DismissError,
                    Focus::Close => Action::Close,
                });
            }
            Action::Escape if self.feedback == Feedback::Empty => {
                self.apply(Action::DismissError);
            }
            Action::Escape | Action::Close => self.closing = true,
            Action::Composition(active) => self.composing = active,
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn validation_retains_value_and_initiating_focus() {
        let mut model = Model::default();
        model.apply(Action::Edit("   ".into()));
        model.apply(Action::Focus(Focus::Validate));
        model.apply(Action::Validate);
        assert_eq!(model.feedback, Feedback::Empty);
        assert_eq!(model.value, "   ");
        assert_eq!(model.focus, Focus::Validate);
    }
    #[test]
    fn editing_clears_stale_feedback_then_validates() {
        let mut model = Model::default();
        model.apply(Action::Validate);
        model.apply(Action::Edit("sample".into()));
        assert_eq!(model.feedback, Feedback::Idle);
        model.apply(Action::Validate);
        assert_eq!(model.feedback, Feedback::Valid);
    }
    #[test]
    fn forward_reverse_and_conditional_error_control() {
        let mut model = Model::default();
        model.apply(Action::Traverse { reverse: true });
        assert_eq!(model.focus, Focus::Close);
        model.apply(Action::Traverse { reverse: false });
        assert_eq!(model.focus, Focus::Input);
        model.apply(Action::Validate);
        model.apply(Action::Focus(Focus::Motion));
        model.apply(Action::Traverse { reverse: false });
        assert_eq!(model.focus, Focus::DismissError);
        model.apply(Action::Activate);
        assert_eq!(model.focus, Focus::Input);
        assert_eq!(model.feedback, Feedback::Idle);
    }
    #[test]
    fn escape_dismisses_error_before_closing() {
        let mut model = Model::default();
        model.apply(Action::Validate);
        model.apply(Action::Escape);
        assert!(!model.closing);
        model.apply(Action::Escape);
        assert!(model.closing);
    }
    #[test]
    fn close_is_terminal_and_does_not_accept_late_actions() {
        let mut model = Model::default();
        model.apply(Action::Close);
        model.apply(Action::Edit("late".into()));
        model.apply(Action::Activate);
        assert!(model.closing);
        assert!(model.value.is_empty());
        assert_eq!(model.feedback, Feedback::Idle);
    }
    #[test]
    fn system_or_unknown_motion_preference_cannot_be_overridden() {
        let mut model = Model::default();
        assert!(model.effective_reduced_motion(None));
        assert!(model.effective_reduced_motion(Some(true)));
        assert!(!model.effective_reduced_motion(Some(false)));
        model.apply(Action::ToggleMotion);
        assert!(model.effective_reduced_motion(Some(false)));
    }
    #[test]
    fn active_composition_preserves_text_focus_and_window() {
        let mut model = Model::default();
        model.apply(Action::Composition(true));
        for action in [
            Action::Validate,
            Action::Traverse { reverse: false },
            Action::Activate,
            Action::Escape,
        ] {
            assert!(!model.apply(action));
        }
        assert_eq!(model.feedback, Feedback::Idle);
        assert_eq!(model.focus, Focus::Input);
        assert!(!model.closing);
        model.apply(Action::Composition(false));
        assert!(model.apply(Action::Validate));
        assert_eq!(model.feedback, Feedback::Empty);
    }
}
