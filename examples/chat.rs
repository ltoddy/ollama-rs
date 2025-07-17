use ollama::Ollama;
use ollama::generation::chat::message::Message;
use ollama::generation::chat::request::ChatMessageRequest;
use tokio_stream::StreamExt;

#[tokio::main]
pub async fn main() {
    let ollama = Ollama::new("http://localhost:11434", "deepseek-r1");

    let mut histories = Vec::<Message>::new();

    // 第一轮对话: 我喜欢吃苹果
    let message = Message::user("我喜欢吃苹果");
    histories.push(message); // 需要把每一次对话消息保存起来
    let req = ChatMessageRequest::new(histories.clone());
    let mut stream = ollama.send_chat_message_stream(req).await.unwrap();
    let mut respond = String::new();
    while let Some(chunk) = stream.next().await {
        if let Ok(chunk) = chunk {
            respond.push_str(&chunk.message.content);
            print!("{}", chunk.message.content);
        }
    }
    histories.push(Message::assistant(&respond)); // AI 返回的 message 类型为 assistant

    println!("\n======================================================\n");

    // 第二轮对话: 我喜欢吃什么
    let message = Message::user("我喜欢吃什么");
    histories.push(message);
    let req = ChatMessageRequest::new(histories);
    let mut stream = ollama.send_chat_message_stream(req).await.unwrap();
    while let Some(chunk) = stream.next().await {
        if let Ok(chunk) = chunk {
            print!("{}", chunk.message.content);
        }
    }
}
