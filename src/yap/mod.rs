use crate::state::State;
use crate::colours::Colours;
mod boot;
mod view;
mod update;

/// The main client struct :)
pub struct Yap {
    state: State,
    colours: Colours,
}
impl Yap {
    pub fn title(&self) -> String {
       // TODO: update title with current information
        "Yap".to_string()
    }

    pub fn theme(&self) -> iced::Theme {
        self.colours.iced_theme()
    }

    pub fn window_settings() -> iced::window::Settings {
        iced::window::Settings {
            ..Default::default()
        }
    }
}
