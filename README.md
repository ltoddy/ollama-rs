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

进行一次对话

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
