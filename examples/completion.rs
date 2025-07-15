use ollama::Ollama;
use ollama::generation::completion::request::GenerationRequest;
use tokio_stream::StreamExt;

#[tokio::main]
pub async fn main() {
    let ollama = Ollama::new("http://localhost:11434", "deepseek-r1");

    let prompt = "你好";
    let req = GenerationRequest::new(prompt);
    let mut stream = ollama.generate_stream(req).await.unwrap();
    while let Some(chunk) = stream.next().await {
        if let Ok(chunk) = chunk {
            print!("{}", chunk.response);
        }
    }
}
