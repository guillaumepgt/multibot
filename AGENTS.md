# Agent Guidelines - MultiBot

This repository contains **MultiBot**, an autonomous multi-agent engineering platform.

## Working Directory
- Repository Root: `/workspace/users/guillaume_prigent44_gmail_com/workspace/multibot`
- Remote: `https://github.com/guillaumepgt/multibot.git`

## Key Files & Structure
- [`ROADMAP_ISSUES.md`](./ROADMAP_ISSUES.md): Full technical breakdown of all 20 issues across 5 milestones.
- `crates/multibot-core`: Core agent engine, domain models, LLM provider abstraction.
- `crates/multibot-server`: Axum web server and WebSocket handlers.
- `crates/multibot-tools`: Shell execution, filesystem sandbox, and Git operations.

## Commands Reference
- Run Cargo checks: `cargo check --workspace`
- Run Cargo tests: `cargo test --workspace`
- Run Rust linter: `cargo clippy --workspace --all-targets`
- Format code: `cargo fmt --all`

## GitHub Issues Workflow
- Always refer to `ROADMAP_ISSUES.md` before picking up an issue.
- Link pull requests and commits to issue numbers using `#<id>` or `closes #<id>`.
