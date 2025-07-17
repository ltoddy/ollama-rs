use serde::{Deserialize, Serialize};

use crate::generation::chat::role::MessageRole;

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Message {
    role: MessageRole, // the role of the message, either system, user, assistant, or tool
    pub content: String, // the content of the message

    #[serde(skip_serializing_if = "Option::is_none")]
    pub thinking: Option<bool>, // (for thinking models) the model's thinking process

    #[serde(skip_serializing_if = "Option::is_none")]
    pub images: Option<Vec<u8>>, // (optional): a list of images to include in the message (for multimodal models such as llava)

                                 // TODO
                                 // tool_calls: Option<Vec<String>>, // (optional): a list of tools in JSON that the model wants to use
                                 // tool_name: Option<Vec<String>>, // (optional): add the name of the tool that was executed to inform the model of the result
}

impl Message {
    pub fn system(content: &str) -> Self {
        Self::new(MessageRole::System, content)
    }

    pub fn assistant(content: &str) -> Self {
        Self::new(MessageRole::System, content)
    }

    pub fn user(content: &str) -> Self {
        Self::new(MessageRole::User, content)
    }

    fn new(role: MessageRole, content: &str) -> Self {
        let content = content.to_owned();
        Self { role, content, ..Default::default() }
    }
}
