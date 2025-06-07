use crate::types::Task;
use crate::message::Message;

impl super::Yap {
    pub fn update(&mut self, msg: Message) -> Task {
        match msg {
            Message::None => Task::none(),
            Message::Exit => iced::exit(),
        }
    }
}
