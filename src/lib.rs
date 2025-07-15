pub mod error;
pub mod generation;

use reqwest::IntoUrl;
use url::Url;

pub use crate::error::Error;

pub type Result<T, E = Error> = std::result::Result<T, E>;

pub struct Ollama {
    url: Url,
    model: String,
    http_client: reqwest::Client,
}

impl Ollama {
    pub fn new<U: IntoUrl>(url: U, model: &str) -> Self {
        Self::new_with_client(url, model, Default::default())
    }

    pub fn new_with_client<U: IntoUrl>(url: U, model: &str, http_client: reqwest::Client) -> Self {
        Self { url: url.into_url().unwrap(), model: model.to_owned(), http_client }
    }
}
