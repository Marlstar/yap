use crate::state::State;
use crate::types::Task;
use crate::theme::Theme;

impl super::Yap {
    pub fn boot() -> (Self, Task) {
        let state = State::read().unwrap_or_default();
        let colours = Theme::default();

        let s = Self {
            state,
            theme: colours,

            titlebar: super::titlebar::Titlebar,
        };
        let task = Task::none();
        return (s, task);
    }
}
