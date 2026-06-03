<p align="center"><strong>A3 Platform</strong> build your AI systems
</p>
<br>

> [!WARNING]
> Early research prototype. The project is focused on testing the architecture concept, not on production-ready behavior. Future updates **will** contain **breaking changes**.

`Actor-based Agent Architecture` models an intelligent system as a network of
addressable, self-contained, isolated agents that exchange asynchronous messages.

A3 Platform is an experimental runtime platform for running addressable agents that communicate through messages.

---

## Quickstart

### Providers

Supported providers in manifests: `anthropic`, `deepseek`, `gemini`, `ollama`, `openai`, `openrouter`, `xai`.

| Provider | Manifest value | Environment |
| --- | --- | --- |
| Anthropic | `anthropic` | `ANTHROPIC_API_KEY` |
| DeepSeek | `deepseek` | `DEEPSEEK_API_KEY` |
| Gemini | `gemini` | `GEMINI_API_KEY` |
| Ollama | `ollama` | optional `OPENAI_API_KEY`, optional `OLLAMA_API_BASE_URL` |
| OpenAI | `openai` | `OPENAI_API_KEY`, optional `OPENAI_BASE_URL` |
| OpenRouter | `openrouter` | `OPENROUTER_API_KEY` |
| xAI | `xai` | `XAI_API_KEY` |

### Transport

Supported transport: `nats`.

| Transport | Manifest value | Environment |
| --- | --- | --- |
| NATS | `nats` | `NATS_URL` |

Create a local `.env` file from the example:
```shell
cp .env.example .env
```
### Run an agent

Run from source:
```shell
cargo run -p a3-cli -- run examples/github.json
```
Or run a release binary:
```shell
chmod +x ./a3-cli-macos-aarch64
./a3-cli-macos-aarch64 run examples/github.json
```
Run each agent in a separate terminal.

### Run the client

Start the web client with Docker Compose:
```shell
docker compose up -d
```
Open:
```shell
http://localhost:3001
```

## License

This repository is licensed under the [Apache-2.0 License](LICENSE).
