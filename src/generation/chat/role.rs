use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum MessageRole {
    #[serde(rename = "system")]
    System,

    #[serde(rename = "assistant")]
    Assistant,

    #[default]
    #[serde(rename = "user")]
    User,

    #[serde(rename = "tool")]
    Tool,
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    pub fn test_message_role() {
        assert_eq!(r#""system""#.to_owned(), serde_json::to_string(&MessageRole::System).unwrap());
        assert_eq!(
            r#""assistant""#.to_owned(),
            serde_json::to_string(&MessageRole::Assistant).unwrap()
        );
        assert_eq!(r#""user""#.to_owned(), serde_json::to_string(&MessageRole::User).unwrap());
        assert_eq!(r#""tool""#.to_owned(), serde_json::to_string(&MessageRole::Tool).unwrap());
    }
}
