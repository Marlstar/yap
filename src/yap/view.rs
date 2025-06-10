use iced::Element;
use iced::widget::{text, button, text_input, container, row, column, horizontal_space};
use crate::message::Message;
use crate::style;

impl super::Yap {
    pub fn view(&self) -> Element<Message> {
        column![
            self.titlebar.view(&self.theme),
        ].into()
    }

}
