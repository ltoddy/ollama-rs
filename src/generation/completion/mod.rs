pub mod request;
pub mod response;

use tokio_stream::StreamExt;

use self::request::GenerationRequest;
use self::response::GenerationResponse;
use crate::Ollama;

impl Ollama {
    pub async fn generate_stream(
        &self,
        req: GenerationRequest,
    ) -> crate::Result<impl tokio_stream::Stream<Item = crate::Result<GenerationResponse>>> {
        let api = self.url.join("/api/generate").unwrap();

        let mut req = req;
        req.model = self.model.clone();
        req.stream = true;

        let resp = self.http_client.post(api).json(&req).send().await?;

        let stream = resp.bytes_stream();
        let stream = stream.map(|bytes| match bytes {
            Ok(bytes) => Ok(serde_json::from_slice::<GenerationResponse>(&bytes)?),
            Err(err) => Err(err.into()),
        });
        Ok(stream)
    }

    pub async fn generate(&self, req: GenerationRequest) -> crate::Result<GenerationResponse> {
        let api = self.url.join("/api/generate").unwrap();

        let mut req = req;
        req.model = self.model.clone();
        req.stream = false;

        let resp = self
            .http_client
            .post(api)
            .json(&req)
            .send()
            .await?
            .json::<GenerationResponse>()
            .await?;
        Ok(resp)
    }
}
