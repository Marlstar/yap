use crate::types::Task;
use crate::message::Message;

impl super::Yap {
    pub fn update(&mut self, msg: Message) -> Task {
        match msg {
            Message::None => Task::none(),
            Message::Exit => self.exit(),
        }
    }

    fn exit(&self) -> Task {
        iced::window::get_latest().and_then(iced::window::close).chain(iced::exit())
    }
}
