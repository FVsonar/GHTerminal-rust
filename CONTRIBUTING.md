# Contributing

Thanks for your interest in contributing to **GH-Terminal**!

## Prerequisites

- Rust stable (edition 2021)
- Node.js 20+
- Tauri CLI v2: `cargo install tauri-cli`

## Development

### Frontend

```bash
cd frontend
npm ci
npm run dev
```

### Desktop app (Tauri)

```bash
cd gh-terminal
cargo tauri dev
```

## Before opening a PR

- Format: `cargo fmt --all`
- Lint: `cargo clippy --workspace --all-targets -- -D warnings`
- Tests: `cargo test -p gh-protocol`
- Frontend build: `cd frontend && npm ci && npm run build`

## Commit messages

Use concise, imperative messages, for example:

- `Fix clippy warnings`
- `CI: adjust linux job`
