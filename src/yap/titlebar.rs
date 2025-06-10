use iced::Element;
use iced::widget::{text, button, text_input, container, row, column, horizontal_space};
use crate::message::Message;
use crate::style;
use crate::theme::Theme;

pub struct Titlebar;
impl Titlebar {
    pub fn view<'a>(&self, theme: &'a Theme) -> Element<'a, Message> {
        let exit = button("Exit").on_press(Message::Exit);

        let titlebar = row![
            horizontal_space(),
            exit
        ];

        let titlebar = container(titlebar).style(style::Container::surface_bg(theme));

        return titlebar.into();
    }
}
