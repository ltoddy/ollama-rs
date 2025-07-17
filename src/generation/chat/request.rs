use serde::Serialize;

use crate::generation::chat::message::Message;

#[derive(Debug, Default, Serialize)]
pub struct ChatMessageRequest {
    pub(crate) model: String,   // (required) the model name
    pub messages: Vec<Message>, // the messages of the chat, this can be used to keep a chat memory

    pub(crate) stream: bool, // if false the response will be returned as a single response object, rather than a stream of objects
}

impl ChatMessageRequest {
    pub fn new(messages: Vec<Message>) -> Self {
        Self { messages, ..Default::default() }
    }
}
