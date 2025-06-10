use crate::types::Task;
use crate::message::Message as GMessage;
use super::Message;

impl super::MessageBar {
    pub fn update(&mut self, msg: Message) -> Task {
        match msg {
            Message::TextInputUpdate(s) => self.text_input_update(s),
            Message::Send(msg) => self.send(msg),
        }
    }

    fn text_input_update(&mut self, s: String) -> Task {
        self.message_input = s;
        Task::none()
    }

    fn send(&mut self, msg: String) -> Task {
        self.message_input.clear();
        Task::done(GMessage::SendMessage(msg))
    }
}
