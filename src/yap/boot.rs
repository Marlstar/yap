use crate::state::State;
use crate::types::Task;
use crate::colours::Colours;

impl super::Yap {
    pub fn boot() -> (Self, Task) {
        let state = State::read().unwrap_or_default();
        let colours = Colours::default();

        let s = Self {
            state,
            colours,
        };
        let task = Task::none();
        return (s, task);
    }
}
