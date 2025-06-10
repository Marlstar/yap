use iced::Element;
use iced::widget::{text, button, text_input, container, row, column, horizontal_space};
use crate::theme::Theme;
use crate::message::Message as GMessage;
use super::Message;

impl super::MessageBar {
    pub fn view(&self, _theme: &Theme) -> Element<GMessage> {
        let message_input = text_input("Message", &self.message_input)
            .on_input(|s| Message::TextInputUpdate(s).msg())
            .on_submit(Message::Send(self.message_input.clone()).msg());

        let send = button("Send")
            .on_press(Message::Send(self.message_input.clone()).msg());

        return row![message_input, send].into();
    }
}
