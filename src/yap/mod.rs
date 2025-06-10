use crate::state::State;
use crate::theme::Theme;

mod boot;
mod view;
mod update;
pub mod titlebar;
pub mod messagebar;

/// The main client struct :)
pub struct Yap {
    state: State,
    theme: Theme,

    titlebar: titlebar::Titlebar,
    messagebar: messagebar::MessageBar,
}
impl Yap {
    pub fn title(&self) -> String {
       // TODO: update title with current information
        "Yap".to_string()
    }

    pub fn theme(&self) -> iced::Theme {
        self.theme.iced_theme()
    }

    pub fn window_settings() -> iced::window::Settings {
        iced::window::Settings {
            ..Default::default()
        }
    }
}
