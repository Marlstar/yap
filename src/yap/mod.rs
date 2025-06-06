use crate::state::State;

mod boot;
mod view;
mod update;

/// The main client struct :)
pub struct Yap {
    state: State,
}
impl Yap {
    pub fn title(&self) -> String {
       // TODO: update title with current information
        "Yap".to_string()
    }
}
