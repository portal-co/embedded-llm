# embedded-llm

A Cargo workspace providing traits and types for interacting with Large Language Models (LLMs) via a generic, I/O-based interface. The core crate is `no_std` compatible, following the conventions of the `embedded-io` / `embedded-hal` ecosystem.

**Status:** Early / experimental. Version 0.1.0. Not published to crates.io (`publish = false`).

**License:** CC0-1.0

**Repository:** https://github.com/portal-co/embedded-llm

---

## Workspace structure

```
crates/
  embedded-llm/        # Core traits (no_std)
  embedded-llm-tools/  # Serde types for tool-use protocol (std)
```

---

## `embedded-llm` (core traits)

`#![no_std]`. Defines six traits that abstract over LLM interactions.

### Message types

```rust
pub enum MessageType { System, User, Assistant, Tool }
```

Messages are passed as `(MessageType, &str)` tuples.

### Synchronous traits

- **`LLMMut`** — starts an LLM session from a `&mut self` reference.
- **`LLMRef`** — starts an LLM session from a `&self` reference.
- **`LLMInstance`** — represents an active session. Has a `send` method that takes an iterator of `(MessageType, &str)` messages and returns `(MessageType, impl embedded_io::Read)`. The response is streamed back via `embedded_io::Read`.

### Async traits

- **`AsyncLLMMut`** — async version of `LLMMut`.
- **`AsyncLLMRef`** — async version of `LLMRef`.
- **`AsyncLLMInstance`** — async version of `LLMInstance`. `send` takes a `futures_core::Stream` of messages and returns `(MessageType, impl embedded_io_async::Read)`.

All traits bound on `embedded_io::ErrorType` for unified error handling.

### Dependencies

| Crate | Version |
|---|---|
| `embedded-io` | 0.7 |
| `embedded-io-async` | 0.7 |
| `futures-core` | 0.3.31 (no default features) |
| `either` | 1.15.0 (no default features) |

---

## `embedded-llm-tools`

Standard library crate. Provides serde-derived types for representing tool calls in the LLM conversation protocol.

### Types

- **`ToolCall`** — represents a tool invocation requested by the assistant. Fields: `id: String`, `name: String`, `arguments: String`.
- **`ToolMessage`** — represents the result of a tool call sent back to the model. Fields: `tool_call_id: String`, `content: String`.

These are serialized to/from JSON (via `serde_json`). They are intended to be used when `MessageType::Tool` is involved in a `send` call.

### Dependencies

| Crate | Version |
|---|---|
| `serde` | 1.0 (with `derive` feature) |
| `serde_json` | 1.0.138 |

---

## Design notes

- The core abstraction is session-based: call `start(system_prompt)` to get an `LLMInstance`, then call `send` with a sequence of messages to get a streamed response.
- Responses are returned as `impl embedded_io::Read` (or the async equivalent), so the caller reads bytes from the stream rather than receiving an allocated string.
- The `MessageType` returned alongside the response indicates what kind of content is in the stream (e.g., `Assistant` for a normal reply, `Tool` for a tool call).
- The `embedded-llm` crate itself contains no concrete implementations — it is purely a trait interface. Implementations are expected to live in downstream crates.
- `embedded-llm-tools` is a separate crate because it pulls in `serde`/`serde_json`, which are not `no_std`.
