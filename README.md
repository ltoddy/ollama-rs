最近开始有在学习 AI 相关的知识, 比如深度学习, 线性代数, 微分, 偏微分, 以及在使用 deepseek 调试大模型,
并准备后面尝试自己做一个 AI Agent, 支持 MCP 能力和对话能力, 如果未来时间富裕,可以考虑做个智能工作流, 甚至自己训练一个模型

# 本地运行开源的大语言模型

deepseek 是开源的 LLM, 并且也在 ollama 平台上发布了, 本地需要安装 ollama, 安装方式参考官方网站: https://ollama.com/

然后安装之后, 执行下面命令就可以把 deepseek 下载到本地 (https://ollama.com/library/deepseek-r1):

> ollama pull deepseek-r1

## 引入本库

Cargo.toml

```toml
[dependencies]
ollama = { git = "https://github.com/ltoddy/ollama-rs" }
```

## 代码示例

### 进行一次对话

```rust
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
```

### 像聊天机器人那样对话

实际上大模型并没有记忆能力,每次和大模型聊天都需要将完整的对话历史发送给它。

```rust
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
```