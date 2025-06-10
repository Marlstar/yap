use crate::types::Task;
use crate::message::Message;

impl super::Yap {
    pub fn update(&mut self, msg: Message) -> Task {
        match msg {
            Message::None => Task::none(),
            Message::Exit => self.exit(),

            Message::SendMessage(msg) => self.send_message(msg),

            Message::MessageBar(msg) => self.messagebar.update(msg),
        }
    }

    fn send_message(&mut self, msg: String) -> Task {
        println!("Sending message: \"{msg}\"");
        Task::none()
    }

    fn exit(&self) -> Task {
        iced::window::get_latest().and_then(iced::window::close).chain(iced::exit())
    }
}
