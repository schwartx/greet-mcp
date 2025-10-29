# greet-mcp

A minimal example demonstrating Rust MCP SDK to Python library conversion.

## Libraries Used

- [Rust MCP SDK](https://github.com/modelcontextprotocol/rust-sdk/) - MCP server implementation
- [PyO3](https://github.com/PyO3/pyo3) - Rust to Python bindings
- [PyO3 Async Runtimes](https://github.com/PyO3/pyo3-async-runtimes) - Async runtime support

## Build and Installation

```bash
uv venv --python 3.13
maturin develop
```

## Claude Code Integration

```
claude mcp add -s project greet -- .venv/bin/python cli/start_server.py
```