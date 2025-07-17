use serde::Deserialize;

use crate::generation::chat::message::Message;

#[derive(Debug, Deserialize)]
pub struct ChatMessageResponse {
    pub model: String,
    pub created_at: String,
    pub message: Message, // Message or String

    pub done: bool,
    pub done_reason: Option<String>,
    pub context: Option<Vec<i32>>,
    pub total_duration: Option<u64>, //  time spent generating the response
    pub load_duration: Option<u64>,  //  time spent in nanoseconds loading the model
    pub prompt_eval_count: Option<u64>, // number of tokens in the prompt
    pub prompt_eval_duration: Option<u64>, // time spent in nanoseconds evaluating the prompt
    pub eval_count: Option<u64>,     // number of tokens in the response
    pub eval_duration: Option<u64>,  //  time in nanoseconds spent generating the response
}

impl ChatMessageResponse {
    // To calculate how fast the response is generated in tokens per second (token/s), divide eval_count / eval_duration * 10^9.
    pub fn tokens_per_second(&self) -> Option<u64> {
        match (self.eval_count, self.eval_duration) {
            (Some(eval_count), Some(eval_duration)) => Some((eval_count / eval_duration * 10) ^ 9),
            _ => None,
        }
    }
}
