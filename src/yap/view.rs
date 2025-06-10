use iced::Element;
use iced::widget::{text, button, text_input, container, row, column, horizontal_space};
use crate::message::Message;
use crate::style;

impl super::Yap {
    pub fn view(&self) -> Element<Message> {
        column![
            self.titlebar(),
        ].into()
    }

    fn titlebar(&self) -> Element<Message> {
        let exit = button("Exit").on_press(Message::Exit);

        let titlebar = row![
            horizontal_space(),
            exit
        ];

        let titlebar = container(titlebar).style(style::Container::surface_bg(&self.colours));

        return titlebar.into();
    }
}
