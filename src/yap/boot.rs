use crate::state::State;
use crate::types::Task;

impl super::Yap {
    pub fn boot() -> (Self, Task) {
        let state = State::read().unwrap_or_default();

        let s = Self {
            state,
        };
        let task = Task::none();
        return (s, task);
    }
}
