use iced::widget::container;
use iced::Background;
use iced::Theme as IcedTheme;
use crate::theme::Theme;

pub struct Container;
impl Container {
    pub fn surface_bg(colours: &Theme) -> impl Fn(&IcedTheme) -> container::Style {
        |_: &IcedTheme| container::Style {
            background: Some(Background::Color(colours.surface)),
            ..Default::default()
        }
    }
}
