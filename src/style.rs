use iced::widget::container;
use iced::Background;
use iced::Theme;
use crate::colours::Colours;

pub struct Container;
impl Container {
    pub fn surface_bg(colours: &Colours) -> impl Fn(&Theme) -> container::Style {
        |_: &Theme| container::Style {
            background: Some(Background::Color(colours.surface)),
            ..Default::default()
        }
    }
}
