use iced::Element;
use iced::widget::column;
use crate::message::Message;

impl super::Yap {
    pub fn view(&self) -> Element<Message> {
        column![
            self.titlebar.view(&self.theme),
            self.messagebar.view(&self.theme),
        ].into()
    }
}
