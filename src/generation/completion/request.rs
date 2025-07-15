use serde::Serialize;

#[derive(Debug, Default, Serialize)]
pub struct GenerationRequest {
    pub(crate) model: String, // (required) the model name

    #[serde(skip_serializing_if = "String::is_empty")]
    pub prompt: String, // the prompt to generate a response for

    pub(crate) stream: bool, //  if false the response will be returned as a single response object, rather than a stream of objects
}

impl GenerationRequest {
    pub fn new(prompt: &str) -> Self {
        Self { prompt: prompt.to_owned(), ..Default::default() }
    }
}
