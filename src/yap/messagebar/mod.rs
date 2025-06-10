mod view;
mod update;

pub struct MessageBar {
    message_input: String,
}
impl MessageBar {
    pub fn new() -> Self {
        Self {
            message_input: String::new(),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Message {
    TextInputUpdate(String),

    Send(String),
}
impl Message {
    pub fn msg(self) -> crate::message::Message {
        crate::message::Message::MessageBar(self)
    }
}
