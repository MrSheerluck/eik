# eik

An AI agent framework for Rust.

## Plan

eik follows the historical evolution of LLM APIs: raw completion to chat to streaming to tool calling to structured output to multi-agent orchestration.

## Tech stack

- **Rust**: async runtime via `tokio`
- **Providers**: OpenAI-compatible, Anthropic, and more (multi-provider)
- **Macros**: `#[tool]` proc macro for schema-from-signature tool definitions
- **MCP**: Model Context Protocol client/server support
- **Observability**: OpenTelemetry tracing

## License

MIT
