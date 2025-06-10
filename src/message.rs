use crate::yap::messagebar;

#[derive(Debug, Clone)]
pub enum Message {
    None,
    Exit,

    SendMessage(String),

    MessageBar(messagebar::Message),
}

impl From<messagebar::Message> for Message {
    fn from(value: messagebar::Message) -> Self {
        Message::MessageBar(value)
    }
}
