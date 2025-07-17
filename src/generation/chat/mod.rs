pub mod message;
pub mod request;
pub mod response;
pub mod role;

use tokio_stream::StreamExt;

use crate::Ollama;
use crate::generation::chat::request::ChatMessageRequest;
use crate::generation::chat::response::ChatMessageResponse;

impl Ollama {
    pub async fn send_chat_message_stream(
        &self,
        req: ChatMessageRequest,
    ) -> crate::Result<impl tokio_stream::Stream<Item = crate::Result<ChatMessageResponse>>> {
        let api = self.url.join("/api/chat").unwrap();

        let mut req = req;
        req.model = self.model.clone();
        req.stream = true;

        let resp = self.http_client.post(api).json(&req).send().await?;

        let stream = resp.bytes_stream();
        let stream = stream.map(|bytes| match bytes {
            Ok(bytes) => Ok(serde_json::from_slice::<ChatMessageResponse>(&bytes)?),
            Err(err) => Err(err.into()),
        });
        Ok(stream)
    }

    pub async fn send_chat_message(
        &self,
        req: ChatMessageRequest,
    ) -> crate::Result<ChatMessageResponse> {
        let api = self.url.join("/api/chat").unwrap();

        let mut req = req;
        req.model = self.model.clone();
        req.stream = false;

        let resp = self
            .http_client
            .post(api)
            .json(&req)
            .send()
            .await?
            .json::<ChatMessageResponse>()
            .await?;
        Ok(resp)
    }
}
